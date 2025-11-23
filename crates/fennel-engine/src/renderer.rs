use kanal::Sender;
use log::warn;
use specs::{ReadExpect, System, WriteExpect};
use crate::camera::Camera;
use crate::ecs::sprite::Sprite;
use crate::ecs::transform::Transform;

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
    /// Text drawable.
    ///
    /// # Fields
    /// * `font`: Font name registered in the resource manager
    /// * `position`: Position in `(f32, f32)` relative to the window
    /// * `text`: The text itself to render
    /// * `color`: RGB tuple of `u8`
    /// * `size`: Font size in `f32`
    Text { font: String, position: (f32, f32), text: String, color: (u8, u8, u8), size: f32 },
    /// Ready to present command
    Present
}

/// A simple queue of [`Drawable`] items to be consumed by a rendering system
pub struct RenderQueue {
    /// List of drawables which are rendered in order from index zero
    pub queue: Vec<Drawable>,
}

/// ECS system that renders Sprite components from [`RenderQueue`]
pub struct QueuedRenderingSystem;


impl<'a> System<'a> for QueuedRenderingSystem {
    type SystemData = (
        WriteExpect<'a, RenderQueue>,
        WriteExpect<'a, Sender<Drawable>>,
        ReadExpect<'a, Camera>
    );

    fn run(&mut self, (mut rq, sender, camera): Self::SystemData) {

        rq.queue.drain(..).for_each(|drawable| {
            match drawable {
                Drawable::Rect { w, h, x, y } => {
                    let (camera_x, camera_y) = camera.world_to_camera((x, y));
                    sender.send(Drawable::Rect { w, h, x: camera_x, y: camera_y }).unwrap_or_else(|e| {
                        warn!("failed to send drawable: {e}");
                    });
                },
                Drawable::Image(sprite) => {
                    let (camera_x, camera_y) = camera.world_to_camera((sprite.transform.position.0, sprite.transform.position.1));
                    sender.send(Drawable::Image(Sprite {
                        image: sprite.image,
                        transform: Transform {
                            position: (camera_x, camera_y),
                            rotation: sprite.transform.rotation,
                            scale: sprite.transform.scale
                        },
                    })).unwrap_or_else(|e| {
                        warn!("failed to send drawable: {e}");
                    });
                },
                Drawable::Text { font, position, text, color, size } => {
                    let (camera_x, camera_y) = camera.world_to_camera(position);
                    sender.send(Drawable::Text { font, position: (camera_x, camera_y), text, color, size }).unwrap_or_else(|e| {
                        warn!("failed to send drawable: {e}");
                    });
                },
                _ => {
                    sender.send(drawable).unwrap_or_else(|e| {
                        warn!("failed to send drawable: {e}");
                    });
                }
            }
        });

        sender.send(Drawable::Present).unwrap_or_else(|e| {
            warn!("failed to send drawable: {e}");
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
