use log::debug;
use sdl3::gpu::ShaderFormat;
use sdl3::Sdl;
use crate::renderer::GPURenderer;

/// Structure representing the window, wrapping video subsystem functions and owning the graphics
pub struct Window {
	pub window: sdl3::video::Window, // i swear its temporary
	pub sdl_context: Sdl,
	pub renderer: GPURenderer
}

pub struct WindowBuilder {
	title: String,
	dimensions: (u32, u32),
	is_fullscreen: bool,
	is_maximized: bool,
	is_resizable: bool,
	is_centered: bool,
}

impl WindowBuilder {
	/// Creates a new [`WindowBuilder`] with all boolean parameters set to `false`, title is empty, dimensions is `(0, 0)`
	pub fn new() -> WindowBuilder {
		Self {
			title: "".to_string(),
			dimensions: (0, 0),
			is_maximized: false,
			is_fullscreen: false,
			is_resizable: false,
			is_centered: false,
		}
	}

	pub fn title(mut self, title: &str) -> Self {
		self.title = title.to_string();
		self
	}

	pub fn dimensions(mut self, dimensions: (u32, u32)) -> Self {
		self.dimensions = dimensions;
		self
	}

	pub fn fullscreen(mut self, is_fullscreen: bool) -> Self {
		self.is_fullscreen = is_fullscreen;
		self
	}

	pub fn maximized(mut self, is_maximized: bool) -> Self {
		self.is_maximized = is_maximized;
		self
	}

	pub fn resizable(mut self, is_resizable: bool) -> Self {
		self.is_resizable = is_resizable;
		self
	}

	pub fn centered(mut self, is_centered: bool) -> Self {
		self.is_centered = is_centered;
		self
	}

	pub fn build(self) -> anyhow::Result<Window> {
		let window = Window::new(
			self.dimensions,
			self.title,
			self.is_fullscreen,
			self.is_maximized,
			self.is_resizable,
			self.is_centered,
		);
		window
	}
}

impl Window {
	/// Creates a new instance of [`Window`]
	///
	/// This function initializes `sdl3` context and acquires video subsystem from it, then creates an `sdl3` window (different from [`Window`]).
	///
	/// # Arguments
	/// * `dimensions`: the window dimensions (width, height)
	/// * `title`: window's title
	pub fn new(
		dimensions: (u32, u32),
		title: String,
		is_fullscreen: bool,
		is_maximized: bool,
		is_resizable: bool,
		is_centered: bool,
	) -> anyhow::Result<Self> {
		let sdl_context = sdl3::init()?;
		let video_subsystem = sdl_context.video()?;

		let mut builder = video_subsystem.window(&title, dimensions.0, dimensions.1);

		let _ = if is_centered {
			builder.position_centered()
		} else {
			&mut builder
		};
		let _ = if is_resizable {
			builder.resizable()
		} else {
			&mut builder
		};
		let _ = if is_fullscreen {
			builder.fullscreen()
		} else {
			&mut builder
		};
		let _ = if is_maximized {
			builder.maximized()
		} else {
			&mut builder
		};

		let window = builder.build()?;

		let gpu = sdl3::gpu::Device::new(
			ShaderFormat::SPIRV | ShaderFormat::DXIL | ShaderFormat::DXBC | ShaderFormat::METALLIB,
			true,
		)?
			.with_window(&window)?;
		let swapchain_format = gpu.get_swapchain_texture_format(&window);
		let renderer = GPURenderer::new(gpu, swapchain_format)?;

		debug!("window ready with title {title} and dimensions {dimensions:?}");

		Ok(Self {
			window,
			sdl_context,
			renderer,
		})
	}
}