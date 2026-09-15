use crate::core::scalar::Real;

/// A point in 3-dimensional space.
pub struct Point {
    pub x: Real,
    pub y: Real,
    pub z: Real,
}

impl Point {
    pub fn new(x: Real, y: Real, z: Real) -> Self {
        Self { x, y, z }
    }
}

#[macro_export]
macro_rules! point {
    ($x: expr, $y: expr, $z: expr) => {
        Point::new($x, $y, $z)
    };
}