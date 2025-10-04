use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use fennel_core::{
    EventHandler, Window,
    events::{self, KeyboardEvent},
    graphics,
    resources::ResourceManager,
};
use sdl3::pixels::Color;
use tokio::runtime::Handle;

struct State {}

#[async_trait::async_trait]
impl EventHandler for State {
    async fn update(&self, _window: &mut Window) -> anyhow::Result<()> {
        Ok(())
    }

    async fn draw(&self, window: &mut Window) -> anyhow::Result<()> {
        window.graphics.canvas.set_draw_color(Color::RGB(0, 0, 0));
        window.graphics.canvas.clear();
        window.graphics
            .draw_image("assets/example.png".to_string(), (0.0, 0.0))
            .expect("failed to draw an image");
        window.graphics.draw_text(
            String::from("hi"),
            (64.0, 64.0),
            String::from("assets/terminus.ttf"),
            Color::RGBA(255, 0, 0, 0),
            16.0,
        )?;
        window.graphics.draw_text(
            String::from("hi"),
            (64.0, 150.0),
            String::from("assets/terminus.ttf"),
            Color::RGBA(255, 0, 0, 0),
            128.0,
        )?;
        window.graphics.canvas.present();
        Ok(())
    }

    fn key_down_event(&self, window: &mut Window, event: KeyboardEvent) -> anyhow::Result<()> {
        println!("{:?}", event.keycode);
        tokio::task::block_in_place(move || {
            Handle::current().block_on(async move {
                window.audio
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
    let graphics = graphics::Graphics::new(
        String::from("my cool window"),
        (500, 500),
        resource_manager.clone(),
    );
    let mut window = Window::new(
        graphics.unwrap(),
        resource_manager,
    );
    events::run(&mut window, Box::new(State {})).await;
}
