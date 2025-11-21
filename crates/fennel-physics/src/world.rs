use crate::body::Body;

/// A struct representing the physical world of the simulation
pub struct PhysicsWorld {
    /// List of all bodies in this world
    bodies: Vec<Box<dyn Body>>
}