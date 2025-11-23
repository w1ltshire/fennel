use std::{path::PathBuf, rc::Rc};

use image::ImageReader;
use sdl3::{pixels::PixelFormat, render::Texture, surface::Surface};

use crate::{graphics::Graphics, resources::LoadableResource};

unsafe impl Send for Image {}
unsafe impl Sync for Image {}

/// Image asset that can be created either from sdl3's `Surface` for rendering fonts, or from a
/// file for rendering pictures
#[derive(Clone)]
pub struct Image {
    /// Resource name
    pub name: String,
    /// SDL3 texture for caching
    pub texture: Rc<Texture<'static>>,
    /// Image width
    pub width: u32,
    /// Image height
    pub height: u32,
}

impl Image {
    pub fn load_from_surface(
        name: String,
        graphics: &mut Graphics,
        surface: Surface,
    ) -> anyhow::Result<Box<dyn LoadableResource>> {
        let texture = unsafe {
            std::mem::transmute::<sdl3::render::Texture<'_>, sdl3::render::Texture<'static>>(
                graphics
                    .texture_creator
                    .create_texture_from_surface(&surface)?,
            )
        };

        Ok(Box::new(Self {
            name,
            texture: Rc::new(texture),
            width: surface.width(),
            height: surface.height(),
        }))
    }
}

impl LoadableResource for Image {
    /// Construct an `Image` from `path` and return it as a boxed trait object.
    fn load(
        path: PathBuf,
        name: String,
        graphics: &mut Graphics,
        _size: Option<f32>,
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
                graphics
                    .texture_creator
                    .create_texture_from_surface(surface)?,
            )
        };

        Ok(Box::new(Self {
            name,
            texture: Rc::new(texture),
            width: img.width(),
            height: img.height(),
        }))
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
