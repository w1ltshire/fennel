use fennel_common::events::{KeyboardEvent, WindowEventHandler};
use fennel_core::{
    resources::{LoadableResource, loadable::Image},
};
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
    let sprite = Image::load(
        "assets/example.png".into(),
        &mut runtime.window.graphics,
        None,
    )?;
    let sprite: &Image = fennel_core::resources::as_concrete(&sprite).unwrap();
    runtime
        .world
        .create_entity()
        .with(Sprite(sprite.clone()))
        .build();
    runtime.dispatcher.dispatch(&runtime.world);
    runtime.run(MyGame).await?;
    Ok(())
}
