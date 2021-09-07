use core::ops::{Add, Mul};

/// Performs linear interpolation between two values.
#[inline(always)]
pub fn linear<T>(a: T, b: T, alpha: f64) -> T
where
    T: Mul<f64, Output = T> + Add<Output = T>,
{
	b * alpha + a * (1.0 - alpha)
}
