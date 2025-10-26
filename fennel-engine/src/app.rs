use std::{
    fs,
    sync::{Arc, Mutex},
};

use fennel_core::{
    Window,
    events::{KeyboardEvent, WindowEventHandler},
    graphics::WindowConfig,
};
use serde::{Deserialize, Serialize};
use specs::{Builder, Dispatcher, DispatcherBuilder, WorldExt};

use crate::{
    ecs::{
        input::InputSystem,
        scene::SceneSystem,
        sprite::{HostPtr, RenderingSystem, Sprite, SpriteFactory},
    },
    events::KeyEvents,
    registry::ComponentRegistry,
    scenes::{ActiveScene, Scene},
};

/// The application struct which contains [`fennel_core::Window`], [`specs::World`] and `specs`
/// `Dispatcher`
pub struct App {
    /// Responsible for GFX and audio
    pub window: fennel_core::Window,
    /// ECS world
    pub world: specs::World,
    /// ECS dispatcher
    pub dispatcher: Dispatcher<'static, 'static>,
    /// Application scenes
    pub scenes: Vec<Scene>,
    /// Registry of component factories for scene drawing
    pub component_registry: ComponentRegistry,
    /// Current active scene
    pub active_scene: ActiveScene
}

/// Builder for [`App`]
#[derive(Default, Debug)]
pub struct AppBuilder {
    name: &'static str,
    dimensions: (u32, u32),
    config: &'static str,
    window_config: WindowConfig,
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

unsafe impl Send for App {}
unsafe impl Sync for App {}

#[async_trait::async_trait]
impl WindowEventHandler for App {
    fn update(&self, _window: &mut Window) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> anyhow::Result<()> {
        self.frame_tick();
        window.graphics.canvas.present();
        Ok(())
    }

    fn key_down_event(&self, _window: &mut Window, event: KeyboardEvent) -> anyhow::Result<()> {
        println!("{:?}", event.keycode);
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
        let ptr: *mut App = &mut self as *mut App;
        fennel_core::events::run(&mut self.window, unsafe { &mut *ptr as &mut App }).await;
        Ok(())
    }

    /// Evaluate systems
    pub fn frame_tick(&mut self) {
        let host_ptr = HostPtr(self as *mut App);
        self.world.insert(host_ptr);
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
        self.world.remove::<HostPtr>();
    }
}

impl AppBuilder {
    /// Create a new [`AppBuilder`]
    pub fn new() -> AppBuilder {
        AppBuilder {
            name: "",
            dimensions: (100, 100),
            config: "",
            window_config: WindowConfig {
                resizable: false,
                fullscreen: false,
                centered: false,
            },
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

    /// Builds an [`App`]
    pub fn build(self) -> anyhow::Result<App> {
        let resource_manager = Arc::new(Mutex::new(fennel_core::resources::ResourceManager::new()));
        let config_reader = fs::read(self.config)?;
        let config: Config = toml::from_slice(&config_reader)?;
        let graphics = fennel_core::graphics::Graphics::new(
            self.name.to_string(),
            self.dimensions,
            resource_manager.clone(),
            |graphics| {
                resource_manager.lock().unwrap().load_dir(config.assets_path.clone().into(), graphics).unwrap();
            },
            self.window_config
        );
        let window = fennel_core::Window::new(
            graphics.expect("failed to initialize graphics"),
            resource_manager,
        );
        let mut component_registry = ComponentRegistry::new();
        let mut world = specs::World::new();
        let mut dispatcher = DispatcherBuilder::new()
            .with_thread_local(RenderingSystem)
            .with(SceneSystem, "scene_system", &[])
            .with(InputSystem, "input_system", &[])
            .build();
        let mut scenes: Vec<Scene> = vec![];

        component_registry.register("sprite", Box::new(SpriteFactory));
        world.register::<Scene>();
        world.register::<Sprite>();
        world.insert(KeyEvents::default());

        for entry in fs::read_dir(config.scenes_path).expect("meow") {
            let scene_reader = fs::read(entry.unwrap().path()).expect("meow");
            let scene: Scene = ron::de::from_bytes(&scene_reader)?; 
            world.create_entity().with(scene.clone()).build();
            scenes.push(scene.clone());
        }

        dispatcher.setup(&mut world);

        Ok(App {
            window,
            world,
            dispatcher,
            scenes,
            component_registry,
            // assuming the initial scene name is `main`
            active_scene: ActiveScene { name: String::from("main"), loaded: false }
        })
    }
}
