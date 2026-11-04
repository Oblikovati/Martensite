//! Null-driver controls exercise actual HLR state and geometric mesh summaries.
use super::*;
use crate::case_view::viewer_session::ViewerSession;
use crate::shape_mesh::TriangulationSummary;
use martensite_tkservice::null_driver::NullGraphicDriver;
use martensite_tkv3d::{
    ais::{AisShape, AisShapeParameters},
    prs_mgr::PrsMgrPresentableObjectMembers,
};

#[test]
fn exact_hlr_is_owned_by_the_named_shape_without_changing_default() {
    let driver = NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let shape = crate::primitive_shapes::box_sized([1.; 3]).unwrap();
    session
        .display("result", &shape, &DisplayRequest::default())
        .unwrap();
    let default = session.context().default_drawer().type_of_hlr();
    session.set_hidden_line_removal(true, None).unwrap();
    crate::hlr_commands::set_object_algorithm(&session, "result", Prs3dTypeOfHlr::Algo).unwrap();
    let object = session.object("result").unwrap().cast::<AisShape>();
    assert_eq!(object.type_of_hlr(), Prs3dTypeOfHlr::Algo);
    assert!(object.attributes().has_own_type_of_hlr());
    assert_eq!(session.context().default_drawer().type_of_hlr(), default);
    assert!(session.view().view_base().computed_mode());
}

#[test]
fn summary_observes_real_triangulations_and_geometry_cleaning() {
    let shape = crate::primitive_shapes::box_sized([1.; 3]).unwrap();
    let before = TriangulationSummary::of(&shape).unwrap();
    assert_eq!((before.faces, before.empty_faces), (6, 6));
    mesh_shape(&shape, &MeshRequest::new(1.)).unwrap();
    let meshed = TriangulationSummary::of(&shape).unwrap();
    assert_eq!(meshed.empty_faces, 0);
    assert!(meshed.triangles > 0 && meshed.nodes > 0);
    mesh_information::print_information(&shape).unwrap();
    crate::shape_mesh::remove_triangulation(&shape).unwrap();
    assert_eq!(TriangulationSummary::of(&shape).unwrap().empty_faces, 6);
}
