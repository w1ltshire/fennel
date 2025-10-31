use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use fennel_core::{
    Window,
    events::{self, WindowEventHandler},
    graphics,
    resources::ResourceManager,
};
use sdl3::pixels::Color;

struct State;

#[async_trait::async_trait]
impl WindowEventHandler for State {
    fn update(&self, _window: &mut Window) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> anyhow::Result<()> {
        window.graphics.canvas.set_draw_color(Color::RGB(0, 0, 0));
        window.graphics.canvas.clear();
        window
            .graphics
            .draw_image("assets/example.png".to_string(), (0.0, 0.0), 0.0, false, false)
            .expect("failed to draw an image");

        window.graphics.canvas.present();
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let resource_manager = Arc::new(Mutex::new(ResourceManager::new()));
    let graphics = graphics::Graphics::new(
        String::from("my cool window"),
        (500, 500),
        resource_manager.clone(),
        |graphics| {
            // we bulk load resources here because either we'll have to deal with
            // ownership of `graphics` (the actual `let graphics`, not the closure
            // argument)
            // you may think of it like about just some initialization func
            resource_manager
                .lock()
                .unwrap()
                .load_dir(PathBuf::from("assets"), graphics)
                .unwrap();
        },
        graphics::WindowConfig { resizable: false, fullscreen: false, centered: false }
    )
    .unwrap();

    let mut window = Window::new(graphics, resource_manager.clone());

    // because events::run takes a `&'static mut dyn WindowEventHandler` as a second argument we
    // need to do this seemingly weird thing (while `app.rs` in fennel-engine has an ass solution
    // with raw pointers lmfao)
    let handler: &'static mut dyn WindowEventHandler = {
        let boxed = Box::new(State);
        Box::leak(boxed) as &'static mut dyn WindowEventHandler
    };
    events::run(&mut window, handler).await;
}
