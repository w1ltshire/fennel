//! A small 2D game framework I'm building just-for-fun and to learn Rust a little bit deeper

use crate::graphics::Graphics;

/// Handling keyboard, window, mouse and other events
pub mod events;
/// Rendering layer and all the related things
pub mod graphics;

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
    pub fn new(name: String, author: String, graphics: Graphics) -> Game {
        Game {
            name,
            author,
            graphics,
        }
    }
}

/// Trait that must be implemented by user's game state struct
/// - `update` is called to advance game state (physics, AI, input processing).
/// - `draw` is called to render the current state using `game.graphics`.
///
/// Both return `anyhow::Result<()>`
pub trait EventHandler {
    fn update(&self, game: &mut Game) -> anyhow::Result<()>;
    fn draw(&self, game: &mut Game) -> anyhow::Result<()>;
}
