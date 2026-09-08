///! # Geometry Module
///!
///! This module handles the coordinates and the precise "shape" of things.
///! How are connected or related is the responsibility of the [topology module](../topology/mod.rs)


pub mod point {
    use crate::core::scalar::Real;

    /// A point in 3-dimensional space.
    pub struct Point {
        pub x: Real,
        pub y: Real,
        pub z: Real,
    }

    // impl Point {

    // }
}

pub mod vector {
    use na::Vector4;
    use nalgebra as na;

    /// A column vector in 3-dimensional space.
    use crate::core::scalar::Real;

    pub struct Vec3 {
        pub x: Real,
        pub y: Real,
        pub z: Real,
    }
}

pub mod surface {
    use super::vector::Vec3;

    pub struct PlanarSurface {
        /// face equation vector
        eq: Vec3,
    }
}
