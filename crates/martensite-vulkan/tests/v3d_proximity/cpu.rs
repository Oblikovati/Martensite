use super::*;
#[test]
fn actual_sphere_cone_reported_overlap_faces_match_authored_topology() {
    for shape in actual_overlapping_faces().unwrap() {
        assert_eq!(
            check_shape_counts(&shape, "overlapped", &authored_overlap_expectation()).unwrap(),
            COUNTS
        );
        assert!(crate::shape_checks::check_shape_valid(&shape).unwrap());
    }
}
#[test]
fn incomplete_proximity_profile_refuses_without_fabricated_compounds() {
    let tool = BRepExtremaShapeProximity::new(0.01);
    assert!(
        report_proximity_profile(&tool, std::time::Duration::ZERO, std::time::Duration::ZERO)
            .is_err()
    );
}

#[test]
fn overlap_count_options_preserve_authored_locations_and_literal_subject() {
    let expected = authored_overlap_expectation();
    assert!(expected.per_location);
    assert_eq!(expected.subject, "Overlapped part of shere");
}
