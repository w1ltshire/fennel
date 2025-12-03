use fennel_gpu::image;

use sdl3::gpu::ShaderFormat;

fn main() -> anyhow::Result<()> {
	let sdl_context = sdl3::init()?;
	let video_subsystem = sdl_context.video()?;

	let window = video_subsystem
		.window("rust-sdl3 demo: Video", 800, 600)
		.position_centered()
		.opengl()
		.build()?;

	let gpu = sdl3::gpu::Device::new(
		ShaderFormat::SPIRV | ShaderFormat::DXIL | ShaderFormat::DXBC | ShaderFormat::METALLIB,
		true,
	)?
		.with_window(&window)?;

	let copy_commands = gpu.acquire_command_buffer()?;
	let copy_pass = gpu.begin_copy_pass(&copy_commands)?;

	let _image = image::create_texture_from_image("assets/Sprite-0001.png", &gpu, &copy_pass)?;

	Ok(())
}