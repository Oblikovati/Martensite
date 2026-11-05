//! Original scaled Boolean cases, including both checkprops and checkview mass paths.
#[path = "boolean_unconditional_case.rs"]
mod unconditional;
/// Preserve boolean/end's checkshape after capture; `check_end_validity(&result)`.
pub fn check_end_validity(result: &Shape) {
    unconditional::check_end(result);
}
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Point3, Transform3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPITransform};

fn scaled(bytes: &[u8], kind: ShapeEnum) -> Shape {
    let original = unconditional::restored_shape(bytes, kind);
    let mut scale = Transform3::default();
    scale.set_scale(Point3::default(), 100.).unwrap();
    BRepBuilderAPITransform::with_shape(&original, scale, false, false)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

fn checked_fuse(first: (&[u8], ShapeEnum), second: &[u8], area: f64) -> Shape {
    let p1 = scaled(first.0, first.1);
    let p2 = scaled(second, ShapeEnum::Solid);
    unconditional::checked_fuse_shapes(&p2, &p1, area)
}

/// Verify checkview's distinct nonadaptive -s branch; `check_view_area(&result)`.
pub fn check_view_area(result: &Shape) {
    unconditional::check_view_result(result);
}

/// Replay original C6 scaling, operand order and area; `c6()`.
pub fn c6() -> Shape {
    checked_fuse(
        (
            include_bytes!(
                "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO900_pro7637_prism_oblong.rle"
            ),
            ShapeEnum::Solid,
        ),
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO900_pro7637_box.rle"
        ),
        1.85425e9,
    )
}

/// Replay original C7 scaling, operand order and area; `c7()`.
pub fn c7() -> Shape {
    checked_fuse(
        (
            include_bytes!(
                "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO900_pro7637c_prism_oblong_dsg.rle"
            ),
            ShapeEnum::Solid,
        ),
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO900_pro7637c_box_dsg.rle"
        ),
        1.85425e9,
    )
}

/// Replay original C8 scaling, operand order and area; `c8()`.
pub fn c8() -> Shape {
    checked_fuse(
        (
            include_bytes!(
                "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO900_cts40185_top.rle"
            ),
            ShapeEnum::Compound,
        ),
        include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/cts40185_ailette.rle"
        ),
        74.7286,
    )
}
