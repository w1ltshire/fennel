use fennel_common::events::{KeyboardEvent, WindowEventHandler};
use fennel_engine::{
    components::sprite::Sprite, events::KeyEvents, runtime::{Runtime, RuntimeBuilder}
};
use specs::{Builder, WorldExt};

struct MyGame;

#[async_trait::async_trait]
impl WindowEventHandler for MyGame {
    type Host = Runtime;
    fn update(&self, _runtime: &mut Runtime) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(&self, runtime: &mut Runtime) -> anyhow::Result<()> {
        runtime.window.graphics.canvas.clear();
        runtime.frame_tick();
        runtime.window.graphics.canvas.present();
        Ok(())
    }

    fn key_down_event(&self, runtime: &mut Runtime, event: KeyboardEvent) -> anyhow::Result<()> {
        let mut events = runtime.world.write_resource::<KeyEvents>();
        events.0.push(event);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut runtime = RuntimeBuilder::new()
        .name("game")
        .dimensions((800, 800))
        .build()
        .unwrap();

    runtime
        .world
        .create_entity()
        .with(Sprite {
            image: String::from("assets/example.png"),
            position: (100.0, 100.0)
        })
        .build();

    runtime.run(MyGame).await?;
    Ok(())
}
