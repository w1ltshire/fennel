use std::{
    fs,
    sync::{Arc, Mutex},
};
use std::time::{Duration, Instant};
use anyhow::Context;
use fennel_core::{
    Window,
    events::{KeyboardEvent, WindowEventHandler},
    graphics::WindowConfig,
};
use log::warn;
use serde::{Deserialize, Serialize};
use specs::{Builder, Component, Dispatcher, DispatcherBuilder, World, WorldExt};
use crate::{
    ecs::sprite::{Sprite, SpriteFactory, SpriteRenderingSystem},
    events::KeyEvents,
    registry::{ComponentFactory, ComponentRegistry},
    renderer::{QueuedRenderingSystem, RenderQueue},
    scenes::{ActiveScene, Scene, SceneSystem},
};
use crate::camera::Camera;
use crate::tiles::{Tile, TileFactory, TileRenderingSystem};
use crate::time::{Tick, TickSystem};

/// The application struct which contains [`Window`], [`World`] and `specs`
/// `Dispatcher`
pub struct App {
    /// Responsible for GFX and audio
    pub window: Window,
    /// ECS world
    world: World,
    /// ECS dispatcher
    dispatcher: Dispatcher<'static, 'static>
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
        self.frame_tick()?;
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> anyhow::Result<()> {
        window.graphics.canvas.present();
        Ok(())
    }

    fn key_down_event(&mut self, _window: &mut Window, event: KeyboardEvent) -> anyhow::Result<()> {
        self.world.write_resource::<KeyEvents>().0.push(event);
        Ok(())
    }
}

/// A raw pointer wrapper to the application
pub struct HostPtr(pub *mut App);

unsafe impl Send for HostPtr {}
unsafe impl Sync for HostPtr {}

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
        fennel_core::events::run(&mut self.window, unsafe { &mut *ptr }, vec![]).await?;
        Ok(())
    }

    fn frame_tick(&mut self) -> anyhow::Result<()> {
        let now = Instant::now();
        let host_ptr = HostPtr(self as *mut App);
        self.world.insert(host_ptr);

        self.dispatcher.dispatch(&self.world);
        self.world.maintain();

        let elapsed = Instant::now().duration_since(now);
        let mut tick = self.world.write_resource::<Tick>();

        if elapsed.as_nanos() < tick.tick_rate as u128 {
            tick.total_elapsed_time += (tick.tick_rate - elapsed.as_nanos() as u64) as f64 / 1_000_000_000.0;
            std::thread::sleep(Duration::from_nanos(16_000_000 - elapsed.as_nanos() as u64));
        } else {
            tick.total_elapsed_time += (elapsed.as_nanos() as u64) as f64 / 1_000_000_000.0;
            warn!("cannot keep up, tick took {} > 16000000 nanoseconds", elapsed.as_nanos());
        }

        drop(tick);

        self.world.remove::<HostPtr>();
        Ok(())
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

    /// Register a system to insert into the dispatcher
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
            |graphics| -> anyhow::Result<()> {
                let mut resource_manager = match resource_manager.try_lock() {
                    Ok(guard) => guard,
                    Err(e) => return Err(anyhow::anyhow!("failed to lock resource_manager: {}", e)),
                };
                resource_manager
                    .load_dir(config.assets_path.clone().into(), graphics)
                    .context("failed to load resources from directory")?;
                Ok(())
            },
            self.window_config,
        ).context("failed to initialize graphics")?;
        let window = Window::new(
            graphics,
            resource_manager,
        );
        let mut dispatcher_builder = DispatcherBuilder::new()
            .with(SceneSystem, "scene_system", &[])
            .with(SpriteRenderingSystem, "sprite_rendering_system", &[])
            .with_thread_local(QueuedRenderingSystem)
            .with(TileRenderingSystem, "tile_rendering_system", &[])
            .with(TickSystem, "tick_system", &[]);

        for reg in self.dispatcher_config.drain(..) {
            dispatcher_builder = reg(&mut dispatcher_builder);
        }

        self.world.register::<Scene>();
        self.world.insert(KeyEvents::default());
        self.world.insert(RenderQueue::new());
        self.world.insert(Camera::new((0.0, 0.0), (0.0, 0.0)));
        self.world.insert(Tick {
            ticks: 0,
            tick_rate: 16_000_000,
            total_elapsed_time: 0.0
        });
        self = self.with_component::<Sprite, SpriteFactory>("sprite", SpriteFactory);
        self = self.with_component::<Tile, TileFactory>("tile", TileFactory);

        let mut scenes: Vec<Scene> = vec![];

        for entry in fs::read_dir(config.scenes_path)? {
            let scene_reader =
                fs::read(entry?.path())?;
            let scene: Scene = ron::de::from_bytes(&scene_reader)?;
            self.world.create_entity().with(scene.clone()).build();
            scenes.push(scene.clone());
        }

        self.world.insert(self.component_registry);
        self.world.insert(ActiveScene {
            name: String::from("main"),
            loaded: false,
        });

        Ok(App {
            window,
            world: self.world,
            dispatcher: dispatcher_builder.build()
        })
    }
}
