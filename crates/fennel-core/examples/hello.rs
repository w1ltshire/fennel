use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use fennel_core::{
    Window,
    events::{self, KeyboardEvent, WindowEventHandler},
    graphics,
};
use sdl3::pixels::Color;
use fennel_resources::manager::ResourceManager;

struct State;

impl WindowEventHandler for State {
    fn update(&mut self, _window: &mut Window) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> anyhow::Result<()> {
        window.graphics.canvas.set_draw_color(Color::RGB(0, 0, 0));
        window.graphics.canvas.clear();
        window
            .graphics
            .draw_image(
                "example".to_string(),
                (0.0, 0.0),
                90.0,
                false,
                false,
            )
            .expect("failed to draw an image");
        window.graphics.draw_text(
            String::from("hi"),
            (64.0, 64.0),
            String::from("Terminus"),
            (255, 0, 0),
            16.0,
        )?;
        window.graphics.draw_text(
            String::from("hi"),
            (64.0, 64.0),
            String::from("Terminus"),
            (255, 0, 0),
            64.0,
        )?;
        window.graphics.canvas.present();
        Ok(())
    }

    fn key_down_event(&mut self, _window: &mut Window, event: KeyboardEvent) -> anyhow::Result<()> {
        println!("{:?}", event.keycode);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let resource_manager = Arc::new(Mutex::new(ResourceManager::new()));
    let graphics = graphics::GraphicsBuilder::new()
        .window_name(String::from("game"))
        .dimensions((500, 500))
        .resource_manager(resource_manager.clone())
        .initializer(|graphics| {
            let mut resource_manager = match resource_manager.try_lock() {
                Ok(guard) => guard,
                Err(e) => return Err(anyhow::anyhow!("failed to lock resource_manager: {}", e)),
            };
            fennel_core::resources::load_dir(&mut resource_manager, PathBuf::from("assets"), graphics)?;
            Ok(())
        })
        .build().expect("failed to build graphics");

    let mut window = Window::new(graphics);

    // because events::run takes a `&'static mut dyn WindowEventHandler` as a second argument we
    // need to ensure event handler lives for the entirety of program's lifetime (huh why does it
    // even take a static reference i don't remember now, it's been like 2 months?)
    let handler: &'static mut dyn WindowEventHandler = {
        let boxed = Box::new(State);
        Box::leak(boxed) as &'static mut dyn WindowEventHandler
    };
    events::run(&mut window, handler, vec![])?;
    Ok(())
}
