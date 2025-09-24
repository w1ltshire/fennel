use image::ImageReader;
use sdl3::{
    pixels::PixelFormat,
    render::{Texture, TextureCreator},
    video::WindowContext,
};
use std::{
    cell::{Ref, RefCell},
    path::PathBuf,
    rc::Rc,
    sync::Arc,
};

use crate::resources::LoadableResource;

/// Simple image asset that stores its file location.
pub struct Image {
    /// Filesystem path to the image.
    pub path: PathBuf,
    pub buffer: Rc<RefCell<Vec<u8>>>,
    pub texture: Rc<Texture<'static>>,
    pub width: u32,
    pub height: u32,
}

impl LoadableResource for Image {
    /// Construct an `Image` from `path` and return it as a boxed trait object.
    ///
    /// # Errors
    /// This implementation never fails, but the signature matches the trait.
    fn load(
        path: PathBuf,
        texture_creator: &Arc<TextureCreator<WindowContext>>,
    ) -> anyhow::Result<Box<dyn LoadableResource>> {
        let img = ImageReader::open(&path)?.decode()?;
        let mut buffer = img.to_rgba8().into_raw();
        let surface = sdl3::surface::Surface::from_data(
            &mut buffer,
            img.width(),
            img.height(),
            img.width() * 4,
            PixelFormat::RGBA32,
        )?;

        let texture = unsafe {
            std::mem::transmute::<sdl3::render::Texture<'_>, sdl3::render::Texture<'static>>(
                texture_creator.create_texture_from_surface(surface)?,
            )
        };

        Ok(Box::new(Self {
            path,
            buffer: Rc::new(RefCell::new(buffer)),
            texture: Rc::new(texture),
            width: img.width(),
            height: img.height(),
        }))
    }

    /// Identifier for the asset is the the string representation of its path.
    fn name(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    fn as_mut_slice(&self) -> &mut [u8] {
        let mut mut_ref = self.buffer.borrow_mut();
        // even more evil shit that PROBABLY :) should be safe because as we know in normal conditions only
        // one thread should access (graphics, audio, ..) its respecting resources
        // otherwise have a SEGFAULT >:3
        unsafe { &mut *(mut_ref.as_mut_slice() as *mut [u8]) }
    }

    fn as_slice(&self) -> Ref<'_, [u8]> {
        Ref::map(self.buffer.borrow(), |v| v.as_slice())
    }
}
