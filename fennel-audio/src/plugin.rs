use std::error::Error;
use rodio::Sink;
use shred::{DispatcherBuilder, World};
use fennel_plugins::Plugin;
use crate::Audio;

extern crate rodio;

pub struct AudioPlugin;

impl AudioPlugin {
	/// Create a new instance of [`AudioPlugin`]
	pub fn new() -> Self {
		Self
	}
}

impl Plugin for AudioPlugin {
	fn prepare(
		&mut self,
		_dispatcher_builder: &mut DispatcherBuilder,
		world: &mut World,
	) -> Result<(), Box<dyn Error>> {
		// sometimes i wish i was born a girl, i just want to wear cool clothes have long cool hair dye it etc
		// wish i could've at least come out as mtf
		// so, insert an `Audio` instance into the world as a resource so then any system can access it, simple isn't it?
		// also insert a vector of sinks! 'cause the user needs to take ownership of a sink, but in a system it will go
		// out of scope anyway, so we provide them a way to store sinks
		let sinks: Vec<Sink> = Vec::new();
		world.insert(sinks);
		world.insert(Audio::new()?);
		Ok(())
	}

	fn update(&mut self, _delta_time: f64) -> Result<(), Box<dyn Error>> {
		Ok(())
	}

	fn name(&self) -> &'static str {
		"audio_plugin"
	}
}

impl Default for AudioPlugin {
	fn default() -> Self {
		Self::new()
	}
}