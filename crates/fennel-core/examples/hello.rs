use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use anyhow::Context;
use fennel_core::{
    Window,
    events::{self, KeyboardEvent, WindowEventHandler},
    graphics,
    resources::ResourceManager,
};
use sdl3::pixels::Color;
use tokio::runtime::Handle;

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

    fn key_down_event(&mut self, window: &mut Window, event: KeyboardEvent) -> anyhow::Result<()> {
        println!("{:?}", event.keycode);
        tokio::task::block_in_place(move || {
            Handle::current().block_on(async move {
                window
                    .audio
                    .play_audio(Path::new("assets/music.ogg"), false)
                    .await
                    .expect("failed to play audio");
                window
                    .audio
                    .play_audio(Path::new("assets/440.wav"), false)
                    .await
                    .expect("failed to play audio");
            })
        });
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
            resource_manager
                .load_dir(PathBuf::from("assets"), graphics)
                .context("failed to load assets from directory")?;
            Ok(())
        })
        .build().expect("failed to build graphics");

    let mut window = Window::new(
        graphics,
        resource_manager,
    );

    // because events::run takes a `&'static mut dyn WindowEventHandler` as a second argument we
    // need to do this seemingly weird thing (while `app.rs` in fennel-engine has an ass solution
    // with raw pointers lmfao)
    let handler: &'static mut dyn WindowEventHandler = {
        let boxed = Box::new(State);
        Box::leak(boxed) as &'static mut dyn WindowEventHandler
    };
    events::run(&mut window, handler, vec![]).await?;
    Ok(())
}
