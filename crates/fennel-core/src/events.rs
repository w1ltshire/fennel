//! `sdl3::event::Event`-driven main loop.
//!
//! This module provides a main loop that processes SDL events, such as keyboard and mouse
//! inputs.
//! Ita lso defines various event structures and a trait for the main window event handler.
//! The main loop continuously polls for events and calls window event handler provided by user.
//!
//! ## Structures
//!
//! - `KeyboardEvent`: Represents keyboard events
//! - `MouseMotionEvent`: Represents mouse motion events
//! - `MouseClickEvent`: Represents mouse button click events
//! - `MouseWheelEvent`: Represents mouse wheel events
//!
//! ## Trait
//!
//! - `WindowEventHandler`: A trait that defines methods for handling events.
//! Implementing types must provide their own logic for updating the application
//! state and drawing to the window for each event type.
//! Call to `window.graphics.canvas.present()` is mandatory in the `draw()` function,
//! because it presents the window itself.
//!
//! ## Main Loop
//!
//! The main loop is executed through the `run` function, which manages event polling,
//! updates, and rendering. It handles quitting the application when an exit event is
//! received.

use log::debug;
use sdl3::event::Event;
use std::time::{Duration, Instant};

use crate::{Window, hooks::Hook};

use sdl3::{
    keyboard::{Keycode, Mod, Scancode},
    mouse::{MouseButton, MouseState, MouseWheelDirection},
};


/// Represents a keyboard event.
#[derive(Debug)]
pub struct KeyboardEvent {
    /// When the event happened (in nanos)
    pub timestamp: u64,
    /// The window with keyboard focus, if any
    pub window_id: u32,
    /// SDL virtual key code
    pub keycode: Option<Keycode>,
    /// SDL physical key code
    pub scancode: Option<Scancode>,
    /// Current key modifier (e.g. LALT)
    pub keymod: Mod,
    /// `true` if this is a key repeat
    pub repeat: bool,
    /// The keyboard instance id, or 0 if unknown or virtual
    pub which: u32,
    /// The platform dependent scancode for this event
    pub raw: u16,
}

pub struct MouseMotionEvent {
    /// When the event happened (in nanos)
    pub timestamp: u64,
    /// The window with mouse focus, if any
    pub window_id: u32,
    /// The mouse instance id, or 0 if unknown or virtual
    pub which: u32,
    /// The current button state
    pub mousestate: MouseState,
    /// X coordinate, relative to window
    pub x: f32,
    /// Y coordinate, relative to window
    pub y: f32,
    /// The relative motion in the X direction
    pub xrel: f32,
    /// The relative motion in the Y direction
    pub yrel: f32,
}

pub struct MouseClickEvent {
    /// When the event happened (in nanos)
    pub timestamp: u64,
    /// The window with mouse focus, if any
    pub window_id: u32,
    /// The mouse instance id in relative mode
    pub which: u32,
    /// The mouse button index
    pub mouse_btn: MouseButton,
    /// 1 for single-click, 2 for double-click, etc.
    pub clicks: u8,
    /// X coordinate, relative to window
    pub x: f32,
    /// Y coordinate, relative to window
    pub y: f32,
}

pub struct MouseWheelEvent {
    /// When the event happened (in nanos)
    pub timestamp: u64,
    /// The window with mouse focus, if any
    pub window_id: u32,
    /// The mouse instance id in relative mode
    pub which: u32,
    /// The amount scrolled horizontally, positive to the right and negative to the left
    pub x: f32,
    /// The amount scrolled vertically, positive away from the user and negative toward the user
    pub y: f32,
    /// Set to one of the MouseWheelDirection defines. When FLIPPED the values in X and Y will be opposite. Multiply by -1 to change them back
    pub direction: MouseWheelDirection,
    /// X coordinate, relative to window
    pub mouse_x: f32,
    /// Y coordinate, relative to window
    pub mouse_y: f32,
}

/// Trait that any type that is to be supplied to [`events::run`] should implement.
pub trait WindowEventHandler {
    /// Update the application logic
    fn update(&mut self, _window: &mut Window) -> anyhow::Result<()>;
    /// Draw the application AND OBLIGATORILY call [`Canvas::present`]
    fn draw(&mut self, _window: &mut Window) -> anyhow::Result<()>;

    /// Handle a key down event
    fn key_down_event(
        &mut self,
        _window: &mut Window,
        _event: KeyboardEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    /// Handle a key up event
    fn key_up_event(&mut self, _window: &mut Window, _event: KeyboardEvent) -> anyhow::Result<()> {
        Ok(())
    }
    /// Handle a mouse motion event
    fn mouse_motion_event(
        &mut self,
        _window: &mut Window,
        _event: MouseMotionEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    /// Handle a mouse button down event
    fn mouse_button_down_event(
        &mut self,
        _window: &mut Window,
        _event: MouseClickEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    /// Handle a mouse button up event
    fn mouse_button_up_event(
        &mut self,
        _window: &mut Window,
        _event: MouseClickEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    /// Handle a mouse wheel event
    fn mouse_wheel_event(
        &mut self,
        _window: &mut Window,
        _event: MouseWheelEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}

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
pub async fn run(
    window: &mut Window,
    state: &'static mut dyn WindowEventHandler,
    mut hooks: Vec<Box<dyn Hook>>,
) -> anyhow::Result<()> {
    let mut event_pump = window
        .graphics
        .sdl_context
        .event_pump()?;
    for hook in &mut hooks {
        debug!("preparing hook {}", hook.name());
        hook.prepare(&mut event_pump, window);
    }

    'running: loop {
        let now = Instant::now();

        // event_PUMP???? HOLY FUCK IS THAT A REFERENCE TO PSYCHOPOMP
        for event in event_pump.poll_iter() {
            for hook in &mut hooks {
                hook.handle(&event);
            }

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
                        window,
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
                    )?,

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
                        window,
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
                    )?,

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
                        window,
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
                    )?,

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
                        window,
                        MouseClickEvent {
                            timestamp,
                            window_id,
                            which,
                            mouse_btn,
                            clicks,
                            x,
                            y,
                        },
                    )?,

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
                        window,
                        MouseClickEvent {
                            timestamp,
                            window_id,
                            which,
                            mouse_btn,
                            clicks,
                            x,
                            y,
                        },
                    )?,

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
                        window,
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
                    )?,
                _ => {}
            }
        }
        // Update window logic and render.
        let _ = state.update(window);
        let _ = state.draw(window);

        for hook in &mut hooks {
            hook.update(&mut event_pump, window);
        }

        // Simple frame limiter: aim for ~1 millisecond minimum frame time.
        let elapsed = Instant::now().duration_since(now).as_nanos() as u64;
        if elapsed < 999_999 {
            tokio::time::sleep(Duration::from_nanos(999_999 - elapsed)).await;
        }
    }
    Ok(())
}
