//! Audio playback thing
///
/// This module provides a simple asynchronous audio player
///
/// # Example
/// ```ignore
/// game.audio.play_audio(Path::new("examples/music.ogg"), false).await?;
/// ```
use std::{
    collections::HashMap, io::BufReader, path::{Path, PathBuf}
};

use rodio::Sink;
use tokio::sync::mpsc::{self, Sender};

/// Holds the command channel
#[derive(Debug, Clone)]
pub struct Audio {
    /// Tokio mpsc sender used to communicate with the audio thread
    channel: Sender<AudioCommand>,
}

/// Commands that can be sent to the audio thread.
#[derive(Debug, Clone)]
pub enum AudioCommand {
    /// Play the file at the given path
    Play(PathBuf),
    /// Stop the current playback
    Stop(String),
}

impl Audio {
    /// Creates a new [`Audio`] instance and spawns the background audio thread
    pub fn new() -> Audio {
        let (tx, mut rx) = mpsc::channel::<AudioCommand>(16);

        tokio::spawn(async move {
            println!("hey im the audio thread nya meow meow >:3");
            let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
                .expect("audio: failed to initialize stream handle");
            let mixer = stream_handle.mixer();
            let mut sinks: HashMap<String, Sink> = HashMap::new();

            while let Some(message) = rx.recv().await {
                match message {
                    AudioCommand::Play(pathbuf) => {
                        let file =
                            std::fs::File::open(&pathbuf).expect("audio: failed to open file");
                        let sink = rodio::play(mixer, BufReader::new(file))
                            .expect("audio: failed to start playback");
                        sink.set_volume(1.0);
                        sinks.insert(pathbuf.to_string_lossy().to_string(), sink);
                    }
                    AudioCommand::Stop(sink_name) => {
                        if let Some(sink) = sinks.get(&sink_name) {
                            sink.stop();
                        }
                    }
                }
            }
        });

        Self { channel: tx }
    }

    /// Sends a `Play` command for the given path
    pub async fn play_audio(
        &mut self,
        path: &Path,
        _interrupt_current_playback: bool,
    ) -> anyhow::Result<()> {
        // TODO: use [`ResourceManager`], respect `_interrupt_current_playback`
        self.channel
            .send(AudioCommand::Play(path.to_path_buf()))
            .await?;
        Ok(())
    }
}

// clippy said i need `Default`
impl Default for Audio {
    /// Shortcut for `Audio::new()`
    fn default() -> Self {
        Self::new()
    }
}
