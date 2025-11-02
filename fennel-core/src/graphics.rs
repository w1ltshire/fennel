//! SDL3-backed graphics helper
//!
//! Provides:
//! - `Graphics`: owned SDL context + drawing canvas
//! - `Graphics::new(...)`: initialize SDL, create a centered resizable window and return [`Graphics`]
//!

use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use sdl3::Sdl;
use sdl3::pixels::{Color, PixelFormat};
use sdl3::render::{Canvas, FRect};
use sdl3::video::Window;

use quick_error::ResultExt;

use crate::resources::font::{DummyFont, Font};
use crate::resources::image::Image;
use crate::resources::{self, LoadableResource, ResourceManager};

pub trait HasWindow {
    fn window_mut(&mut self) -> &mut crate::Window;
}

/// Owned SDL variables used for rendering
///
/// - `canvas`: the drawing surface for the window
/// - `sdl_context`: the SDL context
pub struct Graphics {
    /// The SDL3 canvas, required to draw
    pub canvas: Canvas<Window>,
    /// SDL3 contaxt
    pub sdl_context: Sdl,
    /// SDL3 texture creator
    pub texture_creator: Rc<sdl3::render::TextureCreator<sdl3::video::WindowContext>>,
    /// SDL3 TTF context required for text rendering
    pub ttf_context: sdl3::ttf::Sdl3TtfContext,
    /// Reference to [`resources::ResourceManager`]
    resource_manager: Arc<Mutex<ResourceManager>>,
}

/// Config for [`Graphics::new`] and for [`GraphicsBuilder`]
#[derive(Default, Debug, Clone, Copy)]
pub struct WindowConfig {
    /// Is the window resizable?
    pub resizable: bool,
    /// Is the window fullscreen?
    pub fullscreen: bool,
    /// Is the window centered by default?
    pub centered: bool,
}

/// Builder for creating a Graphics instance.
pub struct GraphicsBuilder<F>
where
    F: Fn(&mut Graphics),
{
    resource_manager: Option<Arc<Mutex<ResourceManager>>>,
    dimensions: (u32, u32),
    name: String,
    initializer: Option<F>,
    config: WindowConfig,
}

impl<F> GraphicsBuilder<F>
where
    F: Fn(&mut Graphics),
{
    /// Create a new empty GraphicsBuilder
    /// By default there is no resource manager or resource initializer, dimensions are 0, 0, name
    /// is empty
    pub fn new() -> GraphicsBuilder<F> {
        GraphicsBuilder {
            resource_manager: None,
            dimensions: (0, 0),
            name: "".to_string(),
            initializer: None,
            config: WindowConfig {
                resizable: false,
                fullscreen: false,
                centered: false,
            },
        }
    }

    /// Set the resource manager
    pub fn resource_manager(
        mut self,
        resource_manager: Arc<Mutex<ResourceManager>>,
    ) -> GraphicsBuilder<F> {
        self.resource_manager = Some(resource_manager);
        self
    }

    /// Set the window dimensions
    pub fn dimensions(mut self, dimensions: (u32, u32)) -> GraphicsBuilder<F> {
        self.dimensions = dimensions;
        self
    }

    /// Set the window name
    pub fn window_name(mut self, name: String) -> GraphicsBuilder<F> {
        self.name = name;
        self
    }

    /// Set the resource initializer (closure)
    pub fn initializer(mut self, initializer: F) -> GraphicsBuilder<F>
    where
        F: Fn(&mut Graphics),
    {
        self.initializer = Some(initializer);
        self
    }

    /// Will the window be resizable?
    pub fn resizable(mut self, resizable: bool) -> GraphicsBuilder<F> {
        self.config.resizable = resizable;
        self
    }

    /// Will the window be fullscreen?
    pub fn fullscreen(mut self, fullscreen: bool) -> GraphicsBuilder<F> {
        self.config.fullscreen = fullscreen;
        self
    }

    /// Will the window be centered?
    pub fn centered(mut self, centered: bool) -> GraphicsBuilder<F> {
        self.config.centered = centered;
        self
    }

    /// Build `Graphics`
    ///
    /// # Panic
    /// Panics if no resource manager or initializer was provided
    pub fn build(self) -> anyhow::Result<Graphics> {
        Ok(Graphics::new(
            self.name,
            self.dimensions,
            self.resource_manager.expect("no resource manager provided"),
            self.initializer.expect("no resource initializer provided"),
            self.config,
        )
        .unwrap())
    }
}

impl<F> Default for GraphicsBuilder<F>
where
    F: Fn(&mut Graphics),
{
    /// Default implementation delegates to `[GraphicsBuilder::new]`
    fn default() -> Self {
        Self::new()
    }
}

