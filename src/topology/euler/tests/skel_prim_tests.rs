use crate::{geometry::point::Point, point};
use super::Solid;

#[test]
fn recognition() {
    let mut solid = Solid::new();
    let mut topol = solid.topology_mut();

    let skel_prim = topol.add_skeletal_primitive(point!(0.0, 0.0, 0.0));

    debug_assert!(
        topol.is_skeletal_primitive(skel_prim.face),
        "Solid not recognized as skeletal primitive!"
    );

    debug_assert!(
        solid.n_faces() == 1
        && solid.n_loops() == 1
        && solid.n_edges() == 0
        && solid.n_half_edges() == 1
        && solid.n_vertices() == 1,
        "Solid has wrong number of objects!"
    );
}

#[test]
fn identity() {
    let mut solid = Solid::new();
    let mut topol = solid.topology_mut();

    let skel_prim = topol.add_skeletal_primitive(point!(0.0, 0.0, 0.0));
    topol.rm_skeletal_primitive(skel_prim.face);

    debug_assert!(
        solid.n_faces() == 0
        && solid.n_loops() == 0
        && solid.n_edges() == 0
        && solid.n_half_edges() == 0
        && solid.n_vertices() == 0,
        "Solid not empty!"
    );
}
