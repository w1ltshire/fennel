use kousa::{events, graphics, EventHandler, Game};
use sdl3::pixels::Color;

struct State {}

impl EventHandler for State {
    fn update(&self, _game: &mut Game) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(&self, game: &mut Game) -> anyhow::Result<()> {
        game.graphics.canvas.set_draw_color(Color::RGB(255, 0, 0));
        game.graphics.canvas.clear(); 
        game.graphics.canvas.present();
        Ok(())
    }
}

fn main() {
    let graphics = graphics::init(String::from("my cool game"), (500, 500));
    let mut game = kousa::Game::new(String::from("my cool game"), String::from("wiltshire"), graphics.unwrap());
    events::run(&mut game, Box::new(State {}));
}