impl Graphics {
    /// Initialize SDL3, create a centered, resizable window and return a [`Graphics`]
    /// container with the canvas and SDL context.
    ///
    /// # Parameters
    /// - `name`: Window title.
    /// - `dimensions`: (width, height) in pixels (u32).
    ///
    /// # Returns
    /// - `Ok(Graphics)` on success.
    /// - `Err(Box<dyn std::error::Error>)` on failure (window/canvas build error).
    ///
    /// # Example
    /// ```ignore
    /// let graphics = graphics::new(String::from("my cool game"), (500, 500), resource_manager, |_| {}, config)?;
    /// ```
    pub fn new<F>(
        name: String,
        dimensions: (u32, u32),
        resource_manager: Arc<Mutex<ResourceManager>>,
        resource_initialization: F,
        config: WindowConfig,
    ) -> Result<Graphics, Box<dyn std::error::Error>>
    where
        F: Fn(&mut Graphics),
    {
        // TODO: allow the user to uh customize video_subsystem configuration 'cuz man this is ass why
        // do we position_centered() and resizable() it by default

        let sdl_context = sdl3::init()?;
        let ttf_context = sdl3::ttf::init().map_err(|e| e.to_string())?;
        let video_subsystem = sdl_context.video()?;

        let mut builder = video_subsystem.window(&name, dimensions.0, dimensions.1);

        let _ = if config.centered {
            builder.position_centered()
        } else {
            &mut builder
        };
        let _ = if config.resizable {
            builder.resizable()
        } else {
            &mut builder
        };
        let _ = if config.fullscreen {
            builder.fullscreen()
        } else {
            &mut builder
        };

        let window = builder.build().map_err(|e| e.to_string())?;

        let canvas = window.into_canvas();
        let texture_creator = canvas.texture_creator();
        let mut graphics = Graphics {
            canvas,
            sdl_context,
            texture_creator: Rc::new(texture_creator),
            ttf_context,
            resource_manager,
        };

        resource_initialization(&mut graphics);

        Ok(graphics)
    }

    /// Draw a rectangle on position (x, y) with dimensions (w, h)
    pub fn draw_rect(
        &mut self,
        w: f32,
        h: f32,
        x: f32,
        y: f32
    ) -> anyhow::Result<()> {
        self.canvas.draw_rect(FRect { x, y, w, h })?;
        Ok(())
    }

    /// Draw a rectangle on position (x, y) with dimensions (w, h)
    pub fn draw_line(
        &mut self,
        p1: (f32, f32),
        p2: (f32, f32)
    ) -> anyhow::Result<()> {
        self.canvas.draw_line(p1, p2)?;
        Ok(())
    }

    /// Cache an image if it isn't cached and draw it on the canvas
    ///
    /// # Returns
    /// - `Ok(())` on success.
    /// - `Err(Box<dyn std::error::Error>)` on failure
    ///
    /// # Example
    /// ```ignore
    /// graphics.draw_image(String::from("examples/example.png"), (0.0, 0.0), 0.0, false, false).await;
    /// ```
    pub fn draw_image(
        &mut self,
        path: String,
        position: (f32, f32),
        rotation: f64,
        flip_horizontal: bool,
        flip_vertical: bool,
    ) -> anyhow::Result<()> {
        let manager = self.resource_manager.clone();
        let mut manager = manager
            .try_lock()
            .context("failed to lock resource_manager")
            .unwrap();

        if !manager.is_cached(path.clone()) {
            // rust programmers when they have to .clone()
            let texture = Image::load(PathBuf::from(path.clone()), "".to_string(), self, None);
            manager.cache_asset(texture?)?; // those question marks are funny hehehe
        }

        let image: &Image = resources::downcast_ref(manager.get_asset(path).unwrap())?;

        let dst_rect = FRect::new(
            position.0,
            position.1,
            image.width as f32,
            image.height as f32,
        );

        self.canvas
            .copy_ex(
                &image.texture,
                None,
                Some(dst_rect),
                rotation,
                None,
                flip_horizontal,
                flip_vertical,
            )
            .unwrap();

        Ok(())
    }

    /// Create a texture from font + text and render it on the canvas
    pub fn draw_text(
        &mut self,
        text: String,
        position: (f32, f32),
        font: String,
        color: Color,
        size: f32,
    ) -> anyhow::Result<()> {
        let manager = self.resource_manager.clone();
        let mut manager = manager
            .try_lock()
            .context("failed to lock resource_manager")
            .unwrap();

        // dumbass solution. either way, i see no other solution to this.
        // as sdl3 requires us to create a texture from font to draw text,
        // we will be caching it as an [`resources::loadable::Image`] under this key
        let cache_key = format!(
            "{}|{}|{}|{:x?}",
            font,
            text,
            size,
            color.to_u32(&PixelFormat::RGBA32)
        );
        let font: &DummyFont = {
            let asset = manager.get_asset(font)?;
            resources::downcast_ref(asset)?
        };

        let font_key = format!("{}|{}", font.name(), size);

        if !manager.is_cached(font_key.clone()) {
            let asset = Font::load(font.path.clone(), font_key.clone(), self, Some(size));
            manager.cache_asset(asset?)?;
        }

        if !manager.is_cached(cache_key.clone()) {
            let font = resources::downcast_ref::<Font>(manager.get_asset(font_key)?)?;
            let surface = font
                .buffer
                .render(&text)
                .blended(color)
                .map_err(|e| anyhow::anyhow!("render error: {e}"))?;
            let image = Image::load_from_surface(cache_key.clone(), self, surface);
            manager.cache_asset(image?)?;
        }
        let texture: &Image = resources::downcast_ref(manager.get_asset(cache_key)?)?;

        let dst_rect = FRect::new(
            position.0,
            position.1,
            texture.width as f32,
            texture.height as f32,
        );
        self.canvas
            .copy_ex(
                &texture.texture,
                None,
                Some(dst_rect),
                0.0,
                None,
                false,
                false,
            )
            .unwrap();
        Ok(())
    }
}
