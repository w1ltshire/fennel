use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use fennel_core::{
    events::{self, WindowEventHandler}, graphics, hooks::Hook, resources::ResourceManager, Window
};
use sdl3::pixels::Color;

struct State;
struct MyHook;

#[async_trait::async_trait]
impl WindowEventHandler for State {
    fn update(&mut self, _window: &mut Window) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> anyhow::Result<()> {
        window.graphics.canvas.set_draw_color(Color::RGB(0, 0, 0));
        window.graphics.canvas.clear();
        window.graphics.draw_text(
            String::from("hi"),
            (64.0, 64.0),
            String::from("Terminus"),
            Color::RGBA(255, 0, 0, 0),
            16.0,
        )?;
        window.graphics.canvas.present();
        Ok(())
    }
}

impl Hook for MyHook {
    fn update(&mut self) {
        print!("hi!");
    }

    fn name(&self) -> String {
        String::from("test hook")
    }
}

#[tokio::main]
async fn main() {
    let resource_manager = Arc::new(Mutex::new(ResourceManager::new()));
    let graphics = graphics::GraphicsBuilder::new()
        .window_name(String::from("game"))
        .dimensions((500, 500))
        .resource_manager(resource_manager.clone())
        .initializer(|graphics| {
            resource_manager
                .lock()
                .unwrap()
                .load_dir(PathBuf::from("assets"), graphics)
                .unwrap();
        })
        .build();
    let mut window = Window::new(graphics.unwrap(), resource_manager);

    let handler: &'static mut dyn WindowEventHandler = {
        let boxed = Box::new(State);
        Box::leak(boxed) as &'static mut dyn WindowEventHandler
    };
    
    events::run(&mut window, handler, vec![Box::new(MyHook)]).await;
}
