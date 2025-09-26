#[cfg(test)]

#[tokio::test]
async fn font_load() {
    use std::path::PathBuf;
    use crate::{Game, graphics, resources::loadable, resources::LoadableResource};

    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        unsafe { std::env::set_var("SDL_VIDEODRIVER", "dummy") };
    });

    let graphics = graphics::Graphics::new(String::from("my cool game"), (500, 500));
    let mut game = Game::new(
        String::from("my cool game"),
        String::from("wiltshire"),
        graphics.unwrap(),
    );

    let asset = loadable::Font::load(PathBuf::from("examples/terminus.ttf"), &mut game.graphics, Some(16.0)).expect("failed to load font");
    game.resource_manager.cache_asset(asset).unwrap();
    game.resource_manager.resources.iter().for_each(|e| println!("{}", e.0));

    let cached_asset = game.resource_manager.get_asset(String::from("Terminus (TTF) 16"));

    // check is:
    // 1. asset cached successfully
    // 2. name generated successfully
    assert!(cached_asset.is_ok());

    let font: &loadable::Font = crate::resources::as_concrete(cached_asset.unwrap()).unwrap();
    assert_eq!(font.size, 16.0);
}

#[tokio::test]
async fn image_load() {
    use std::path::PathBuf;
    use crate::{Game, graphics, resources::loadable, resources::LoadableResource};

    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        unsafe { std::env::set_var("SDL_VIDEODRIVER", "dummy") };
    });

    let graphics = graphics::Graphics::new(String::from("my cool game"), (500, 500));
    let mut game = Game::new(
        String::from("my cool game"),
        String::from("wiltshire"),
        graphics.unwrap(),
    );

    let asset = loadable::Image::load(PathBuf::from("examples/example.png"), &mut game.graphics, None).expect("failed to load image");
    game.resource_manager.cache_asset(asset).unwrap();
    game.resource_manager.resources.iter().for_each(|e| println!("{}", e.0));

    let cached_asset = game.resource_manager.get_asset(String::from("examples/example.png"));

    // check is:
    // 1. asset cached successfully
    // 2. name generated successfully
    assert!(cached_asset.is_ok());

    let image: &loadable::Image = crate::resources::as_concrete(cached_asset.unwrap()).unwrap();

    // the example image dimensions is 128 x 128, unless some fuck replaces it this will pass
    assert_eq!(image.width, 128);
    assert_eq!(image.height, 128);
    assert_eq!(image.width, image.texture.width());
    assert_eq!(image.height, image.texture.height());
}
