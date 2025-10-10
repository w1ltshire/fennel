use fennel_core::{graphics::HasWindow, Window};
use specs::{Join, ReadStorage, System, WriteExpect};

use crate::runtime::Runtime;

pub struct HostPtr(pub *mut Runtime);
unsafe impl Send for HostPtr {}
unsafe impl Sync for HostPtr {}

pub struct Sprite {
    pub image: String,
    pub position: (f32, f32)
}

impl specs::Component for Sprite {
    type Storage = specs::VecStorage<Self>;
}

pub struct RenderingSystem;

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (ReadStorage<'a, Sprite>, WriteExpect<'a, HostPtr>);

    fn run(&mut self, (sprites, mut host_ptr): Self::SystemData) {
        let runtime: &mut Runtime = unsafe { &mut *host_ptr.0 };
        let window: &mut Window = runtime.window_mut();

        for sprite in (&sprites).join() {
            window.graphics.draw_image(sprite.image.clone(), sprite.position).unwrap();
        }
    }
}

