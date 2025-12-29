use std::error::Error;
use fennel_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
	let mut audio = Audio::new()?;
	let sink1 = audio.play_file("assets/music.ogg")?;
	let sink2 = audio.play_file("assets/440.wav")?;
	std::thread::sleep(std::time::Duration::from_secs(5));
	println!("stopped first sink");
	sink1.stop();
	std::thread::sleep(std::time::Duration::from_secs(5));
	println!("stopped second sink");
	sink2.stop();
	Ok(())
}