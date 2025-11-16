//! Scenes are compositions of entities/components and scripts the user sees on their screen.
//! A scene has a name, a list of scripts it uses (doesn't actually owns them, retrieves from
//! resource manager) and a list of entities (same as with scripts)
use log::debug;
use ron::Value;
use serde::Deserialize;
use specs::{Component, DenseVecStorage};
use specs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System};

use crate::{app::App, ecs::sprite::HostPtr};

/// Scene struct
#[derive(Deserialize, Debug, Clone, Component)]
pub struct Scene {
    /// Scene internal name
    pub name: String,
    /// List of entities in a scene
    pub entities: Vec<EntityDescriptor>,
}

/// Descriptor of an entity in scene config
#[derive(Deserialize, Debug, Clone)]
pub struct EntityDescriptor {
    /// Entity internal id
    pub id: String,
    /// List of components in an entity
    pub components: Vec<ComponentDescriptor>,
}

/// Descriptor of a component in scene config
#[derive(Deserialize, Debug, Clone)]
pub struct ComponentDescriptor {
    /// Component internal id
    pub id: String,
    /// Component configuration
    pub config: Value,
}

/// Struct holding active scene information
pub struct ActiveScene {
    /// Scene name in the config
    pub name: String,
    /// Have the scene been successfully loaded by [`crate::ecs::scene::SceneSystem`]?
    pub loaded: bool,
}

/// Scene loading system
pub struct SceneSystem;

impl<'a> System<'a> for SceneSystem {
    type SystemData = (
        ReadStorage<'a, Scene>,
        ReadExpect<'a, HostPtr>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (scenes, host_ptr, entities, lazy): Self::SystemData) {
        let runtime: &mut App = unsafe { &mut *host_ptr.0 };
        for scene in scenes
            .join()
            .filter(|s| s.name == runtime.active_scene.name)
        {
            if !runtime.active_scene.loaded {
                for ent_def in &scene.entities {
                    for component in &ent_def.components {
                        debug!(
                            "loading component {} with parameters {:?}",
                            component.id, component.config
                        );
                        let entity = entities.create();
                        let factory = runtime
                            .component_registry
                            .get(&component.id)
                            .unwrap_or_else(|| panic!("factory {} not found", component.id));
                        factory.insert_lazy(&lazy, entity, &component.config);
                    }
                }
                runtime.active_scene.loaded = true;
            }
        }
    }
}
