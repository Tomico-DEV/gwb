use slotmap::{SlotMap, new_key_type};

pub mod face;
pub mod edge;
pub mod edge_loop;
pub mod half_edge;
pub mod vertex;

use crate::core::scalar::UInteger;

use super::{TopologyWrite, TopologyRead, Topology};
use face::*;
use edge::*;
use edge_loop::*;
use half_edge::*;
use vertex::*;

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

// convenience methods for solid
impl Solid {
    /// Create a new empty solid
    pub fn new() -> Self {
        Self {
            faces: SlotMap::with_key(),
            edge_loops: SlotMap::with_key(),
            edges: SlotMap::with_key(),
            half_edges: SlotMap::with_key(),
            vertices: SlotMap::with_key(),
        }
    }

    /// Get a topology context for this solid.
    pub fn topology(&self) -> Topology<&Self> {
        Topology { solid: self }
    }

    /// Get a mutable topology context for this solid.
    pub fn topology_mut(&mut self) -> Topology<&mut Self> {
        Topology { solid: self }
    }

    pub fn n_faces(&self) -> UInteger {
        self.faces.len() as UInteger
    }
    pub fn n_loops(&self) -> UInteger {
        self.edge_loops.len() as UInteger
    }
    pub fn n_edges(&self) -> UInteger {
        self.edges.len() as UInteger
    }
    pub fn n_half_edges(&self) -> UInteger {
        self.half_edges.len() as UInteger
    }
    pub fn n_vertices(&self) -> UInteger {
        self.vertices.len() as UInteger
    }
}


impl TopologyRead for Solid {
    fn vertex(&self, key: VertexKey) -> &Vertex {
        &self.vertices[key]
    }
    fn half_edge(&self, key: HalfEdgeKey) -> &HalfEdge {
        &self.half_edges[key]
    }
    fn edge(&self, key: EdgeKey) -> &Edge {
        &self.edges[key]
    }
    fn edge_loop(&self, key: EdgeLoopKey) -> &EdgeLoop {
        &self.edge_loops[key]
    }
    fn face(&self, key: FaceKey) -> &Face {
        &self.faces[key]
    }

    fn he_has_neighbors(&self, key: HalfEdgeKey) -> bool {
        // If next is different, prev must be different as well
        self.half_edge(key).get_next() != key
    }
    fn edge_loop_is_empty(&self, key: EdgeLoopKey) -> bool {
        let he_key = self.edge_loop(key).half_edge;
        !self.he_has_neighbors(he_key)
    }
}

impl TopologyWrite for Solid {
    fn vertex_mut(&mut self, key: VertexKey) -> &mut Vertex {
        &mut self.vertices[key]
    }
    fn half_edge_mut(&mut self, key: HalfEdgeKey) -> &mut HalfEdge {
        &mut self.half_edges[key]
    }
    fn edge_mut(&mut self, key: EdgeKey) -> &mut Edge {
        &mut self.edges[key]
    }
    fn edge_loop_mut(&mut self, key:EdgeLoopKey) -> &mut EdgeLoop {
        &mut self.edge_loops[key]
    }
    fn face_mut(&mut self, key: FaceKey) -> &mut Face {
        &mut self.faces[key]
    }
    
    /// Moves a vertex into the solid
    fn add_vertex(&mut self, vertex: Vertex) -> VertexKey {
        self.vertices.insert(vertex)
    }
    /// Moves a vertex out of the solid.
    /// 
    /// Will panic if you try to remove a vertex that still has an associated
    /// half-edge.
    /// 
    /// Only call this after you have removed all references to the vertex!!
    fn rm_vertex(&mut self, key: VertexKey) -> Vertex {
        debug_assert!(
            self.vertex(key).half_edge.is_none(),
            "rm_vertex: Vertex still belongs to an half-edge!"
        );
        self.vertices.remove(key).unwrap_or_else(
            ||panic!("rm_vertex: Tried removing a vertex that doesn't exist!")
        )
    }

    /// Add a halfedge to the solid.
    /// Automatically sets the half-edge as the parent of the vertex,
    /// and sets its own neighbors as itself
    fn add_half_edge(&mut self, half_edge: HalfEdge) -> HalfEdgeKey {
        let key = self.half_edges.insert(half_edge);

        // set self as vertex parent
        let vertex_key = self.half_edge(key).vertex;
        let vertex = self.vertex_mut(vertex_key);
        vertex.half_edge = Some(key);

        // set self as neighbors
        let neighbor = HalfEdgeNeighbors { next: key, prev: key };
        self.set_he_neighbors(key, neighbor);
        
        key
    }
    /// Moves a half-edge out of the solid. Automatically
    /// sets the child vertex's parent to none
    /// 
    /// Will panic if the half-edge still belongs to an edge loop or an edge,
    /// or if it has neighbors other than itself.
    /// 
    /// Only call this after you have removed all references to the half-edge!
    fn rm_half_edge(&mut self, key: HalfEdgeKey) -> HalfEdge {
        debug_assert!(
            !self.he_has_neighbors(key),
            "rm_half_edge: Half-edge still has neighbors!"
        );
        debug_assert!(
            self.half_edge(key).edge.is_none(),
            "rm_half_edge: Half-edge still belongs to an edge!"
        );
        debug_assert!(
            self.half_edge(key).edge_loop.is_none(),
            "rm_half_edge: Half-edge still belongs to a loop!"
        );

        // set child vertex parent to none
        let vtx_key = self.half_edge(key).vertex;
        self.vertex_mut(vtx_key).half_edge = None;
        
        self.half_edges.remove(key).unwrap_or_else(
            ||panic!("rm_half_edge: Tried removing a half-edge that doesn't exist!")
        )
    }

