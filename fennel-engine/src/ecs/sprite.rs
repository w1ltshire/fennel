use serde::{Deserialize, Serialize};
use specs::{Join, ReadStorage, System, WriteExpect};

use crate::app::App;

/// A raw pointer wrapper to the application
pub struct HostPtr(pub *mut App);

unsafe impl Send for HostPtr {}
unsafe impl Sync for HostPtr {}

/// A simple renderable sprite.
///
/// # Fields
/// - image: identifier or path of the image to draw
/// - position: tuple (x, y) position on screen
#[derive(Deserialize, Serialize)]
pub struct Sprite {
    pub image: String,
    pub position: (f32, f32)
}

impl specs::Component for Sprite {
    type Storage = specs::VecStorage<Self>;
}

/// ECS system that renders Sprite components.
///
/// The system reads all Sprite components from the world and obtains a mutable
/// reference to the host App through the HostPtr resource
pub struct RenderingSystem;

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (ReadStorage<'a, Sprite>, WriteExpect<'a, HostPtr>);

    fn run(&mut self, (sprites, mut host_ptr): Self::SystemData) {
        let runtime: &mut App = unsafe { &mut *host_ptr.0 };
        let window = &mut runtime.window;

        for sprite in (&sprites).join() {
            window.graphics.draw_image(sprite.image.clone(), sprite.position).unwrap();
        }
    }
}
