use slotmap::{SlotMap, new_key_type};

use super::edge::*;
use super::edge_loop::*;
use super::face::*;
use super::half_edge::*;
use super::vertex::*;

new_key_type! { pub struct SolidKey; }

/// A solid.
pub struct Solid {
    /// A SlotMap of the [faces](faces.rs) this solid owns
    faces: SlotMap<FaceKey, Face>,
    /// A SlotMap of the [loops](edge_loop.rs) this solid owns
    edge_loops: SlotMap<EdgeLoopKey, EdgeLoop>,
    /// A SlotMap of the [edges](edge.rs) this solid owns
    edges: SlotMap<EdgeKey, Edge>,
    /// A SlotMap of the [half-edges](half_edge.rs) this solid owns
    half_edges: SlotMap<HalfEdgeKey, HalfEdge>,
    /// A SlotMap of the [vertices](vertex.rs) this solid owns
    vertices: SlotMap<VertexKey, Vertex>,
}
