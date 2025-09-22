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
                    timestamp,
                    window_id,
                    keycode: Some(keycode),
                    scancode: Some(scancode),
                    keymod,
                    repeat,
                    which,
                    raw,
                } => state
                    .key_down_event(
                        game,
                        timestamp,
                        window_id,
                        Some(keycode),
                        Some(scancode),
                        keymod,
                        repeat,
                        which,
                        raw,
                    )
                    .unwrap(),

                Event::KeyUp {
                    timestamp,
                    window_id,
                    keycode: Some(keycode),
                    scancode: Some(scancode),
                    keymod,
                    repeat,
                    which,
                    raw,
                } => state
                    .key_up_event(
                        game,
                        timestamp,
                        window_id,
                        Some(keycode),
                        Some(scancode),
                        keymod,
                        repeat,
                        which,
                        raw,
                    )
                    .unwrap(),

                Event::MouseMotion {
                    timestamp,
                    window_id,
                    which,
                    mousestate,
                    x,
                    y,
                    xrel,
                    yrel,
                } => state
                    .mouse_motion_event(
                        game, timestamp, window_id, which, mousestate, x, y, xrel, yrel,
                    )
                    .unwrap(),

                Event::MouseButtonDown {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => state
                    .mouse_button_down_event(
                        game, timestamp, window_id, which, mouse_btn, clicks, x, y,
                    )
                    .unwrap(),

                Event::MouseButtonUp {
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                } => state
                    .mouse_button_up_event(
                        game, timestamp, window_id, which, mouse_btn, clicks, x, y,
                    )
                    .unwrap(),

                Event::MouseWheel {
                    timestamp,
                    window_id,
                    which,
                    x,
                    y,
                    direction,
                    mouse_x,
                    mouse_y,
                } => state
                    .mouse_wheel_event(
                        game, timestamp, window_id, which, x, y, direction, mouse_x, mouse_y,
                    )
                    .unwrap(),
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
