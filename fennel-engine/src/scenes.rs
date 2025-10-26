//! Scenes are compositions of entities/components and scripts the user sees on their screen.
//! A scene has a name, a list of scripts it uses (doesn't actually owns them, retrieves from
//! resource manager) and a list of entities (same as with scripts)
use ron::Value;
use serde::Deserialize;
use specs::{Component, DenseVecStorage};

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
