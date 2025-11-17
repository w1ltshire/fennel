use std::{any::Any, cell::Ref, collections::HashMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    graphics::Graphics,
    resources::{font::DummyFont, image::Image},
};

pub mod font;
pub mod image;

/// Manages a collection of loadable resources indexed by their name
pub struct ResourceManager {
    /// Map resource name to a type that implements [`LoadableResource`] trait
    pub resources: HashMap<String, Box<dyn LoadableResource>>,
}

unsafe impl Send for ResourceManager {}
unsafe impl Sync for ResourceManager {}

#[derive(Deserialize, Serialize, Debug)]
enum AssetType {
    Image,
    Audio,
    Font,
}

#[derive(Deserialize, Serialize, Debug)]
/// Manifest structure of an asset presented in manifest
struct Asset {
    name: String,
    path: String,
    #[serde(rename(deserialize = "type"))]
    class: AssetType,
}

#[derive(Deserialize, Debug)]
/// Assets package manifest
struct Manifest {
    pub assets: Vec<Asset>,
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
        name: String,
        graphics: &mut Graphics,
        size: Option<f32>,
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

/// evil &Box\<dyn LoadableResource> to &T
#[allow(clippy::borrowed_box)] // i have no idea how can this be done better because here we box a
// trait
/// Downcast a '&Box<dyn LoadableResource>' to a concrete type
pub fn downcast_ref<T: 'static + LoadableResource>(
    b: &Box<dyn LoadableResource>,
) -> anyhow::Result<&T> {
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

    /// Loads assets from a directory, which must contain a manifest
    ///
    /// # Errors
    /// Returns an error if manifest does not exist in the target directory
    pub fn load_dir(&mut self, path: PathBuf, graphics: &mut Graphics) -> anyhow::Result<()> {
        let manifest_file = fs::read(path.join("manifest.toml"))?;
        let manifest: Manifest = toml::from_slice(&manifest_file)?;
        for asset in manifest.assets {
            match asset.class {
                AssetType::Image => {
                    let path = path.join(asset.path);
                    let image = Image::load(
                        path.clone(),
                        path.to_str()
                            .expect("failed to convert path to string")
                            .to_string(),
                        graphics,
                        None,
                    )?;
                    println!("{:?}", image.name());
                    self.cache_asset(image)?;
                }
                AssetType::Audio => {}
                AssetType::Font => {
                    let path = path.join(asset.path);
                    let font = DummyFont::load(path, asset.name, graphics, None)?;
                    println!("{:?}", font.name());
                    self.cache_asset(font)?;
                }
            }
        }
        Ok(())
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
        let asset = self
            .resources
            .get(&name)
            .unwrap_or_else(|| panic!("asset {name} not found"));
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
