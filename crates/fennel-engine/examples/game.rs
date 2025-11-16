use fennel_engine::app::AppBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = AppBuilder::new()
        .name("game")
        .dimensions((800, 800))
        .config("crates/fennel-engine/examples/game.toml")
        .build()
        .expect("failed to build an app");

    app.run().await?;
    Ok(())
}
