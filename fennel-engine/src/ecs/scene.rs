use specs::{Join, ReadStorage, System, WriteExpect};

use crate::{
    app::App,
    ecs::sprite::HostPtr,
    scenes::Scene,
};

/// Scene drawing system
pub struct SceneSystem;

impl<'a> System<'a> for SceneSystem {
    type SystemData = (ReadStorage<'a, Scene>, WriteExpect<'a, HostPtr>);
    fn run(&mut self, (scenes, mut host_ptr): Self::SystemData) {
        let _runtime: &mut App = unsafe { &mut *host_ptr.0 };

        for scene in (scenes).join() {

        }
    }
}
