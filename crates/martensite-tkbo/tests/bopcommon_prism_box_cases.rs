//! Native Rust translations of prism-and-box common regressions.
#![allow(non_snake_case)]

use martensite_tkbo::brep_algo_api::{BRepAlgoAPICommon, BRepAlgoAPIFuse};
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Point3, Vector3};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeBox, BRepPrimAPIMakePrism};
use martensite_tktopalgo::{brep_builder_api::BRepBuilderAPIMakeShape, brep_gprop::BRepGProp};

use super::restored_shape;

fn box_shape(source: &str, origin: Point3, size: f64) -> Shape {
    BRepPrimAPIMakeBox::at_point(origin, size, size, size)
        .unwrap_or_else(|failure| panic!("{source}: box failed: {failure:?}"))
        .shape()
        .unwrap_or_else(|failure| panic!("{source}: box result failed: {failure:?}"))
        .clone()
}

fn prism(source: &str, fixture: &[u8], direction: Vector3) -> Shape {
    let fixture = restored_shape(fixture);
    let mut maker = BRepPrimAPIMakePrism::new(&fixture, direction, false, true)
        .unwrap_or_else(|failure| panic!("{source}: prism failed: {failure:?}"));
    maker
        .shape()
        .unwrap_or_else(|failure| panic!("{source}: prism result failed: {failure:?}"))
        .clone()
}

fn first_solid(source: &str, shape: &Shape) -> Shape {
    let mut explorer = Explorer::new(shape, ShapeEnum::Solid, ShapeEnum::Shape)
        .unwrap_or_else(|failure| panic!("{source}: solid exploration failed: {failure:?}"));
    Iterator::next(&mut explorer).unwrap_or_else(|| panic!("{source}: expected first solid"))
}

fn fused_boxes(source: &str, small_origin: Point3) -> Shape {
    let base = box_shape(source, Point3::default(), 10.);
    let small = box_shape(source, small_origin, 4.);
    let fused = BRepAlgoAPIFuse::from_shapes(&base, &small)
        .unwrap_or_else(|failure| panic!("{source}: bopfuse failed: {failure:?}"));
    assert!(fused.is_done(), "{source}: bopfuse incomplete");
    fused.shape().clone()
}

fn common_area(source: &str, small_origin: Point3, fixture: &[u8], direction: Vector3) -> f64 {
    let boxes = first_solid(source, &fused_boxes(source, small_origin));
    let prism = prism(source, fixture, direction);
    let result = BRepAlgoAPICommon::from_shapes(&boxes, &prism)
        .unwrap_or_else(|failure| panic!("{source}: bopcommon failed: {failure:?}"));
    assert!(result.is_done(), "{source}: bopcommon incomplete");
    let mut properties = GProps::default();
    BRepGProp::surface_properties(result.shape(), &mut properties, false, false)
        .unwrap_or_else(|failure| panic!("{source}: area failed: {failure:?}"));
    properties.mass()
}

fn assert_common_area(
    source: &str,
    small_origin: Point3,
    fixture: &[u8],
    direction: Vector3,
    expected_area: f64,
) {
    let area = common_area(source, small_origin, fixture, direction);
    if expected_area == 0. {
        assert!(
            area.abs() <= 1e-6,
            "{source}: expected empty area, got {area}"
        );
        return;
    }
    assert!(
        (area - expected_area).abs() / expected_area <= 0.01,
        "{source}: area {area}"
    );
}

/// Replays `tests/boolean/bopcommon_complex/K6` without command-host at runtime.
#[test]
fn BOPCommonComplex__K6() {
    let source = "tests/boolean/bopcommon_complex/K6";
    let result = BRepAlgoAPICommon::from_shapes(
        &fused_boxes(source, Point3::new(3., 3., 6.)),
        &prism(
            source,
            include_bytes!(
                "../../../external/martensite-test-data/dataset-7.9.0/brep/A1_rr_7.brep"
            ),
            Vector3::new(0., 0., 4.),
        ),
    )
    .unwrap_or_else(|failure| panic!("{source}: bopcommon failed: {failure:?}"));
    assert!(result.is_done(), "{source}: bopcommon incomplete");
    let mut properties = GProps::default();
    BRepGProp::surface_properties(result.shape(), &mut properties, false, false)
        .unwrap_or_else(|failure| panic!("{source}: area failed: {failure:?}"));
    assert!(
        properties.mass().abs() <= 1e-6,
        "{source}: expected empty area, got {}",
        properties.mass()
    );
}

#[test]
fn BOPCommonComplex__K7() {
    let source = "tests/boolean/bopcommon_complex/K7";
    let boxes = first_solid(source, &fused_boxes(source, Point3::new(3., 3., 6.)));
    let result = BRepAlgoAPICommon::from_shapes(
        &boxes,
        &prism(
            source,
            include_bytes!(
                "../../../external/martensite-test-data/dataset-7.9.0/brep/A1_rr_7.brep"
            ),
            Vector3::new(0., 0., -4.),
        ),
    )
    .unwrap_or_else(|failure| panic!("{source}: bopcommon failed: {failure:?}"));
    assert!(result.is_done(), "{source}: bopcommon incomplete");
    let mut properties = GProps::default();
    BRepGProp::surface_properties(result.shape(), &mut properties, false, false).unwrap();
    assert!(
        (properties.mass() - 96.).abs() / 96. <= 0.01,
        "{source}: area {}",
        properties.mass()
    );
}

#[test]
fn BOPCommonComplex__K8() {
    assert_common_area(
        "tests/boolean/bopcommon_complex/K8",
        Point3::new(0., 0., 6.),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/C1_rr_8.brep"),
        Vector3::new(0., 0., -4.),
        96.,
    );
}

#[test]
fn BOPCommonComplex__K9() {
    assert_common_area(
        "tests/boolean/bopcommon_complex/K9",
        Point3::new(0., 0., 6.),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/C1_rr_8.brep"),
        Vector3::new(0., 0., 4.),
        0.,
    );
}

#[test]
fn BOPCommonComplex__L1() {
    assert_common_area(
        "tests/boolean/bopcommon_complex/L1",
        Point3::new(0., 0., 2.),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/E1_rr_7.brep"),
        Vector3::new(0., -4., 0.),
        0.,
    );
}

#[test]
fn BOPCommonComplex__L2() {
    assert_common_area(
        "tests/boolean/bopcommon_complex/L2",
        Point3::new(0., 0., 2.),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/E1_rr_7.brep"),
        Vector3::new(0., 4., 0.),
        96.,
    );
}
