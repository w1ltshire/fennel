use ron::Value;
use specs::{Entity, World};
use std::collections::HashMap;

/// All components must have a factory implementing this trait to be able created from a scene
/// config
pub trait ComponentFactory: Send + Sync {
    /// Build a component from `value` and insert it into `entity` of `world`
    fn insert(&self, world: &mut World, entity: Entity, value: &Value);
}

/// Registry of component name - component factory
pub struct ComponentRegistry {
    map: HashMap<String, Box<dyn ComponentFactory>>,
}

impl ComponentRegistry {
    /// Creates a new instance of [`ComponentRegistry`]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Registers a component factory
    pub fn register(&mut self, name: &str, f: Box<dyn ComponentFactory>) {
        self.map.insert(name.to_string(), f);
    }

    /// Fetches a component factory by name
    pub fn get(&self, name: &str) -> Option<&dyn ComponentFactory> {
        self.map.get(name).map(|v| &**v)
    }
}

impl Default for ComponentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
