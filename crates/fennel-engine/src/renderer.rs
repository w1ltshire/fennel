use log::warn;
use specs::{System, WriteExpect};
use tokio::sync::mpsc::UnboundedSender;
use crate::ecs::sprite::Sprite;

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
    type SystemData = (WriteExpect<'a, RenderQueue>, WriteExpect<'a, UnboundedSender<Drawable>>);

    fn run(&mut self, (mut rq, sender): Self::SystemData) {
        rq.queue.drain(..).for_each(|drawable| {
            sender.send(drawable).unwrap_or_else(|e| {
                warn!("failed to send drawable: {e}");
            });
        });
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
