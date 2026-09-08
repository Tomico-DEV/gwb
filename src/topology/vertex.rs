use slotmap::new_key_type;

use super::half_edge::HalfEdgeKey;

use crate::geometry::point::Point;


new_key_type! { pub struct VertexKey; }

/// A topological vertex
pub struct Vertex {
    /// Coordinate of the vertex
    coord: Point,
    /// [HalfEdge](half_edge.rs) associated with this vertex
    half_edge: HalfEdgeKey,
}
