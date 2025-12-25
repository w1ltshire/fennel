use log::error;
use ron::Value;
use specs::{Entity, Join, LazyUpdate, ReadStorage, System, World, WorldExt, WriteExpect};
use fennel_core::graphics::{Drawable, Sprite};
use crate::{
    registry::ComponentFactory, renderer::RenderQueue
};

impl_component_factory!(SpriteFactory, Sprite);

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