use serde::Deserialize;

/// Transform component, containing position in the window, scale and rotation
#[derive(Deserialize, Debug, Clone)]
pub struct Transform {
    /// Position in the window (x, y)
    pub position: (f32, f32),
    /// Scale
    pub scale: f64,
    /// Rotation
    pub rotation: f64,
}

impl specs::Component for Transform {
    type Storage = specs::VecStorage<Self>;
}

impl Transform {
    /// Creates a new instance of [`Transform`]
    pub fn new(position: (f32, f32), scale: f64, rotation: f64) -> Self {
        Self {
            position,
            scale,
            rotation
        }
    }
}
