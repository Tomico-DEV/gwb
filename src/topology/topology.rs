use slotmap::SlotMap;

use super::face::{Face, FaceKey};
use super::edge_loop::{EdgeLoop, EdgeLoopKey};
use super::edge::{Edge, EdgeKey};
use super::half_edge::{HalfEdge, HalfEdgeKey};
use super::vertex::{Vertex, VertexKey};

/// Topology accessor/context for `Solid`
pub struct Topology<'a> {
    /// A reference to the SlotMap containing faces
    faces: &'a SlotMap<FaceKey, Face>,
    /// A reference to the SlotMap containing loops
    edge_loops: &'a SlotMap<EdgeLoopKey, EdgeLoop>,
    /// A reference to the SlotMap containing edges
    edges: &'a SlotMap<EdgeKey, Edge>,
    /// A reference to the SlotMap containing half-edges
    half_edges: &'a SlotMap<HalfEdgeKey, HalfEdge>,
    /// A reference to the SlotMap containing vertices
    vertices: &'a SlotMap<VertexKey, Vertex>,
}

