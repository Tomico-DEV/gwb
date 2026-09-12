///! This module is responsible for Euler operations

use super::solid::*;
use super::face::*;
use super::edge_loop::*;
use super::half_edge::*;
use super::vertex::*;

use crate::geometry::point::Point;
use crate::topology::edge::{Edge, EdgeKey};

/// A collection of keys belonging to a skeletal primitive
/// 
/// Returned by `new_skeletal_primitive`
pub struct SkeletalPrimtive {
    face: FaceKey,
    edge_loop:  EdgeLoopKey,
    half_edge: HalfEdgeKey,
    vertex: VertexKey
}

// skeletal primitives
impl Solid {
    /// Create a new skeletal primitive from nothing
    /// 
    /// Returns a tuple of the created lone vertex, 
    pub fn new_skeletal_primitive(&mut self, origin: Point) 
    -> SkeletalPrimtive
    {
        // Add lone vertex
        let vertex = self.add_vertex(Vertex::new(origin));
        // Add empty halfedge and an empty loop
        let half_edge = self.add_half_edge(HalfEdge::new(vertex));
        let edge_loop = self.add_edge_loop(EdgeLoop::new(half_edge));
        // Add empty face 
        let face = self.add_face(Face::new(edge_loop));

        SkeletalPrimtive { face, edge_loop, half_edge, vertex }
    }
}

pub struct ExtendVertexRes {
    edge: EdgeKey,
    vertex: VertexKey,
}

impl Solid {
    /// Split a vertex at a halfedge, moving the edges contained with a "wedge"
    /// to the new vertex and assigning the loops of two newly created halfedges
    /// to those of the first and the last wedges.
    /// 
    /// Range is specified by [wedge_start, wedge_end)
    /// 
    /// Returns the keys to the newly created edge and vertex.
    /// 
    /// # Panics
    /// Panics if half-edges in [wedge_start, wedge_end] are of a different verex
    pub fn split_vertex(
        &mut self,
        anchor: VertexKey,
        wedge_start: HalfEdgeKey,
        wedge_end: HalfEdgeKey,
        coord: Point) 
    -> ExtendVertexRes {
        // check if wedge_start and wedge_end are on the same vertex
        if anchor != self.get_half_edge(wedge_start).vertex
            || anchor != self.get_half_edge(wedge_end).vertex
        {
            panic!("split_vertex: anchor does not belong to wedge_start or wedge_end!")
        }
        
        // create new vertex
        let new_v = self.add_vertex(Vertex::new(coord));

        // reassign half-edges in [wedge_start, wedge_end) to the new vertex
        // Walk thorugh half-edges and
        let mut he = wedge_start;
        let mut past_start = false;
        while he != wedge_end {
            // check if this will become an infinite loop
            if past_start && he == wedge_start {
                panic!("split_vertex: Detected a cycle of wedge_start that does not go through wedge_end!")
            }
            
            self.get_half_edge(he).vertex = new_v;

            // come back to anchor but in context of the next half-edge.
            // 
            // twin takes us to the opposing halfedge which starts at the vertex
            // before the anchor, then next takes us back the anchor but in the 
            // context of a different half-edge!
            he = self.get_he_twin(he).
                unwrap_or_else(
                    ||panic!("split_vertex: half-edge in [wedge_start, wedge_end) has no twin!")
                ).get_next();
            
            past_start = true;
        }

        // edge case: "Empty" half edge (half edge with no neighbors,
        // obtained from `new_skeletal_primitive`
        let he_old_new: HalfEdgeKey;
        if self.get_half_edge(wedge_start).neighbors.is_none() {
            // don't make a new old -> new half-edge;
            // use the empty half-edge instead
            he_old_new = wedge_start;
        } else {
            he_old_new = self.add_half_edge(HalfEdge::new(anchor));
        }

        // make new edge between the anchor and the new vertex
        let he_new_old = self.add_half_edge(HalfEdge::new(new_v));

        // insert into the loops
        // wedge_end stayed at the anchor and we insert a new
        // halfedge that goes before wedge_end, starting at the new vertex 
        self.insert_he_before(he_old_new, wedge_start);
        self.insert_he_before(he_new_old, wedge_end);

        // now we identify properly them with an edge
        let edge = self.add_edge(Edge::new(he_new_old, he_old_new));
        
        ExtendVertexRes { edge: edge, vertex: new_v }
    }
} 
    
