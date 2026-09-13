use slotmap::new_key_type;

use super::half_edge::HalfEdgeKey;
use crate::{geometry::point::Point, topology::solid::Solid};

new_key_type! { pub struct VertexKey; }

/// A topological vertex
pub struct Vertex {
    /// Coordinate of the vertex
    pub coord: Point,
    /// [HalfEdge](half_edge.rs) associated with this vertex
    pub half_edge: Option<HalfEdgeKey>,
}

impl Vertex {
    /// Create a new unlinked vertex
    pub fn new(point: Point) -> Self {
        Self {
            coord: point,
            half_edge: None
        }
    }
}