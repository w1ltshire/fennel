/// A struct representing gravity
#[derive(Debug)]
pub struct Gravity {
    pub(crate) acceleration: f32,
}

impl Gravity {
    /// Create a new [`Gravity`] instance
    ///
    /// # Arguments
    /// * `acceleration`: floating-point value of the acceleration
    pub fn new(acceleration: f32) -> Self {
        Self {
            acceleration,
        }
    }
}