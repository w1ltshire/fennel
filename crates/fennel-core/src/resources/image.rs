use std::{path::PathBuf, rc::Rc};
use std::any::Any;
use image::ImageReader;
use sdl3::{pixels::PixelFormat, render::Texture, surface::Surface};
use fennel_resources::resource::Resource;
use crate::graphics::Graphics;

unsafe impl Send for Image {}
unsafe impl Sync for Image {}

/// Image asset that can be created either from sdl3's `Surface` for rendering fonts, or from a
/// file for rendering pictures
#[derive(Clone)]
pub struct Image {
    inner: Rc<InnerImage>,
    name: String
}

#[derive(Clone)]
pub(crate) struct InnerImage {
    /// SDL3 texture for caching
    pub(crate) texture: Rc<Texture<'static>>,
    /// Image width
    pub(crate) width: u32,
    /// Image height
    pub(crate) height: u32,
}

impl Resource for Image {
    fn data(&self) -> &dyn Any {
        &self.inner as &dyn Any
    }

    fn data_mut(&mut self) -> &mut dyn Any {
        &mut self.inner as &mut dyn Any
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Image {
    pub fn load_from_surface(
        name: String,
        graphics: &mut Graphics,
        surface: Surface,
    ) -> anyhow::Result<Self> {
        let texture = unsafe {
            std::mem::transmute::<Texture<'_>, Texture<'static>>(
                graphics
                    .texture_creator
                    .create_texture_from_surface(&surface)?,
            )
        };
        let inner = InnerImage {
            texture: Rc::new(texture),
            width: surface.width(),
            height: surface.height(),
        };

        Ok(Self {
            name,
            inner: Rc::new(inner),
        })
    }

    pub fn load(
        path: PathBuf,
        name: String,
        graphics: &mut Graphics,
    ) -> anyhow::Result<Self> {
        let img = ImageReader::open(&path)?.decode()?;
        let mut buffer = img.to_rgba8().into_raw();
        let surface = Surface::from_data(
            &mut buffer,
            img.width(),
            img.height(),
            img.width() * 4,
            PixelFormat::RGBA32,
        )?;

        let texture = unsafe {
            std::mem::transmute::<Texture<'_>, Texture<'static>>(
                graphics
                    .texture_creator
                    .create_texture_from_surface(surface)?,
            )
        };
        let width = texture.width().clone();
        let height = texture.height().clone();

        let inner = InnerImage {
            texture: Rc::new(texture),
            width,
            height,
        };

        Ok(Self {
            name,
            inner: Rc::new(inner),
        })
    }

    pub fn height(&self) -> u32 {
        self.inner.height
    }

    pub fn width(&self) -> u32 {
        self.inner.width
    }
    
    pub fn texture(&self) -> Rc<Texture<'static>> {
        self.inner.texture.clone()
    }
}