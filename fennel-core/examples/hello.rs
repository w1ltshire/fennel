use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use fennel_core::{
    Window,
    events::{self, KeyboardEvent, WindowEventHandler},
    graphics,
    resources::ResourceManager,
};
use sdl3::pixels::Color;
use tokio::runtime::Handle;

struct State;

#[async_trait::async_trait]
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
                90.0,
                false,
                false,
            )
            .expect("failed to draw an image");
        window.graphics.draw_text(
            String::from("hi"),
            (64.0, 64.0),
            String::from("Terminus"),
            Color::RGBA(255, 0, 0, 0),
            16.0,
        )?;
        window.graphics.draw_text(
            String::from("hi"),
            (64.0, 64.0),
            String::from("Terminus"),
            Color::RGBA(255, 0, 0, 0),
            64.0,
        )?;
        window.graphics.canvas.present();
        Ok(())
    }

    fn key_down_event(&mut self, window: &mut Window, event: KeyboardEvent) -> anyhow::Result<()> {
        println!("{:?}", event.keycode);
        tokio::task::block_in_place(move || {
            Handle::current().block_on(async move {
                window
                    .audio
                    .play_audio(Path::new("assets/music.ogg"), false)
                    .await
                    .unwrap();
            })
        });
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let resource_manager = Arc::new(Mutex::new(ResourceManager::new()));
    let graphics = graphics::GraphicsBuilder::new()
        .window_name(String::from("game"))
        .dimensions((500, 500))
        .resource_manager(resource_manager.clone())
        // we bulk load resources here because either we'll have to deal with
        // ownership of `graphics` (the actual `let graphics`, not the closure
        // argument)
        // you may think of it like about just some initialization func
        .initializer(|graphics| {
            resource_manager
                .lock()
                .unwrap()
                .load_dir(PathBuf::from("assets"), graphics)
                .unwrap();
        })
        .build();
    let mut window = Window::new(graphics.unwrap(), resource_manager);

    // because events::run takes a `&'static mut dyn WindowEventHandler` as a second argument we
    // need to do this seemingly weird thing (while `app.rs` in fennel-engine has an ass solution
    // with raw pointers lmfao)
    let handler: &'static mut dyn WindowEventHandler = {
        let boxed = Box::new(State);
        Box::leak(boxed) as &'static mut dyn WindowEventHandler
    };
    events::run(&mut window, handler).await;
}
