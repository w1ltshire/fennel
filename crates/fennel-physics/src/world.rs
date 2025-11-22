use crate::body::Body;
use crate::gravity::Gravity;

/// A struct representing the physical world of the simulation
#[derive(Debug)]
pub struct PhysicsWorld {
    /// List of all bodies in this world
    pub bodies: Vec<Box<dyn Body>>,
    pub gravity: Gravity,
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}

impl PhysicsWorld {
    /// Create a new instance of [`PhysicsWorld`]
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            gravity: Gravity::new(9.81)
        }
    }

    /// Add a new body to the world
    ///
    /// # Arguments
    /// * `body`: type that implements trait [`Body`]
    pub fn add_body(&mut self, body: Box<dyn Body>) {
        self.bodies.push(body);
    }

    pub fn step(&mut self, delta_time: f32) {
        for body in self.bodies.iter_mut() {
            body.update(delta_time, self.gravity.acceleration);
        }

        self.handle_collisions();
    }

    pub fn handle_collisions(&mut self) {

    }
}