use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use fennel_core::{
    EventHandler, Game,
    events::{self, KeyboardEvent},
    graphics,
    resources::ResourceManager,
};
use sdl3::pixels::Color;
use tokio::runtime::Handle;

struct State {}

#[async_trait::async_trait]
impl EventHandler for State {
    async fn update(&self, _game: &mut Game) -> anyhow::Result<()> {
        Ok(())
    }

    async fn draw(&self, game: &mut Game) -> anyhow::Result<()> {
        game.graphics.canvas.set_draw_color(Color::RGB(0, 0, 0));
        game.graphics.canvas.clear();
        game.graphics
            .draw_image("examples/example.png".to_string(), (0.0, 0.0))
            .expect("failed to draw an image");
        game.graphics.draw_text(
            String::from("hi"),
            (64.0, 64.0),
            String::from("examples/terminus.ttf"),
            Color::RGBA(255, 0, 0, 0),
            16.0,
        )?;
        game.graphics.draw_text(
            String::from("hi"),
            (64.0, 150.0),
            String::from("examples/terminus.ttf"),
            Color::RGBA(255, 0, 0, 0),
            128.0,
        )?;
        game.graphics.canvas.present();
        Ok(())
    }

    fn key_down_event(&self, game: &mut Game, event: KeyboardEvent) -> anyhow::Result<()> {
        println!("{:?}", event.keycode);
        tokio::task::block_in_place(move || {
            Handle::current().block_on(async move {
                game.audio
                    .play_audio(Path::new("examples/music.ogg"), false)
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
        String::from("my cool game"),
        (500, 500),
        resource_manager.clone(),
    );
    let mut game = Game::new(
        String::from("my cool game"),
        String::from("wiltshire"),
        graphics.unwrap(),
        resource_manager,
    );
    events::run(&mut game, Box::new(State {})).await;
}
