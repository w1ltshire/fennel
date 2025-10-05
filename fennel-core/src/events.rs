//! `sdl3::event::Event`-driven main loop.

use fennel_common::events::{
    KeyboardEvent, MouseClickEvent, MouseMotionEvent, MouseWheelEvent, WindowEventHandler,
};
use sdl3::event::Event;
use std::time::{Duration, Instant};

use crate::graphics::HasWindow;

/// Run the main loop.
///
/// Parameters:
/// - `window`: mutable reference to your `Window`. required because [`Window`] contains required
///   gfx variables
/// - `state`: boxed implementation of [`EventHandler`] that receives update/draw calls
///
/// Behavior:
/// - Polls SDL events each frame and breaks the loop on `Event::Quit`.
/// - Calls `state.update(window)` then `state.draw(game)` each frame.
///
/// Example:
/// ```ignore
/// let mut window = Window::new("cool title".into(), "cool author".into(), graphics);
/// events::run(&mut window, Box::new(my_handler));
/// ```
pub async fn run<H, Host>(host: &mut Host, state: H)
where
    H: WindowEventHandler<Host = Host> + Send + Sync + 'static,
    Host: HasWindow,
{
    let mut event_pump = host.window_mut().graphics.sdl_context.event_pump().unwrap();

    'running: loop {
        let now = Instant::now();

        // event_PUMP???? HOLY FUCK IS THAT A REFERENCE TO PSYCHOPOMP
        for event in event_pump.poll_iter() {
            match event {
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
                        host,
                        KeyboardEvent {
                            timestamp,
                            window_id,
                            keycode: Some(keycode),
                            scancode: Some(scancode),
                            keymod,
                            repeat,
                            which,
                            raw,
                        },
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
                        host,
                        KeyboardEvent {
                            timestamp,
                            window_id,
                            keycode: Some(keycode),
                            scancode: Some(scancode),
                            keymod,
                            repeat,
                            which,
                            raw,
                        },
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
                        host,
                        MouseMotionEvent {
                            timestamp,
                            window_id,
                            which,
                            mousestate,
                            x,
                            y,
                            xrel,
                            yrel,
                        },
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
                        host,
                        MouseClickEvent {
                            timestamp,
                            window_id,
                            which,
                            mouse_btn,
                            clicks,
                            x,
                            y,
                        },
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
                        host,
                        MouseClickEvent {
                            timestamp,
                            window_id,
                            which,
                            mouse_btn,
                            clicks,
                            x,
                            y,
                        },
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
                        host,
                        MouseWheelEvent {
                            timestamp,
                            window_id,
                            which,
                            x,
                            y,
                            direction,
                            mouse_x,
                            mouse_y,
                        },
                    )
                    .unwrap(),
                _ => {}
            }
        }

        // Update window logic and render.
        let _ = state.update(host);
        let _ = state.draw(host);

        // Simple frame limiter: aim for ~1 millisecond minimum frame time.
        let elapsed = Instant::now().duration_since(now).as_nanos() as u64;
        if elapsed < 999_999 {
            std::thread::sleep(Duration::from_nanos(999_999 - elapsed));
        }
    }
}
