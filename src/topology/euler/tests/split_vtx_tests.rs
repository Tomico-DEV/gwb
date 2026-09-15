use crate::{geometry::point::Point, point};
use super::Solid;

#[test]
fn split_vtx() {
    let mut solid = Solid::new();
    let mut topol = solid.topology_mut();

    let skel_prim = topol.add_skeletal_primitive(point!(0.0, 0.0, 0.0));
    let res = topol.split_vertex(
        skel_prim.vertex, 
        skel_prim.half_edge, skel_prim.half_edge,
        point!(1.0, 0.0, 0.0)
    );

    debug_assert!(
        topol.edge_is_strut_neg(res.edge)
        && topol.edge_is_strut_pos(res.edge),
        "Created edge is not a strut edge!"
    );
}
