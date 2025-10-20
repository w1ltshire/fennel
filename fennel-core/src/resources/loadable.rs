use anyhow::bail;
use image::ImageReader;
use sdl3::{pixels::PixelFormat, render::Texture, surface::Surface};
use std::{
    path::PathBuf,
    rc::Rc,
};

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
    /// Image heiht
    pub height: u32,
}

/// Simple font asset
pub struct Font {
    /// Filesystem path to the font.
    pub path: PathBuf,
    /// Font family name
    pub family_name: String,
    /// Point size
    pub size: f32,
    /// Vector of bytes containing the font data
    pub buffer: Rc<sdl3::ttf::Font<'static>>,
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
            name: path.to_string_lossy().to_string(),
            texture: Rc::new(texture),
            width: img.width(),
            height: img.height(),
        }))
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl LoadableResource for Font {
    fn load(
        path: PathBuf,
        graphics: &mut Graphics,
        size: Option<f32>,
    ) -> anyhow::Result<Box<dyn LoadableResource>>
    where
        Self: Sized,
    {
        if size.is_none() {
            bail!("no font size was provided");
        }

        let font = graphics.ttf_context.load_font(&path, size.unwrap())?;
        Ok(Box::new(Self {
            path,
            family_name: font
                .face_family_name()
                .expect("failed to get font family name"),
            size: size.unwrap(),
            buffer: Rc::new(font),
        }))
    }

    fn name(&self) -> String {
        format!("{}|{}", self.path.to_string_lossy(), self.size)
    }
}
