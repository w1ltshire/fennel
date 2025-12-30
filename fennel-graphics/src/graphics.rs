//! SDL3-backed graphics helper
//!
//! Provides:
//! - `Graphics`: owned SDL context + drawing canvas
//! - `Graphics::new(...)`: initialize SDL, create a centered resizable window and return [`Graphics`]
//!

use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use anyhow::Context;
use sdl3::Sdl;
use sdl3::pixels::{Color, PixelFormat};
use sdl3::render::{Canvas, FRect};
use sdl3::video::Window;
use serde::Deserialize;
use fennel_resources::manager::ResourceManager;
use crate::resources::font::{Font, InternalDummyFont, InternalFont};
use crate::resources::image::{Image, InnerImage};

/// Owned SDL variables used for rendering
///
/// - `canvas`: the drawing surface for the window
/// - `sdl_context`: the SDL context
pub struct Graphics {
    /// The SDL3 canvas, required to draw
    pub canvas: Canvas<Window>,
    /// SDL3 context
    pub sdl_context: Sdl,
    /// SDL3 texture creator
    pub texture_creator: Rc<sdl3::render::TextureCreator<sdl3::video::WindowContext>>,
    /// SDL3 TTF context required for text rendering
    pub ttf_context: sdl3::ttf::Sdl3TtfContext,
    /// Reference to [`ResourceManager`]
    resource_manager: Arc<Mutex<ResourceManager>>,
}

/// Config for [`Graphics::new`] and for [`GraphicsBuilder`]
#[derive(Default, Debug, Clone, Copy)]
pub struct WindowConfig {
    /// Is the window resizable?
    pub is_resizable: bool,
    /// Is the window fullscreen?
    pub is_fullscreen: bool,
    /// Is the window centered by default?
    pub is_centered: bool,
}

/// Builder for creating a Graphics instance.
pub struct GraphicsBuilder<F>
where
    F: Fn(&mut Graphics) -> anyhow::Result<()>,
{
    resource_manager: Option<Arc<Mutex<ResourceManager>>>,
    dimensions: (u32, u32),
    name: String,
    initializer: Option<F>,
    config: WindowConfig,
}


/// A drawable primitive that can be queued for rendering
///
/// Variants:
/// - Image(Sprite) - a sprite to draw
/// - Rect { w, h, x, y } - a rectangle specified with width, height, x and y position (all `f32`)
#[derive(Debug)]
pub enum Drawable {
    /// A sprite. Use for queueing render of some image
    Image(Sprite),
    /// A basic rectangle
    Rect { w: f32, h: f32, x: f32, y: f32 },
    /// Text drawable.
    ///
    /// # Fields
    /// * `font`: Font name registered in the resource manager
    /// * `position`: Position in `(f32, f32)` relative to the window
    /// * `text`: The text itself to render
    /// * `color`: RGB tuple of `u8`
    /// * `size`: Font size in `f32`
    Text { font: String, position: (f32, f32), text: String, color: (u8, u8, u8), size: f32 },
}

/// A simple renderable sprite.
///
/// # Fields
/// - image: identifier or path of the image to draw
/// - position: tuple (x, y) position on screen
#[derive(Deserialize, Debug, Clone)]
pub struct Sprite {
    /// Sprite asset id in the resource manager
    pub image: String,
    /// Representing sprite's transformation in the 2D world
    pub transform: Transform,
    /// Is this sprite fixed on screen? (not affected by camera)
    pub fixed: bool
}

impl specs::Component for Sprite {
    type Storage = specs::VecStorage<Self>;
}


/// Transform component, containing position in the window, scale and rotation
#[derive(Deserialize, Debug, Clone)]
pub struct Transform {
    /// Position in the window (x, y)
    pub position: (f32, f32),
    /// Scale
    pub scale: f64,
    /// Rotation
    pub rotation: f64,
}

impl specs::Component for Transform {
    type Storage = specs::VecStorage<Self>;
}

impl Sprite {
    /// Creates a new instance of [`Sprite`]
    ///
    /// # Arguments
    /// * `image`: [`String`] identifier of the image in the resource manager
    /// * `transform`: [`Transform`] of the sprite (position, scale, rotation)
    /// * `fixed`: is this sprite fixed on the screen?
    pub fn new(image: String, transform: Transform, fixed: bool) -> Self {
        Self {
            image,
            transform,
            fixed
        }
    }
}

impl Transform {
    /// Creates a new instance of [`Transform`]
    pub fn new(position: (f32, f32), scale: f64, rotation: f64) -> Self {
        Self {
            position,
            scale,
            rotation
        }
    }
}

