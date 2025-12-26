//! # Plugins
//! This crate provides a simple trait for creating plugins within the Fennel engine.
//! Plugins allow extending the capabilities of the engine and also are an essential piece of it,
//! as plugins are the backbone for most of the engine's features.
//!
//! An example is the `fennel_core::plugin::GraphicsPlugin` plugin, which provides the most important
//! part of the engine, the graphics.
//!
//! ## [`Plugin::update`]
//! This function is called every tick (16 TPS) in the runtime **synchronously**, so if:
//! - your plugin is blocking or does heavy computations and/or
//! - needs to run independently of the runner thread <br/>
//!
//! consider facilitating the usage of channels and threads.

use std::collections::HashMap;
use specs::DispatcherBuilder;
use specs::prelude::{Resource, ResourceId};
use specs::shred::cell::AtomicRefCell;

/// A trait that all plugins must implement to be inserted into `App`
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use specs::World;
/// use fennel_plugins::Plugin;
/// use specs::prelude::{ResourceId, Resource};
/// use specs::shred::cell::AtomicRefCell;
///
/// struct MyCoolPlugin;
///
/// impl Plugin for MyCoolPlugin {
///     fn prepare(&mut self, dependencies: HashMap<String, &AtomicRefCell<Box<(dyn Resource + 'static)>>>) -> anyhow::Result<()> {
///         // initialize your plugin here
///         Ok(())
///     }
///
///     fn update(&mut self, delta_time: f64) -> anyhow::Result<()> {
///         // update your plugin state
///         Ok(())
///     }
///
///     fn resource_dependencies(&mut self) -> HashMap<String, ResourceId> {
///         HashMap::new()
///     }
///
///     fn name(&self) -> &'static str {
///         "my_cool_plugin"
///     }
/// }
/// ```
pub trait Plugin {
	/// Prepare/initialize the plugin, return a result of the initialization.
	///
	/// # Arguments
	/// * `dependencies`: [`HashMap`] keyed by a [`String`] with value [`AtomicRefCell`] with a box with [`Resource`] inside it.
	/// * `register_system`: A closure used for registering additional systems into the ECS.
	fn prepare(
		&mut self,
		dependencies: HashMap<String, &AtomicRefCell<Box<dyn Resource>>>,
		dispatcher_builder: &mut DispatcherBuilder,
	) -> anyhow::Result<()>;

	/// Update the plugin state, return a result of this
	fn update(&mut self, delta_time: f64) -> anyhow::Result<()>;
	/// Return a list of your resource dependencies here or an empty [`Vec`] if you don't need any resources.
	/// Use the [`ResourceId::new`] function to acquire a [`ResourceId`] instance.
	///
	/// Usually you want the dependency to be an [`std::sync::Arc`], [`std::rc::Rc`] or some sort of channel receiver/sender,
	/// as under the hood `fennel_runtime` clones the resource.
	///
	/// The resource is taken from the runtime's ECS world.
	fn resource_dependencies(&self) -> HashMap<String, ResourceId>;
	/// Return the plugin's name; must be unique and not change
	fn name(&self) -> &'static str;
}