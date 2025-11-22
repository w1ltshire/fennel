//! Audio playback thing
///
/// This module provides a simple asynchronous audio player
///
/// # Example
/// ```ignore
/// game.audio.play_audio(Path::new("examples/music.ogg"), false).await?;
/// ```
use std::{
    collections::HashMap,
    io::BufReader,
    path::{Path, PathBuf},
};
use std::fs::File;
use log::{error, warn};
use rodio::{OutputStreamBuilder, Sink};
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
            println!("hey I'm the audio thread nya meow meow >:3");
            let stream_handle = match OutputStreamBuilder::open_default_stream() {
                Ok(handle) => handle,
                Err(e) => {
                    error!("audio: failed to initialize stream handle: {}", e);
                    return;
                }
            };
            let mixer = stream_handle.mixer();
            let mut sinks: HashMap<String, Sink> = HashMap::new();

            while let Some(message) = rx.recv().await {
                match message {
                    AudioCommand::Play(pathbuf) => {
                        match File::open(&pathbuf) {
                            Ok(file) => {
                                let sink = match rodio::play(mixer, BufReader::new(file)) {
                                    Ok(sink) => sink,
                                    Err(e) => {
                                        error!("audio: failed to start playback for {}: {}", pathbuf.display(), e);
                                        continue;
                                    }
                                };
                                sink.set_volume(1.0);
                                sinks.insert(pathbuf.to_string_lossy().to_string(), sink);
                            }
                            Err(e) => {
                                error!("audio: failed to open file {}: {}", pathbuf.display(), e);
                            }
                        }
                    }
                    AudioCommand::Stop(sink_name) => {
                        if let Some(sink) = sinks.get(&sink_name) {
                            sink.stop();
                        } else {
                            warn!("audio: no sink found for {}", sink_name);
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
