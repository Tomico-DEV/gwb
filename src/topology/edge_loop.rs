use slotmap::new_key_type;

use super::half_edge::HalfEdgeKey;

new_key_type! { pub struct EdgeLoopKey; }

/// A loop of [HalfEdges](half_edge.rs).
pub struct EdgeLoop {
    /// A key to a half-edge belonging to this loop.
    half_edge: HalfEdgeKey,
}
