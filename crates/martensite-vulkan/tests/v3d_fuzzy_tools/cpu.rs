use super::*;
#[test]
fn actual_all_five_tool_fuzzy_fuse_keeps_authored_area_topology_validity() {
    let result = actual_fuzzy_five_tool_fuse().unwrap();
    case_geometry_checks(&result).unwrap();
    assert_eq!(
        check_shape_counts(&result, "result", &authored_fuzzy_expectation()).unwrap(),
        COUNTS
    );
    assert!(crate::shape_checks::check_shape_valid(&result).unwrap());
}

#[test]
fn fuzzy_count_options_preserve_authored_locations_and_literal_subject() {
    let expected = authored_fuzzy_expectation();
    assert!(expected.per_location);
    assert_eq!(expected.subject, "fuzzy booleans with multiple tools");
}
