use std::{
    collections::HashMap,
    fs,
    io::{BufReader, Cursor},
    path::{Path, PathBuf},
    thread,
};

use anyhow::bail;
use tokio::sync::mpsc::{self, Sender};

#[derive(Debug, Clone)]
pub struct Audio {
    pub channel: Sender<AudioCommand>,
    pub cached_audio: HashMap<String, Box<Cursor<Vec<u8>>>>,
}

#[derive(Debug, Clone)]
pub enum AudioCommand {
    Play(PathBuf),
    Stop,
}

impl Audio {
    pub fn new() -> Audio {
        let (tx, mut rx) = mpsc::channel::<AudioCommand>(16);

        tokio::spawn(async move {
            println!("hey im the audio thread nya meow meow >:3");
            let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
                .expect("audio: failed to initialize stream handle");
            let mixer = stream_handle.mixer();
            let mut current_sink: Option<rodio::Sink> = None;

            while let Some(message) = rx.recv().await {
                println!("{:?}", message);
                if let AudioCommand::Play(pathbuf) = message {
                    if let Some(sink) = current_sink {
                        sink.stop();
                    }

                    let file = std::fs::File::open(pathbuf.to_string_lossy().to_string()).unwrap();
                    println!("{:?}", file.metadata());
                    let sink = rodio::play(mixer, BufReader::new(file)).unwrap();
                    sink.set_volume(1.0);
                    current_sink = Some(sink);
                }
                thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        Self {
            channel: tx,
            cached_audio: HashMap::new(),
        }
    }

    pub fn load_asset(&mut self, path: &Path) -> anyhow::Result<()> {
        if self
            .cached_audio
            .contains_key(&path.to_string_lossy().to_string())
        {
            bail!("resource already cached")
        }

        let data: Vec<u8> = fs::read(path)?;
        self.cached_audio.insert(
            path.to_string_lossy().to_string(),
            Box::new(Cursor::new(data)),
        );
        Ok(())
    }

    pub async fn play_audio(
        &mut self,
        path: &Path,
        _interrupt_current_playback: bool,
    ) -> anyhow::Result<()> {
        if !self
            .cached_audio
            .contains_key(&path.to_string_lossy().to_string())
        {
            // as we do not want to crash just because the resource wasn't cached we'll cache it
            // here and now
            println!("caching");
            self.load_asset(path)?;
        }
        self.channel
            .send(AudioCommand::Play(path.to_path_buf()))
            .await?;
        /*
        // .unwrap() here should be safe because we cached the resource if it wasn't already cached
        // before
        let resource: Box<Cursor<Vec<u8>>> = self.cached_audio.get(&path.to_string_lossy().to_string()).unwrap().clone();

        if !self.playing.contains(&path.to_string_lossy().to_string()) {
            println!("playing");
            self.playing.push(path.to_string_lossy().to_string());
            rodio::play(&self.mixer, resource)?;
        }
        */
        Ok(())
    }
}

impl Default for Audio {
    fn default() -> Self {
        Self::new()
    }
}
