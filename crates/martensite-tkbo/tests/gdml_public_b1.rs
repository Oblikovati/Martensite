//! Native replay of the analytic public exchange-model boolean regression.
#![allow(non_snake_case)]

#[path = "bop_test_utilities/mod.rs"]
mod bop_test_utilities;

use bop_test_utilities::{CreateBox, CreateCylinder, PerformCut, PerformFuse, RotateAroundAxis};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topexp::{self, Explorer},
    topods::{self, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Point3, Transform3, Vector3};
use martensite_tkmath::toploc::Location;
use martensite_tktopalgo::brep_gprop::BRepGProp;

fn translate(shape: &mut Shape, x: f64, y: f64, z: f64) {
    let mut transform = Transform3::default();
    transform.set_translation(Vector3::new(x, y, z));
    shape.move_by(&Location::new(transform), false).unwrap();
}

fn transformed_secondary_box() -> martensite_tkbrep::topods::Shape {
    let mut shape = CreateBox(Point3::new(-127., -152.4, -381.), 254., 304.8, 762.);
    RotateAroundAxis(
        &mut shape,
        Point3::new(0., 0., 0.),
        [-0.57735026918962584; 3],
        120.,
    );
    translate(&mut shape, 0., 0., 127.);
    RotateAroundAxis(&mut shape, Point3::new(0., 0., 0.), [0., 1., 0.], 90.);
    translate(&mut shape, 508., 508., -317.5);
    shape
}

#[test]
fn GdmlPublic__B1() {
    let mut primary = CreateBox(Point3::new(-190.5, -254., -381.), 381., 508., 762.);
    RotateAroundAxis(&mut primary, Point3::new(0., 0., 0.), [0., 1., 0.], 90.);
    translate(&mut primary, 508., 508., -317.5);
    let result = PerformFuse(&primary, &transformed_secondary_box());
    assert!(!result.is_null());
}

fn set_shape_subshape_tolerance(shape: &Shape, kind: ShapeEnum, tolerance: f64) {
    let builder = BRepBuilder::default();
    let mut explorer = Explorer::new(shape, kind, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let current = explorer.current();
        match kind {
            ShapeEnum::Vertex => builder
                .update_vertex_tolerance(&topods::vertex(current).unwrap(), tolerance)
                .unwrap(),
            ShapeEnum::Edge => builder
                .update_edge_tolerance(&topods::edge(current).unwrap(), tolerance)
                .unwrap(),
            ShapeEnum::Face => builder
                .update_face_tolerance(&topods::face(current).unwrap(), tolerance)
                .unwrap(),
            _ => unreachable!(),
        }
        explorer.next().unwrap();
    }
}

fn set_shape_tolerance(shape: &Shape, tolerance: f64) {
    for kind in [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face] {
        set_shape_subshape_tolerance(shape, kind, tolerance);
    }
}

fn assert_relative(actual: f64, expected: f64) {
    assert!((actual - expected).abs() / expected <= 0.01);
}

fn shape_count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut values = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut values).unwrap();
    values.extent()
}

fn transformed_cylinder_for_b2() -> martensite_tkbrep::topods::Shape {
    let mut shape = CreateCylinder(76.2, 1016.);
    set_shape_tolerance(&shape, 1.016e-5);
    translate(&mut shape, 0., 0., -508.);
    RotateAroundAxis(
        &mut shape,
        Point3::new(0., 0., 0.),
        [
            -0.70710677745571249,
            3.6866829955513545e-9,
            0.70710678491738266,
        ],
        -179.99999957753724,
    );
    translate(
        &mut shape,
        186.12362050792001,
        -136.17581735759262,
        126.99999945592533,
    );
    RotateAroundAxis(
        &mut shape,
        Point3::new(0., 0., 0.),
        [
            -0.679047659633503,
            -0.27890596661104489,
            0.67904766970737829,
        ],
        148.8317924119616,
    );
    translate(&mut shape, 0., -309.31813765, -164.27444069);
    shape
}

fn transformed_box_for_b2() -> martensite_tkbrep::topods::Shape {
    let mut shape = CreateBox(
        Point3::new(-321.876380825, -263.17581868, -254.),
        643.75276165,
        526.35163736,
        508.,
    );
    RotateAroundAxis(
        &mut shape,
        Point3::new(0., 0., 0.),
        [
            -0.679047659633503,
            -0.27890596661104489,
            0.67904766970737829,
        ],
        148.8317924119616,
    );
    translate(&mut shape, 0., -309.31813765, -164.27444069);
    shape
}

#[test]
fn GdmlPublic__B2() {
    let result = PerformCut(&transformed_box_for_b2(), &transformed_cylinder_for_b2());
    let mut surface = GProps::default();
    BRepGProp::surface_properties(&result, &mut surface, false, false).unwrap();
    let mut volume = GProps::default();
    BRepGProp::volume_properties(&result, &mut volume, false, false, false).unwrap();
    assert_relative(surface.mass(), 2.13824e6);
    assert_relative(volume.mass(), 1.60388e8);
    for (kind, expected) in [
        (ShapeEnum::Vertex, 10),
        (ShapeEnum::Edge, 15),
        (ShapeEnum::Wire, 9),
        (ShapeEnum::Face, 7),
        (ShapeEnum::Shell, 1),
        (ShapeEnum::Solid, 1),
    ] {
        assert_eq!(shape_count(&result, kind), expected, "{kind:?}");
    }
}

fn transformed_cylinder_for_b3() -> martensite_tkbrep::topods::Shape {
    let mut shape = CreateCylinder(76.2, 508.);
    set_shape_tolerance(&shape, 5.08e-6);
    translate(&mut shape, 0., 0., -254.);
    RotateAroundAxis(
        &mut shape,
        Point3::new(0., 0., 0.),
        [
            -0.70710677745571249,
            3.6866829955513545e-9,
            0.70710678491738266,
        ],
        -179.99999957753724,
    );
    translate(
        &mut shape,
        -67.876379483809757,
        -136.17581735118694,
        127.0000021362337,
    );
    RotateAroundAxis(
        &mut shape,
        Point3::new(0., 0., 0.),
        [
            -0.679047659633503,
            -0.27890596661104489,
            0.67904766970737829,
        ],
        148.8317924119616,
    );
    translate(&mut shape, 0., -309.31813765, -164.27444069);
    shape
}

#[test]
fn GdmlPublic__B3() {
    let source = "tests/boolean/gdml_public/B3";
    let result = PerformCut(&transformed_box_for_b2(), &transformed_cylinder_for_b3());
    assert!(!result.is_null(), "{source}: cut result");
}
