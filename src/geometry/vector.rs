use na::Vector4;
use nalgebra as na;

/// A column vector in 3-dimensional space.
use crate::core::scalar::Real;

pub struct Vec3 {
    pub x: Real,
    pub y: Real,
    pub z: Real,
}

impl Vec3 {
    /// Create a new zero vector
    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}