use sdl3::keyboard::Scancode;
use fennel_runtime::app::AppBuilder;
use specs::{Join, System, WriteExpect, WriteStorage};
use fennel_core::graphics::Sprite;
use fennel_core::plugin::GraphicsPlugin;
use fennel_runtime::camera::Camera;
use fennel_runtime::events::KeyEvents;
use fennel_runtime::scenes::ActiveScene;

// example system
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
        .config("crates/fennel-runtime/examples/game.toml")
        .register_system(SysA, "sys_a", &[])
        .with_plugin(GraphicsPlugin::new("game", (800, 600), "assets"))
        .build()?;

    app.run().await?;
    Ok(())
}
