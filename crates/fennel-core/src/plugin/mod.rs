use std::path::Path;
use std::sync::{Arc, Mutex};
use kanal::Receiver;
use log::{debug, error};
use specs::{DispatcherBuilder, World};
use fennel_plugins::Plugin;
use fennel_resources::manager::ResourceManager;
use crate::graphics::{Drawable, Graphics, WindowConfig};
use crate::Window;
use crate::events;
use crate::events::WindowEventHandler;
use crate::plugin::system::{Camera, QueuedRenderingSystem, RenderQueue};

pub mod system;

/// The graphics module plugin for `fennel_runtime`
pub struct GraphicsPlugin {
	name: &'static str,
	dimensions: (u32, u32),
	assets_path: String,
}

struct EventHandler {
	render_receiver: Receiver<Vec<Drawable>>
}

impl GraphicsPlugin {
	/// Create a new instance of [`GraphicsPlugin`]
	///
	/// # Arguments
	/// * `name`: name of the window
	/// * `dimensions`: dimensions of the window
	/// * `assets_path`: path to assets directory
	pub fn new<P: AsRef<Path> + ToString>(name: &'static str, dimensions: (u32, u32), assets_path: P) -> Self {
		Self {
			name,
			dimensions,
			assets_path: assets_path.to_string()
		}
	}
}

impl WindowEventHandler for EventHandler {
	fn update(&mut self, _window: &mut Window) -> anyhow::Result<()> {
		Ok(())
	}

	fn draw(&mut self, window: &mut Window) -> anyhow::Result<()> {
		if let Ok(Some(queue)) = self.render_receiver.try_recv() {
			window.graphics.canvas.clear();
			for drawable in queue {
				match drawable {
					Drawable::Image(sprite) => {
						window.graphics.draw_image(
							sprite.image,
							sprite.transform.position,
							sprite.transform.rotation,
							false,
							false
						).unwrap_or_else(|e| { error!("failed to draw image: {e}"); });
					},
					Drawable::Text { font, position, text, color, size } => {
						window.graphics.draw_text(text, position, font, color, size)
							.unwrap_or_else(|e| { error!("failed to draw text: {e}"); });
					},
					Drawable::Rect { w, h, x, y } => {
						window.graphics.draw_rect(x, y, w, h)
							.unwrap_or_else(|e| { error!("failed to draw rectangle: {e}"); });
					}
				}
			}
		}
		window.graphics.canvas.present();
		Ok(())
	}
}

impl Plugin for GraphicsPlugin {
	fn prepare(
		&mut self,
		dispatcher_builder: &mut DispatcherBuilder,
		world: &mut World,
	) -> anyhow::Result<()> {
		// performance cost should be acceptable for these `.clone()`s as these are called only once
		let name = self.name;
		let dimensions = self.dimensions;
		let assets_path = self.assets_path.clone();
		let (render_sender, render_receiver) = kanal::bounded::<Vec<Drawable>>(16);

		world.insert(RenderQueue::new());
		world.insert(Camera::new((0.0, 0.0), (0.0, 0.0)));
		world.insert(render_sender);
		dispatcher_builder.add_thread_local(QueuedRenderingSystem);

		std::thread::spawn(move || {
			let resource_manager = Arc::new(Mutex::new(ResourceManager::new()));
			let graphics = Graphics::new(
				String::from(name),
				dimensions,
				resource_manager.clone(),
				|mut graphics| -> anyhow::Result<()> {
					let mut resource_manager = match resource_manager.try_lock() {
						Ok(guard) => guard,
						Err(e) => return Err(anyhow::anyhow!("failed to lock resource_manager: {}", e)),
					};
					crate::resources::load_dir(&mut resource_manager, assets_path.parse()?, &mut graphics)?;
					Ok(())
				},
				WindowConfig::default(),
			).expect("failed to create graphics");
			let mut window = Window::new(graphics);
			let handler: &'static mut dyn WindowEventHandler = {
				let boxed = Box::new(EventHandler {
					render_receiver
				});
				Box::leak(boxed) as &'static mut dyn WindowEventHandler
			};
			debug!("entering the graphics event loop! {:?}", std::thread::current().id());
			events::run(&mut window, handler, Vec::new()).unwrap();
		});
		Ok(())
	}

	fn update(&mut self, _delta_time: f64) -> anyhow::Result<()> {
		Ok(())
	}

	fn name(&self) -> &'static str {
		"graphics_plugin"
	}
}