use nalgebra::Vector2;
use crate::body::Body;
#[cfg(feature = "specs")]
use specs::Component;

/// A struct representing a 2D rigid body
#[derive(Debug)]
pub struct RigidBody {
    /// 2D position
    position: Vector2<f32>,
    /// Body's mass in kg
    mass: f32,
    /// Body's velocity, vector quantity with a direction
    velocity: Vector2<f32>,
    /// Body's force in Newtons
    force: Vector2<f32>,
    /// Body's acceleration in m/s^2
    acceleration: Vector2<f32>,
}

#[cfg(feature = "specs")]
impl Component for RigidBody {
    type Storage = VecStorage<Self>;
}

impl Default for RigidBody {
    fn default() -> Self {
        Self::new()
    }
}

impl RigidBody {
    /// Create a new instance of [`RigidBody`] with all fields set to 0.0
    pub fn new() -> Self {
        Self {
            position: Vector2::new(0.0, 0.0),
            mass: 0.0,
            velocity: Vector2::new(0.0, 0.0),
            force: Vector2::new(0.0, 0.0),
            acceleration: Vector2::new(0.0, 0.0)
        }
    }
}

impl Body for RigidBody {
    fn get_position(&self) -> &Vector2<f32> {
        &self.position
    }

    fn set_position(&mut self, new_position: Vector2<f32>) {
        self.position = new_position;
    }

    fn get_velocity(&self) -> &Vector2<f32> {
        &self.velocity
    }

    fn set_velocity(&mut self, new_velocity: Vector2<f32>) {
        self.velocity = new_velocity;
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }

    fn set_mass(&mut self, new_mass: f32) {
        self.mass = new_mass;
    }

    fn apply_force(&mut self, force: Vector2<f32>) {
        self.force += force;
    }

    fn update(&mut self, delta_time: f32, acceleration: f32) {
        if self.mass <= 0.0 {
            return;
        }

        // what the fuck? why do we divide the `g` by 2.0? but without the division it gives seemingly wrong results
        let gravitational_force = Vector2::new(0.0, -self.get_mass() * acceleration / 2.0);

        self.force = gravitational_force;
        self.acceleration = self.force / self.get_mass();
        self.velocity += self.acceleration * delta_time;
        self.position += self.velocity * delta_time;
    }
}