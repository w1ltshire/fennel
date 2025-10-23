use sdl3::keyboard::Keycode;
use specs::{ReadStorage, System, WriteExpect};

use crate::{ecs::sprite::{HostPtr, Sprite}, events::KeyEvents};

/// Basic input system
pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (WriteExpect<'a, KeyEvents>, ReadStorage<'a, Sprite>, WriteExpect<'a, HostPtr>);
    fn run(&mut self, data: Self::SystemData) {
        let mut events = data.0;
        for event in events.0.drain(..) {
            match event.keycode {
                None => {},
                Some(Keycode::D) => {
                    println!("1");
                },
                _ => {}
            }
        }
    }
}
