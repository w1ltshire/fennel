pub mod plugin;

use std::error::Error;
use std::fs::File;
use std::path::Path;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, SpatialSink};

pub struct Audio {
	stream_handle: OutputStream
}

impl Audio {
	/// Create a new instance of [`Audio`]
	pub fn new() -> Result<Self, Box<dyn Error>> {
		let stream_handle = OutputStreamBuilder::open_default_stream()?;
		Ok(Self {
			stream_handle
		})
	}

	/// Play audio from a file.
	///
	/// # Arguments
	/// * `path` - path to the audio file
	///
	/// # Returns
	/// [`Sink`] wrapped in a [`Result`], which you must take ownership of, because otherwise
	/// the audio won't be played.
	pub fn play_file<P: AsRef<Path>>(&mut self, path: P) -> Result<Sink, Box<dyn Error>> {
		let file = File::open(path)?;
		let source = Decoder::try_from(file)?;
		let sink = Sink::connect_new(self.stream_handle.mixer());
		sink.append(source);
		Ok(sink)
	}

	/// Play audio from a file.
	///
	/// # Arguments
	/// * `path` - path to the audio file
	///
	/// # Returns
	/// [`Sink`] wrapped in a [`Result`], which you must take ownership of, because otherwise
	/// the audio won't be played.
	pub fn play_file_spatial<P: AsRef<Path>>(
		&mut self,
		path: P,
		emitter_position: [f32; 3],
		left_ear: [f32; 3],
		right_ear: [f32; 3],
	) -> Result<SpatialSink, Box<dyn Error>> {
		let file = File::open(path)?;
		let source = Decoder::try_from(file)?;
		let sink = SpatialSink::connect_new(self.stream_handle.mixer(), emitter_position, left_ear, right_ear);
		sink.append(source);
		Ok(sink)
	}
}