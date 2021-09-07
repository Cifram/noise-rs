//! An ultra-light private math library to make our short lives easier as we
//! implement super-complex noise stuff.

pub(crate) mod interpolate;
pub(crate) mod s_curve;
pub mod select;
pub mod vectors;

pub use crate::vectors::*;
pub use crate::select::*;
