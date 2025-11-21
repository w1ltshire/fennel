use nalgebra::Vector2;
use crate::shapes_2d::position::Position;

/// A struct representing a 2D rigid body
pub struct RigidBody {
    /// 2D position
    position: Position,
    /// Body's mass in kg
    mass: f32,
    /// Body's velocity
    velocity: Vector2<f32>,
    /// Body's force in Newtons
    force: f32
}

#[cfg(feature = "specs")]
impl Component for RigidBody {
    type Storage = VecStorage<Self>;
}