//! SDL3-backed graphics helper
//!
//! Provides:
//! - `Graphics`: owned SDL context + drawing canvas
//! - `init(...)`: initialize SDL, create a centered resizable window and return `Graphics`
//!

use std::path::Path;

use image::{EncodableLayout, ImageReader};
use sdl3::pixels::PixelFormat;
use sdl3::Sdl;
use sdl3::render::{Canvas, FPoint, FRect};
use sdl3::video::Window;

/// Owned SDL variables used for rendering
///
/// - `canvas`: the drawing surface for the window
/// - `sdl_context`: the SDL context
pub struct Graphics {
    pub canvas: Canvas<Window>,
    pub sdl_context: Sdl,
}

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
/// ```no_run
/// let graphics = graphics::init(String::from("my cool game"), (500, 500));
/// ```
pub fn init(name: String, dimensions: (u32, u32)) -> Result<Graphics, Box<dyn std::error::Error>> {
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

    Ok(Graphics { canvas, sdl_context })
}

impl Graphics {
    pub fn draw_image(&mut self, path: String, position: (f32, f32)) -> anyhow::Result<()> {
        // ATM this function does not cache anything and gathers all data and calls everything
        // again and again, this is not very performant but this works as a temporary prototype
        // TODO: cache textures
        let path = Path::new(&path);
        let img = ImageReader::open(path)?.decode()?;
        let mut binding = img.clone();
        let surface = sdl3::surface::Surface::from_data(binding.as_mut_rgba8().unwrap(), img.width(), img.height(), img.width() * 4, PixelFormat::RGBA32).map_err(|e| e.to_string()).unwrap();

        let texture_width = surface.width();
        let texture_height = surface.height();

        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(surface)
            .map_err(|e| e.to_string()).unwrap();

        let mut dst_rect = FRect::new(
            position.0,
            position.1,
            texture_width as f32,
            texture_height as f32,
        );

         self.canvas
            .copy_ex(&texture, None, Some(dst_rect), 0.0, None, false, false)
            .unwrap();

        Ok(())
    }
}
