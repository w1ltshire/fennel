use std::fs::File;
use std::path::Path;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};

pub struct Audio {
	stream_handle: OutputStream
}

impl Audio {
	/// Create a new instance of [`Audio`]
	pub fn new() -> anyhow::Result<Self> {
		let stream_handle = OutputStreamBuilder::open_default_stream()?;
		Ok(Self {
			stream_handle
		})
	}

	pub fn play_file<P: AsRef<Path>>(&mut self, path: P) -> anyhow::Result<Sink> {
		let file = File::open(path)?;
		let source = Decoder::try_from(file)?;
		let sink = Sink::connect_new(self.stream_handle.mixer());
		sink.append(source);
		Ok(sink)
	}
}