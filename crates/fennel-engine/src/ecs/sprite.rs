use log::error;
use ron::Value;
use specs::{Entity, Join, LazyUpdate, ReadStorage, System, World, WorldExt, WriteExpect};
use fennel_core::graphics::{Drawable, Sprite};
use crate::{
    registry::ComponentFactory, renderer::RenderQueue,
};

/// Factory for [`Sprite`]
pub struct SpriteFactory;

impl ComponentFactory for SpriteFactory {
    fn insert(&self, world: &mut World, entity: Entity, value: &Value) {
        match Value::into_rust::<Sprite>(value.clone()) {
            Ok(sprite) => {
                let _ = world
                    .write_storage::<Sprite>()
                    .insert(entity, sprite);
            }
            Err(e) => {
                error!("failed to construct a sprite for entity {:?}: {}", entity, e);
            }
        }
    }

    fn insert_lazy(&self, lazy: &LazyUpdate, entity: Entity, value: &Value) {
        match Value::into_rust::<Sprite>(value.clone()) {
            Ok(sprite) => {
                lazy.insert(entity, sprite);
            }
            Err(e) => {
                error!("failed to construct a sprite for entity {:?}: {}", entity, e);
            }
        }
    }
}

/// ECS system that queues [`Sprite`]s for rendering
///
/// The system reads all Sprite components from the world and obtains a mutable
/// reference to the host App through the HostPtr resource
pub struct SpriteRenderingSystem;

impl<'a> System<'a> for SpriteRenderingSystem {
    type SystemData = (ReadStorage<'a, Sprite>, WriteExpect<'a, RenderQueue>);

    fn run(&mut self, (sprites, mut rq): Self::SystemData) {
        for sprite in (&sprites).join() {
            rq.queue
                .push(Drawable::Image(sprite.clone()));
        }
    }
}