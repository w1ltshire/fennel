#[cfg(test)]

#[tokio::test]
async fn window_creation() {
    use std::process::exit;

    use crate::{EventHandler, Game, events, graphics};
    struct TestGameState {}

    #[async_trait::async_trait]
    impl EventHandler for TestGameState {
        async fn update(&self, _game: &mut Game) -> anyhow::Result<()> {
            Ok(())
        }

        async fn draw(&self, game: &mut Game) -> anyhow::Result<()> {
            use sdl3::pixels::Color;
            game.graphics.canvas.set_draw_color(Color::RGB(0, 0, 0));
            game.graphics.canvas.clear();
            game.graphics
                .draw_image(
                    "examples/example.png".to_string(),
                    (0.0, 0.0),
                    &mut game.resource_manager,
                )
                .expect("failed to draw an image");
            game.graphics.canvas.present();
            assert!(game.resource_manager.is_cached("examples/example.png".to_string()));
            exit(0);
        }
    }

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
    events::run(&mut game, Box::new(TestGameState {})).await;
}
