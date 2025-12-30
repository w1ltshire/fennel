use sdl3::gpu::ShaderFormat;
use fennel_gpu::renderer::GPURenderer;

fn main() -> anyhow::Result<()> {
	env_logger::init();
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

	let swapchain_format = gpu.get_swapchain_texture_format(&window);
	let mut renderer = GPURenderer::new(gpu, swapchain_format)?;
	let command_buffer = renderer.device.acquire_command_buffer()?;
	let _image = renderer.create_texture_from_image("assets/Sprite-0001.png", &command_buffer)?;

	Ok(())
}