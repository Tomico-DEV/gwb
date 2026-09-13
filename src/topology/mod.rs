///! # Topology Module
///!
///! Topology is how things are connected; the precise coordinates or "where things are"
///! are the [geometry module](../geometry/mod.rs)'s responsibility.

mod solid;
pub use solid::SolidKey;

pub mod euler;

use solid::face::{Face, FaceKey};
use solid::edge_loop::{EdgeLoop, EdgeLoopKey};
use solid::edge::{Edge, EdgeKey};
use solid::half_edge::{HalfEdge, HalfEdgeKey, HalfEdgeNeighbors};
use solid::vertex::{Vertex, VertexKey};

/// Trait for read-only access to a solid
pub trait TopologyRead {
    /// Get a vertex from a key
    fn vertex(&self, key: VertexKey) -> &Vertex;
    /// Get a half edge from a key
    fn half_edge(&self, key: HalfEdgeKey) -> &HalfEdge;
    /// Get an edge from a key
    fn edge(&self, key: EdgeKey) -> &Edge;
    /// Get an edge loop from a key
    fn edge_loop(&self, key: EdgeLoopKey) -> &EdgeLoop;
    /// Get a face from a key
    fn face(&self, key: FaceKey) -> &Face;
}

/// Trait for read-write access to a solid
pub trait TopologyWrite: TopologyRead {
    /// Get a mutable vertex from a key
    fn vertex_mut(&mut self, key: VertexKey) -> &mut Vertex;
    /// Get a mutable half edge from a key
    fn half_edge_mut(&mut self, key: HalfEdgeKey) -> &mut HalfEdge;
    /// Get a mutable edge from a key
    fn edge_mut(&mut self, key: EdgeKey) -> &mut Edge;
    /// Get a mutable edge loop from a key
    fn edge_loop_mut(&mut self, key:EdgeLoopKey) -> &mut EdgeLoop;
    /// Get a mutable face from a key
    fn face_mut(&mut self, key: FaceKey) -> &mut Face;

    /// Add a vertex to the graph
    fn add_vertex(&mut self, vertex: Vertex) -> VertexKey;
    /// Move a vertex out of the graph
    fn rm_vertex(&mut self, key: VertexKey) -> Vertex;
    /// Add a half-edge to the graph
    fn add_half_edge(&mut self, half_edge: HalfEdge) -> HalfEdgeKey;
    /// Move a half-edge out of the graph
    fn rm_half_edge(&mut self, key: HalfEdgeKey) -> HalfEdge;
    /// Add an edge loop to the graph
    fn add_edge_loop(&mut self, edge_loop: EdgeLoop) -> EdgeLoopKey;
    /// Move an edge loop out of the graph
    fn rm_edge_loop(&mut self, key: EdgeLoopKey) -> EdgeLoop;
    /// Add an edge to the graph
    fn add_edge(&mut self, edge: Edge) -> EdgeKey;
    /// Move an edge out of the graph
    fn rm_edge(&mut self, key: EdgeKey) -> Edge;
    /// Add a face to the graph
    fn add_face(&mut self, face: Face) -> FaceKey;
    /// Move a face out of the graph
    fn rm_face(&mut self, key: FaceKey) -> Face;

    /// Set the neighbor of a half-edge.
    fn set_he_neighbors(
        &mut self,
        he_key: HalfEdgeKey,
        neighbors: HalfEdgeNeighbors
    );
}

/// Topology accessor/context for `Solid` or a solid-like
pub struct Topology<S> {
    solid: S,
}

impl<S: TopologyRead> Topology<S> {
    /// Get the key to the twin half-edge of a half-edge, if it exists
    pub fn he_twin_key(&self, he_key: HalfEdgeKey)
    -> Option<HalfEdgeKey> {
        let he = self.solid.half_edge(he_key);
        let edge = self.solid.edge(he.edge?);

        Some(edge.get_twin(he_key))
    }

    /// Get the twin half-edge of a half-edge, if it exists
    pub fn he_twin(&self, he_key: HalfEdgeKey) 
    -> Option<&HalfEdge> {
        Some(self.solid.half_edge(self.he_twin_key(he_key)?))
    }
}

impl<S: TopologyWrite + TopologyRead> Topology<S> {
    /// Get the mutable twin half-edge of a half-edge, if it exists
    pub fn he_twin_mut(&mut self, he_key: HalfEdgeKey)
    -> Option<&mut HalfEdge> {
        Some(self.solid.half_edge_mut(self.he_twin_key(he_key)?))
    }

    /// Insert a half-edge into a loop after a specified half-edge
    pub fn insert_he_after(&mut self, he_key: HalfEdgeKey, target_key: HalfEdgeKey) {
        let target = self.solid.half_edge(target_key);
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
        let he = self.solid.half_edge_mut(he_key); 
        he.edge_loop = target_loop;
        
        self.solid.set_he_neighbors(he_key, he_neigh);
        self.solid.set_he_neighbors(target_key, target_neigh);
    }

    /// Insert a half-edge into a loop before a specified half-edge
    pub fn insert_he_before(&mut self, he_key: HalfEdgeKey, target_key: HalfEdgeKey) {
        let target = self.solid.half_edge(target_key);
        let target_loop = target.edge_loop;
        
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
        let he = self.solid.half_edge_mut(he_key); 
        he.edge_loop = target_loop;
        self.solid.set_he_neighbors(he_key, he_neigh);
        self.solid.set_he_neighbors(target_key, target_neigh);
    }

    /// Set the origin half-edge of a loop
    pub fn set_loop_origin(&mut self, edge_loop: EdgeLoopKey, origin: HalfEdgeKey) {
        self.solid.edge_loop_mut(edge_loop).half_edge = origin;
    }
}

