use super::super::solid::edge::*;
use super::super::solid::edge_loop::*;
use super::super::solid::face::*;
use super::super::solid::half_edge::*;
use super::super::solid::vertex::*;
///! This module is responsible for Euler operations
use super::super::{Topology, TopologyRead, TopologyWrite};
use crate::geometry::point::Point;

pub mod split_vertex;
pub use split_vertex::*;

/// A collection of keys belonging to a skeletal primitive
///
/// Returned by `new_skeletal_primitive`
pub struct SkeletalPrimtive {
    pub face: FaceKey,
    pub edge_loop: EdgeLoopKey,
    pub half_edge: HalfEdgeKey,
    pub vertex: VertexKey,
}

// skeletal primitives
impl<S: TopologyRead> Topology<S> {
    /// Check if the structure belonging to a face is a
    /// skeletal primitive
    pub fn is_skeletal_primitive(&self, face_key: FaceKey) -> bool {
        let outer_key = self.solid.face(face_key).outer;
        let outer = self.solid.edge_loop(outer_key);
        let he_key = outer.half_edge;
        let he = self.solid.half_edge(he_key);

        self.solid.face(face_key).inner.is_empty()
            && self.solid.edge_loop_is_empty(outer_key)
            && he.edge.is_none()
    }
}

impl<S: TopologyRead + TopologyWrite> Topology<S> {
    /// Create a new skeletal primitive from nothing
    ///
    /// Returns a tuple of the created lone vertex,
    pub fn add_skeletal_primitive(&mut self, origin: Point) -> SkeletalPrimtive {
        // Add lone vertex
        let vertex = self.solid.add_vertex(Vertex::new(origin));
        // Add empty halfedge and an empty loop
        let half_edge = self.solid.add_half_edge(HalfEdge::new(vertex));
        let edge_loop = self.solid.add_edge_loop(EdgeLoop::new(half_edge));
        // Add empty face
        let face = self.solid.add_face(Face::new(edge_loop));

        SkeletalPrimtive {
            face,
            edge_loop,
            half_edge,
            vertex,
        }
    }

    /// Remove a skeletal primitive by specifying its face
    ///
    /// Panics if the face belongs to something other than a skeletal primitive
    pub fn rm_skeletal_primitive(&mut self, face_key: FaceKey) {
        // check the loop is empty and has no edge
        debug_assert!(
            self.is_skeletal_primitive(face_key),
            "rm_skeletal_primitive: face does not belong to a skeletal primitive!"
        );

        // ok, we're probably good
        let outer_key = self.solid.face(face_key).outer;
        let he_key = self.solid.edge_loop(outer_key).half_edge;
        let vtx_key = self.solid.half_edge(he_key).vertex;
        self.solid.rm_face(face_key);
        self.solid.rm_edge_loop(outer_key);
        self.solid.rm_half_edge(he_key);
        self.solid.rm_vertex(vtx_key);
    }
}

// low level Euler operators


pub struct SplitFaceRes {
    pub face: FaceKey,
    pub edge_loop: EdgeLoopKey,
    pub edge: EdgeKey,
}


// Low level euler operators

impl<S: TopologyRead + TopologyWrite> Topology<S> {
    /// Split a face between two vertices of two half-edges belonging to the same face.
    /// Half-edges in [start, end) are assigned to a new loop
    ///
    /// If used on the same half-edges, a new empty loop and a half-edge that has zero
    /// length which is inserted into the old loop are created, and are identified
    /// with an edge.
    pub fn split_face(&mut self, start: HalfEdgeKey, end: HalfEdgeKey) -> SplitFaceRes {
        // create new half-edges
        let he_start_end = HalfEdge::new(self.solid.half_edge(start).vertex);
        let he_start_end = self.solid.add_half_edge(he_start_end); // positive
        let he_end_start = HalfEdge::new(self.solid.half_edge(end).vertex);
        let he_end_start = self.solid.add_half_edge(he_end_start); // negative

        // identify with edge
        let new_edge = self.solid.add_edge(Edge::new(he_start_end, he_end_start));

        // create new loop and face
        let new_loop = self.solid.add_edge_loop(EdgeLoop::new(he_end_start));
        let new_face = self.solid.add_face(Face::new(new_loop));

        // reassign half-edges to new loop
        let mut he_key = start;
        let mut past_start = false;
        while he_key != end {
            // check if this will become an infinite loop
            debug_assert!(
                !(past_start && he_key == start),
                "split_face: Detected a cycle of `start` that does not go through `end`!"
            );

            let he = self.solid.half_edge_mut(he_key);
            he.edge_loop = Some(new_loop);
            he_key = he.get_next();

            past_start = true;
        }

        // now insert the new half-edges into cycles
        self.insert_he_before(he_end_start, start);
        self.insert_he_before(he_start_end, end);

        // fix the loops so they are separated (see diagram in docs and this will make sense lol)

        // get neighborhoods of the new halfedges
        let end_start_neigh = self.solid.half_edge(he_end_start).neighbors();
        let start_end_neigh = self.solid.half_edge(he_start_end).neighbors();

        // fix the previous halfedges
        self.solid
            .half_edge_mut(end_start_neigh.prev)
            .set_next(he_start_end);
        self.solid
            .half_edge_mut(start_end_neigh.prev)
            .set_next(he_end_start);

        // because we modifed the previous half-edges' destination,
        // we need to update our prev pointer too
        self.solid
            .half_edge_mut(he_start_end)
            .set_prev(end_start_neigh.prev);
        self.solid
            .half_edge_mut(he_end_start)
            .set_prev(start_end_neigh.prev);

        // reset the loop assignment
        self.solid.half_edge_mut(he_end_start).edge_loop = Some(new_loop); // insert_he broke this
        let end_start_loop = self.solid.half_edge_mut(he_start_end).get_edge_loop();
        self.set_loop_origin(end_start_loop, he_start_end);

        SplitFaceRes {
            face: new_face,
            edge_loop: new_loop,
            edge: new_edge,
        }
    }

