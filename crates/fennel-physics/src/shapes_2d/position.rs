/// A struct representing a 2D position
pub struct Position(f32, f32);

impl Position {
    /// Constructs a new `Position` with the given x and y coordinates
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the position
    /// * `y` - The y-coordinate of the position
    ///
    /// # Example
    ///
    /// ```
    /// use fennel_physics::shapes_2d::position::Position;
    /// let pos = Position::new(1.0, 2.0);
    /// ```
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }

    /// Returns the x-coordinate of the position
    ///
    /// # Example
    ///
    /// ```
    /// use fennel_physics::shapes_2d::position::Position;
    /// let pos = Position::new(1.0, 2.0);
    /// assert_eq!(pos.x(), 1.0);
    /// ```
    pub fn x(&self) -> f32 {
        self.0
    }

    /// Returns the y-coordinate of the position
    ///
    /// # Example
    ///
    /// ```
    /// use fennel_physics::shapes_2d::position::Position;
    /// let pos = Position::new(1.0, 2.0);
    /// assert_eq!(pos.y(), 2.0);
    /// ```
    pub fn y(&self) -> f32 {
        self.1
    }

    /// Sets the x-coordinate of the position to the specified value
    ///
    /// # Arguments
    ///
    /// * `x` - The new x-coordinate to set
    ///
    /// # Example
    ///
    /// ```
    /// use fennel_physics::shapes_2d::position::Position;
    /// let mut pos = Position::new(1.0, 2.0);
    /// pos.set_x(3.0);
    /// assert_eq!(pos.x(), 3.0);
    /// ```
    pub fn set_x(&mut self, x: f32) {
        self.0 = x
    }

    /// Sets the y-coordinate of the position to the specified value
    ///
    /// # Arguments
    ///
    /// * `y` - The new y-coordinate to set
    ///
    /// # Example
    ///
    /// ```
    /// use fennel_physics::shapes_2d::position::Position;
    /// let mut pos = Position::new(1.0, 2.0);
    /// pos.set_y(4.0);
    /// assert_eq!(pos.y(), 4.0);
    /// ```
    pub fn set_y(&mut self, y: f32) {
        self.1 = y
    }
}