use sdl3::gpu::ShaderFormat;
use fennel_gpu::renderer::GPURenderer;

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

	let mut renderer = GPURenderer::new(gpu)?;

	let _image = renderer.create_texture_from_image("assets/Sprite-0001.png")?;

	Ok(())
}