use std::collections::HashMap;
use std::fs;
use std::time::{Duration, Instant};
use log::{debug, error, warn};
use serde::{Deserialize, Serialize};
use specs::{Builder, Component, Dispatcher, DispatcherBuilder, World, WorldExt};
use fennel_core::graphics::{Drawable, Sprite};
use fennel_plugins::Plugin;
use crate::{
    ecs::sprite::{SpriteFactory, SpriteRenderingSystem},
    events::KeyEvents,
    registry::{ComponentFactory, ComponentRegistry},
    scenes::{ActiveScene, Scene, SceneSystem},
};
use crate::camera::Camera;
use crate::renderer::{QueuedRenderingSystem, RenderQueue};
use crate::tiles::{Tile, TileFactory, TileRenderingSystem};
use crate::time::{Tick, TickSystem};

/// The application struct which contains [`World`] and `specs`
/// `Dispatcher`
pub struct App {
    /// ECS world
    world: World,
    /// ECS dispatcher
    dispatcher: Dispatcher<'static, 'static>,
    plugins: Vec<Box<dyn Plugin + 'static + Send + Sync>>,
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
    world: World,
    component_registry: ComponentRegistry,
    dispatcher_config: Vec<Reg>,
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
    pub async fn run(mut self) -> anyhow::Result<()> {
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
            name: "",
            dimensions: (100, 100),
            config: "",
            world: World::new(),
            component_registry: ComponentRegistry::new(),
            dispatcher_config: Vec::new(),
            plugins: Vec::new(),
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

    /// Register a plugin
    pub fn with_plugin<C: Plugin + 'static + Send + Sync>(mut self, plugin: C) -> AppBuilder {
        self.plugins.push(Box::new(plugin));
        self
    }

    /// Builds an [`App`]
    pub fn build(mut self) -> anyhow::Result<App> {
        let config_reader = fs::read(self.config)?;
        let config: Config = toml::from_slice(&config_reader)?;
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

        let (render_tx, render_rx) = kanal::unbounded::<Vec<Drawable>>();

        self.world.insert(self.component_registry);
        self.world.insert(ActiveScene {
            name: String::from("main"),
            loaded: false,
        });
        self.world.insert(RenderQueue::new());
        self.world.insert(render_rx);
        self.world.insert(render_tx);

        self.plugins.iter_mut().for_each(|plugin| {
            let mut dependencies = HashMap::new();
            plugin.resource_dependencies().iter().for_each(|resource| {
                debug!("plugin `{}` requested resource {:?} with key `{}`, trying to fetch...", plugin.name(), resource.1, resource.0);
                let fetched_resource = unsafe { self.world.try_fetch_internal(resource.1.clone()) };
                match fetched_resource {
                    Some(fetched_resource) => {
                        dependencies.insert(resource.0.to_string(), fetched_resource);
                        debug!("got resource {:?} for plugin {}", resource, plugin.name());
                    }
                    None => {
                        error!("failed to fetch resource {:?} for plugin {}, plugin not loaded", resource, plugin.name());
                        return;
                    }
                }
            });
            plugin.prepare(dependencies).unwrap_or_else(|e| {
                error!("failed to prepare plugin: {e}");
            });
        });

        Ok(App {
            world: self.world,
            dispatcher: dispatcher_builder.build(),
            plugins: self.plugins,
        })
    }
}
