use super::*;
use crate::shape_checks::{MassCheck, MassKind, check_mass};
use crate::shape_report_checks::{ShapeCounts, check_max_tolerance, check_shape_counts};
use martensite_tkg3d::top_abs::ShapeEnum;

#[test]
fn case_large_tolerance_face_keeps_counts_and_genuine_expected_area_failure() {
    let shape = read_dataset_shape("OCC21415-face_bad_meshed.brep").unwrap();
    let expected = [1, 2, 1, 1, 0, 0, 0, 0, 5];
    assert_eq!(
        check_shape_counts(&shape, "result", &ShapeCounts::all(expected)).unwrap(),
        expected
    );
    let mass = check_mass(&shape, &MassCheck::near(MassKind::Area, 400000.))
        .unwrap()
        .mass();
    assert!((crate::shape_checks::six_significant_digits(mass) - 400000.).abs() > 4000.);
}

fn assert_section_counts(result: &Shape) {
    assert_eq!(
        crate::boolean_typed_explode::all_distinct(result, ShapeEnum::Vertex).len(),
        2
    );
    assert_eq!(
        crate::boolean_typed_explode::all_distinct(result, ShapeEnum::Edge).len(),
        1
    );
}

#[test]
fn real_case_section_has_authored_topology_length_and_tolerance() {
    let (result, _) = short_section::source_section().unwrap();
    assert_section_counts(&result);
    let mass = check_mass(&result, &MassCheck::near(MassKind::Length, 0.125))
        .unwrap()
        .mass();
    assert!((crate::shape_checks::six_significant_digits(mass) - 0.125).abs() <= 0.00125);
    let tolerance = check_max_tolerance(&result, "0.0001").unwrap();
    assert!((tolerance - 0.0001).abs() <= 0.000101);
}

#[test]
fn case_fourth_edge_dump_has_no_authored_duplicate_node_pattern() {
    let shape = polygon_nodes::checked_mesh().unwrap();
    let dump = polygon_nodes::fourth_edge_dump(&shape).unwrap();
    assert!(dump.contains("PolygonOnTriangulations"));
    assert!(!dump.contains("5 5"));
    assert!(
        crate::mesh_topology_check::check_mesh_topology(&shape)
            .unwrap()
            .is_clean()
    );
}
