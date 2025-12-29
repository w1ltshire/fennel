use std::error::Error;
use shred::{DispatcherBuilder, World};
use fennel_plugins::Plugin;
use crate::Audio;

pub struct AudioPlugin {
	audio: Audio
}

impl AudioPlugin {
	/// Create a new instance of [`AudioPlugin`]
	pub fn new() -> Result<Self, Box<dyn Error>> {
		Ok(Self {
			audio: Audio::new()?
		})
	}
}

impl Plugin for AudioPlugin {
	fn prepare(&mut self, _dispatcher_builder: &mut DispatcherBuilder, _world: &mut World) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	fn update(&mut self, _delta_time: f64) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	fn name(&self) -> &'static str {
		"audio_plugin"
	}
}