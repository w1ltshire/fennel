//! A small 2D game framework I'm building just-for-fun and to learn Rust a little bit deeper

use std::sync::{Arc, Mutex};

use sdl3::{
    keyboard::{Keycode, Mod, Scancode},
    mouse::{MouseButton, MouseState, MouseWheelDirection},
};

use crate::{audio::Audio, graphics::Graphics, resources::ResourceManager};

/// Audio playback
pub mod audio;
/// Entity-component-system
pub mod ecs;
/// Handling keyboard, window, mouse and other events
pub mod events;
/// Rendering layer and all the related things
pub mod graphics;
/// Resource management
pub mod resources;
/// Tests
mod tests;

unsafe impl Send for Game {}
unsafe impl Sync for Game {}

/// Main game struct
///
/// Holds basic metadata and a reference to the graphics subsystem.
/// User must create a [`Game`] in order to feed it to the EventHandler
pub struct Game {
    /// Human-readable game title.
    pub name: String,
    /// Author or owner of the game.
    pub author: String,
    /// Graphics subsystem used to render frames.
    pub graphics: Graphics,
    /// Audio subsystem
    pub audio: Audio,
    /// Resource management
    pub resource_manager: Arc<Mutex<ResourceManager>>,
}

impl Game {
    /// Create a new [`Game`] instance.
    ///
    /// # Parameters
    /// - `name`: title of the game
    /// - `author`: author/owner name
    /// - `graphics`: initialized graphics subsystem
    ///
    /// # Returns
    /// A [`Game`] instance ready to be used by an [`EventHandler`].
    pub fn new(
        name: String,
        author: String,
        graphics: Graphics,
        resource_manager: Arc<Mutex<ResourceManager>>,
    ) -> Game {
        let audio = Audio::new();
        Game {
            name,
            author,
            graphics,
            audio,
            resource_manager,
        }
    }
}

/// Trait that must be implemented by user's game state struct
/// - `update` is called to advance game state (physics, AI, input processing).
/// - `draw` is called to render the current state using `game.graphics`.
#[async_trait::async_trait]
pub trait EventHandler {
    /// Updates the game state.
    ///
    /// This method should contain the logic for updating the game state.
    ///
    /// # Arguments
    ///
    /// - `game` - A mutable reference to the game state.
    ///
    /// # Returns
    ///
    /// - `Result<()>` - `()` if everythings fine, otherwise you should return an error if
    /// something failed in your logics
    async fn update(&self, game: &mut Game) -> anyhow::Result<()>;

    /// Draws the game
    ///
    /// This method should contain the logic for drawing on the game's canvas
    ///
    /// # Arguments
    ///
    /// - `game` - A mutable reference to the game state.
    ///
    /// # Returns
    ///
    /// - `Result<()>` - `()` if everythings fine, otherwise you should return an error if
    /// something failed in your logics
    async fn draw(&self, game: &mut Game) -> anyhow::Result<()>;

    /// Handles a key‑down event.
    ///
    /// # Parameters
    /// - `game` – Mutable `Game` reference.
    /// - `timestamp` – Event timestamp.
    /// - `window_id` – Window identifier.
    /// - `keycode` – Optional keycode of the pressed key.
    /// - `scancode` – Optional scancode of the pressed key.
    fn key_down_event(
        &self,
        _game: &mut Game,
        _timestamp: u64,
        _window_id: u32,
        _keycode: Option<Keycode>,
        _scancode: Option<Scancode>,
        _keymod: Mod,
        _repeat: bool,
        _which: u32,
        _raw: u16,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    /// Handles a key‑up event. Parameters mirror `key_down_event`
    fn key_up_event(
        &self,
        _game: &mut Game,
        _timestamp: u64,
        _window_id: u32,
        _keycode: Option<Keycode>,
        _scancode: Option<Scancode>,
        _keymod: Mod,
        _repeat: bool,
        _which: u32,
        _raw: u16,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    /// Handles mouse‑motion events.
    ///
    /// # Parameters
    /// - `game` – Mutable `Game` reference.
    /// - `timestamp` – Event timestamp.
    /// - `window_id` – Window identifier.
    /// - `x`, `y` – Absolute cursor coordinates.
    /// - `xrel`, `yrel` – Relative motion since the previous event.
    fn mouse_motion_event(
        &self,
        _game: &mut Game,
        _timestamp: u64,
        _window_id: u32,
        _which: u32,
        _mousestate: MouseState,
        _x: f32,
        _y: f32,
        _xrel: f32,
        _yrel: f32,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    /// Handles a mouse‑button‑down event.
    ///
    /// # Parameters
    /// - `game` – Mutable `Game` reference.
    /// - `timestamp` – Event timestamp.
    /// - `window_id` – Window identifier.
    /// - `mouse_btn` – Button that was pressed.
    /// - `clicks` – Number of consecutive clicks.
    /// - `x`, `y` – Cursor position at the time of the press.
    fn mouse_button_down_event(
        &self,
        _game: &mut Game,
        _timestamp: u64,
        _window_id: u32,
        _which: u32,
        _mouse_btn: MouseButton,
        _clicks: u8,
        _x: f32,
        _y: f32,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    /// Handles a mouse‑button‑up event. Parameters mirror
    /// `mouse_button_down_event`
    fn mouse_button_up_event(
        &self,
        _game: &mut Game,
        _timestamp: u64,
        _window_id: u32,
        _which: u32,
        _mouse_btn: MouseButton,
        _clicks: u8,
        _x: f32,
        _y: f32,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    /// Handles a mouse‑wheel (scroll) event.
    ///
    /// # Parameters
    /// - `game` – Mutable `Game` reference.
    /// - `timestamp` – Event timestamp.
    /// - `window_id` – Window identifier.
    /// - `x`, `y` – Scroll amount along each axis.
    /// - `direction` – Scroll direction probably i don't know.
    /// - `mouse_x`, `mouse_y` – Cursor position when the wheel event occurred.
    fn mouse_wheel_event(
        &self,
        _game: &mut Game,
        _timestamp: u64,
        _window_id: u32,
        _which: u32,
        _x: f32,
        _y: f32,
        _direction: MouseWheelDirection,
        _mouse_x: f32,
        _mouse_y: f32,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
