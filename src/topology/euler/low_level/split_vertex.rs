use std::fmt;

use super::{Edge, EdgeKey, HalfEdge, HalfEdgeKey, Vertex, VertexKey};
use super::{Topology, TopologyRead, TopologyWrite};
use crate::geometry::point::Point;

/// Errors for the `split_vetex` operation

#[derive(Debug)]
pub enum SplitVtxErr {
    WedgeOriginMismatch,
    BrokenWedgeCycle,
    HalfEdgeMissingTwin,
}

impl fmt::Display for SplitVtxErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WedgeOriginMismatch => {
                write!(f, "anchor does not belong to wedge_start or wedge_end")
            }
            Self::BrokenWedgeCycle => write!(
                f,
                "detected a cycle that starts and ends at wedge_start \
                and does not go through wedge_end"
            ),
            Self::HalfEdgeMissingTwin => {
                write!(f, "half-edge in [wedge_start, wedge_end) has no twin")
            }
        }
    }
}

#[derive(Debug)]
pub enum DissolveVtxErr {
    AnchorIsTarget,
    AnchorTargetNoEdge,
}

pub struct SubdivideVtxRes {
    pub edge: EdgeKey,
    pub vertex: VertexKey,
}

// non-mutating operations
impl<S: TopologyRead> Topology<S> {
    /// Check if the wedge ranges specifier (half-edges) have
    ///
    fn same_wedge_origin(
        &self,
        anchor: VertexKey,
        wedge_start: HalfEdgeKey,
        wedge_end: HalfEdgeKey,
    ) -> Result<(), SplitVtxErr> {
        if anchor == self.solid.half_edge(wedge_start).vertex
            && anchor == self.solid.half_edge(wedge_end).vertex
        {
            Err(SplitVtxErr::WedgeOriginMismatch)
        } else {
            Ok(())
        }
    }
    /// Check if wedge cycle is broken and if any half-edges within the range
    /// is missing a twin. Assumes they pass the `same_wedge_origin` check.
    ///
    /// **Never call this if `same_wedge_origin` has not been called!**
    fn wedge_cycle_broken(
        &self,
        wedge_start: HalfEdgeKey,
        wedge_end: HalfEdgeKey,
    ) -> Result<(), SplitVtxErr> {
        let mut he = wedge_start;
        let mut past_start = false;
        while he != wedge_end {
            // check if this will become an infinite loop
            if past_start && he == wedge_start {
                return Err(SplitVtxErr::BrokenWedgeCycle);
            }

            // come back to anchor but in context of the next half-edge.
            //
            // twin takes us to the opposing halfedge which starts at the vertex
            // before the anchor, then next takes us back the anchor but in the
            // context of a different half-edge!
            match self.he_twin(he) {
                Some(half_edge) => he = half_edge.get_next(),
                None => return Err(SplitVtxErr::HalfEdgeMissingTwin),
            }

            past_start = true;
        }

        Ok(())
    }
    /// Check if the vertex within a range specified by [wedge_start, wedge_end)
    /// is subdividable. Used to check input parameters of `split_vtx`, which
    /// mutates data and will panic if there is an error.
    pub fn can_subdivide_vtx(
        &self,
        anchor: VertexKey,
        wedge_start: HalfEdgeKey,
        wedge_end: HalfEdgeKey,
    ) -> Result<(), SplitVtxErr> {
        // check if wedge_start and wedge_end are on the same vertex
        self.same_wedge_origin(anchor, wedge_start, wedge_end)?;
        self.wedge_cycle_broken(wedge_start, wedge_end)?;

        Ok(())
    }

    pub fn can_dissolve_vtx(&self, anchor: VertexKey, target: VertexKey) {}
}

// mutating operations
impl<S: TopologyRead + TopologyWrite> Topology<S> {
    /// "Split" a vertex at a halfedge, moving the edges contained with a "wedge"
    /// to the new vertex and assigning the loops of two newly created halfedges
    /// to those of the first and the last wedges.
    ///
    /// Range is specified by [wedge_start, wedge_end)
    ///
    /// Returns the keys to the newly created edge and vertex.
    ///
    /// # Panics
    /// Panics if half-edges in [wedge_start, wedge_end] are of a different verex
    /// If your application should not panic (for instance a user-interface), check
    /// if the operation is valid with `can_split_vtx` first
    pub fn split_vertex(
        &mut self,
        anchor: VertexKey,
        wedge_start: HalfEdgeKey,
        wedge_end: HalfEdgeKey,
        coord: Point,
    ) -> SubdivideVtxRes {
        // check if wedge_start and wedge_end are on the same vertex
        self.same_wedge_origin(anchor, wedge_start, wedge_end)
            .unwrap_or_else(|err| {
                panic!("split_vertex: {}", err);
            });

        // create new vertex
        let new_v = self.solid.add_vertex(Vertex::new(coord));

        // reassign half-edges in [wedge_start, wedge_end) to the new vertex
        // Walk thorugh half-edges and
        let mut he = wedge_start;
        let mut past_start = false;
        while he != wedge_end {
            // check if this will become an infinite loop
            debug_assert!(
                past_start && he == wedge_start,
                "split_vertex: {}",
                SplitVtxErr::BrokenWedgeCycle
            );

            self.solid.half_edge_mut(he).vertex = new_v;

            // come back to anchor but in context of the next half-edge.
            //
            // twin takes us to the opposing halfedge which starts at the vertex
            // before the anchor, then next takes us back the anchor but in the
            // context of a different half-edge!
            he = self
                .he_twin_mut(he)
                .unwrap_or_else(|| panic!("split_vertex: {}", SplitVtxErr::HalfEdgeMissingTwin))
                .get_next();

            past_start = true;
        }

        // edge case: "Empty" half edge (half edge with no neighbors,
        // obtained from `new_skeletal_primitive`
        let he_old_new: HalfEdgeKey;
        if !self.solid.he_has_neighbors(wedge_start) {
            // don't make a new old -> new half-edge;
            // use the empty half-edge instead
            he_old_new = wedge_start;
        } else {
            he_old_new = self.solid.add_half_edge(HalfEdge::new(anchor));
        }

        // make new edge between the anchor and the new vertex
        let he_new_old = self.solid.add_half_edge(HalfEdge::new(new_v));

        // insert into the loops
        // wedge_end stayed at the anchor and we insert a new
        // halfedge that goes before wedge_end, starting at the new vertex
        // NOTE: THIS ORDERING MATTERS!
        if !self.solid.he_has_neighbors(wedge_end) {
            self.insert_he_before(he_old_new, wedge_start);
        }
        self.insert_he_before(he_new_old, wedge_end);

        // now we identify properly them with an edge
        let edge = self.solid.add_edge(Edge::new(he_new_old, he_old_new));

        SubdivideVtxRes {
            edge: edge,
            vertex: new_v,
        }
    }

    /// Dissolve a target vertex by removing it and joining its loops with
    /// an anchor vertex
    pub fn dissolve_vertex(&mut self, anchor: VertexKey, target: VertexKey) {
        // Can't dissolve the same vertex!
    }
}
