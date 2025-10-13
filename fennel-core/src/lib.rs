//! Core window-related API for Fennel engine
use std::sync::{Arc, Mutex};

use crate::{audio::Audio, events::{KeyboardEvent, MouseClickEvent, MouseMotionEvent, MouseWheelEvent, WindowEventHandler}, graphics::{Graphics, HasWindow}, resources::ResourceManager};

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
    pub resource_manager: Arc<Mutex<ResourceManager>>,
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
    pub fn new(graphics: Graphics, resource_manager: Arc<Mutex<ResourceManager>>) -> Window {
        Window {
            graphics,
            audio: Audio::new(),
            resource_manager,
        }
    }
}

impl HasWindow for Window {
    fn window_mut(&mut self) -> &mut Self {
        self
    }
}

pub struct CoreHandler;
#[async_trait::async_trait]
impl WindowEventHandler for CoreHandler {
    fn update(&self, _window: &mut Window) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(&mut self, _window: &mut Window) -> anyhow::Result<()> {
        Ok(())
    }

    fn key_down_event(&self, _window: &mut Window, _event: KeyboardEvent) -> anyhow::Result<()> {
        Ok(())
    }

    fn key_up_event(&self, _window: &mut Window, _event: KeyboardEvent) -> anyhow::Result<()> {
        Ok(())
    }

    fn mouse_motion_event(
        &self,
        _window: &mut Window,
        _event: MouseMotionEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn mouse_button_down_event(
        &self,
        _window: &mut Window,
        _event: MouseClickEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn mouse_button_up_event(
        &self,
        _window: &mut Window,
        _event: MouseClickEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn mouse_wheel_event(
        &self,
        _window: &mut Window,
        _event: MouseWheelEvent,
    ) -> anyhow::Result<()> {
        Ok(())
    }
}
