//! # fennel-physics
//! A physics engine library for the Fennel game engine
//!
//! # Cargo features
//! `specs`: Provide specs' component implementations for most of the types
#![forbid(unsafe_code)]

/// Module providing two-dimensional physics code
pub mod shapes_2d;
/// Module providing physical world implementation
pub mod world;
/// Module providing basic body traits
pub mod body;
/// Module providing gravity functionality
pub mod gravity;
/// Module providing collision detection by leveraging the AABB concept
pub mod aabb;
#[cfg(test)]
mod tests;