use fennel_gpu::windowing::WindowBuilder;

fn main() -> anyhow::Result<()> {
	let _window = WindowBuilder::new()
		.title("test")
		.dimensions((640, 480))
		.build()?;

	Ok(())
}