//! Scenes are compositions of entities/components and scripts the user sees on their screen.
//! A scene has a name, a list of scripts it uses (doesn't actually own them, retrieves from
//! resource manager) and a list of entities (same as with scripts)

use log::debug;
use ron::Value;
use serde::Deserialize;
use specs::{Component, DenseVecStorage, WriteExpect};
use specs::{Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System};
use fennel_registry::ComponentRegistry;

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
    /// Has the scene been successfully loaded by [`SceneSystem`]?
    pub loaded: bool,
}

/// Scene loading system
pub struct SceneSystem;

impl<'a> System<'a> for SceneSystem {
    type SystemData = (
        ReadStorage<'a, Scene>,
        Entities<'a>,
        WriteExpect<'a, ActiveScene>,
        ReadExpect<'a, ComponentRegistry>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (scenes, entities, mut active_scene, component_registry, lazy): Self::SystemData) {
        let active_name = active_scene.name.clone();
        for scene in scenes
            .join()
            .filter(|s| s.name == active_name)
        {
            if !active_scene.loaded {
                for ent_def in &scene.entities {
                    for component in &ent_def.components {
                        debug!(
                            "loading component {} with parameters {:?}",
                            component.id, component.config
                        );
                        let entity = entities.create();

                        // iirc no way to return an error from a system, and it's really a fatal error if a factory doesn't
                        // exist (means either the scene is defined wrongly or the developer didn't register a factory) so
                        // panic is justified
                        let factory = component_registry
                            .get(&component.id)
                            .unwrap_or_else(|| panic!("factory {} not found", component.id));

                        factory.insert_lazy(&lazy, entity, &component.config);
                    }
                }
                active_scene.loaded = true;
            }
        }
    }
}
