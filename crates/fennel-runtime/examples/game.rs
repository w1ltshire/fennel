use log::debug;
use specs::{ReadExpect, System};
use fennel_core::plugin::event_handler::PluginEvent;
use fennel_runtime::app::AppBuilder;
use fennel_core::plugin::GraphicsPlugin;

struct MySystem;

impl<'a> System<'a> for MySystem {
    type SystemData = ReadExpect<'a, Vec<PluginEvent>>;

    fn run(&mut self, events: Self::SystemData) {
        events
            .iter()
            .filter(|event| matches!(event, PluginEvent::KeyboardEvent(_)))
            .for_each(|event| debug!("{:?}", event));
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let app = AppBuilder::new()
        .config("crates/fennel-runtime/examples/game.toml")
        .with_plugin(GraphicsPlugin::new("game", (800, 600), "assets"))
        .register_system(MySystem, "my_system", &["event_gather_system"])
        .build()?;

    app.run().await?;
    Ok(())
}
