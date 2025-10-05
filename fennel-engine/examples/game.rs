use fennel_core::{events::KeyboardEvent, EventHandler, Window};
use fennel_engine::runtime::RuntimeBuilder;
use specs::{Component, VecStorage, WorldExt, Builder};

struct MyGame;

#[async_trait::async_trait]
impl EventHandler for MyGame {
    async fn update(&self, _window: &mut Window) -> anyhow::Result<()> {
        Ok(())
    }

    async fn draw(&self, window: &mut Window) -> anyhow::Result<()> {
        window.graphics.canvas.clear();
        window.graphics
            .draw_image("assets/example.png".to_string(), (0.0, 0.0))
            .expect("failed to draw an image");
        window.graphics.canvas.present();
        Ok(())
    }

    fn key_down_event(&self, _window: &mut Window, event: KeyboardEvent) -> anyhow::Result<()> {
        println!("{:?}", event.keycode);
        Ok(())
    }
}

#[derive(Debug)]
struct Pos(f32);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut runtime = RuntimeBuilder::new()
        .name("game")
        .dimensions((500, 500))
        .build()
        .unwrap();
    runtime.world.register::<Pos>();
    runtime.world.create_entity().with(Pos(2.0)).build();
    runtime.run(Box::new(MyGame)).await?;
    Ok(())
}
