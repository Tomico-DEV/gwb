use crate::core::scalar::Real;

// here we use nalgebra as our underlying math framework, but
// we will intentionally hide it behind our own code to allow
// it to be changed out in the future for whatever reason
use nalgebra::{self as na, RealField}; 
use na::Vector4;


/// A point in 3-dimensional space.
pub struct Point {
    pub x: Real,
    pub y: Real,
    pub z: Real
}

// impl Point {
    
// }

