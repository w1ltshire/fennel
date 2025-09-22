//! SDL3-backed graphics helper
//!
//! Provides:
//! - `Graphics`: owned SDL context + drawing canvas
//! - `init(...)`: initialize SDL, create a centered resizable window and return `Graphics`
//!

use std::collections::HashMap;
use std::path::Path;

use image::ImageReader;
use sdl3::Sdl;
use sdl3::pixels::PixelFormat;
use sdl3::render::{Canvas, FRect};
use sdl3::video::Window;

/// Owned SDL variables used for rendering
///
/// - `canvas`: the drawing surface for the window
/// - `sdl_context`: the SDL context
pub struct Graphics {
    pub canvas: Canvas<Window>,
    pub sdl_context: Sdl,
    pub texture_creator: sdl3::render::TextureCreator<sdl3::video::WindowContext>,
    texture_cache: HashMap<String, CachedTexture>,
}

#[derive(Clone)]
struct CachedTexture {
    buffer: Vec<u8>,
    width: u32,
    height: u32,
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
    /// ```no_run
    /// let graphics = graphics::init(String::from("my cool game"), (500, 500));
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
            texture_creator,
            texture_cache: HashMap::new(),
        })
    }

    pub fn draw_image(&mut self, path: String, position: (f32, f32)) -> anyhow::Result<()> {
        if !self.texture_cache.contains_key(&path) {
            let path = Path::new(&path);
            let img = ImageReader::open(path)?.decode()?;
            let buffer = img.to_rgba8().into_raw();

            self.texture_cache.insert(
                path.display().to_string(),
                CachedTexture {
                    buffer,
                    width: img.width(),
                    height: img.height(),
                },
            );
        }
        let buffer: &mut CachedTexture = &mut self.texture_cache.get(&path).unwrap().clone(); // .unwrap() should be safe here
        // because in the previous if block we
        // make sure there is a texture with
        // this key in cache
        // otherwise i dunno :3
        let surface = sdl3::surface::Surface::from_data(
            buffer.buffer.as_mut_slice(),
            buffer.width,
            buffer.height,
            buffer.width * 4,
            PixelFormat::RGBA32,
        )
        .map_err(|e| e.to_string())
        .unwrap();

        let dst_rect = FRect::new(
            position.0,
            position.1,
            surface.width() as f32,
            surface.height() as f32,
        );

        let texture = self
            .texture_creator
            .create_texture_from_surface(surface)
            .map_err(|e| e.to_string())
            .unwrap();

        self.canvas
            .copy_ex(&texture, None, Some(dst_rect), 0.0, None, false, false)
            .unwrap();

        Ok(())
    }
}
