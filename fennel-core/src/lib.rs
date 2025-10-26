//! Core window-related API for Fennel engine
use std::sync::{Arc, Mutex};

use crate::{
    audio::Audio,
    graphics::{Graphics, HasWindow},
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
