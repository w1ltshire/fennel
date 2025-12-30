use kanal::{Receiver, Sender};
use log::error;
use specs::{ReadExpect, System, WriteExpect};
use crate::graphics::{Drawable, Transform};
use crate::plugin::event_handler::PluginEvent;

/// A simple queue of [`Drawable`] items to be consumed by a rendering system
pub struct RenderQueue {
	/// List of drawables which are rendered in order from index zero
	pub queue: Vec<Drawable>,
}

/// ECS system that renders Sprite components from [`RenderQueue`]
pub struct QueuedRenderingSystem;

pub(crate) struct EventGatherSystem;

pub(crate) struct CleanupSystem;

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

impl<'a> System<'a> for EventGatherSystem {
	type SystemData = (WriteExpect<'a, Receiver<PluginEvent>>, WriteExpect<'a, Vec<PluginEvent>>);

	fn run(&mut self, (receiver, mut batch): Self::SystemData) {
		loop {
			let received = receiver.try_recv();
			match received {
				Ok(Some(event)) => batch.push(event),
				Err(_) => break,
				Ok(None) => break,
			}
		}
	}
}

impl<'a> System<'a> for CleanupSystem {
	type SystemData = (WriteExpect<'a, Vec<PluginEvent>>,);

	fn run(&mut self, mut batch: Self::SystemData) {
		batch.0.clear();
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