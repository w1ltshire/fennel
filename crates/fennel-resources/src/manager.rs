use std::collections::HashMap;
use crate::resource::Resource;

pub struct ResourceManager {
	cache: HashMap<&'static str, Box<dyn Resource>>,
}

impl ResourceManager {
	/// Create a new instance of [`ResourceManager`]
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
	/// manager.insert(resource);
	/// ```
	pub fn insert<T: Resource + 'static>(&mut self, resource: T) {
		self.cache.insert(resource.name(), Box::new(resource));
	}

	/// Remove a type implementing [`Resource`] from [`ResourceManager`]
	///
	/// # Arguments
	/// * `name`: the unique name of the resource
	///
	/// # Returns
	/// An [`Option`] with [`Box<dyn Resource>`] inside of it, which is [`Some`] if
	/// the resource exists, [`None`] if not.
	///
	/// # Examples
	/// ```ignore
	/// use fennel_resources::manager::ResourceManager;
	/// let manager = ResourceManager::new();
	/// let same_resource = manager.remove("my_resource");
	/// ```
	pub fn remove(&mut self, name: &'static str) -> Option<Box<dyn Resource>> {
		self.cache.remove(name)
	}

	/// Get a type implementing [`Resource`] from [`ResourceManager`]
	///
	/// # Arguments
	/// * `name`: the unique name of the resource
	///
	/// # Returns
	/// An [`Option`] with an immutable reference to [`Box<dyn Resource>`] inside of it,
	/// which is [`Some`] if the resource exists, [`None`] if not.
	///
	/// # Examples
	/// ```ignore
	/// use fennel_resources::manager::ResourceManager;
	/// let manager = ResourceManager::new();
	/// let resource = manager.get("my_resource");
	/// ```
	pub fn get(&self, name: &'static str) -> Option<&Box<dyn Resource>> {
		self.cache.get(name)
	}

	/// Get a type implementing [`Resource`] from [`ResourceManager`]
	///
	/// # Arguments
	/// * `name`: the unique name of the resource
	///
	/// # Returns
	/// An [`Option`] with a mutable reference to [`Box<dyn Resource>`] inside of it,
	/// which is [`Some`] if the resource exists, [`None`] if not.
	///
	/// # Examples
	/// ```ignore
	/// use fennel_resources::manager::ResourceManager;
	/// let manager = ResourceManager::new();
	/// let resource = manager.get("my_resource");
	/// ```
	pub fn get_mut(&mut self, name: &'static str) -> Option<&mut Box<dyn Resource>> {
		self.cache.get_mut(name)
	}
}