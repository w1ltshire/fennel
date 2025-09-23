use std::path::Path;

use fennel_engine::{EventHandler, Game, events, graphics};
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
            .draw_image(
                "examples/example.png".to_string(),
                (0.0, 0.0),
                &mut game.resource_manager,
            )
            .expect("failed to draw an image");
        game.graphics.canvas.present();
        Ok(())
    }

    fn key_down_event(
        &self,
        game: &mut Game,
        _timestamp: u64,
        _window_id: u32,
        keycode: Option<sdl3::keyboard::Keycode>,
        _scancode: Option<sdl3::keyboard::Scancode>,
        _keymod: sdl3::keyboard::Mod,
        _repeat: bool,
        _which: u32,
        _raw: u16,
    ) -> anyhow::Result<()> {
        println!("{:?}", keycode);
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
    let graphics = graphics::Graphics::new(String::from("my cool game"), (500, 500));
    let mut game = fennel_engine::Game::new(
        String::from("my cool game"),
        String::from("wiltshire"),
        graphics.unwrap(),
    );
    events::run(&mut game, Box::new(State {})).await;
}
