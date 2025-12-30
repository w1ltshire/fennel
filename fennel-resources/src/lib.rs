//! # Resource manager for the `Fennel` game engine
//! This library provides a simple resource manager for the `Fennel` game engine, enabling easy management
//! of various assets, such as scenes, images, audio, fonts, etc.
//!
//! ## Modules
//! This library consists of the following modules:
//! - [`manager`] - Implements the functionality of the resource manager, including caching, insertion, and fetching resources
//! - [`resource`] - Defines the traits for resources to implement to be compatible with the resource manager

use thiserror::Error;

#[derive(Error, Debug)]
/// Possible errors
pub enum ResourceError {
	#[error("requested resource was not found in cache")]
	ResourceDoesNotExist,
}

/// The resource manager itself
pub mod manager;
/// All the things to represent a resource type
pub mod resource;
#[cfg(test)]
mod tests;