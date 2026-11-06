use super::*;

#[test]
fn copied_fine_mesh_survives_both_coarser_requests() {
    let original = retained::cone_face().unwrap();
    mesh_shape(&original, &MeshRequest::new(0.1)).unwrap();
    let baseline = TriangulationSummary::of(&original).unwrap();
    let copy = copied_shape(&original, true).unwrap();
    assert!(!copy.is_same(&original));
    mesh_shape(&original, &MeshRequest::new(1.)).unwrap();
    mesh_shape(&copy, &MeshRequest::new(1.)).unwrap();
    for shape in [&original, &copy] {
        let summary = TriangulationSummary::of(shape).unwrap();
        assert_eq!(
            (summary.triangles, summary.nodes),
            (baseline.triangles, baseline.nodes)
        );
        assert!((summary.max_deflection - baseline.max_deflection).abs() <= 1e-7);
    }
}

#[test]
fn default_copy_has_independent_topology_and_can_be_remeshed() {
    let original = retained::cone_face().unwrap();
    let copy = copied_shape(&original, false).unwrap();
    assert!(!copy.is_same(&original));
    mesh_shape(&original, &MeshRequest::new(0.1)).unwrap();
    mesh_shape(&copy, &MeshRequest::new(0.1)).unwrap();
    let first = TriangulationSummary::of(&original).unwrap();
    let second = TriangulationSummary::of(&copy).unwrap();
    assert_eq!(
        (first.triangles, first.nodes),
        (second.triangles, second.nodes)
    );
    assert_eq!(first.max_deflection, second.max_deflection);
}

#[test]
fn copy_display_mode_is_object_owned_and_context_default_is_preserved() {
    use crate::case_view::{
        SourceViewOutput,
        viewer_session::{DisplayRequest, ViewerSession},
    };
    use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let shape = crate::primitive_shapes::box_sized([1., 2., 3.]).unwrap();
    let before = session.context().display_mode();
    copy_pair::display_copied_mesh(&mut session, &shape).unwrap();
    assert_eq!(session.context().display_mode(), before);
    assert_eq!(session.object("b").unwrap().display_mode(), 1);
    session
        .display("later", &shape, &DisplayRequest::default())
        .unwrap();
    assert_eq!(session.context().display_mode(), before);
    assert!(!session.object("later").unwrap().has_display_mode());
    session.close();
}
