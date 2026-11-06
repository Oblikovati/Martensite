use crate::case_view::viewer_session::ViewerSession;
use crate::mesh_view_cases::meshed::{hollow_cut_result, hollow_cut_scene};
use crate::shape_mesh::{MeshRequest, TriangulationSummary, mesh_shape};
use martensite_tkservice::aspect::AspectInteriorStyle;
use martensite_tkservice::graphic3d::{Graphic3dAspectsEdgeMembers, Graphic3dAspectsMembers};
use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;

#[test]
fn actual_case_boolean_rotation_mesh_keeps_all_authored_geometry_checks() {
    let shape = hollow_cut_result().unwrap();
    assert!(crate::shape_checks::check_shape_valid(&shape).unwrap());
    mesh_shape(&shape, &MeshRequest::new(0.1)).unwrap();
    let summary = TriangulationSummary::of(&shape).unwrap();
    assert_eq!((summary.triangles, summary.nodes), (1013, 578));
    assert!((summary.max_deflection - 0.1164052220738387).abs() <= 1e-6);
    assert!(
        crate::mesh_topology_check::check_mesh_topology(&shape)
            .unwrap()
            .is_clean()
    );
}

#[test]
fn actual_null_driver_presentation_retains_hollow_alias_and_visible_edges() {
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let shape = crate::primitive_shapes::sphere(1.).unwrap();
    hollow_cut_scene(&mut session, &shape).unwrap();
    let aspects = session
        .object("result")
        .unwrap()
        .attributes()
        .shading_aspect()
        .aspect();
    assert_eq!(aspects.interior_style(), AspectInteriorStyle::Empty);
    assert!(aspects.to_draw_edges());
}
