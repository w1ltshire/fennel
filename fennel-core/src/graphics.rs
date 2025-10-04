//! SDL3-backed graphics helper
//!
//! Provides:
//! - `Graphics`: owned SDL context + drawing canvas
//! - `Graphics::new(...)`: initialize SDL, create a centered resizable window and return [`Graphics`]
//!

use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use sdl3::Sdl;
use sdl3::pixels::{Color, PixelFormat};
use sdl3::render::{Canvas, FRect};
use sdl3::video::Window;

use quick_error::ResultExt;

use crate::resources::loadable::{Font, Image};
use crate::resources::{self, LoadableResource, ResourceManager, loadable};

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
    /// let graphics = graphics::new(String::from("my cool game"), (500, 500))?;
    /// ```
    pub fn new(
        name: String,
        dimensions: (u32, u32),
        resource_manager: Arc<Mutex<ResourceManager>>,
    ) -> Result<Graphics, Box<dyn std::error::Error>> {
        // TODO: allow the user to uh customize video_subsystem configuration 'cuz man this is ass why
        // do we position_centered() and resizable() it by default

        let sdl_context = sdl3::init()?;
        let ttf_context = sdl3::ttf::init().map_err(|e| e.to_string())?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(&name, dimensions.0, dimensions.1)
            .position_centered()
            .resizable()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas();
        let texture_creator = canvas.texture_creator();
        Ok(Graphics {
            canvas,
            sdl_context,
            texture_creator: Rc::new(texture_creator),
            ttf_context,
            resource_manager,
        })
    }

    /// Cache an image if it isn't cached and draw it on the canvas
    ///
    /// # Parameters
    /// - `path`: Path to the image
    /// - `position`: Where to draw the image in the window (x,y) in pixels (f32).
    ///
    /// # Returns
    /// - `Ok(())` on success.
    /// - `Err(Box<dyn std::error::Error>)` on failure
    ///
    /// # Example
    /// ```ignore
    /// graphics.draw_image(String::from("examples/example.png"), (0.0, 0.0)).await;
    /// ```
    pub fn draw_image(&mut self, path: String, position: (f32, f32)) -> anyhow::Result<()> {
        let manager = self.resource_manager.clone();
        let mut manager = manager
            .try_lock()
            .context("failed to lock resource_manager")
            .unwrap();

        if !manager.is_cached(path.clone()) {
            // rust programmers when they have to .clone()
            let texture = loadable::Image::load(PathBuf::from(path.clone()), self, None);
            manager.cache_asset(texture?)?; // those question marks are funny hehehe
        }

        let image: &Image = resources::as_concrete(manager.get_asset(path).unwrap())?;

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
                0.0,
                None,
                false,
                false,
            )
            .unwrap();

        Ok(())
    }

    /// Create a texture from font + text and render it on the canvas
    pub fn draw_text(
        &mut self,
        text: String,
        position: (f32, f32),
        font_path: String,
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
            font_path,
            text,
            size,
            color.to_u32(&PixelFormat::RGBA32)
        );
        let font_key = format!("{font_path}|{size}");
        let font: &Font = {
            if !manager.is_cached(font_key.clone()) {
                let asset = loadable::Font::load(font_path.clone().into(), self, Some(size));
                manager.cache_asset(asset?)?;
            }
            let asset = manager.get_asset(font_key)?;
            resources::as_concrete(asset)?
        };

        if !manager.is_cached(cache_key.clone()) {
            let surface = font
                .buffer
                .render(&text)
                .blended(color)
                .map_err(|e| anyhow::anyhow!("render error: {e}"))?;
            let image = Image::load_from_surface(cache_key.clone(), self, surface);
            manager.cache_asset(image?)?;
        }
        let texture: &Image = resources::as_concrete(manager.get_asset(cache_key)?)?;

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
