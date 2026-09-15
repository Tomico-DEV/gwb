use super::Solid;
use super::{Vertex, HalfEdge, HalfEdgeNeighbors};
use super::super::TopologyWrite;
use crate::{geometry::point::Point, point};

#[test]
fn set_he_neighbors_test() {
    let mut solid = Solid::new();

    let vert = Vertex::new(point!(0.0, 0.0, 0.0));
    let vert_key = solid.add_vertex(vert);
    
    let he = HalfEdge::new(vert_key);
    let he_key = solid.add_half_edge(he);

    let he1 = HalfEdge::new(vert_key);
    let he1_key = solid.add_half_edge(he1);
    let he2 = HalfEdge::new(vert_key);
    let he2_key = solid.add_half_edge(he2);
    
    let valid_neigh_1 = HalfEdgeNeighbors {
        prev: he_key,
        next: he_key,
    };

    solid.set_he_neighbors(he_key, valid_neigh_1);

    let valid_neigh_2 = HalfEdgeNeighbors {
        prev: he1_key,
        next: he2_key,
    };

    solid.set_he_neighbors(he_key, valid_neigh_2);    
}