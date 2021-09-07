//! A procedural noise generation library for Rust.
//!
//! # Example
//!
//! ```rust
//! use noise::{NoiseFn, Perlin, Seedable};
//!
//! let perlin = Perlin::new(1);
//! let val = perlin.get([42.4, 37.7, 2.8]);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_copy_implementations)]

#[cfg(feature = "images")]
#[macro_use]
extern crate alloc;

pub mod core;
mod gradient;
pub mod math;
pub mod permutationtable;
#[cfg(feature = "images")]
pub mod noise_image_builder;

pub use crate::core::*;
pub use crate::math::*;
pub use crate::permutationtable::*;
