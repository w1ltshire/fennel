use anyhow::bail;
use std::{
    path::PathBuf,
    rc::Rc,
};

use crate::{graphics::Graphics, resources::LoadableResource};

/// Font asset
pub struct Font {
    /// Filesystem path to the font.
    pub path: PathBuf,
    /// Font family name
    pub family_name: String,
    /// Internal name
    name: String,
    /// Point size
    pub size: f32,
    /// Smart pointer to sdl3's `Font`
    pub buffer: Rc<sdl3::ttf::Font<'static>>,
}

/// Font asset to be able to use fonts of various sizes
pub struct DummyFont {
    /// Filesystem path to the font.
    pub path: PathBuf,
    /// Internal name
    name: String,
}

impl LoadableResource for DummyFont {
    fn load(
        path: PathBuf,
        name: String,
        _graphics: &mut Graphics,
        _size: Option<f32>,
    ) -> anyhow::Result<Box<dyn LoadableResource>>
    where
        Self: Sized, {

        Ok(Box::new(Self {
            path,
            name,
        }))
    }

    fn name(&self) -> String {
        self.name.to_string()
    }
}
impl LoadableResource for Font {
    // TODO: improve font internal naming, it's quite confusing as of now
    fn load(
        path: PathBuf,
        name: String,
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
            name,
            size: size.unwrap(),
            buffer: Rc::new(font),
        }))
    }

    fn name(&self) -> String {
        self.name.to_string()
    }
}
