//! Exact no-fixture box-bound regressions from bugs/modalg_7/bug29311_{3,5,6,7,9,11}.
use martensite_tkbrep::topods::Shape;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::{
    bnd::{BoundingBox3, OrientedBoundingBox},
    gp::{Axis1, Direction3, Point3, Transform3},
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::{
    brep_bnd_lib::BRepBndLib,
    brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPITransform},
    brep_gprop::BRepGProp,
};

fn box_at(origin: [f64; 3], dimensions: [f64; 3]) -> Shape {
    BRepPrimAPIMakeBox::at_point(
        Point3::new(origin[0], origin[1], origin[2]),
        dimensions[0],
        dimensions[1],
        dimensions[2],
    )
    .unwrap()
    .shape()
    .unwrap()
    .clone()
}

fn rotated(shape: &Shape, direction: [f64; 3], degrees: f64) -> Shape {
    let axis = Axis1::new(
        Point3::default(),
        Direction3::new(direction[0], direction[1], direction[2]).unwrap(),
    );
    let mut transform = Transform3::default();
    transform.set_rotation(axis, degrees.to_radians()).unwrap();
    BRepBuilderAPITransform::with_shape(shape, transform, false, false)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

fn axis_bounds(shape: &Shape) -> BoundingBox3 {
    let mut bounds = BoundingBox3::default();
    BRepBndLib::add(shape, &mut bounds, true).unwrap();
    bounds
}

fn oriented_bounds(shape: &Shape) -> OrientedBoundingBox {
    let mut bounds = OrientedBoundingBox::default();
    BRepBndLib::add_obb(shape, &mut bounds, true, false, true).unwrap();
    bounds
}

fn assert_axis_pair(first: &Shape, second: &Shape, separated: bool) {
    let first = axis_bounds(first);
    let second = axis_bounds(second);
    assert_eq!(first.is_out_box(&second), separated);
    assert_eq!(second.is_out_box(&first), separated);
}

fn assert_oriented_pair(first: &Shape, second: &Shape, separated: bool) {
    let first = oriented_bounds(first);
    let second = oriented_bounds(second);
    assert_eq!(first.is_out_box(&second), separated);
    assert_eq!(second.is_out_box(&first), separated);
}

#[test]
fn bounding_explicit_corners_builds_box_with_original_volume() {
    let mut maker =
        BRepPrimAPIMakeBox::from_points(Point3::new(50., 100., 30.), Point3::new(180., 200., 100.))
            .unwrap();
    let result = maker.shape().unwrap();
    let mut properties = GProps::default();
    BRepGProp::volume_properties_eps(result, &mut properties, 1e-4, false, false).unwrap();
    let measured = properties.mass();
    assert_ne!(measured, 0.0);
    assert!(((910000.0 - measured) / 910000.0).abs() <= 1e-2);
}

#[test]
fn separated_boxes_are_outside_in_both_axis_comparisons() {
    let first = box_at([100., 60., 140.], [20., 10., 80.]);
    let second = box_at([210., 200., 80.], [120., 60., 90.]);
    assert_axis_pair(&first, &second, true);
}

#[test]
fn contained_boxes_interfere_in_both_axis_comparisons() {
    let first = box_at([0.; 3], [300.; 3]);
    let second = box_at([100.; 3], [50.; 3]);
    assert_axis_pair(&first, &second, false);
}

#[test]
fn partially_overlapping_boxes_interfere_in_both_axis_comparisons() {
    let first = box_at([0.; 3], [300.; 3]);
    let second = box_at([200.; 3], [111., 222., 333.]);
    assert_axis_pair(&first, &second, false);
}

#[test]
fn rotated_containing_box_interferes_in_both_oriented_comparisons() {
    let first = rotated(&box_at([-300.; 3], [600.; 3]), [1., 2., 3.], -28.);
    let second = rotated(&box_at([0.; 3], [100.; 3]), [5., 6., 7.], 34.);
    assert_oriented_pair(&first, &second, false);
}

#[test]
fn axis_overlap_does_not_imply_oriented_box_interference() {
    let first = rotated(&box_at([0.; 3], [300., 100., 100.]), [0., 1., 0.], -30.);
    let second = box_at([200., 0., 50.], [50.; 3]);
    assert_axis_pair(&first, &second, false);
    assert_oriented_pair(&first, &second, true);
}
