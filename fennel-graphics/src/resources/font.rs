use anyhow::bail;
use std::{path::PathBuf, rc::Rc};
use std::any::Any;
use fennel_resources::resource::Resource;
use crate::graphics::Graphics;

/// Font asset
pub struct InternalFont {
    /// Filesystem path to the font.
    pub path: PathBuf,
    /// Font family name
    pub family_name: String,
    /// Point size
    pub size: f32,
    /// Smart pointer to sdl3's `Font`
    pub buffer: Rc<sdl3::ttf::Font<'static>>,
}

pub struct Font {
    internal: Rc<InternalFont>,
    name: String,
}

/// Font asset to be able to use fonts of various sizes
pub struct DummyFont {
    inner: InternalDummyFont
}

pub(crate) struct InternalDummyFont {
    /// Filesystem path to the font.
    pub(crate) path: PathBuf,
    /// Internal name
    pub(crate) name: String,
}

impl Resource for DummyFont {
    fn data(&self) -> &dyn Any {
        &self.inner as &dyn Any
    }

    fn data_mut(&mut self) -> &mut dyn Any {
        &mut self.inner as &mut dyn Any
    }

    fn name(&self) -> String {
        self.inner.name.clone()
    }
}

impl Resource for Font {
    fn data(&self) -> &dyn Any {
        &self.internal as &dyn Any
    }

    fn data_mut(&mut self) -> &mut dyn Any {
        &mut self.internal as &mut dyn Any
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Font {
    pub(crate) fn load(
        path: PathBuf,
        name: String,
        graphics: &mut Graphics,
        size: Option<f32>,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let size = match size {
            Some(size) => size,
            None => bail!("Font size not provided"),
        };
        let font = graphics
            .ttf_context
            .load_font(&path, size)?;
        let family_name = match font.face_family_name() {
            Some(name) => name,
            None => bail!("failed to get font family name"),
        };

        let internal = InternalFont {
            path,
            family_name,
            size,
            buffer: Rc::new(font),
        };

        Ok(Self {
            internal: Rc::new(internal),
            name
        })
    }
}

impl DummyFont {
    pub fn new(path: PathBuf, name: String) -> Self {
        Self {
            inner: InternalDummyFont {
                path,
                name,
            }
        }
    }
}