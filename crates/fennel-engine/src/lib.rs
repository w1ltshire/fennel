//! A library providing abstractions over `fennel-core`, a simpler API and an ECS
/// Application layer module
pub mod app;
/// Module providing basic systems like rendering on top of `specs`
pub mod ecs;
/// Helper event types for ECS
pub mod events;
/// Module providing component registry
pub mod registry;
/// Module providing advanced rendering functionality
pub mod renderer;
/// Module providing functionality of scenes
pub mod scenes;
/// Module providing time (tick) functionality
pub mod time;
/// Module providing tiles functionality
pub mod tiles;
/// Module providing rendering camera functionality
pub mod camera;
#[cfg(test)]
mod tests;