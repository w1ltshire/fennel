use std::sync::{Arc, Mutex};

use fennel_core::{events, EventHandler};
use specs::WorldExt;

pub struct Runtime {
    pub window: fennel_core::Window,
    pub world: specs::World
}

#[derive(Default, Debug)]
pub struct RuntimeBuilder {
    name: &'static str,
    dimensions: (u32, u32)
}

impl Runtime {
    pub async fn run(&mut self, game_state: Box<dyn EventHandler>) -> anyhow::Result<()> {
        events::run(&mut self.window, game_state).await;
        Ok(())
    }
}

impl RuntimeBuilder {
    pub fn new() -> RuntimeBuilder {
        RuntimeBuilder {
            name: "",
            dimensions: (100, 100)
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
        let window = fennel_core::Window::new(graphics.expect("failed to initialize graphics"), resource_manager);

        Ok(Runtime {
            window,
            world: specs::World::new()
        })
    }
}
