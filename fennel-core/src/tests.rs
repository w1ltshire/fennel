#[cfg(test)]
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::resources::ResourceManager;

fn create_window() -> crate::Window {
    static SDL_INIT: std::sync::Once = std::sync::Once::new();
    SDL_INIT.call_once(|| unsafe { std::env::set_var("SDL_VIDEODRIVER", "dummy") });

    let resouce_manager = Arc::new(Mutex::new(ResourceManager::new()));
    let gfx =
        crate::graphics::Graphics::new("my cool game".into(), (500, 500), resouce_manager.clone(), |_| {}, crate::graphics::WindowConfig { resizable: false, fullscreen: false, centered: true })
            .unwrap();
    crate::Window::new(gfx, resouce_manager.clone())
}

#[tokio::test]
async fn image_load() {
    use crate::{resources::downcast_ref, resources::image::Image, resources::LoadableResource};

    let mut game = create_window();

    let asset = Image::load(
        PathBuf::from("../assets/example.png"),
        String::from("assets/example.png"),
        &mut game.graphics,
        None,
    )
    .expect("failed to load image");

    let mut manager = game.resource_manager.lock().unwrap();

    manager.cache_asset(asset).unwrap();

    let cached = manager
        .get_asset("assets/example.png".to_string())
        .expect("image not cached");

    let img: &Image = downcast_ref(cached).unwrap();

    assert_eq!(img.width, 128);
    assert_eq!(img.height, 128);
    assert_eq!(img.width, img.texture.width());
    assert_eq!(img.height, img.texture.height());
}
