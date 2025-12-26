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

use std::any::Any;
use specs::{DispatcherBuilder, World};
use specs::prelude::ResourceId;

/// A struct that represents a plugin dependency descriptor
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct DependencyDescriptor {
	resource: ResourceId,
}

impl DependencyDescriptor {
	/// Create a new instance of [`DependencyDescriptor`]
	pub fn new<T: Any + Send + Sync>() -> Self {
		Self {
			resource: ResourceId::new::<T>(),
		}
	}

	/// Returns the [`ResourceId`]
	pub fn resource(&self) -> ResourceId {
		self.resource.clone()
	}
}

/// A trait that all plugins must implement to be inserted into `App`
///
/// # Example
/// ```
/// use std::collections::HashMap;
/// use specs::{DispatcherBuilder, World};
/// use fennel_plugins::{Plugin, DependencyDescriptor};
/// use specs::prelude::{ResourceId, Resource};
/// use specs::shred::cell::AtomicRefCell;
///
/// struct MyCoolPlugin;
///
/// impl Plugin for MyCoolPlugin {
///     fn prepare(
/// 		&mut self,
/// 		dispatcher_builder: &mut DispatcherBuilder,
/// 		world: &mut World
/// 	) -> anyhow::Result<()> {
///         // initialize your plugin here
///         Ok(())
///     }
///
///     fn update(&mut self, delta_time: f64) -> anyhow::Result<()> {
///         // update your plugin state
///         Ok(())
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
	/// * `dispatcher_builder`: a mutable reference to [`DispatcherBuilder`] so the plugin can register its own systems
	/// * `world`: a mutable reference to [`World`] so the plugin can register components, insert resources, e.t.c.
	fn prepare(
		&mut self,
		dispatcher_builder: &mut DispatcherBuilder,
		world: &mut World,
	) -> anyhow::Result<()>;
	/// Update the plugin state, return a result of this
	fn update(&mut self, delta_time: f64) -> anyhow::Result<()>;
	/// Return the plugin's name; must be unique and not change
	fn name(&self) -> &'static str;
}