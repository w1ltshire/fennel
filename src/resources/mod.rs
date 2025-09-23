use std::{any::Any, cell::Ref, collections::HashMap, path::PathBuf};

/// Module containing implementations of [`LoadableResource`] such as [`Image`]
pub mod loadable;

/// Manages a collection of loadable resources indexed by their name
pub struct ResourceManager {
    /// Map resource name to a type that implements [`LoadableResource`] trait
    pub resources: HashMap<String, Box<dyn LoadableResource>>,
}

/// Trait that all loadable assets must implement
pub trait LoadableResource: Any {
    /// Load a resource from `path` and return it boxed
    ///
    /// # Errors
    /// Returns an error if the file cannot be read or parsed
    fn load(path: PathBuf) -> anyhow::Result<Box<dyn LoadableResource>>
    where
        Self: Sized;

    /// Eaasy-to-use identifier for the resource
    fn name(&self) -> String;

    /// Return a mutable slice that the graphics thread can pass to SDL
    fn as_mut_slice(&self) -> &mut [u8];

    /// Return an immutable slice for read‑only access
    fn as_slice(&self) -> Ref<'_, [u8]>;
}

/// evil &Box<dyn LoadableResource> to &T
pub fn as_concrete<T: 'static + LoadableResource>(b: &Box<dyn LoadableResource>) -> &T {
    let dyn_ref: &dyn LoadableResource = b.as_ref();

    let any_ref = dyn_ref as &dyn Any;

    any_ref
        .downcast_ref::<T>()
        .expect("incorrect concrete type")
}

impl ResourceManager {
    /// Create a new manager with empty `resources` field
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    /// Insert a loaded asset into the cache
    ///
    /// The asset is stored under the key returned by `asset.name()`
    pub fn cache_asset(&mut self, asset: Box<dyn LoadableResource>) -> anyhow::Result<()> {
        self.resources.insert(asset.name(), asset);
        Ok(())
    }

    // here i have NO fucking idea should it be `&Box<dyn LoadableResource>` or whatever
    // self.resources.get returns a reference to the resource, so basically a reference to Box
    // but afaik Box is a pointer, and for me it feels a bit fucking wrong to uh return a
    // reference to a pointer >:3 and also clippy is angry at me for doing this
    pub fn get_asset(&mut self, name: String) -> anyhow::Result<&Box<dyn LoadableResource>> {
        let asset = self.resources.get(&name).unwrap();
        Ok(asset)
    }

    pub fn is_cached(&mut self, name: String) -> bool {
        self.resources.contains_key(&name)
    }
}

impl Default for ResourceManager {
    /// `default()` is equivalent to `ResourceManager::new()`.
    fn default() -> Self {
        Self::new()
    }
}
