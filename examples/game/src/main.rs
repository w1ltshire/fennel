use sdl3::keyboard::Scancode;
use specs::{Join, ReadExpect, System, WriteStorage};
use fennel_2d::sprite::{SpriteFactory, SpriteRenderingSystem};
use fennel_graphics::graphics::Sprite;
use fennel_graphics::plugin::event_handler::PluginEvent;
use fennel_runtime::app::AppBuilder;
use fennel_graphics::plugin::GraphicsPlugin;

struct MySystem;

impl<'a> System<'a> for MySystem {
    type SystemData = (ReadExpect<'a, Vec<PluginEvent>>, WriteStorage<'a, Sprite>);

    fn run(&mut self, (events, mut sprites): Self::SystemData) {
        events
            .iter()
            .filter(|event| matches!(event, PluginEvent::KeyboardEvent(_)))
            .for_each(|event| {
               for sprite in (&mut sprites).join() {
                   if let PluginEvent::KeyboardEvent(event) = event {
                       match event.scancode.unwrap() {
                           Scancode::D => {
                               sprite.transform.position.0 += 4.0;
                           },
                           Scancode::A => {
                               sprite.transform.position.0 -= 4.0;
                           }
                           _ => {},
                       }
                   }
               }
            });
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let app = AppBuilder::new()
        .config("examples/game/game.toml")
        .with_component::<Sprite, SpriteFactory>("sprite", SpriteFactory)
        .with_plugin(GraphicsPlugin::new("game", (800, 600), "assets"))
        .register_system(MySystem, "my_system", &["event_gather_system"])
        .register_system(SpriteRenderingSystem, "sprite_rendering_system", &[])
        .build()?;

    app.run()?;
    Ok(())
}
