use log::error;
use ron::Value;
use serde::Deserialize;
use specs::{Entity, Join, LazyUpdate, ReadStorage, System, World, WorldExt, WriteExpect};
use crate::ecs::sprite::Sprite;
use crate::registry::ComponentFactory;
use crate::renderer::{Drawable, RenderQueue};

/// A struct to represent a tile in the world
#[derive(Debug, Deserialize)]
pub struct Tile {
    /// Texture ID in the resource manager
    pub texture: Sprite,
}

impl specs::Component for Tile {
    type Storage = specs::VecStorage<Self>;
}

/// Factory for [`TileFactory`]
pub(crate) struct TileFactory;

impl ComponentFactory for TileFactory {
    fn insert(&self, world: &mut World, entity: Entity, value: &Value) {
        match Value::into_rust::<Tile>(value.clone()) {
            Ok(tile) => {
                let _ = world
                    .write_storage::<Tile>()
                    .insert(entity, tile);
            }
            Err(e) => {
                error!("failed to construct a tile for entity {:?}: {}", entity, e);
            }
        }
    }

    fn insert_lazy(&self, lazy: &LazyUpdate, entity: Entity, value: &Value) {
        match Value::into_rust::<Tile>(value.clone()) {
            Ok(tile) => {
                lazy.insert(entity, tile);
            }
            Err(e) => {
                error!("failed to construct a tile for entity {:?}: {}", entity, e);
            }
        }
    }
}

/// ECS system that queues [`Sprite`]s for rendering
///
/// The system reads all Sprite components from the world and obtains a mutable
/// reference to the host App through the HostPtr resource
pub(crate) struct TileRenderingSystem;

impl<'a> System<'a> for TileRenderingSystem {
    type SystemData = (ReadStorage<'a, Tile>, WriteExpect<'a, RenderQueue>);

    fn run(&mut self, (tiles, mut rq): Self::SystemData) {
        for tile in (&tiles).join() {
            rq.queue
                .push(Drawable::Image(tile.texture.clone()));
        }
    }
}