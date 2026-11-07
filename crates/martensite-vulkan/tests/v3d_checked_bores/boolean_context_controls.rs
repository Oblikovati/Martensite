//! The direct-context obligation follows the actual analyzer representative, never coordinates.
use super::*;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;

#[test]
fn repeated_orientations_require_the_actual_first_direct_context() {
    let cube = crate::primitive_shapes::box_sized([2.; 3]).unwrap();
    let analyzer = BRepCheckAnalyzer::new(&cube, false, false, false).unwrap();
    let face = entities(&cube, ShapeEnum::Face).unwrap().remove(0);
    let occurrences = entities(&face, ShapeEnum::Vertex).unwrap();
    let mut representatives = MapOfShape::default();
    for vertex in &occurrences {
        representatives.add(vertex.clone());
    }
    assert!(occurrences.len() > representatives.extent());
    assert_eq!(
        required_children(&analyzer, &face, ShapeEnum::Vertex).unwrap(),
        representatives.extent()
    );
    let first = &occurrences[0];
    let checker = analyzer.result(first).unwrap();
    assert!(checker.get().unwrap().is_status_on_shape(&face));
}

#[test]
fn a_wrong_parent_keeps_missing_direct_context_incomplete() {
    let cube = crate::primitive_shapes::box_sized([1.; 3]).unwrap();
    let other = crate::primitive_shapes::box_at([10.; 3], [1.; 3]).unwrap();
    let analyzer = BRepCheckAnalyzer::new(&cube, false, false, false).unwrap();
    let vertex = entities(&cube, ShapeEnum::Vertex).unwrap().remove(0);
    let wrong_parent = entities(&other, ShapeEnum::Face).unwrap().remove(0);
    let checker = analyzer.result(&vertex).unwrap();
    assert!(!checker.get().unwrap().is_status_on_shape(&wrong_parent));
    let cause = check_context(&analyzer, &vertex, &wrong_parent).unwrap_err();
    assert_eq!(cause.exception_type(), "BOP4_IncompleteCoverage");
    assert!(!checker.get().unwrap().is_status_on_shape(&wrong_parent));
}

#[test]
fn coincident_coordinates_do_not_merge_distinct_topology_or_locations() {
    let first = BRepBuilderAPIMakeVertex::new(Point3::new(0., 0., 0.))
        .unwrap()
        .vertex()
        .unwrap();
    let other = BRepBuilderAPIMakeVertex::new(Point3::new(0., 0., 0.))
        .unwrap()
        .vertex()
        .unwrap();
    let located =
        crate::primitive_shapes::rotated_about(first.as_ref(), [0.; 3], [0., 0., 1.], 90.).unwrap();
    assert!(!first.is_partner(&other));
    assert!(first.is_partner(&located) && !first.is_same(&located));
    let mut seen = MapOfShape::default();
    assert!(seen.add(first.into_shape()));
    assert!(seen.add(other.into_shape()));
    assert!(seen.add(located));
    assert_eq!(seen.extent(), 3);
}
