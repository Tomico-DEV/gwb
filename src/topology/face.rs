use slotmap::new_key_type;

use super::edge_loop::EdgeLoopKey;

use crate::geometry::surface::PlanarSurface;


new_key_type! { pub struct FaceKey; }


// A topological face
pub struct Face {
    /// The key to outer (bounding) loop of this face
    outer: EdgeLoopKey,
    /// A list of keys to the inner loops (rings) of this face
    inner: Vec<EdgeLoopKey>,
    /// The surface of this face
    surface: PlanarSurface,
}
