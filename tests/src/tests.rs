#[cfg(test)]
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use fennel_core::{
    Window,
    graphics::{Graphics, WindowConfig},
    resources::{LoadableResource, ResourceManager, downcast_ref, image::Image},
};

fn create_window() -> Window {
    static SDL_INIT: std::sync::Once = std::sync::Once::new();
    SDL_INIT.call_once(|| unsafe { std::env::set_var("SDL_VIDEODRIVER", "dummy") });

    let resouce_manager = Arc::new(Mutex::new(ResourceManager::new()));
    let gfx = Graphics::new(
        "my cool game".into(),
        (500, 500),
        resouce_manager.clone(),
        |_| {},
        WindowConfig {
            is_resizable: false,
            is_fullscreen: false,
            is_centered: true,
        },
    )
    .expect("failed to create a window");
    Window::new(gfx, resouce_manager.clone())
}

#[tokio::test]
async fn image_load() {
    let mut game = create_window();

    let asset = Image::load(
        PathBuf::from("../assets/example.png"),
        String::from("assets/example.png"),
        &mut game.graphics,
        None,
    )
    .expect("failed to load image");

    let mut manager = game
        .resource_manager
        .lock()
        .expect("failed to acquire resource_manager lock");

    manager
        .cache_asset(asset)
        .expect("failed to cache an asset");

    let cached = manager
        .get_asset("assets/example.png".to_string())
        .expect("image not cached");

    let img: &Image = downcast_ref(cached).expect("failed to downcast gathered asset");

    assert_eq!(img.width, 128);
    assert_eq!(img.height, 128);
    assert_eq!(img.width, img.texture.width());
    assert_eq!(img.height, img.texture.height());
}
