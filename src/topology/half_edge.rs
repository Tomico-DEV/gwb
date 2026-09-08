use slotmap::new_key_type;

use super::edge_loop::EdgeLoopKey;
use super::edge::EdgeKey;
use super::vertex::VertexKey;

new_key_type! { pub struct HalfEdgeKey; }

/// A half-edge.
///
/// Has information about its neighbor so traversing connected half-edges is possible.
pub struct HalfEdge {
    /// The [Vertex](vertex.rs) associated with this half-edge
    vertex: VertexKey,
    /// The key to the next half-edge
    next: HalfEdgeKey,
    /// The key to the previous half-edge
    prev: HalfEdgeKey,
    /// The key to the edge associated with thsi half-edge
    edge: EdgeKey,
    /// The key to the loop this half-edge belongs to
    edge_loop: EdgeLoopKey,
}
