use fennel_engine::{
    ecs::sprite::Sprite, app::AppBuilder
};
use specs::{Builder, WorldExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = AppBuilder::new()
        .name("game")
        .dimensions((800, 800))
        .config("fennel-engine/examples/game.toml")
        .build()
        .unwrap();

    app
        .world
        .create_entity()
        .with(Sprite {
            image: String::from("assets/example.png"),
            position: (100.0, 100.0)
        })
        .build();

    app
        .world
        .create_entity()
        .with(Sprite {
            image: String::from("assets/example.png"),
            position: (300.0, 100.0)
        })
        .build();

    app.run().await?;
    Ok(())
}