    /// Insert an edge and automatically initialize its half-edges
    fn add_edge(&mut self, edge: Edge) -> EdgeKey {
        let key = self.edges.insert(edge);

        // set edge as parent
        let edge = &self.edges[key];
        let pos_key = edge.pos;
        let neg_key = edge.neg;

        let pos = self.half_edge_mut(pos_key);
        pos.edge = Some(key);

        let neg = self.half_edge_mut(neg_key);
        neg.edge = Some(key);

        key
    }
    /// Moves an edge out of the solid. Automatically sets the children
    /// half-edges' edge to None.
    fn rm_edge(&mut self, key: EdgeKey) -> Edge {
        let edge = self.edge(key).clone();
        self.half_edge_mut(edge.neg).edge = None;
        self.half_edge_mut(edge.pos).edge = None;
        
        self.edges.remove(key).unwrap_or_else(
            ||panic!("rm_edge: Tried removing an edge that does not exist!")
        )
    }
    
    /// Add an edge loop to the solid. Automatically sets the edge loop as
    /// the parent of its half-edge
    fn add_edge_loop(&mut self, edge_loop: EdgeLoop) -> EdgeLoopKey {
        let key = self.edge_loops.insert(edge_loop);

        // set half-edge loop parent 
        let he = self.edge_loop(key).half_edge;
        self.half_edge_mut(he).edge_loop = Some(key);

        key
    }
    /// Move an edge loop out of the solid. Can only be done
    /// if only an empty half-edge is present in the loop
    /// 
    /// Will panic if the edge loop still belongs to a face, or if
    /// the loop contains more than one half-edge
    fn rm_edge_loop(&mut self, key: EdgeLoopKey) -> EdgeLoop {
        let he_key = self.edge_loop(key).half_edge;
        debug_assert!(
            !self.he_has_neighbors(he_key),
            "rm_edge_loop: Tried removing a loop that still has a cycle of half-edges!"
        );

        // reset half-edge parent
        self.half_edge_mut(he_key).edge_loop = None;
        
        debug_assert!(
            self.edge_loop(key).face.is_none(),
            "rm_edge_loop: Tried removing a loop that still belonged to a face!"
        );

        self.edge_loops.remove(key).unwrap_or_else(
            ||panic!("rm_edge_loop: Tried removing an edge loop that does not exist!")
        )
    }
    
    /// Move a face into the solid.
    /// Automatically sets the face as the parent of its edge loop.
    fn add_face(&mut self, face: Face) -> FaceKey {
        let key = self.faces.insert(face);
        // set outer
        let outer_key = self.face(key).outer;
        self.edge_loop_mut(outer_key).face = Some(key);
        // set inner
        let inner = self.face(key).inner.clone();
        for edge_loop in inner {
            self.edge_loop_mut(edge_loop).face = Some(key);
        }

        key
    }
    /// Move a face out of the solid. Automatically sets the children
    /// loops to have a None parent face.
    fn rm_face(&mut self, key: FaceKey) -> Face {
        // outer
        let outer_key = self.face(key).outer;
        self.edge_loop_mut(outer_key).face = None;
        // inner 
        let inner_keys = self.face(key).inner.clone();
        for key in inner_keys {
            self.edge_loop_mut(key).face = None;
        }
        self.faces.remove(key).unwrap_or_else(
            ||panic!("rm_face: tried removing a face that doesn't exist!")
        )
    }

    /// Set the neighor of a half-edge.
    /// 
    /// Ensures invalid states aren't allowed
    /// (eg prev: self, next: different and vice versa)
    fn set_he_neighbors(
        &mut self,
        he_key: HalfEdgeKey,
        neighbors: HalfEdgeNeighbors
    ) {
        debug_assert!(
            !((neighbors.prev == he_key) ^ (neighbors.next == he_key)),
            "set_he_neighbor: Tried setting an invalid neighbor!"
        );

        self.half_edge_mut(he_key).neighbors = Some(neighbors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod topol_write;
}