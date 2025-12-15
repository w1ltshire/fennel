use std::collections::HashMap;
use crate::resource::Resource;

/// A struct that represents the resource manager, which stores the resources
pub struct ResourceManager {
	cache: HashMap<&'static str, Box<dyn Resource>>,
}

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
		self.cache.insert(resource.name(), Box::new(resource));
	}

	/// Remove a type implementing [`Resource`] from [`ResourceManager`]
	///
	/// # Arguments
	/// * `name`: the unique name of the resource
	///
	/// # Returns
	/// An [`Option`] with [`Box<dyn Resource>`] inside it, which is [`Some`] if
	/// the resource exists, [`None`] if not.
	///
	/// # Examples
	/// ```ignore
	/// use fennel_resources::manager::ResourceManager;
	/// let manager = ResourceManager::new();
	/// manager.insert(MyResource { name: "my_resource" });
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

	/// Get a type implementing [`Resource`] from [`ResourceManager`]. This function follows Rust's
	/// borrow rules, you can only have one mutable reference to a resource.
	///
	/// # Arguments
	/// * `name`: the unique name of the resource
	///
	/// # Returns
	/// An [`Option`] with a mutable reference to [`Box<dyn Resource>`] inside it,
	/// which is [`Some`] if the resource exists, [`None`] if not.
	///
	/// # Examples
	/// ```ignore
	/// use fennel_resources::manager::ResourceManager;
	/// let manager = ResourceManager::new();
	/// let resource = manager.get_mut("my_resource");
	/// ```
	pub fn get_mut(&mut self, name: &'static str) -> Option<&mut Box<dyn Resource>> {
		self.cache.get_mut(name)
	}
}