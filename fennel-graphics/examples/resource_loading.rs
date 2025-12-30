use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use fennel_graphics::{
    Window,
    events::{self, WindowEventHandler},
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
                "assets/example.png".to_string(),
                (0.0, 0.0),
                0.0,
                false,
                false,
            )
            .expect("failed to draw an image");

        window.graphics.canvas.present();
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let resource_manager = Arc::new(Mutex::new(ResourceManager::new()));
    let graphics = graphics::Graphics::new(
        String::from("my cool window"),
        (500, 500),
        resource_manager.clone(),
        |graphics| {
            let mut resource_manager = match resource_manager.try_lock() {
                Ok(guard) => guard,
                Err(e) => return Err(anyhow::anyhow!("failed to lock resource_manager: {}", e)),
            };
            fennel_graphics::resources::load_dir(&mut resource_manager, PathBuf::from("assets"), graphics)?;
            Ok(())
        },
        graphics::WindowConfig::default(),
    )
    .expect("failed to create graphics");

    let mut window = Window::new(graphics);

    let handler: &'static mut dyn WindowEventHandler = {
        let boxed = Box::new(State);
        Box::leak(boxed) as &'static mut dyn WindowEventHandler
    };
    events::run(&mut window, handler, vec![])?;
    Ok(())
}
