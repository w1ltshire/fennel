use ron::Value;
use serde::Deserialize;
use specs::{Entity, Join, LazyUpdate, ReadStorage, System, World, WorldExt, WriteExpect};

use crate::{app::App, registry::ComponentFactory};

/// A raw pointer wrapper to the application
pub struct HostPtr(pub *mut App);

unsafe impl Send for HostPtr {}
unsafe impl Sync for HostPtr {}

/// A simple renderable sprite.
///
/// # Fields
/// - image: identifier or path of the image to draw
/// - position: tuple (x, y) position on screen
#[derive(Deserialize, Debug)]
pub struct Sprite {
    /// Sprite asset id in the resource manager
    pub image: String,
    /// Sprite position on the screen
    pub position: (f32, f32),
}

impl specs::Component for Sprite {
    type Storage = specs::VecStorage<Self>;
}

/// Factory for [`Sprite`]
pub struct SpriteFactory;

impl ComponentFactory for SpriteFactory {
    fn insert(&self, world: &mut World, entity: Entity, value: &Value) {
        let sprite = ron::value::Value::into_rust::<Sprite>(value.clone());
        println!("{:#?}", sprite);
        world.write_storage::<Sprite>().insert(entity, sprite.expect("failed to construct a sprite")).unwrap();
    }

    fn insert_lazy(&self, lazy: &LazyUpdate, entity: Entity, value: &Value) {
        let sprite = ron::value::Value::into_rust::<Sprite>(value.clone())
            .expect("failed to construct a sprite");
        lazy.insert(entity, sprite);
    }
}

/// ECS system that renders Sprite components.
///
/// The system reads all Sprite components from the world and obtains a mutable
/// reference to the host App through the HostPtr resource
pub struct RenderingSystem;

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (ReadStorage<'a, Sprite>, WriteExpect<'a, HostPtr>);

    fn run(&mut self, (sprites, mut host_ptr): Self::SystemData) {
        let runtime: &mut App = unsafe { &mut *host_ptr.0 };
        let window = &mut runtime.window;

        for sprite in (&sprites).join() {
            window
                .graphics
                .draw_image(sprite.image.clone(), sprite.position)
                .unwrap();
        }
    }
}
