//! Independent identity fixtures pin enumeration before its structural move.
use super::typed_exploration::{all_distinct, first_distinct};
use martensite_tkbrep::topods::{Builder, Compound, Shape};
use martensite_tkg3d::top_abs::{Orientation, ShapeEnum};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;

fn compound(children: &[Shape]) -> Shape {
    let mut result = Compound::default();
    Builder.make_compound(&mut result);
    for child in children {
        Builder.add(&mut result, child).unwrap();
    }
    result.into_shape()
}
fn vertex() -> Shape {
    BRepBuilderAPIMakeVertex::new(Point3::new(3., 4., 5.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}
#[test]
fn source_root_is_excluded_while_typed_child_is_preserved() {
    let child = vertex();
    let root = compound(std::slice::from_ref(&child));
    // A matching root stops Explorer descent; seeding excludes that root.
    assert!(all_distinct(&root, ShapeEnum::Compound).is_empty());
    assert!(all_distinct(&child, ShapeEnum::Vertex).is_empty());
    let actual = all_distinct(&root, ShapeEnum::Vertex);
    assert_eq!(actual.len(), 1);
    assert!(actual[0].is_same(&child));
    assert!(first_distinct(&root, ShapeEnum::Vertex).is_same(&child));
}

#[test]
fn repeated_handle_and_reversed_orientation_share_one_binding() {
    let source = vertex();
    let root = compound(&[source.clone(), source.oriented(Orientation::Reversed)]);
    let actual = all_distinct(&root, ShapeEnum::Vertex);
    assert_eq!(actual.len(), 1);
    assert!(actual[0].is_same(&source));
    assert!(first_distinct(&root, ShapeEnum::Vertex).is_same(&source));
}
#[test]
fn equal_coordinates_never_merge_distinct_identities() {
    let first = vertex();
    let second = vertex();
    assert!(!first.is_same(&second));
    let root = compound(&[first.clone(), second.clone()]);
    let actual = all_distinct(&root, ShapeEnum::Vertex);
    assert_eq!(actual.len(), 2);
    assert!(actual[0].is_same(&first));
    assert!(actual[1].is_same(&second));
}
