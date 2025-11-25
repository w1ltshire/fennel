use log::warn;
use specs::{ReadExpect, System, WriteExpect};
use crate::app::HostPtr;
use crate::camera::Camera;
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
    /// Text drawable.
    ///
    /// # Fields
    /// * `font`: Font name registered in the resource manager
    /// * `position`: Position in `(f32, f32)` relative to the window
    /// * `text`: The text itself to render
    /// * `color`: RGB tuple of `u8`
    /// * `size`: Font size in `f32`
    Text { font: String, position: (f32, f32), text: String, color: (u8, u8, u8), size: f32 },
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
        ReadExpect<'a, Camera>,
        WriteExpect<'a, HostPtr>
    );

    fn run(&mut self, (mut rq, camera, mut host_ptr): Self::SystemData) {
        let app = unsafe { &mut *host_ptr.0 };
        app.window.graphics.canvas.clear();
        for drawable in rq.queue.drain(..) {
            match drawable {
                Drawable::Image(sprite) => {
                    if !sprite.fixed {
                        let (camera_x, camera_y) = camera.world_to_camera((sprite.transform.position.0, sprite.transform.position.1));
                        app
                            .window
                            .graphics
                            .draw_image(
                                sprite.image,
                                (camera_x, camera_y),
                                sprite.transform.rotation,
                                false,
                                false,
                            )
                            .unwrap_or_else(|e| warn!("failed to draw image: {:?}", e));
                    } else {
                        app
                            .window
                            .graphics
                            .draw_image(
                                sprite.image,
                                sprite.transform.position,
                                sprite.transform.rotation,
                                false,
                                false,
                            )
                            .unwrap_or_else(|e| warn!("failed to draw image: {:?}", e));
                    }
                },
                Drawable::Rect { w, h, x, y } => {
                    let (camera_x, camera_y) = camera.world_to_camera((x, y));
                    app
                        .window
                        .graphics
                        .draw_rect(w, h, camera_x, camera_y)
                        .unwrap_or_else(|e| warn!("failed to draw rect: {:?}", e));
                },
                Drawable::Text { font, position, text, color, size } => {
                    let (camera_x, camera_y) = camera.world_to_camera(position);
                    app
                        .window
                        .graphics
                        .draw_text(text, (camera_x, camera_y), font, color, size)
                        .unwrap_or_else(|e| warn!("failed to draw text: {:?}", e));
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
