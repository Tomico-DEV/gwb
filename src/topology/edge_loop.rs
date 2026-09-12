use slotmap::new_key_type;

use super::face::FaceKey;
use super::half_edge::HalfEdgeKey;

new_key_type! { pub struct EdgeLoopKey; }

/// A loop of [HalfEdges](half_edge.rs).
pub struct EdgeLoop {
    /// A key to a half-edge belonging to this loop
    pub half_edge: HalfEdgeKey,
    /// The face associated with this loop
    pub face: Option<FaceKey>,
}

impl EdgeLoop {
    /// Create a new edge loop containing a halfedge
    pub fn new(half_edge: HalfEdgeKey) -> Self {
        Self {
            half_edge,
            face: None
        }
    }
}