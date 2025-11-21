/// Represents the velocity and direction of a body in a 2D space
#[derive(Copy, Clone, Debug)]
pub struct Velocity {
    /// The velocity itself
    velocity: f32,
    /// The direction the velocity is pointed to
    direction: Direction,
}

/// Enum representing possible directions in which a body can move
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Right, // Moving towards the right
    Left,  // Moving towards the left
    Up,    // Moving upwards
    Down,  // Moving downwards
}

impl Velocity {
    /// Creates a new Velocity instance with the specified velocity and direction
    ///
    /// # Parameters
    /// - `velocity`: A floating-point (f32 precision) number representing the speed
    /// - `direction`: The direction in which the body is moving
    ///
    /// # Returns
    /// A new instance of `Velocity`
    pub fn new(velocity: f32, direction: Direction) -> Self {
        Self {
            velocity,
            direction,
        }
    }

    /// Sets the velocity of the body
    ///
    /// # Parameters
    /// - `velocity`: A floating-point (f32 precision) number representing the new speed
    pub fn set_velocity(&mut self, velocity: f32) {
        self.velocity = velocity;
    }

    /// Sets the direction of the body's movement
    ///
    /// # Parameters
    /// - `direction`: The new direction to set, represented by the Direction enum
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    /// Retrieves the current velocity of the body
    ///
    /// # Returns
    /// A floating-point (f32 precision) number representing the current speed
    pub fn velocity(&self) -> f32 {
        self.velocity
    }

    /// Retrieves the current direction of the body's movement
    ///
    /// # Returns
    /// The current direction, represented by the Direction enum
    pub fn direction(&self) -> Direction {
        self.direction
    }
}