    /// Checks if the half-edge on the positive direction is a strut edge
    pub fn edge_is_strut_pos(&self, edge_key: EdgeKey) -> bool {
        let pos_key = self.solid.edge(edge_key).pos;
        let neg_key = self.solid.edge(edge_key).neg;
        let pos_neigh = self.solid.half_edge(pos_key).neighbors();

        pos_neigh.next == neg_key
    }

    pub fn edge_is_strut_neg(&self, edge_key: EdgeKey) -> bool {
        let pos_key = self.solid.edge(edge_key).pos;
        let neg_key = self.solid.edge(edge_key).neg;
        let neg_neigh = self.solid.half_edge(neg_key).neighbors();

        neg_neigh.prev == pos_key
    }

    /// Split a loop along half-edges that occur twice, which if created properly,
    /// should be an edge. The half-edges following the postive side are assigned
    /// to the new loop.
    ///
    /// For example, if you had a loop with one outer loop and one inner loop and
    /// the positive side of the edge points towards the inner loop, the inner
    /// loop will be assigned to the new loop.
    ///
    /// It will not delete the both half-edges if they belong to a strut edge;
    /// only the ones on the side that is not a strut are deleted, otherwise
    /// they are repurposed
    pub fn split_ring_from_loop(&mut self, edge_key: EdgeKey) -> EdgeLoopKey {
        // Check if this op is valid

        // do the half-edges of this edge belong to the same loop?
        let edge = self.solid.edge(edge_key);
        let neg_key = edge.neg;
        let pos_key = edge.pos;

        let neg = self.solid.half_edge(neg_key);
        let pos = self.solid.half_edge(pos_key);

        debug_assert!(
            neg.edge_loop == pos.edge_loop,
            "split_ring_from_loop: tried removing an edge that is not part of the same loop!"
        );

        // de-identify so ww can remove the half-edges properly
        self.solid.rm_edge(edge_key);

        // fix neighbor topology before half-edge removal
        let neg_edge_loop = self.solid.half_edge(neg_key).get_edge_loop();
        let neg_neigh = self.solid.half_edge(neg_key).neighbors();
        let pos_neigh = self.solid.half_edge(pos_key).neighbors();

        // for the new loop (positive side)
        if self.edge_is_strut_pos(edge_key) {
            // edge case: strut edge on positive side
            // use neg as the new empty half-edge
            let neigh = HalfEdgeNeighbors {
                next: neg_key,
                prev: neg_key,
            };
            self.solid.set_he_neighbors(neg_key, neigh);
        } else {
            // remove edge
            self.solid.rm_half_edge(pos_key);

            self.solid
                .half_edge_mut(neg_neigh.next)
                .set_prev(pos_neigh.prev);
            self.solid
                .half_edge_mut(pos_neigh.prev)
                .set_next(neg_neigh.next);
        }

        // for the unchanged loop (neg side)
        if self.edge_is_strut_neg(edge_key) {
            // edge case: strut edge on negative side
            // make pos empty
            let neigh = HalfEdgeNeighbors {
                next: pos_key,
                prev: pos_key,
            };
            self.solid.set_he_neighbors(pos_key, neigh);
        } else {
            // remove edge
            self.solid.rm_half_edge(neg_key);

            self.solid
                .half_edge_mut(neg_neigh.prev)
                .set_next(pos_neigh.next);
            self.solid
                .half_edge_mut(pos_neigh.next)
                .set_prev(neg_neigh.prev);
        }

        // associate half-edges with new loop
        let new_loop = self.solid.add_edge_loop(EdgeLoop::new(pos_neigh.next));
        let mut he = self.solid.half_edge(pos_neigh.next).get_next();
        // keep going till we come back to the origin of the next loop
        while he != pos_neigh.next {
            self.solid.half_edge_mut(he).edge_loop = Some(new_loop);
        }

        // reassign loop origin (old loop, negative side)
        self.set_loop_origin(neg_edge_loop, neg_neigh.prev);

        new_loop
    }
}
