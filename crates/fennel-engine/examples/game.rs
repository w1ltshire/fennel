use std::{thread::sleep, time::Duration};

use fennel_engine::app::AppBuilder;
use specs::System;

struct SysA;

impl<'a> System<'a> for SysA {
    type SystemData = ();

    fn run(&mut self, _data: ()) {
        sleep(Duration::from_secs(1));
        println!("nya");
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = AppBuilder::new()
        .name("game")
        .dimensions((800, 800))
        .config("crates/fennel-engine/examples/game.toml")
        .register_system(SysA, "sys_a", &[])
        .build()
        .expect("failed to build an app");

    app.run().await?;
    Ok(())
}
