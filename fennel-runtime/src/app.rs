use std::fs;
use std::time::{Duration, Instant};
use log::{debug, error, warn};
use serde::{Deserialize, Serialize};
use specs::{Builder, Component, Dispatcher, DispatcherBuilder, World, WorldExt};
use fennel_plugins::Plugin;
use fennel_registry::{ComponentFactory, ComponentRegistry};
use crate::scenes::{ActiveScene, Scene, SceneSystem};
use crate::time::{Tick, TickSystem};

type SystemRegistration = Box<
    dyn FnOnce(&mut DispatcherBuilder<'static, 'static>) -> DispatcherBuilder<'static, 'static>
    + Send,
>;

/// The application struct which contains [`World`] and `specs`
/// `Dispatcher`
pub struct App {
    /// ECS world
    world: World,
    /// ECS dispatcher
    dispatcher: Dispatcher<'static, 'static>,
    plugins: Vec<Box<dyn Plugin + 'static + Send + Sync>>,
}

/// Builder for [`App`]
#[derive(Default)]
pub struct AppBuilder {
    config: &'static str,
    world: World,
    component_registry: ComponentRegistry,
    dispatcher_builder: DispatcherBuilder<'static, 'static>,
    dispatcher_config: Vec<SystemRegistration>,
    plugins: Vec<Box<dyn Plugin + Send + Sync>>,
}

/// Application config defined by user
#[derive(Deserialize, Serialize, Debug)]
struct Config {
    /// Path to scenes directory
    scenes_path: String,
    /// First scene to display
    initial_scene: String,
}

impl App {
    /// Runs the event loop, must be called only once, UB otherwise
    pub fn run(mut self) -> anyhow::Result<()> {
        loop {
            self.frame_tick()?;
        }
    }

    fn frame_tick(&mut self) -> anyhow::Result<()> {
        let now = Instant::now();

        self.dispatcher.dispatch(&self.world);
        self.world.maintain();

        let elapsed = Instant::now().duration_since(now);
        let mut tick = self.world.write_resource::<Tick>();

        self.plugins.iter_mut().for_each(|plugin| {
            plugin.update(1.0).unwrap_or_else(|e| { // 1.0 is a temporary value
                error!("failed to update plugin: {e}");
            });
        });

        if elapsed.as_nanos() < tick.tick_rate as u128 {
            tick.total_elapsed_time += (tick.tick_rate - elapsed.as_nanos() as u64) as f64 / 1_000_000_000.0;
            std::thread::sleep(Duration::from_nanos(16_000_000 - elapsed.as_nanos() as u64));
        } else {
            tick.total_elapsed_time += (elapsed.as_nanos() as u64) as f64 / 1_000_000_000.0;
            warn!("cannot keep up, tick took {} > 16000000 nanoseconds", elapsed.as_nanos());
        }

        drop(tick);
        Ok(())
    }
}

impl AppBuilder {
    /// Create a new [`AppBuilder`]
    pub fn new() -> AppBuilder {
        AppBuilder {
            config: "",
            world: World::new(),
            component_registry: ComponentRegistry::new(),
            dispatcher_builder: DispatcherBuilder::new(),
            dispatcher_config: Vec::new(),
            plugins: Vec::new(),
        }
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
        debug!("registering system {name} with deps {dep:?}");
        let reg: SystemRegistration = Box::new(|builder: &mut DispatcherBuilder<'static, 'static>| {
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

    /// Register a plugin
    pub fn with_plugin<C: Plugin + 'static + Send + Sync>(mut self, plugin: C) -> AppBuilder {
        self.plugins.push(Box::new(plugin));
        self
    }

    /// Builds an [`App`]
    pub fn build(mut self) -> anyhow::Result<App> {
        let config_reader = fs::read(self.config)?;
        let config: Config = toml::from_slice(&config_reader)?;
        self.dispatcher_builder.add(SceneSystem, "scene_system", &[]);
        //self.dispatcher_builder.add(SpriteRenderingSystem, "sprite_rendering_system", &[]);
        self.dispatcher_builder.add(TickSystem, "tick_system", &[]);

        self.world.register::<Scene>();
        self.world.insert(Tick {
            ticks: 0,
            tick_rate: 16_000_000,
            total_elapsed_time: 0.0
        });
        //self = self.with_component::<Sprite, SpriteFactory>("sprite", SpriteFactory);

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

        self.plugins.iter_mut().for_each(|plugin| {
            plugin.prepare(&mut self.dispatcher_builder, &mut self.world).unwrap_or_else(|e| {
                error!("failed to prepare plugin: {e}");
            });
        });

        for reg in self.dispatcher_config.drain(..) {
            self.dispatcher_builder = reg(&mut self.dispatcher_builder);
        }

        #[cfg(debug_assertions)]
        self.dispatcher_builder.print_par_seq();

        Ok(App {
            world: self.world,
            dispatcher: self.dispatcher_builder.build(),
            plugins: self.plugins,
        })
    }
}