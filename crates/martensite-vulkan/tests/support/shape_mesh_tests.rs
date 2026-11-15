use super::*;

#[test]
fn mesh_request_converts_degrees_and_clamps_deflection() {
    let parameters = MeshRequest::new(0.).with_angle_degrees(180.).parameters();
    assert_eq!(parameters.deflection, precision::confusion());
    assert_eq!(parameters.deflection_interior, precision::confusion());
    assert_eq!(parameters.angle, std::f64::consts::PI);
}

#[test]
fn empty_shape_has_no_faces() {
    assert_eq!(
        TriangulationSummary::of(&Shape::default()).unwrap(),
        TriangulationSummary::default()
    );
}
