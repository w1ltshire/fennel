use std::ffi::{CStr, CString};
use std::path::Path;
use anyhow::bail;
use sdl3::gpu::{CopyPass, Device, Texture, TextureCreateInfo, TextureFormat, TextureRegion, TextureTransferInfo, TextureType, TextureUsage, TransferBufferUsage};
use sdl3::surface::Surface;
use sdl3::sys::error::SDL_GetError;

/// Creates a [`Texture`] from a path
pub fn create_texture_from_image(
	image_path: impl AsRef<Path>,
	gpu: &Device,
	copy_pass: &CopyPass,
) -> anyhow::Result<Texture<'static>> {
	// pray to the Machine God so all those unsafe blocks won't cause an UB or segfault
	// Hail, Spirit of the Machine, Essence Divine, In your code and circuitry, the stars align.
	// By the Omnissiah's will, we commune and bind, Data sanctified, logic refined.

	let c_string = CString::new(image_path.as_ref().to_str().unwrap())?; // this unwrap is ass. session terminated
	let path_ptr = c_string.as_ptr();
	let raw_surface = unsafe { sdl3_image_sys::image::IMG_Load(path_ptr) };

	if raw_surface.is_null() {
		let error = unsafe { CStr::from_ptr(SDL_GetError()) };
		bail!("surface pointer is null: {}", error.to_str()?);
	}

	let surface = unsafe { Surface::from_ll(raw_surface) };
	let image_size = surface.size();
	let size_bytes = surface.pixel_format().bytes_per_pixel() as u32 * image_size.0 * image_size.1;

	let texture = gpu.create_texture(
		TextureCreateInfo::new()
			.with_format(TextureFormat::R8g8b8a8Unorm)
			.with_type(TextureType::_2D)
			.with_width(image_size.0)
			.with_height(image_size.1)
			.with_layer_count_or_depth(1)
			.with_num_levels(1)
			.with_usage(TextureUsage::SAMPLER),
	)?;

	let transfer_buffer = gpu
		.create_transfer_buffer()
		.with_size(size_bytes)
		.with_usage(TransferBufferUsage::UPLOAD)
		.build()?;

	let mut buffer_mem = transfer_buffer.map::<u8>(gpu, false);
	surface.with_lock(|image_bytes| {
		buffer_mem.mem_mut().copy_from_slice(image_bytes);
	});
	buffer_mem.unmap();

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

	Ok(texture)
}