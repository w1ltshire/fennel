/// A struct to represent the camera in the world
#[derive(Debug)]
pub struct Camera {
    /// Position of the camera in world coordinates
    pub position: (f32, f32),
    /// Dimensions of the viewable area
    pub viewport: (f32, f32),
}

impl specs::Component for Camera {
    type Storage = specs::VecStorage<Self>;
}

impl Camera {
    /// Create a new instance of [`Camera`]
    pub fn new(position: (f32, f32), viewport: (f32, f32)) -> Self {
        Camera { position, viewport }
    }

    /// Transform world coordinates to camera coordinates
    pub fn world_to_camera(&self, world_pos: (f32, f32)) -> (f32, f32) {
        (world_pos.0 - self.position.0, world_pos.1 - self.position.1)
    }
}