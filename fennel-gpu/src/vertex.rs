/// A vertex structure for 2D and 3D graphics
#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Vertex {
	/// Vertex X axis position
	pub x: f32,
	/// Vertex Y axis position
	pub y: f32,
	/// Vertex Z axis position
	pub z: f32,
	/// Texture coordinate
	pub u: f32,
	/// Texture coordinate
	pub v: f32,
}