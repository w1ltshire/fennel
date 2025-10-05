use std::sync::{Arc, Mutex};

use fennel_core::graphics::HasWindow;
use specs::{Dispatcher, DispatcherBuilder, WorldExt};

use crate::components::sprite::{RenderingSystem, Sprite};

pub struct Runtime {
    pub window: fennel_core::Window,
    pub world: specs::World,
    pub dispatcher: Dispatcher<'static, 'static>,
}

#[derive(Default, Debug)]
pub struct RuntimeBuilder {
    name: &'static str,
    dimensions: (u32, u32),
}

impl HasWindow for Runtime {
    fn window_mut(&mut self) -> &mut fennel_core::Window {
        &mut self.window
    }
}

impl Runtime {
    pub async fn run<H>(&mut self, game_state: H) -> anyhow::Result<()>
    where
        H: fennel_common::events::WindowEventHandler<Host = Runtime> + Send + Sync + 'static,
    {
        fennel_core::events::run(self, game_state).await;
        Ok(())
    }
}

impl RuntimeBuilder {
    pub fn new() -> RuntimeBuilder {
        RuntimeBuilder {
            name: "",
            dimensions: (100, 100),
        }
    }

    pub fn name(mut self, name: &'static str) -> RuntimeBuilder {
        self.name = name;
        self
    }

    pub fn dimensions(mut self, dimensions: (u32, u32)) -> RuntimeBuilder {
        self.dimensions = dimensions;
        self
    }

    pub fn build(&self) -> anyhow::Result<Runtime> {
        let resource_manager = Arc::new(Mutex::new(fennel_core::resources::ResourceManager::new()));
        let graphics = fennel_core::graphics::Graphics::new(
            self.name.to_string(),
            self.dimensions,
            resource_manager.clone(),
        );
        let window = fennel_core::Window::new(
            graphics.expect("failed to initialize graphics"),
            resource_manager,
        );
        let mut world = specs::World::new();
        let mut dispatcher = DispatcherBuilder::new()
            .with(RenderingSystem, "rendering_system", &[])
            .build();
        dispatcher.setup(&mut world);
        world.register::<Sprite>();

        Ok(Runtime {
            window,
            world,
            dispatcher,
        })
    }
}
