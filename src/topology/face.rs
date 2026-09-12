use slotmap::new_key_type;

use super::edge_loop::EdgeLoopKey;
use crate::geometry::surface::PlanarSurface;
use crate::geometry::vector::Vec3;

new_key_type! { pub struct FaceKey; }

/// A topological face.
pub struct Face {
    /// The key to outer (bounding) loop of this face
    outer: EdgeLoopKey,
    /// A list of keys to the inner loops (rings) of this face
    inner: Vec<EdgeLoopKey>,
    /// The surface of this face
    surface: PlanarSurface,
}

impl Face {
    /// New empty face.
    /// 
    /// Surface is initialized with 0, 0, 0 
    pub fn new(boundary: EdgeLoopKey) -> Self {
        Self {
            outer: boundary,
            inner: Vec::<EdgeLoopKey>::new(),
            surface: PlanarSurface {
                eq: Vec3::zero()
            }
        }
    }
}