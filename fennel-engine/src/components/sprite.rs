use specs::{Component, Join, ReadStorage, System, VecStorage};

pub struct Sprite(pub fennel_core::resources::loadable::Image);

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}

pub struct RenderingSystem;

impl<'a> System<'a> for RenderingSystem {
    type SystemData = ReadStorage<'a, Sprite>;

    fn run(&mut self, sprite: Self::SystemData) {
        for sprite in (&sprite).join() {
            println!("{:?}", sprite.0.name);
        }
    }
}
