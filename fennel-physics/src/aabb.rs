use nalgebra::Vector3;

/// Structure representing the bounding box
#[derive(Debug)]
pub struct BoundingBox {
	/// Width of the bounding box
	pub width: f32,
	/// Height of the bounding box
	pub height: f32,
	/// Position of the bounding box.
	/// Use `0.0` for the third value and ignore it if the bounding box is in a 2D dimension
	pub position: Vector3<f32>,
}

impl BoundingBox {
	/// Creates a new instance of [`BoundingBox`]
	///
	/// # Arguments
	/// * `width`: width of the bounding box
	/// * `height`: height of the bounding box
	/// * `position`: `Vector3<f32>` of the bounding box. Use `0.0` for the third (z) value if the box is in a 2D dimension
	pub fn new(width: f32, height: f32, position: Vector3<f32>) -> Self {
		Self {
			width,
			height,
			position,
		}
	}

	/// Checks if a collision happened between two bounding boxes
	///
	/// # Arguments
	/// * `other`: immutable reference to a box with which the collision should be checked
	///
	/// # Examples
	/// ```
	/// use nalgebra::Vector3;
	/// use fennel_physics::aabb::BoundingBox;
	/// let first_box = BoundingBox::new(10.0, 10.0, Vector3::new(3.0, 4.0, 0.0)); // 2d plane
	/// let second_box  = BoundingBox::new(10.0, 10.0, Vector3::new(3.0, 6.0, 0.0));
	/// assert!(first_box.check_collision(&second_box)); // boxes collide
	/// ```
	pub fn check_collision(&self, other: &BoundingBox) -> bool {
		other.position.x >= self.position.x &&
			other.position.x <= self.position.x + self.width &&
			other.position.y >= self.position.y &&
			other.position.y <= self.position.y + self.height &&
			other.position.z >= self.position.z
	}
}