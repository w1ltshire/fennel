use crate::shapes_2d::position::Position;
use crate::shapes_2d::velocity::Velocity;

/// A struct representing a 2D rigid body
pub struct RigidBody {
    /// 2D position
    position: Position,
    /// Body's mass in kg
    mass: f32,
    /// Body's velocity
    velocity: Velocity
}

#[cfg(feature = "specs")]
impl Component for RigidBody {
    type Storage = VecStorage<Self>;
}