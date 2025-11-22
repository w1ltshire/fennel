use std::fmt::Debug;
use nalgebra::Vector2;

/// Trait that all bodies must implement to be in the [`PhysicalWorld`]
pub trait Body: Debug {
    /// Get the body's position
    fn get_position(&self) -> &Vector2<f32>;
    /// Set the body's position
    fn set_position(&mut self, position: Vector2<f32>);
    /// Get the body's velocity
    fn get_velocity(&self) -> &Vector2<f32>;
    /// Set the body's velocity
    fn set_velocity(&mut self, velocity: Vector2<f32>);
    /// Get the body's mass
    fn get_mass(&self) -> f32;
    /// Set the body's mass
    fn set_mass(&mut self, mass: f32);
    /// Apply a force to the body
    fn apply_force(&mut self, force: Vector2<f32>);
    /// Update body's state
    fn update(&mut self, delta_time: f32, acceleration: f32);
}