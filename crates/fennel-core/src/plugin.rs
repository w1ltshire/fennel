use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use anyhow::Context;
use kanal::Receiver;
use log::{debug, error};
use specs::prelude::{Resource, ResourceId};
use specs::shred::cell::AtomicRefCell;
use fennel_plugins::Plugin;
use crate::graphics::{Drawable, Graphics, WindowConfig};
use crate::resources::ResourceManager;
use crate::Window;
use crate::events;
use crate::events::WindowEventHandler;

/// The graphics module plugin for `fennel_engine`
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
					_ => {}
				}
			}
		}
		window.graphics.canvas.present();
		Ok(())
	}
}

impl Plugin for GraphicsPlugin {
	fn prepare(&mut self, dependencies: HashMap<String, &AtomicRefCell<Box<dyn Resource>>>) -> anyhow::Result<()> {
		// performance cost should be acceptable for these `.clone()`s as these are called only once
		let name = self.name;
		let dimensions = self.dimensions.clone();
		let assets_path = self.assets_path.clone();

		// the current dependency system is quite janky, but hey, it doesn't segfaults, panics or produces UB (I hope)
		let mut receiver: Option<Receiver<Vec<Drawable>>> = None;
		dependencies.iter().for_each(|dep| {
			debug!("received dependency {}", dep.0);
			if dep.0 == "render_rx" {
				let dep = dep.1.borrow();
				receiver = Some(dep.downcast_ref::<Receiver<Vec<Drawable>>>().unwrap().clone());
			}
		});

		std::thread::spawn(move || {
			let resource_manager = Arc::new(Mutex::new(ResourceManager::new()));
			let graphics = Graphics::new(
				String::from(name),
				dimensions,
				resource_manager.clone(),
				|graphics| -> anyhow::Result<()> {
					let mut resource_manager = match resource_manager.try_lock() {
						Ok(guard) => guard,
						Err(e) => return Err(anyhow::anyhow!("failed to lock resource_manager: {}", e)),
					};
					resource_manager
						.load_dir(PathBuf::from(assets_path.clone()), graphics)
						.context("failed to load resources from directory")?;
					Ok(())
				},
				WindowConfig::default(),
			).expect("failed to create graphics");
			let mut window = Window::new(
				graphics,
				resource_manager,
			);
			let handler: &'static mut dyn WindowEventHandler = {
				let boxed = Box::new(EventHandler {
					render_receiver: receiver.unwrap().clone()
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

	fn resource_dependencies(&mut self) -> HashMap<String, ResourceId> {
		let mut map = HashMap::new();
		map.insert("render_rx".to_string(), ResourceId::new::<Receiver<Vec<Drawable>>>());
		map
	}

	fn name(&self) -> &'static str {
		"graphics_plugin"
	}
}