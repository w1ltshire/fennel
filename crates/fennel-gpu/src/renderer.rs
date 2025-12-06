//! This module provides the [`GPURenderer`] structure, which allows for various GPU-related operations.
//! [`renderer`] is the main module you'll need to do GPU rendering.
//!
//! # Example
//! ```ignore
//! use fennel_gpu::renderer::GPURenderer;
//! fn main() -> anyhow::Result<()> {
//!     let gpu_device = sdl3::gpu::Device::new(sdl3::gpu::ShaderFormat::SPIRV, true)?;
//!     let mut renderer = GPURenderer::new(gpu_device)?;
//!     Ok(())
//!}
//! ```

use std::ffi::{CStr, CString};
use std::path::Path;
use anyhow::bail;
use sdl3::gpu::{CommandBuffer, Device, Texture, TextureCreateInfo, TextureFormat, TextureRegion, TextureTransferInfo, TextureType, TextureUsage, TransferBufferUsage};
use sdl3::surface::Surface;
use sdl3::sys::error::SDL_GetError;

/// A structure representing the GPU renderer.
pub struct GPURenderer {
	device: Device,
	command_buffer: CommandBuffer,
}

impl GPURenderer {
	/// Creates a new [`GPURenderer`] instance, taking ownership of the provided GPU device.
	///
	/// # Arguments
	/// * `device`: instance of [`Device`]
	pub fn new(device: Device) -> anyhow::Result<Self> {
		let command_buffer = device.acquire_command_buffer()?;
		Ok(Self { device, command_buffer })
	}

	/// Creates a [`Texture`] from a specified image file path.
	///
	/// This function takes an image file at `image_path` and creates a GPU
	/// texture from it.
	///
	/// # Parameters
	/// - `image_path`: A path to the image file
	/// - `gpu`: A reference to the GPU device to create the texture
	/// - `copy_pass`: A reference to the `CopyPass` to load the texture into GPU
	///
	/// # Returns
	/// Returns a result containing the created [`Texture`] on success, or an error
	/// if anything fails.
	///
	/// # Safety
	/// This function interacts with raw pointers and C functions. Ensure the validity
	/// of an image file path, tho an error will be returned if any of the pointers are null.
	///
	/// # Example
	/// ```ignore
	/// let texture = fennel_gpu::renderer::GPURenderer::create_texture_from_image("assets/Sprite-0001.png")?;
	/// Ok(())
	/// ```
	pub fn create_texture_from_image(
		&mut self,
		image_path: impl AsRef<Path>
	) -> anyhow::Result<Texture<'static>> {
		let surface = unsafe { self.load_surface(image_path)? };
		let image_size = surface.size();
		let size_bytes = surface.pixel_format().bytes_per_pixel() as u32 * image_size.0 * image_size.1;

		let texture = self.device.create_texture(
			TextureCreateInfo::new()
				.with_format(TextureFormat::R8g8b8a8Unorm)
				.with_type(TextureType::_2D)
				.with_width(image_size.0)
				.with_height(image_size.1)
				.with_layer_count_or_depth(1)
				.with_num_levels(1)
				.with_usage(TextureUsage::SAMPLER),
		)?;

		let transfer_buffer = self.device
			.create_transfer_buffer()
			.with_size(size_bytes)
			.with_usage(TransferBufferUsage::UPLOAD)
			.build()?;

		let mut buffer_mem = transfer_buffer.map::<u8>(&self.device, false);
		surface.with_lock(|image_bytes| {
			buffer_mem.mem_mut().copy_from_slice(image_bytes);
		});
		buffer_mem.unmap();

		let copy_pass = self.device.begin_copy_pass(&self.command_buffer)?;

		copy_pass.upload_to_gpu_texture(
			TextureTransferInfo::new()
				.with_transfer_buffer(&transfer_buffer)
				.with_offset(0),
			TextureRegion::new()
				.with_texture(&texture)
				.with_layer(0)
				.with_width(image_size.0)
				.with_height(image_size.1)
				.with_depth(1),
			false,
		);

		self.device.end_copy_pass(copy_pass);

		Ok(texture)
	}

	unsafe fn load_surface(&mut self, image_path: impl AsRef<Path>) -> anyhow::Result<Surface<'static>> {
		// pray to the Machine God so all those unsafe blocks won't cause an UB or segfault
		// Hail, Spirit of the Machine, Essence Divine; In your code and circuitry, the stars align.
		// By the Omnissiah's will, we commune and bind, Data sanctified, logic refined.

		let c_string = CString::new(image_path.as_ref().to_str().unwrap())?; // this `unwrap` is ass. session terminated
		let path_ptr = c_string.as_ptr();
		let raw_surface = unsafe { sdl3_image_sys::image::IMG_Load(path_ptr) };

		if raw_surface.is_null() {
			let error = unsafe { CStr::from_ptr(SDL_GetError()) };
			bail!("surface pointer is null: {}", error.to_str()?);
		}

		Ok(unsafe { Surface::from_ll(raw_surface) })
	}
}