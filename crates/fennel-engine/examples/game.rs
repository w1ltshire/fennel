use fennel_engine::app::AppBuilder;
use specs::System;

struct SysA;

impl<'a> System<'a> for SysA {
    type SystemData = ();

    fn run(&mut self, _data: ()) {
        // std::thread::sleep(std::time::Duration::from_millis(183)); // artificial delay, aim for ~5.5 tps
        // println!("nya");
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let app = AppBuilder::new()
        .name("game")
        .dimensions((800, 800))
        .config("crates/fennel-engine/examples/game.toml")
        .register_system(SysA, "sys_a", &[])
        .build()?;

    app.run().await?;
    Ok(())
}
