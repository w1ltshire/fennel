use log::warn;
use specs::{System, WriteExpect};

use crate::{
    app::App,
    ecs::sprite::{HostPtr, Sprite},
};

/// A drawable primitive that can be queued for rendering
///
/// Variants:
/// - Image(Sprite) - a sprite to draw
/// - Rect { w, h, x, y } - a rectangle specified with width, height, x and y position (all `f32`)
#[derive(Debug)]
pub enum Drawable {
    /// A sprite. Use for queueing render of some image
    Image(Sprite),
    /// A basic rectangle
    Rect { w: f32, h: f32, x: f32, y: f32 },
}

/// A simple queue of [`Drawable`] items to be consumed by a rendering system
pub struct RenderQueue {
    /// List of drawables which are rendered in order from index zero
    pub queue: Vec<Drawable>,
}

/// ECS system that renders Sprite components from [`RenderQueue`]
pub struct QueuedRenderingSystem;

impl<'a> System<'a> for QueuedRenderingSystem {
    type SystemData = (WriteExpect<'a, RenderQueue>, WriteExpect<'a, HostPtr>);

    fn run(&mut self, (mut rq, mut host_ptr): Self::SystemData) {
        let runtime: &mut App = unsafe { &mut *host_ptr.0 };
        let window = &mut runtime.window;
        for drawable in rq.queue.drain(..) {
            match drawable {
                Drawable::Image(sprite) => {
                    window
                        .graphics
                        .draw_image(
                            sprite.image,
                            sprite.transform.position,
                            sprite.transform.rotation,
                            false,
                            false,
                        )
                        .unwrap_or_else(|e| { warn!("failed to draw an image: {e}") });
                },
                Drawable::Rect { w, h, x, y } => {
                    window
                        .graphics
                        .draw_rect(w, h, x, y)
                        .unwrap_or_else(|e| { warn!("failed to draw a rectangle: {e}") });
                }
            }
        }
    }
}

impl RenderQueue {
    /// Creates a new instance of [`RenderQueue`]
    pub fn new() -> Self {
        Self { queue: vec![] }
    }
}

impl Default for RenderQueue {
    fn default() -> Self {
        Self::new()
    }
}
