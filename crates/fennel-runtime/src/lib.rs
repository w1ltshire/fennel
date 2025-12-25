//! # `fennel-runtime`
//! The Fennel engine's runtime module, which basically serves as the main entry point into the application (game).
//! Runtime contains only essential pieces: an ECS, time management (ticks), scene management.
//! It "glues" together everything else, you name it: renderer, tiles, sprites, animations, maybe some RPG-like systems.

/// Application layer module
pub mod app;
/// Module providing component registry
#[macro_use]
pub mod registry;
/// Module providing basic systems like rendering on top of `specs`
pub mod ecs;
/// Helper event types for ECS
pub mod events;
/// Module providing advanced rendering functionality
pub mod renderer;
/// Module providing functionality of scenes
pub mod scenes;
/// Module providing time (tick) functionality
pub mod time;
/// Module providing rendering camera functionality
pub mod camera;
#[cfg(test)]
mod tests;