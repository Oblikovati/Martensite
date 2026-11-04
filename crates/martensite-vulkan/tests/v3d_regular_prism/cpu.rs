use super::*;
#[test]
fn actual_periodic_profile_prism_and_regular_first_face_keep_authored_grid_counts() {
    let prism = geometry::actual_prism().unwrap();
    assert!(crate::shape_checks::check_shape_valid(&prism).unwrap());
    let regular = geometry::tessellated_first_face(&prism).unwrap();
    let summary = crate::shape_mesh::TriangulationSummary::of(&regular).unwrap();
    assert_eq!((summary.triangles, summary.nodes), (5000, 2601));
    authored_counts(&regular, 5000., 2601.).unwrap();
}
