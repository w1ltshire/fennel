use fennel_engine::{EventHandler, Game, events, graphics};
use sdl3::pixels::Color;

struct State {}

impl EventHandler for State {
    fn update(&self, _game: &mut Game) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(&self, game: &mut Game) -> anyhow::Result<()> {
        game.graphics.canvas.set_draw_color(Color::RGB(0, 0, 0));
        game.graphics.canvas.clear();
        game.graphics
            .draw_image("./examples/example.png".to_string(), (0.0, 0.0))
            .expect("failed to draw an image");
        game.graphics.canvas.present();
        Ok(())
    }

    fn key_down_event(
            &self,
            _game: &mut Game,
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
        Ok(())
    }
}

fn main() {
    let graphics = graphics::Graphics::new(String::from("my cool game"), (500, 500));
    let mut game = fennel_engine::Game::new(
        String::from("my cool game"),
        String::from("wiltshire"),
        graphics.unwrap(),
    );
    events::run(&mut game, Box::new(State {}));
}
