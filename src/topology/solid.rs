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
    
    /// Get the key of a twin of a half-edge from a `HalfEdgeKey`, if it exists
    /// 
    /// # Panics
    /// Panics if the edge is invalid (does not contain the half-edge that led to it)
    pub fn get_he_twin_key(&self, he_key: HalfEdgeKey) -> Option<HalfEdgeKey> {
        let he = &self.half_edges[he_key];

        let edge_key = he.edge?;
        let edge = &self.edges[edge_key];
        
        Some(edge.get_twin(he_key))
    }

    /// Get the twin ("opposite" half-edge) of a half-edge from its key, if it exists
    /// 
    /// # Panics
    /// Panics if the edge is invalid (does not contain the half-edge that led to it),
    /// or if the twin's key is invalid
    pub fn get_he_twin(&mut self, he_key: HalfEdgeKey) -> Option<&mut HalfEdge> {
        let twin_key = self.get_he_twin_key(he_key)?;
        Some(self.get_half_edge(twin_key))
    }

    /// Insert a half-edge into a loop after a specified half-edge
    pub fn insert_he_after(&mut self, he_key: HalfEdgeKey, target_key: HalfEdgeKey) {
        let target = &self.half_edges[target_key];
        let target_loop = target.edge_loop;

        // configure he's neighbors
        let he_neigh = HalfEdgeNeighbors {
            prev: target_key,
            next: target.get_next()
        };

        let target_neigh = HalfEdgeNeighbors {
            prev: target.get_prev(),
            next: he_key
        };

        // now set neighbors
        let he = &mut self.get_half_edge(he_key); 
        he.neighbors = Some(he_neigh);
        he.edge_loop = target_loop;
        self.get_half_edge(target_key).neighbors = Some(target_neigh);
    }

    /// Insert a half-edge into a loop before a specified half-edge
    pub fn insert_he_before(&mut self, he_key: HalfEdgeKey, target_key: HalfEdgeKey) {
        let target = &self.half_edges[target_key];

        // configure he's neighbors
        let he_neigh = HalfEdgeNeighbors {
            prev: target.get_prev(),
            next: target_key
        };

        let target_neigh = HalfEdgeNeighbors {
            prev: target.get_prev(),
            next: he_key
        };

        // now set neighbors
        self.get_half_edge(he_key).neighbors = Some(he_neigh);
        self.get_half_edge(target_key).neighbors = Some(target_neigh);
    }
}

// Methods for adding and getting stuff from a Solid
impl Solid {
    /// Moves a vertex into the solid
    pub fn add_vertex(&mut self, vertex: Vertex) -> VertexKey {
        self.vertices.insert(vertex)
    }

    /// Get a mutable vertex from a `VertexKey`
    pub fn get_vertex(&mut self, key: VertexKey) -> &mut Vertex {
        &mut self.vertices[key]
    }

    /// Add an empty halfedge to the solid.
    /// 
    /// Empty = has no neighbors. Gains neighbors through operators
    pub fn add_half_edge(&mut self, half_edge: HalfEdge) -> HalfEdgeKey {
        self.half_edges.insert(half_edge)
    }

    /// Get a mutable half-edge from a `HalfEdgeKey`
    pub fn get_half_edge(&mut self, key: HalfEdgeKey) -> &mut HalfEdge {
        &mut self.half_edges[key]
    }

    /// Insert an edge and automatically initialize its half-edges
    pub fn add_edge(&mut self, edge: Edge) -> EdgeKey {
        let key = self.edges.insert(edge);

        // set edge as parent
        let edge = &self.edges[key];
        let pos_key = edge.pos;
        let neg_key = edge.neg;

        let pos = self.get_half_edge(neg_key);
        pos.edge = Some(key);

        let neg = self.get_half_edge(pos_key);
        neg.edge = Some(key);

        key
    }

    /// Get a mutable edge from an `EdgeKey`
    pub fn get_edge(&mut self, key: EdgeKey) -> &mut Edge {
        &mut self.edges[key]
    }

    /// Add an edge loop to the solid. Automatically sets the edge loop as
    /// the parent of the half-edge
    pub fn add_edge_loop(&mut self, edge_loop: EdgeLoop) -> EdgeLoopKey {
        let key = self.edge_loops.insert(edge_loop);

        // set half-edge loop parent 
        let edge_loop = self.get_edge_loop(key);
        let he = edge_loop.half_edge;
        self.get_half_edge(he).edge_loop = Some(key);

        key
    }

    /// Get a mutable edge loop from an `EdgeLoopKey`
    pub fn get_edge_loop(&mut self, key: EdgeLoopKey) -> &mut EdgeLoop {
        &mut self.edge_loops[key]
    }

    /// Create a face in the solid
    pub fn add_face(&mut self, face: Face) -> FaceKey {
        self.faces.insert(face)
    }

    /// Get a face from a `FaceKey`
    pub fn get_face(&mut self, key: FaceKey) -> &mut Face {
        &mut self.faces[key]
    }
}
