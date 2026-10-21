//! No-fixture source-preservation cases from bug25957_1 and bug33591.
#[path = "common/geometry_io.rs"]
mod geometry_io;
use geometry_io::written;
use martensite_tkbrep::{brep_tools, topods::Shape};
use martensite_tkmath::gp::{Axis1, Direction3, Point3, Transform3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tktopalgo::{
    brep_builder_api::{
        BRepBuilderAPIMakeShape, BRepBuilderAPINurbsConvert, BRepBuilderAPITransform,
    },
    brep_check::BRepCheckAnalyzer,
};

fn complete_sphere(radius: f64) -> Shape {
    BRepPrimAPIMakeSphere::new(radius)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

#[test]
fn locked_source_sphere_remains_valid_after_nurbs_conversion() {
    crate::isolated_primitive_case::run_primitive_case(
        "sphere_copy_regressions::locked_source_sphere_remains_valid_after_nurbs_conversion",
        locked_source_sphere_remains_valid_after_nurbs_conversion_body,
    );
}

fn locked_source_sphere_remains_valid_after_nurbs_conversion_body() {
    let source = complete_sphere(10.0);
    source.set_locked(true).unwrap();
    let mut operation = BRepBuilderAPINurbsConvert::new();
    operation.perform(&source, false).unwrap();
    assert!(operation.is_done());
    let _converted: &Shape = operation.shape().unwrap();
    assert!(
        BRepCheckAnalyzer::new(&source, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
}

fn geometry_copy(shape: &Shape, transform: Transform3) -> Shape {
    BRepBuilderAPITransform::with_shape(shape, transform, true, false)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

fn copied_quarter_turn(shape: &Shape) -> Shape {
    let axis = Axis1::new(Point3::default(), Direction3::new(0., 1., 0.).unwrap());
    let mut transform = Transform3::default();
    transform.set_rotation(axis, 90_f64.to_radians()).unwrap();
    geometry_copy(shape, transform)
}

fn dumped_surface_count(shape: &Shape) -> usize {
    let bytes = written(|stream| brep_tools::dump(shape, stream).unwrap());
    let dump = String::from_utf8(bytes).unwrap();
    dump.lines()
        .find_map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            match fields.as_slice() {
                ["Dump", "of", count, "surfaces", ..] => Some(count.parse().unwrap()),
                _ => None,
            }
        })
        .expect("shape dump: expected 'Dump of <count> surfaces' record")
}

#[test]
fn copied_sphere_rotation_then_translation_retains_one_dumped_surface() {
    crate::isolated_primitive_case::run_primitive_case(
        "sphere_copy_regressions::copied_sphere_rotation_then_translation_retains_one_dumped_surface",
        copied_sphere_rotation_then_translation_retains_one_dumped_surface_body,
    );
}

fn copied_sphere_rotation_then_translation_retains_one_dumped_surface_body() {
    let source = complete_sphere(80.0);
    let rotated = copied_quarter_turn(&source);
    let result = geometry_copy(&rotated, Transform3::translation([0., 0., 200.]));
    assert_eq!(dumped_surface_count(&result), 1);
}
