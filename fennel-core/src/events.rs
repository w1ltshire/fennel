//! `sdl3::event::Event`-driven main loop.

use sdl3::event::Event;
use std::time::{Duration, Instant};

use crate::Window;

use sdl3::{
    keyboard::{Keycode, Mod, Scancode},
    mouse::{MouseButton, MouseState, MouseWheelDirection},
};

pub struct KeyboardEvent {
    pub timestamp: u64,
    pub window_id: u32,
    pub keycode: Option<Keycode>,
    pub scancode: Option<Scancode>,
    pub keymod: Mod,
    pub repeat: bool,
    pub which: u32,
    pub raw: u16,
}

pub struct MouseMotionEvent {
    pub timestamp: u64,
    pub window_id: u32,
    pub which: u32,
    pub mousestate: MouseState,
    pub x: f32,
    pub y: f32,
    pub xrel: f32,
    pub yrel: f32,
}

pub struct MouseClickEvent {
    pub timestamp: u64,
    pub window_id: u32,
    pub which: u32,
    pub mouse_btn: MouseButton,
    pub clicks: u8,
    pub x: f32,
    pub y: f32,
}

pub struct MouseWheelEvent {
    pub timestamp: u64,
    pub window_id: u32,
    pub which: u32,
    pub x: f32,
    pub y: f32,
    pub direction: MouseWheelDirection,
    pub mouse_x: f32,
    pub mouse_y: f32,
}

#[async_trait::async_trait]
pub trait WindowEventHandler: Send + Sync {
    fn update(&mut self, _window: &mut Window) -> anyhow::Result<()>;
    fn draw(&mut self, _window: &mut Window) -> anyhow::Result<()>;

    fn key_down_event(&mut self, _window: &mut Window, _event: KeyboardEvent) -> anyhow::Result<()> {
        Ok(())
    }
    fn key_up_event(&mut self, _window: &mut Window, _event: KeyboardEvent) -> anyhow::Result<()> {
        Ok(())
    }
    fn mouse_motion_event(
        &mut self,
        _window: &mut Window,
        _event: MouseMotionEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn mouse_button_down_event(
        &mut self,
        _window: &mut Window,
        _event: MouseClickEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    fn mouse_button_up_event(
        &mut self,
        _window: &mut Window,
        _event: MouseClickEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }
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
pub async fn run(window: &mut Window, state: &'static mut dyn WindowEventHandler) {
    let mut event_pump = window.graphics.sdl_context.event_pump().unwrap();

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
                    )
                    .unwrap(),
                _ => {}
            }
        }
        // Update window logic and render.
        let _ = state.update(window);
        let _ = state.draw(window);

        // Simple frame limiter: aim for ~1 millisecond minimum frame time.
        let elapsed = Instant::now().duration_since(now).as_nanos() as u64;
        if elapsed < 999_999 {
            std::thread::sleep(Duration::from_nanos(999_999 - elapsed));
        }
    }
}
