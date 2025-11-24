use sdl3::keyboard::Scancode;
use fennel_engine::app::AppBuilder;
use specs::{Join, System, WriteExpect, WriteStorage};
use fennel_engine::camera::Camera;
use fennel_engine::ecs::sprite::Sprite;
use fennel_engine::events::KeyEvents;
use fennel_engine::scenes::ActiveScene;

struct SysA;

impl<'a> System<'a> for SysA {
    type SystemData = (WriteExpect<'a, KeyEvents>, WriteStorage<'a, Sprite>, WriteExpect<'a, ActiveScene>, WriteExpect<'a, Camera>);

    fn run(&mut self, (mut key_events, mut sprite, mut active_scene, mut camera): Self::SystemData) {
        // std::thread::sleep(std::time::Duration::from_millis(183)); // artificial delay, aim for ~5.5 tps
        // println!("nya");
        for sprite in (&mut sprite).join() {
            key_events.0.drain(..).for_each(|event| {
                if let Some(scancode) = event.scancode {
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
                        Scancode::Space => {
                            active_scene.name = String::from("test");
                            active_scene.loaded = false;
                        },
                        Scancode::Up => {
                            camera.position.1 += 1.0;
                        },
                        Scancode::Down => {
                            camera.position.1 -= 1.0;
                        },
                        Scancode::Left => {
                            camera.position.0 += 1.0;
                        },
                        Scancode::Right => {
                            camera.position.0 -= 1.0;
                        }
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
