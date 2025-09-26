use std::{any::Any, cell::Ref, collections::HashMap, path::PathBuf};

use crate::graphics::Graphics;

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
    /// # Arguments
    /// `path`: path to the resoruce file
    /// `graphics`: current [`Graphics`] instance which holds `texture_creator` and `ttf_context`
    /// `size`: optional size for any resoruce that needs it
    ///
    /// # Errors
    /// Returns an error if the file cannot be read or parsed
    fn load(
        path: PathBuf,
        graphics: &mut Graphics,
        size: Option<f32>
    ) -> anyhow::Result<Box<dyn LoadableResource>>
    where
        Self: Sized;

    /// Eaasy-to-use identifier for the resource
    fn name(&self) -> String;

    /// Return a mutable slice that the graphics thread can pass to SDL
    ///
    /// If the resource does not have a buffer, then it mustn't implement this function
    fn as_mut_slice(&self) -> Option<&mut [u8]> {
        None
    }

    /// Return an immutable slice for read‑only access
    ///
    /// If the resource does not have a buffer, then it mustn't implement this function
    fn as_slice(&self) -> Option<Ref<'_, [u8]>> {
        None
    }
}

/// evil &Box<dyn LoadableResource> to &T
#[allow(clippy::borrowed_box)] // i have no idea how can this be done better because here we box a
                               // trait
pub fn as_concrete<T: 'static + LoadableResource>(b: &Box<dyn LoadableResource>) -> anyhow::Result<&T> {
    let dyn_ref: &dyn LoadableResource = b.as_ref();

    let any_ref = dyn_ref as &dyn Any;

    Ok(any_ref
        .downcast_ref::<T>()
        .expect("incorrect concrete type"))
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
    #[allow(clippy::borrowed_box)] // same reason as in `as_concrete`
    pub fn get_asset(&self, name: String) -> anyhow::Result<&Box<dyn LoadableResource>> {
        let asset = self.resources.get(&name).unwrap();
        Ok(asset)
    }

    /// Check if a resource is cached
    pub fn is_cached(&self, name: String) -> bool {
        self.resources.contains_key(&name)
    }
}

impl Default for ResourceManager {
    /// `default()` is equivalent to `ResourceManager::new()`.
    fn default() -> Self {
        Self::new()
    }
}
