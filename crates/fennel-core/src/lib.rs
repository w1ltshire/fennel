//! Core window-related API for Fennel engine

use crate::graphics::Graphics;

/// Audio playback
pub mod audio;
/// Handling keyboard, window, mouse and other events
pub mod events;
/// Rendering layer and all the related things
pub mod graphics;
/// Hooks to inject into [`events::run`]
pub mod hooks;
/// Resource management
pub mod resources;
/// The graphics plugin
pub mod plugin;

unsafe impl Send for Window {}
unsafe impl Sync for Window {}

/// Main window struct
///
/// Holds basic metadata and a reference to the graphics subsystem.
/// User must create a [`Window`] in order to feed it to the EventHandler
pub struct Window {
    /// Graphics subsystem used to render frames.
    pub graphics: Graphics,
}

impl Window {
    /// Create a new [`Window`] instance.
    ///
    /// # Parameters
    /// - `name`: title of the window
    /// - `graphics`: initialized graphics subsystem
    ///
    /// # Returns
    /// A [`Window`] instance ready to be used by an [`events::WindowEventHandler`].
    pub fn new(graphics: Graphics) -> Window {
        Window {
            graphics
        }
    }
}