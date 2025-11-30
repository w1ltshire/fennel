use kanal::Sender;
use log::error;
use specs::{ReadExpect, System, WriteExpect};
use fennel_core::graphics::{Drawable, Transform};
use crate::camera::Camera;

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
        WriteExpect<'a, Sender<Vec<Drawable>>>,
    );

    fn run(&mut self, (mut rq, camera, sender): Self::SystemData) {
        let mut drained_drawables: Vec<Drawable> = rq.queue.drain(..).collect();
        for drawable in &mut drained_drawables {
            match drawable {
                Drawable::Image(sprite) => {
                    if !sprite.fixed {
                        let (camera_x, camera_y) = camera.world_to_camera((sprite.transform.position.0, sprite.transform.position.1));
                        sprite.transform = Transform::new((camera_x, camera_y), sprite.transform.rotation, sprite.transform.scale);
                    }
                },
                Drawable::Rect { w, h, x, y } => {
                    let (camera_x, camera_y) = camera.world_to_camera((*x, *y));
                    *drawable = Drawable::Rect {
                        w: *w,
                        h: *h,
                        x: camera_x,
                        y: camera_y,
                    };
                },
                Drawable::Text { font, position, text, color, size } => {
                    let (camera_x, camera_y) = camera.world_to_camera(*position);
                    *drawable = Drawable::Text {
                        font: font.clone(),
                        position: (camera_x, camera_y),
                        text: text.clone(),
                        color: *color,
                        size: *size,
                    };
                }
            }
        }
        sender.send(drained_drawables).unwrap_or_else(|e| error!("failed to send queue: {}", e));
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