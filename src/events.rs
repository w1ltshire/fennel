//! `sdl3::event::Event`-driven main loop.

use sdl3::event::Event;
use std::time::{Duration, Instant};

use crate::{EventHandler, Game};

/// Run the main loop.
///
/// Parameters:
/// - `game`: mutable reference to your `Game`. required because [`Game`] contains required
///   gfx variables
/// - `state`: boxed implementation of [`EventHandler`] that receives update/draw calls
///
/// Behavior:
/// - Polls SDL events each frame and breaks the loop on `Event::Quit`.
/// - Calls `state.update(game)` then `state.draw(game)` each frame.
///
/// Example:
/// ```no_run
/// let mut game = Game::new("cool title".into(), "cool author".into(), graphics);
/// run(&mut game, Box::new(my_handler));
/// ```
pub fn run(game: &mut Game, state: Box<dyn EventHandler>) {
    'running: loop {
        let now = Instant::now();

        // event_PUMP???? HOLY FUCK IS THAT A REFERENCE TO PSYCHOPOMP
        for event in game.graphics.sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                // TODO: let the user register their own handlers
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(_keycode),
                    which: _,
                    ..
                } => {}
                Event::KeyUp {
                    keycode: Some(_keycode),
                    which: _,
                    ..
                } => {}
                _ => {}
            }
        }

        // Update game logic and render.
        let _ = state.update(game);
        let _ = state.draw(game);

        // Simple frame limiter: aim for ~1 millisecond minimum frame time.
        let elapsed = Instant::now().duration_since(now).as_nanos() as u64;
        if elapsed < 999_999 {
            std::thread::sleep(Duration::from_nanos(999_999 - elapsed));
        }
    }
}
