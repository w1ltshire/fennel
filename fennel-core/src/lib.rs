//! Core window-related API for Fennel engine
use std::sync::{Arc, Mutex};

use crate::{
    audio::Audio,
    events::{KeyboardEvent, MouseClickEvent, MouseMotionEvent, MouseWheelEvent},
    graphics::Graphics,
    resources::ResourceManager,
};

/// Audio playback
pub mod audio;
/// Handling keyboard, window, mouse and other events
pub mod events;
/// Rendering layer and all the related things
pub mod graphics;
/// Resource management
pub mod resources;
/// Tests
mod tests;

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

/// Main window struct
///
/// Holds basic metadata and a reference to the graphics subsystem.
/// User must create a [`Window`] in order to feed it to the EventHandler
pub struct Window {
    /// Graphics subsystem used to render frames.
    pub graphics: Graphics,
    /// Audio subsystem
    pub audio: Audio,
    /// Resource manager
    pub resource_manager: Arc<Mutex<ResourceManager>>
}

impl Window {
    /// Create a new [`Window`] instance.
    ///
    /// # Parameters
    /// - `name`: title of the window
    /// - `graphics`: initialized graphics subsystem
    ///
    /// # Returns
    /// A [`Window`] instance ready to be used by an [`EventHandler`].
    pub fn new(
        graphics: Graphics,
        resource_manager: Arc<Mutex<ResourceManager>>,
    ) -> Window {
        Window {
            graphics,
            audio: Audio::new(),
            resource_manager,
        }
    }
}

/// Trait that must be implemented by your window state struct
/// - `update` is called to advance window state (physics, AI, input processing).
/// - `draw` is called to render the current state using `window.graphics`.
#[async_trait::async_trait]
pub trait EventHandler {
    /// Updates the window state.
    ///
    /// This method should contain the logic for updating the window state.
    ///
    /// # Arguments
    ///
    /// - `window` - A mutable reference to the game state.
    ///
    /// # Returns
    ///
    /// - `Result<()>` - `()` if everythings fine, otherwise you should return an error if
    /// something failed in your logics
    async fn update(&self, window: &mut Window) -> anyhow::Result<()>;

    /// Draws the window
    ///
    /// This method should contain the logic for drawing on the window's canvas
    ///
    /// # Arguments
    ///
    /// - `window` - A mutable reference to the game state.
    ///
    /// # Returns
    ///
    /// - `Result<()>` - `()` if everythings fine, otherwise you should return an error if
    /// something failed in your logics
    async fn draw(&self, window: &mut Window) -> anyhow::Result<()>;

    /// Handles a key‑down event.
    ///
    /// # Parameters
    /// - `window` – Mutable `Window` reference.
    /// - `timestamp` – Event timestamp.
    /// - `window_id` – Window identifier.
    /// - `keycode` – Optional keycode of the pressed key.
    /// - `scancode` – Optional scancode of the pressed key.
    fn key_down_event(&self, _window: &mut Window, _event: KeyboardEvent) -> anyhow::Result<()> {
        Ok(())
    }

    /// Handles a key‑up event. Parameters mirror `key_down_event`
    fn key_up_event(&self, _window: &mut Window, _event: KeyboardEvent) -> anyhow::Result<()> {
        Ok(())
    }

    /// Handles mouse‑motion events.
    ///
    /// # Parameters
    /// - `window` – Mutable `Window` reference.
    /// - `timestamp` – Event timestamp.
    /// - `window_id` – Window identifier.
    /// - `x`, `y` – Absolute cursor coordinates.
    /// - `xrel`, `yrel` – Relative motion since the previous event.
    fn mouse_motion_event(&self, _window: &mut Window, _event: MouseMotionEvent) -> anyhow::Result<()> {
        Ok(())
    }

    /// Handles a mouse‑button‑down event.
    ///
    /// # Parameters
    /// - `window` – Mutable `Window` reference.
    /// - `timestamp` – Event timestamp.
    /// - `window_id` – Window identifier.
    /// - `mouse_btn` – Button that was pressed.
    /// - `clicks` – Number of consecutive clicks.
    /// - `x`, `y` – Cursor position at the time of the press.
    fn mouse_button_down_event(
        &self,
        _window: &mut Window,
        _event: MouseClickEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    /// Handles a mouse‑button‑up event. Parameters mirror
    /// `mouse_button_down_event`
    fn mouse_button_up_event(
        &self,
        _window: &mut Window,
        _event: MouseClickEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    /// Handles a mouse‑wheel (scroll) event.
    ///
    /// # Parameters
    /// - `window` – Mutable `Window` reference.
    /// - `timestamp` – Event timestamp.
    /// - `window_id` – Window identifier.
    /// - `x`, `y` – Scroll amount along each axis.
    /// - `direction` – Scroll direction probably i don't know.
    /// - `mouse_x`, `mouse_y` – Cursor position when the wheel event occurred.
    fn mouse_wheel_event(&self, _window: &mut Window, _event: MouseWheelEvent) -> anyhow::Result<()> {
        Ok(())
    }
}
