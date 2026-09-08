use slotmap::new_key_type;

use super::half_edge::HalfEdgeKey;

new_key_type! { pub struct EdgeKey; }

/// An edge, used for the identification of two half-edges.
pub struct Edge {
    /// The first half-edge
    left: HalfEdgeKey,
    /// The second half-edge
    right: HalfEdgeKey,
}
