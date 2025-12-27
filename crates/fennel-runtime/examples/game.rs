use fennel_runtime::app::AppBuilder;
use fennel_core::plugin::GraphicsPlugin;

/*
no mappings of like events or whatever eeeehhhh i dunno >:3
there was a system y'know you can see it in some commit soooooooo uhhhh
yeah i need to do sdl3 event mapping or smth like that in graphics plugin
as of now, i'm lazy. Too bad!
*/

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let app = AppBuilder::new()
        .config("crates/fennel-runtime/examples/game.toml")
        .with_plugin(GraphicsPlugin::new("game", (800, 600), "assets"))
        .build()?;

    app.run().await?;
    Ok(())
}
