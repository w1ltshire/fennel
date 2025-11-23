use sdl3::keyboard::Scancode;
use fennel_engine::app::AppBuilder;
use specs::{Join, System, WriteExpect, WriteStorage};
use fennel_engine::ecs::sprite::Sprite;
use fennel_engine::events::KeyEvents;

struct SysA;

impl<'a> System<'a> for SysA {
    type SystemData = (WriteExpect<'a, KeyEvents>, WriteStorage<'a, Sprite>);

    fn run(&mut self, (mut key_events, mut sprite): Self::SystemData) {
        // std::thread::sleep(std::time::Duration::from_millis(183)); // artificial delay, aim for ~5.5 tps
        // println!("nya");
        for mut sprite in (&mut sprite).join() {
            key_events.0.drain(..).for_each(|event| {
                if let Some(scancode) = event.scancode {
                    println!("{:?}", event);
                    match scancode {
                        Scancode::W => {
                            sprite.transform.position.1 -= 4.0;
                        },
                        Scancode::S => {
                            sprite.transform.position.1 += 4.0;
                        },
                        Scancode::A => {
                            sprite.transform.position.0 -= 4.0;
                        },
                        Scancode::D => {
                            sprite.transform.position.0 += 4.0;
                        },
                        _ => {}
                    }
                }
            });
        }
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
