use super::*;
#[test]
fn actual_parallel_prepared_build_keeps_independent_overlap_union_volume() {
    let first = crate::primitive_shapes::box_sized([2.; 3]).unwrap();
    let second = crate::primitive_shapes::box_at([1., 0., 0.], [2.; 3]).unwrap();
    let result = operation::parallel_general_build(&[first, second]).unwrap();
    let volume = check_mass(&result, &MassCheck::near(MassKind::Volume, 12.))
        .unwrap()
        .mass();
    assert!((volume - 12.).abs() < 1e-12);
    assert!(crate::shape_checks::check_shape_valid(&result).unwrap());
}

#[test]
fn authored_topology_options_keep_distinct_locations_and_report_subject() {
    let source = authored_topology_expectation();
    assert!(source.per_location);
    assert_eq!(source.subject, "result obtained by Common operator");
    let expected = [2, 0, 0, 0, 0, 0, 0, 1, 3];
    let fixture = ShapeCounts {
        expected: ShapeCounts::all(expected).expected,
        ..source
    };
    let actual = check_shape_counts(&two_located_vertex_occurrences(), "result", &fixture).unwrap();
    assert_eq!(actual, expected);
}
fn two_located_vertex_occurrences() -> Shape {
    use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
    let first = BRepBuilderAPIMakeVertex::new(martensite_tkmath::gp::Point3::default())
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape();
    let moved = crate::primitive_shapes::translated(&first, [1., 0., 0.]).unwrap();
    assert!(first.is_partner(&moved));
    crate::primitive_shapes::compound_of(&[first, moved]).unwrap()
}
