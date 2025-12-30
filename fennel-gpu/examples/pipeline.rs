use sdl3::gpu::ShaderStage;
use fennel_gpu::windowing::WindowBuilder;

fn main() -> anyhow::Result<()> {
	env_logger::init();
	let mut window = WindowBuilder::new()
		.title("test")
		.dimensions((640, 480))
		.build()?;

	let vert_shader = window.renderer.create_shader(
		c"main",
		ShaderStage::Vertex,
		include_bytes!(concat!(env!("OUT_DIR"), "/vertex.glsl.spv")),
		1)?;
	let frag_shader = window.renderer.create_shader(
		c"main",
		ShaderStage::Fragment,
		include_bytes!(concat!(env!("OUT_DIR"), "/fragment.glsl.spv")),
		1)?;

	let _pipeline = window.renderer.create_pipeline(&frag_shader, &vert_shader)?;

	Ok(())
}