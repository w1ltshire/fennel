use std::path::Path;
use std::sync::{Arc, Mutex};
use log::debug;
use specs::{DispatcherBuilder, World};
use fennel_plugins::Plugin;
use fennel_resources::manager::ResourceManager;
use crate::graphics::{Drawable, Graphics, WindowConfig};
use crate::Window;
use crate::events;
use crate::events::WindowEventHandler;
use crate::plugin::event_handler::{EventHandler, PluginEvent};
use crate::plugin::system::{Camera, QueuedRenderingSystem, RenderQueue};

pub mod system;
mod event_handler;

/// The graphics module plugin for `fennel_runtime`
pub struct GraphicsPlugin {
	name: &'static str,
	dimensions: (u32, u32),
	assets_path: String,
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
		let (event_sender, event_receiver) = kanal::unbounded::<PluginEvent>();
		let plugin_event_vec: Vec<PluginEvent> = Vec::new();
		
		world.insert(RenderQueue::new());
		world.insert(Camera::new((0.0, 0.0), (0.0, 0.0)));
		world.insert(render_sender);
		world.insert(event_receiver);
		world.insert(plugin_event_vec);
		dispatcher_builder.add(QueuedRenderingSystem, "queued_rendering_system", &[]);

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
					render_receiver,
					event_sender,
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