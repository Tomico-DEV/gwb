use slotmap::new_key_type;

use super::half_edge::HalfEdgeKey;

new_key_type! { pub struct EdgeKey; }

/// An edge, used for the identification of two half-edges.
pub struct Edge {
    /// The first half-edge in the positive direction
    pub pos: HalfEdgeKey,
    /// The second half-edge in the negative direction
    pub neg: HalfEdgeKey,
}

impl Edge {
    /// Create a new edge.
    /// 
    /// This edge will be set as the twins' parent upon being
    /// added into a solid
    pub fn new(pos: HalfEdgeKey, neg: HalfEdgeKey) -> Self {
        Self { pos, neg }
    }

    /// Get the twin of a half-edge belonging to this Edge
    /// 
    /// # Panics
    /// Panics if the given half-edge key does not belong to this Edge
    pub fn get_twin(&self, key: HalfEdgeKey) -> HalfEdgeKey {
        if key == self.pos {
            self.neg
        } else if key == self.neg {
            self.pos
        } else {
            panic!("he_twin_key: Half-edge does not belong to this edge!");
        }
    }
}