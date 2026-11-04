use super::*;
use martensite_tkservice::null_driver::NullGraphicDriver;
use martensite_tkv3d::{ais::AisTypeOfIso, prs_mgr::PrsMgrPresentableObjectMembers};

fn null_session() -> ViewerSession {
    ViewerSession::with_graphic_driver(&NullGraphicDriver::new_handle(), 409, 409).unwrap()
}
#[test]
fn midpoint_command_text_matches_independent_45_degree_pose_and_direct_rotation() {
    let mut session = null_session();
    let animation =
        rotation::rotation_scene(&mut session, &rotation::rotation_shapes().unwrap()).unwrap();
    crate::v3d_anim_cases::play_at(&session, &animation, 1.).unwrap();
    animation.stop().unwrap();
    let text = session.transformation_text("b2").unwrap();
    assert_eq!(
        text,
        "Location: 71.2965 27.8751 0\nRotation: 0 0 0.382683 0.92388\nScale:    1\n"
    );
    assert_eq!(rotation::direct_rotation(&session).unwrap(), text);
}
#[test]
fn reactivation_retains_both_actual_preview_children() {
    let mut session = null_session();
    parent_selection::child_scene(&mut session).unwrap();
    parent_selection::attach_and_reactivate(&mut session).unwrap();
    let parent = session.object("b").unwrap();
    let children = parent.children();
    assert_eq!(children.len(), 2);
    for name in ["p1", "p2"] {
        assert!(
            children
                .iter()
                .any(|c| c.object_address() == session.object(name).unwrap().object_address())
        );
    }
}
#[test]
fn object_isolines_are_copied_without_changing_default_counts() {
    let mut session = null_session();
    session
        .display(
            "ff",
            &periodic_isoline::periodic_face().unwrap(),
            &Default::default(),
        )
        .unwrap();
    let before = session.context().iso_number(AisTypeOfIso::Both);
    periodic_isoline::configure_isolines(&session).unwrap();
    assert!(session.context().iso_on_plane());
    assert_eq!(session.context().iso_number(AisTypeOfIso::Both), before);
    let drawer = session.object("ff").unwrap().attributes();
    assert_eq!(drawer.u_iso_aspect().number(), 50);
    assert_eq!(drawer.v_iso_aspect().number(), 50);
}

#[test]
fn mesh_information_counts_actual_unique_edge_polygons_and_authored_parameters() {
    let shape = crate::primitive_shapes::box_sized([1., 2., 3.]).unwrap();
    crate::shape_mesh::mesh_shape(&shape, &crate::shape_mesh::MeshRequest::new(1.)).unwrap();
    let info = mesh_information::information(&shape).unwrap();
    assert_eq!(info.polygon_representations, 24);
    assert_eq!(info.summary.triangles, 12);
    assert_eq!(info.summary.nodes, 24);
    assert_eq!(info.parameters[0], 1.);
    assert_eq!(info.parameters[1], 0.5);
}
