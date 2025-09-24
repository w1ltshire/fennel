//! SDL3-backed graphics helper
//!
//! Provides:
//! - `Graphics`: owned SDL context + drawing canvas
//! - `Graphics::new(...)`: initialize SDL, create a centered resizable window and return [`Graphics`]
//!

use std::path::PathBuf;
use std::sync::Arc;

use sdl3::Sdl;
use sdl3::render::{Canvas, FRect};
use sdl3::video::Window;

use crate::resources::loadable::Image;
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
    pub texture_creator: Arc<sdl3::render::TextureCreator<sdl3::video::WindowContext>>,
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
    ) -> Result<Graphics, Box<dyn std::error::Error>> {
        // TODO: allow the user to uh customize video_subsystem configuration 'cuz man this is ass why
        // do we position_centered() and resizable() it by default

        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap(); // TODO: get this fucking unwrap out of
        // here and replace with something more
        // cool

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
            texture_creator: Arc::new(texture_creator),
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
    pub fn draw_image(
        &mut self,
        path: String,
        position: (f32, f32),
        manager: &mut ResourceManager,
    ) -> anyhow::Result<()> {
        if !manager.is_cached(path.clone()) {
            // rust programmers when they have to .clone()
            let texture = loadable::Image::load(PathBuf::from(path.clone()), &self.texture_creator);
            manager.cache_asset(texture?)?; // those question marks are funny hehehe
        }

        let image: &Image = resources::as_concrete(manager.get_asset(path).unwrap());

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
}
