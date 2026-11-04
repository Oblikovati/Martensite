use super::*;
use crate::shape_mesh::TriangulationSummary;

#[test]
fn status_words_preserve_actual_native_bit_order_and_zero() {
    assert_eq!(meshing::status_words(0), "NoError");
    assert_eq!(meshing::status_words(128), "Reused ");
    assert_eq!(
        meshing::status_words(511),
        "OpenWire SelfIntersectingWire Failure ReMesh UnorientedWire TooFewPoints Outdated Reused UserBreak "
    );
    assert_eq!(meshing::status_words(1024), "");
}

#[test]
fn both_actual_mesh_strategies_keep_requested_parameters_and_generate_triangles() {
    for algorithm in [MeshAlgoType::Watson, MeshAlgoType::Delabella] {
        let shape = crate::primitive_shapes::sphere(1.).unwrap();
        let result = meshing::mesh_with_strategy(
            &shape,
            MeshRequest::new(0.5).with_angle_degrees(50.),
            algorithm,
        )
        .unwrap();
        assert_eq!(result.status & 4, 0);
        assert_eq!(result.effective.mesh_algo, algorithm);
        assert_eq!(result.effective.deflection, 0.5);
        assert_eq!(result.effective.angle, 50. * std::f64::consts::PI / 180.);
        assert!(TriangulationSummary::of(&shape).unwrap().triangles > 0);
    }
}

#[test]
fn authored_broken_mesh_is_reused_with_three_nodes() {
    let shape = crate::dataset_shape::read_dataset_shape("bug32692_broken_mesh.brep").unwrap();
    let request = MeshRequest {
        parallel: true,
        ..MeshRequest::new(0.01)
    };
    let result = meshing::mesh_with_strategy(&shape, request, MeshAlgoType::Default).unwrap();
    assert_eq!(meshing::status_words(result.status), "Reused ");
    assert!(result.effective.in_parallel);
    let summary = TriangulationSummary::of(&shape).unwrap();
    assert_eq!(summary.nodes, 3);
    assert!(summary.triangles > 0 && summary.faces > 0);
}

#[test]
fn comparison_command_keeps_both_named_shapes_displayed_in_shaded_mode() {
    use crate::case_view::{SourceViewOutput, viewer_session::ViewerSession};
    use martensite_tkv3d::prs_mgr::PrsMgrPresentableObjectMembers;
    let driver = martensite_tkservice::null_driver::NullGraphicDriver::new_handle();
    let mut session = ViewerSession::with_graphic_driver(&driver, 409, 409).unwrap();
    let first = crate::primitive_shapes::box_sized([1., 2., 3.]).unwrap();
    let second = crate::primitive_shapes::sphere(1.).unwrap();
    comparison::display_edges(&mut session, &[("t1", &first), ("t2", &second)]).unwrap();
    for name in ["t1", "t2"] {
        let object = session.object(name).unwrap();
        assert!(session.context().is_displayed(object));
        assert_eq!(object.display_mode(), 1);
    }
    session.close();
}
