//! Library providing low-level graphics functionality for the Fennel game engine, excluding the windowing.
//!
//! This library focuses on rendering graphics using GPU.
//! It provides a main [`GPURenderer`] structure, which allows for various GPU-related operations.
//!
//! # Errors
//! This crate uses the [`anyhow`] crate for error handling.

pub mod renderer;
pub mod vertex;