impl<F> GraphicsBuilder<F>
where
    F: Fn(&mut Graphics) -> anyhow::Result<()>,
{
    /// Create a new empty GraphicsBuilder
    /// By default there is no resource manager or resource initializer; dimensions are (0, 0), name
    /// is empty
    pub fn new() -> GraphicsBuilder<F> {
        GraphicsBuilder {
            resource_manager: None,
            dimensions: (0, 0),
            name: "".to_string(),
            initializer: None,
            config: WindowConfig {
                is_resizable: false,
                is_fullscreen: false,
                is_centered: false,
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
        F: Fn(&mut Graphics) -> anyhow::Result<()>,
    {
        self.initializer = Some(initializer);
        self
    }

    /// Will the window be resizable?
    pub fn resizable(mut self, resizable: bool) -> GraphicsBuilder<F> {
        self.config.is_resizable = resizable;
        self
    }

    /// Will the window be fullscreen?
    pub fn fullscreen(mut self, fullscreen: bool) -> GraphicsBuilder<F> {
        self.config.is_fullscreen = fullscreen;
        self
    }

    /// Will the window be centered?
    pub fn centered(mut self, centered: bool) -> GraphicsBuilder<F> {
        self.config.is_centered = centered;
        self
    }

    /// Build `Graphics`
    ///
    /// # Panic
    /// Panics if no resource manager or initializer was provided
    pub fn build(self) -> anyhow::Result<Graphics> {
        let resource_manager = match self.resource_manager {
            Some(resource_manager) => resource_manager,
            None => return Err(anyhow::anyhow!("no resource manager supplied")),
        };

        let initializer = match self.initializer {
            Some(initializer) => initializer,
            None => return Err(anyhow::anyhow!("no initializer supplied")),
        };

        Graphics::new(
            self.name,
            self.dimensions,
            resource_manager,
            initializer,
            self.config,
        )
    }
}

impl<F> Default for GraphicsBuilder<F>
where
    F: Fn(&mut Graphics) -> anyhow::Result<()>,
{
    /// Default implementation delegates to [`GraphicsBuilder::new`]
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
    ) -> anyhow::Result<Graphics>
    where
        F: Fn(&mut Graphics) -> anyhow::Result<()>,
    {
        let sdl_context = sdl3::init()?;
        let ttf_context = sdl3::ttf::init()?;
        let video_subsystem = sdl_context.video()?;

        let mut builder = video_subsystem.window(&name, dimensions.0, dimensions.1);

        let _ = if config.is_centered {
            builder.position_centered()
        } else {
            &mut builder
        };
        let _ = if config.is_resizable {
            builder.resizable()
        } else {
            &mut builder
        };
        let _ = if config.is_fullscreen {
            builder.fullscreen()
        } else {
            &mut builder
        };

        let window = builder.build()?;

        let canvas = window.into_canvas();
        let texture_creator = canvas.texture_creator();
        let mut graphics = Graphics {
            canvas,
            sdl_context,
            texture_creator: Rc::new(texture_creator),
            ttf_context,
            resource_manager,
        };

        resource_initialization(&mut graphics)?;

        Ok(graphics)
    }

    /// Draw a rectangle on position (x, y) with dimensions (w, h)
    pub fn draw_rect(&mut self, width: f32, height: f32, x: f32, y: f32) -> anyhow::Result<()> {
        self.canvas.draw_rect(FRect {
            x,
            y,
            w: width,
            h: height,
        })?;
        Ok(())
    }

    /// Draw a rectangle on position (x, y) with dimensions (w, h)
    pub fn draw_line(&mut self, p1: (f32, f32), p2: (f32, f32)) -> anyhow::Result<()> {
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
    /// graphics.draw_image(String::from("examples/example.png"), (0.0, 0.0), 0.0, false, false);
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

        let mut manager = match manager.try_lock() {
            Ok(guard) => guard,
            Err(e) => return Err(anyhow::anyhow!("failed to lock resource_manager: {}", e)),
        };
        let key = path.clone();
        if !manager.is_cached(&key) {
            // rust programmers when they have to .clone()
            let texture = Image::load(PathBuf::from(path.clone()), "".to_string(), self);
            manager.insert(texture?);
        }

        let image = manager.get(&key)?.data()
            .downcast_ref::<Rc<InnerImage>>().context("failed to downcast image")?;

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
            )?;

        Ok(())
    }

    /// Create a texture from font + text and render it on the canvas
    pub fn draw_text(
        &mut self,
        text: String,
        position: (f32, f32),
        font: String,
        color: (u8, u8, u8),
        size: f32,
    ) -> anyhow::Result<()> {
        let color = Color::RGB(color.0, color.1, color.2);
        let manager = self.resource_manager.clone();

        let mut manager = match manager.try_lock() {
            Ok(guard) => guard,
            Err(e) => return Err(anyhow::anyhow!("failed to lock resource_manager: {}", e)),
        };

        // dumbass solution. either way, i see no other solution to this.
        // as sdl3 requires us to create a texture from font to draw text,
        // we will be caching it as an [`resources::image::Image`] under this key
        let cache_key = format!(
            "{}|{}|{}|{:x?}",
            font,
            text,
            size,
            color.to_u32(&PixelFormat::RGBA32)
        );
        let font: &InternalDummyFont = {
            manager.get(&font)?.data()
                .downcast_ref::<InternalDummyFont>().context("failed to downcast font")?
        };

        let font_key = format!("{}|{}", font.name, size);

        if !manager.is_cached(&font_key.clone()) {
            let asset = Font::load(font.path.clone(), font_key.clone(), self, Some(size));
            manager.insert(asset?);
        }
        
        if !manager.is_cached(&cache_key.clone()) {
            let font = manager.get(&font_key)?.data()
                .downcast_ref::<Rc<InternalFont>>().context("failed to downcast font")?;
            let surface = font.buffer
                .render(&text)
                .blended(color)
                .map_err(|e| anyhow::anyhow!("render error: {e}"))?;
            let image = Image::load_from_surface(cache_key.clone(), self, surface);
            manager.insert(image?);
        }
        let texture = manager.get(&cache_key)?.data()
            .downcast_ref::<Rc<InnerImage>>().context("failed to downcast image")?;

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
            )?;
        Ok(())
    }
}
