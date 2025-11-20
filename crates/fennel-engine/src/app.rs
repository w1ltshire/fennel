use std::{
    fs,
    sync::{Arc, Mutex},
};
use fennel_core::{
    Window,
    events::{KeyboardEvent, WindowEventHandler},
    graphics::WindowConfig,
};
use log::debug;
use serde::{Deserialize, Serialize};
use specs::{Builder, Component, DispatcherBuilder, World, WorldExt};

use crate::{
    ecs::sprite::{HostPtr, Sprite, SpriteFactory, SpriteRenderingSystem},
    events::KeyEvents,
    registry::{ComponentFactory, ComponentRegistry},
    renderer::{QueuedRenderingSystem, RenderQueue},
    scenes::{ActiveScene, Scene, SceneSystem},
};
use crate::threads::ThreadSafeDispatcher;

/// The application struct which contains [`Window`], [`World`] and `specs`
/// `Dispatcher`
pub struct App {
    /// Responsible for GFX and audio
    pub window: Window,
    /// ECS dispatcher
    pub dispatcher: ThreadSafeDispatcher,
    /// Application scenes
    pub scenes: Vec<Scene>,
    /// Registry of component factories for scene drawing
    pub component_registry: ComponentRegistry,
    /// Current active scene
    pub active_scene: ActiveScene,
}

type Reg = Box<
    dyn FnOnce(&mut DispatcherBuilder<'static, 'static>) -> DispatcherBuilder<'static, 'static>
    + Send,
>;

/// Builder for [`App`]
#[derive(Default)]
pub struct AppBuilder {
    name: &'static str,
    dimensions: (u32, u32),
    config: &'static str,
    window_config: WindowConfig,
    world: World,
    component_registry: ComponentRegistry,
    dispatcher_config: Vec<Reg>,
}

/// Application config defined by user
#[derive(Deserialize, Serialize, Debug)]
struct Config {
    /// Path to assets directory
    assets_path: String,
    /// Path to scenes directory
    scenes_path: String,
    /// First scene to display
    initial_scene: String,
}

impl WindowEventHandler for App {
    fn update(&mut self, _window: &mut Window) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> anyhow::Result<()> {
        self.frame_tick();
        window.graphics.canvas.present();
        Ok(())
    }

    fn key_down_event(&mut self, _window: &mut Window, event: KeyboardEvent) -> anyhow::Result<()> {
        debug!("pushing event to resource KeyEvents");
        self.dispatcher.world().write_resource::<KeyEvents>().0.push(event);
        Ok(())
    }
}

impl App {
    /// Runs the event loop, must be called only once, UB otherwise
    pub async fn run(mut self) -> anyhow::Result<()> {
        // you know what? fuck you and your borrow checker.
        // i'm 100% sure this app is single-threaded and its 11 pm
        // at the moment so i'm not gonna solve this shit in some
        // safe way
        // as long this works and doesn't SEGFAULTs i'll keep it
        //
        // TODO: make it safe
        let ptr: *mut App = &mut self as *mut App;
        fennel_core::events::run(&mut self.window, unsafe { &mut *ptr }, vec![]).await;
        Ok(())
    }

    /// Evaluate systems
    pub fn frame_tick(&mut self) {
        let host_ptr = HostPtr(self as *mut App);
        self.dispatcher.world().insert(host_ptr);
        // std::thread::sleep(Duration::from_millis(16));
        self.dispatcher.dispatch();
        self.dispatcher.world().maintain();
        self.dispatcher.world().remove::<HostPtr>();
    }
}

impl AppBuilder {
    /// Create a new [`AppBuilder`]
    pub fn new() -> AppBuilder {
        AppBuilder {
            name: "",
            dimensions: (100, 100),
            config: "",
            window_config: WindowConfig::default(),
            world: World::new(),
            component_registry: ComponentRegistry::new(),
            dispatcher_config: vec![],
        }
    }

    /// Set the window name
    pub fn name(mut self, name: &'static str) -> AppBuilder {
        self.name = name;
        self
    }

    /// Set the window dimensions
    pub fn dimensions(mut self, dimensions: (u32, u32)) -> AppBuilder {
        self.dimensions = dimensions;
        self
    }

    /// Set the application config
    pub fn config(mut self, path: &'static str) -> AppBuilder {
        self.config = path;
        self
    }

    pub fn register_system<S>(
        mut self,
        sys: S,
        name: &'static str,
        dep: &'static [&'static str],
    ) -> Self
    where
            for<'a> S: specs::System<'a> + Send + 'static,
    {
        let reg: Reg = Box::new(|builder: &mut DispatcherBuilder<'static, 'static>| {
            let b = std::mem::take(builder);
            b.with(sys, name, dep)
        });

        self.dispatcher_config.push(reg);
        self
    }

    /// Register a component in the ECS
    pub fn with_component<C: Component, F: ComponentFactory + 'static>(
        mut self,
        name: &'static str,
        component_factory: F,
    ) -> AppBuilder
    where
        <C as Component>::Storage: Default,
    {
        self.world.register::<C>();
        self.component_registry
            .register(name, Box::new(component_factory));
        self
    }

    /// Builds an [`App`]
    pub fn build(mut self) -> anyhow::Result<App> {
        let resource_manager = Arc::new(Mutex::new(fennel_core::resources::ResourceManager::new()));
        let config_reader = fs::read(self.config)?;
        let config: Config = toml::from_slice(&config_reader)?;
        let graphics = fennel_core::graphics::Graphics::new(
            self.name.to_string(),
            self.dimensions,
            resource_manager.clone(),
            |graphics| {
                resource_manager
                    .lock()
                    .expect("failed to acquire resource_manager lock")
                    .load_dir(config.assets_path.clone().into(), graphics)
                    .expect("failed to load resources from directory");
            },
            self.window_config,
        );
        let window = Window::new(
            graphics.expect("failed to initialize graphics"),
            resource_manager,
        );
        let mut dispatcher_builder = DispatcherBuilder::new()
            .with_thread_local(QueuedRenderingSystem)
            .with(SceneSystem, "scene_system", &[])
            .with(SpriteRenderingSystem, "sprite_rendering_system", &[]);

        for reg in self.dispatcher_config.drain(..) {
            dispatcher_builder = reg(&mut dispatcher_builder);
        }

        self.world.register::<Scene>();
        self.world.insert(KeyEvents::default());
        self.world.insert(RenderQueue::new());
        self = self.with_component::<Sprite, SpriteFactory>("sprite", SpriteFactory);

        let mut dispatcher = dispatcher_builder.build();

        let mut scenes: Vec<Scene> = vec![];

        for entry in fs::read_dir(config.scenes_path).expect("meow") {
            let scene_reader =
                fs::read(entry.expect("failed to read directory").path()).expect("meow");
            let scene: Scene = ron::de::from_bytes(&scene_reader)?;
            self.world.create_entity().with(scene.clone()).build();
            scenes.push(scene.clone());
        }

        dispatcher.setup(&mut self.world);

        Ok(App {
            window,
            dispatcher: ThreadSafeDispatcher::new(dispatcher, self.world),
            scenes,
            component_registry: self.component_registry,
            // assuming the initial scene name is `main`
            active_scene: ActiveScene {
                name: String::from("main"),
                loaded: false,
            },
        })
    }
}