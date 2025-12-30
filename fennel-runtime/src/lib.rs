//! # `fennel-runtime`
//! The Fennel engine's runtime module, which basically serves as the main entry point into the application (game).
//! Runtime contains only essential pieces: an ECS, time management (ticks), scene management.
//! It "glues" together everything else, you name it: renderer, tiles, sprites, animations, maybe some RPG-like systems.
//!
//! Previously this was called `fennel-engine`, but it's not the actual engine, because the actual engine is a combination
//! of all crates and plugins. "Runtime" is the best word to describe this crate.

/// Application layer module
pub mod app;
/// Module providing advanced rendering functionality
pub mod renderer;
/// Module providing functionality of scenes
pub mod scenes;
/// Module providing time (tick) functionality
pub mod time;
#[cfg(test)]
mod tests;