use std::any::Any;

/// Trait that types representing a resource must implement
pub trait Resource {
	/// Return immutable resource data reference
	fn data(&self) -> &dyn Any;

	/// Return mutable resource data reference
	fn data_mut(&mut self) -> &mut dyn Any;

	/// Return the resource name which should be unique
	fn name(&self) -> String;
}