use specs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System};

use crate::{
    app::App,
    ecs::sprite::HostPtr,
    scenes::Scene,
};

/// Scene drawing system
pub struct SceneSystem;

impl<'a> System<'a> for SceneSystem {
    type SystemData = (ReadStorage<'a, Scene>, ReadExpect<'a, HostPtr>, Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (scenes, host_ptr, entities, lazy): Self::SystemData) {
        let runtime: &mut App = unsafe { &mut *host_ptr.0 };
        for scene in scenes.join().filter(|s| s.name == runtime.active_scene.name) {
            if !runtime.active_scene.loaded {
                for ent_def in &scene.entities {
                    for component in &ent_def.components {
                        println!("loading component {} with parameters {:?}", component.id, component.config);
                        let entity = entities.create();
                        let factory = runtime.component_registry.get(&component.id)
                            .unwrap_or_else(|| {
                                panic!("factory {} not found", component.id)
                            });
                        factory.insert_lazy(&lazy, entity, &component.config);
                    }
                }
                runtime.active_scene.loaded = true;
            }
        }
    }
}
