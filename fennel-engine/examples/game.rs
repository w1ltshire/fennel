use fennel_common::events::{KeyboardEvent, WindowEventHandler};
use fennel_engine::{
    components::sprite::Sprite,
    runtime::{Runtime, RuntimeBuilder},
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

    fn key_down_event(&self, _runtime: &mut Runtime, event: KeyboardEvent) -> anyhow::Result<()> {
        println!("{:?}", event.keycode);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut runtime = RuntimeBuilder::new()
        .name("game")
        .dimensions((500, 500))
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
    runtime
        .world
        .create_entity()
        .with(Sprite {
            image: String::from("assets/example.png"),
            position: (256.0, 100.0)
        })
        .build();
    runtime.run(MyGame).await?;
    Ok(())
}
