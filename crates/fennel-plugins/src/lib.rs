//! # Plugins
//! This crate provides a simple trait for creating plugins within the Fennel engine.
//! Plugins allow to extend the capabilities of the engine, and also are an essential piece of it,
//! as plugins are the backbone for most of the engine's features.
//!
//! An example is the `fennel_core::plugin::GraphicsPlugin` plugin, which provides the most important
//!	part of the engine, the graphics.

use std::collections::HashMap;
use specs::prelude::{Resource, ResourceId};
use specs::shred::cell::AtomicRefCell;

/// A trait that all plugins must implement in order to be inserted into `App`
///
/// # Example
/// ```
/// use specs::World;
/// use fennel_plugins::Plugin;
/// struct MyCoolPlugin;
///
/// impl Plugin for MyCoolPlugin {
/// 	fn prepare(&mut self, _world: *mut World) -> anyhow::Result<()> {
///  		// initialize your plugin here
///    		Ok(())
///    	}
///
///  	fn update(&mut self, delta_time: f64) -> anyhow::Result<()> {
/// 		// update your plugin state
/// 		Ok(())
/// 	}
///
/// 	fn name(&self) -> &'static str {
///    		"my_cool_plugin"
/// 	}
/// }
/// ```
pub trait Plugin {
	/// Prepare/initialize the plugin, return a result of this
	fn prepare(&mut self, dependencies: HashMap<String, &AtomicRefCell<Box<dyn Resource>>>) -> anyhow::Result<()>;
	/// Update the plugin state, return a result of this
	fn update(&mut self, delta_time: f64) -> anyhow::Result<()>;
	/// Return a list of your resource dependencies here or an empty [`Vec`] if you don't need any resources.
	/// Use the [`ResourceId::new`] function to acquire a [`ResourceId`] instance.
	///
	/// Usually you want the dependency to be an [`Arc`], [`Rc`] or some sort of channel receiver/sender,
	/// as under the hood `fennel_engine` clones the resource.
	fn resource_dependencies(&mut self) -> HashMap<String, ResourceId>;
	/// Return the plugin's name; must be unique and not change
	fn name(&self) -> &'static str;
}