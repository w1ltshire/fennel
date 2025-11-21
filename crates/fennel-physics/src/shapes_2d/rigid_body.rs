use crate::shapes_2d::position::Position;

/// A struct representing a 2D rigid body
pub struct RigidBody {
    /// 2D position
    position: Position
}

#[cfg(feature = "specs")]
impl Component for RigidBody {
    type Storage = VecStorage<Self>;
}