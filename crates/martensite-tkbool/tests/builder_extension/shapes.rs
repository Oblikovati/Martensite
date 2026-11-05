use super::*;

macro_rules! list_slot {
    ($test:ident, $read:ident, $write:ident) => {
        #[test]
        fn $test() {
            let mut selected = builder();
            let untouched = builder();
            let shape = distinct_shape();
            let mut edit = selected.extension_state_mut();
            edit.$write().append(shape.clone());
            let read = selected.extension_state();
            assert_eq!(read.$read().extent(), 1);
            assert!(read.$read().first().unwrap().is_same(&shape));
            assert!(untouched.extension_state().$read().is_empty());
            selected.extension_state_mut().$write().clear();
            assert!(selected.extension_state().$read().is_empty());
        }
    };
}
list_slot!(
    avoided_faces_have_owned_nodes,
    avoided_faces,
    avoided_faces_mut
);
list_slot!(
    avoided_edges_have_owned_nodes,
    avoided_edges,
    avoided_edges_mut
);
list_slot!(
    avoided_vertices_have_owned_nodes,
    avoided_vertices,
    avoided_vertices_mut
);
list_slot!(
    working_faces_have_owned_nodes,
    working_faces,
    working_faces_mut
);
list_slot!(
    working_edges_have_owned_nodes,
    working_edges,
    working_edges_mut
);
list_slot!(
    working_solids_have_owned_nodes,
    working_solids,
    working_solids_mut
);

fn distinct_edge() -> Edge {
    let mut edge = Edge::default();
    BRepBuilder::default().make_edge(&mut edge).unwrap();
    edge
}
fn distinct_face() -> Face {
    let mut face = Face::default();
    BRepBuilder::default().make_face(&mut face);
    face
}
fn distinct_solid() -> Solid {
    let mut solid = Solid::default();
    BRepBuilder::default().make_solid(&mut solid);
    solid
}

macro_rules! shape_slot {
    ($test:ident, $read:ident, $write:ident, $fixture:ident) => {
        #[test]
        fn $test() {
            let mut selected = builder();
            let untouched = builder();
            let shape = $fixture();
            *selected.extension_state_mut().$write() = shape.clone();
            assert!(selected.extension_state().$read().is_same(&shape));
            assert!(untouched.extension_state().$read().is_null());
            *selected.extension_state_mut().$write() = $fixture();
            assert!(!selected.extension_state().$read().is_same(&shape));
        }
    };
}
shape_slot!(
    first_operand_rebinds_live_slot,
    first_operand,
    first_operand_mut,
    distinct_shape
);
shape_slot!(
    second_operand_rebinds_live_slot,
    second_operand,
    second_operand_mut,
    distinct_shape
);
shape_slot!(
    reference_edge_rebinds_live_slot,
    reference_edge,
    reference_edge_mut,
    distinct_edge
);
shape_slot!(
    destination_edge_rebinds_live_slot,
    destination_edge,
    destination_edge_mut,
    distinct_edge
);
shape_slot!(
    reference_face_rebinds_live_slot,
    reference_face,
    reference_face_mut,
    distinct_face
);
shape_slot!(
    destination_face_rebinds_live_slot,
    destination_face,
    destination_face_mut,
    distinct_face
);
shape_slot!(
    reference_solid_rebinds_live_slot,
    reference_solid,
    reference_solid_mut,
    distinct_solid
);
shape_slot!(
    destination_solid_rebinds_live_slot,
    destination_solid,
    destination_solid_mut,
    distinct_solid
);
