use slotmap::new_key_type;

use super::edge::EdgeKey;
use super::edge_loop::EdgeLoopKey;
use super::vertex::VertexKey;

new_key_type! { pub struct HalfEdgeKey; }

/// The neighbors of a half-edge.
#[derive(Clone, Copy)]
pub struct HalfEdgeNeighbors {
    pub next: HalfEdgeKey,
    pub prev: HalfEdgeKey,
}


/// A half-edge.
///
/// Has information about its neighbor so traversing connected half-edges is possible.
pub struct HalfEdge {
    /// The [Vertex](vertex.rs) associated with this half-edge
    pub vertex: VertexKey,
    /// The key to the next half-edge
    pub neighbors: Option<HalfEdgeNeighbors>,
    /// The key to the edge associated with thsi half-edge
    pub edge: Option<EdgeKey>,
    /// The key to the loop this half-edge belongs to
    pub edge_loop: Option<EdgeLoopKey>,
}

impl HalfEdge {
    /// Create an empty half-edge
    pub fn new(vertex: VertexKey) -> Self {
        // Associate vertex with half-edge
        Self {
            vertex,
            neighbors: None,
            edge: None,
            edge_loop: None
        }
    }

    /// Get neighbors.
    /// 
    /// # Panics
    /// Panics if neighbors are missing (improperly initialized)
    pub fn get_neighbors(&self) -> HalfEdgeNeighbors {
        self.neighbors.unwrap_or_else(||panic!("get_neighbor: half-edge is uninitialized!"))
    }

    /// Get next half-edge in the loop
    ///
    /// # Panics
    /// Panics if neighbors are missing (improperly initialized)
    pub fn get_next(&self) -> HalfEdgeKey {
        self.get_neighbors().next
    }

    /// Get previous half-edge in the loop
    ///
    /// # Panics
    /// Panics if neighbors are missing (improperly initialized)
    pub fn get_prev(&self) -> HalfEdgeKey {
        self.get_neighbors().prev
    }

    
}