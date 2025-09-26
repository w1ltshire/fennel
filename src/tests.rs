use std::sync::{Arc, Mutex};
#[cfg(test)]
use std::path::PathBuf;

use crate::resources::ResourceManager;

fn make_game() -> crate::Game {
    static SDL_INIT: std::sync::Once = std::sync::Once::new();
    SDL_INIT.call_once(|| unsafe { std::env::set_var("SDL_VIDEODRIVER", "dummy") });

    let resouce_manager = Arc::new(Mutex::new(ResourceManager::new()));
    let gfx = crate::graphics::Graphics::new(
        "my cool game".into(),
        (500, 500),
        resouce_manager.clone()
    )
    .unwrap();
    crate::Game::new(
        "my cool game".into(),
        "wiltshire".into(),
        gfx,
        resouce_manager.clone()
    )
}

#[tokio::test]
async fn font_load() {
    use crate::{resources::loadable, resources::as_concrete};
    use crate::resources::LoadableResource;

    let mut game = make_game();

    let asset = loadable::Font::load(
        PathBuf::from("examples/terminus.ttf"),
        &mut game.graphics,
        Some(16.0),
    )
    .expect("failed to load font");
    let mut manager = game.resource_manager.lock().unwrap();

    manager.cache_asset(asset).unwrap();

    let cached = manager
        .get_asset("examples/terminus.ttf|16".to_string())
        .expect("font not cached");

    let font: &loadable::Font = as_concrete(cached).unwrap();
    assert_eq!(font.size, 16.0);
}

#[tokio::test]
async fn image_load() {
    use crate::{resources::loadable, resources::as_concrete};
    use crate::resources::LoadableResource;

    let mut game = make_game();

    let asset = loadable::Image::load(
        PathBuf::from("examples/example.png"),
        &mut game.graphics,
        None,
    )
    .expect("failed to load image");

    let mut manager = game.resource_manager.lock().unwrap();

    manager.cache_asset(asset).unwrap();

    let cached = manager
        .get_asset("examples/example.png".to_string())
        .expect("image not cached");

    let img: &loadable::Image = as_concrete(cached).unwrap();

    assert_eq!(img.width, 128);
    assert_eq!(img.height, 128);
    assert_eq!(img.width, img.texture.width());
    assert_eq!(img.height, img.texture.height());
}
