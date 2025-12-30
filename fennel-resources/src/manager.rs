use std::collections::HashMap;
use anyhow::Context;
use crate::resource::Resource;

/// A struct that represents the resource manager, which stores the resources
pub struct ResourceManager {
	cache: HashMap<String, Box<dyn Resource>>,
}

unsafe impl Send for ResourceManager {}
unsafe impl Sync for ResourceManager {}

impl ResourceManager {
	/// Create a new instance of [`ResourceManager`]
	///
	/// # Returns
	/// A new empty instance of [`ResourceManager`]
	pub fn new() -> Self {
		Self {
			cache: HashMap::new(),
		}
	}

	/// Insert a type implementing [`Resource`] into [`ResourceManager`]
	///
	/// # Arguments
	/// * `resource`: type implementing [`Resource`] trait
	///
	/// # Examples
	/// ```ignore
	/// use fennel_resources::manager::ResourceManager;
	/// let manager = ResourceManager::new();
	/// manager.insert(MyResource { name: "my_resource" });
	/// ```
	pub fn insert<T: Resource + 'static>(&mut self, resource: T) {
		let name = resource.name().clone().to_string();
		self.cache.insert(name, Box::new(resource));
	}

	/// Remove a type implementing [`Resource`] from [`ResourceManager`]
	///
	/// # Arguments
	/// * `name`: the unique name of the resource
	///
	/// # Returns
	/// An [`anyhow::Result`] with [`Box<dyn Resource>`] inside it, which is [`Some`] if
	/// the resource exists, [`None`] if not.
	///
	/// # Examples
	/// ```ignore
	/// use fennel_resources::manager::ResourceManager;
	/// let manager = ResourceManager::new();
	/// manager.insert(MyResource { name: "my_resource" });
	/// let same_resource = manager.remove("my_resource");
	/// ```
	pub fn remove(&mut self, name: &str) -> anyhow::Result<Box<dyn Resource>> {
		self.cache.remove(name).context("resource does not exist")
	}

	/// Get a type implementing [`Resource`] from [`ResourceManager`]
	///
	/// # Arguments
	/// * `name`: the unique name of the resource
	///
	/// # Returns
	/// An [`anyhow::Result`] with an immutable reference to [`Box<dyn Resource>`] inside of it,
	/// which is [`Some`] if the resource exists, [`None`] if not.
	///
	/// # Examples
	/// ```ignore
	/// use fennel_resources::manager::ResourceManager;
	/// let manager = ResourceManager::new();
	/// let resource = manager.get("my_resource");
	/// ```
	pub fn get(&self, name: &str) -> anyhow::Result<&dyn Resource> {
		Ok(self.cache.get(name).context("resource does not exist")?.as_ref())
	}

	/// Get a type implementing [`Resource`] from [`ResourceManager`]. This function follows Rust's
	/// borrow rules, you can only have one mutable reference to a resource.
	///
	/// # Arguments
	/// * `name`: the unique name of the resource
	///
	/// # Returns
	/// An [`anyhow::Result`] with a mutable reference to [`Box<dyn Resource>`] inside it,
	/// which is [`Some`] if the resource exists, [`None`] if not.
	///
	/// # Examples
	/// ```ignore
	/// use fennel_resources::manager::ResourceManager;
	/// let manager = ResourceManager::new();
	/// let resource = manager.get_mut("my_resource");
	/// ```
	pub fn get_mut(&mut self, name: &str) -> anyhow::Result<&mut dyn Resource> {
		Ok(self.cache.get_mut(name).context("resource does not exist")?.as_mut())
	}

	/// Determines whether a resource exists in the cache and returns a boolean
	pub fn is_cached(&self, name: &str) -> bool {
		self.cache.contains_key(name)
	}
}

impl Default for ResourceManager {
	fn default() -> Self {
		Self::new()
	}
}