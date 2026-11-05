//! Native Rust translations of prism-and-box two-step cut regressions.
#![allow(non_snake_case)]

use martensite_tkbo::{
    bop_algo::{BOPAlgoBOP, BOPAlgoPaveFiller, Operation},
    brep_algo_api::BRepAlgoAPIFuse,
};
use martensite_tkbrep::{brep::BRepBuilder, brep_tools, topexp::Explorer, topods::Shape};
use martensite_tkernel::{message::ProgressRange, ncollection::List, storage::IoStream};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::gp::{Point3, Vector3};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeBox, BRepPrimAPIMakePrism};
use martensite_tktopalgo::{brep_builder_api::BRepBuilderAPIMakeShape, brep_gprop::BRepGProp};

fn restored_shape(bytes: &[u8]) -> Shape {
    let header_end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .expect("BRep fixture: expected drawable header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[header_end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .expect("BRep fixture: expected readable shape");
    shape
}

fn box_shape(source: &str, origin: Point3) -> Shape {
    BRepPrimAPIMakeBox::at_point(origin, 4., 4., 4.)
        .unwrap_or_else(|failure| panic!("{source}: box failed: {failure:?}"))
        .shape()
        .unwrap_or_else(|failure| panic!("{source}: box result failed: {failure:?}"))
        .clone()
}

fn fused_boxes(source: &str, small_origin: Point3) -> Shape {
    let mut base = BRepPrimAPIMakeBox::at_point(Point3::default(), 10., 10., 10.)
        .unwrap_or_else(|failure| panic!("{source}: base box failed: {failure:?}"));
    let base = base
        .shape()
        .unwrap_or_else(|failure| panic!("{source}: base box result failed: {failure:?}"));
    let fused = BRepAlgoAPIFuse::from_shapes(base, &box_shape(source, small_origin))
        .unwrap_or_else(|failure| panic!("{source}: bopfuse failed: {failure:?}"));
    assert!(fused.is_done(), "{source}: bopfuse incomplete");
    fused.shape().clone()
}

fn first_solid(source: &str, shape: &Shape) -> Shape {
    let mut explorer = Explorer::new(shape, ShapeEnum::Solid, ShapeEnum::Shape)
        .unwrap_or_else(|failure| panic!("{source}: solid exploration failed: {failure:?}"));
    Iterator::next(&mut explorer).unwrap_or_else(|| panic!("{source}: expected first solid"))
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

fn prepared_filler(source: &str, first: &Shape, second: &Shape) -> BOPAlgoPaveFiller {
    let arguments: List<Shape> = [first.clone(), second.clone()].into_iter().collect();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&arguments);
    filler.options_mut().set_run_parallel(false);
    filler.perform();
    assert!(
        !filler.options().has_errors(),
        "{source}: bop preparation failed"
    );
    filler
}

fn tuc_area(source: &str, first: &Shape, second: &Shape) -> f64 {
    let mut filler = prepared_filler(source, first, second);
    let mut tuc = BOPAlgoBOP::new();
    tuc.add_argument(first);
    tuc.add_tool(second);
    tuc.set_operation(Operation::Cut21);
    tuc.options_mut().set_run_parallel(false);
    tuc.perform_with_filler(&mut filler);
    assert!(!tuc.options().has_errors(), "{source}: boptuc failed");
    let mut properties = GProps::default();
    BRepGProp::surface_properties(tuc.shape(), &mut properties, false, false)
        .unwrap_or_else(|failure| panic!("{source}: area failed: {failure:?}"));
    properties.mass()
}

fn assert_tuc_area(
    source: &str,
    small_origin: Point3,
    fixture: &[u8],
    direction: Vector3,
    select_solid: bool,
    expected_area: f64,
) {
    let fused = fused_boxes(source, small_origin);
    let first = if select_solid {
        first_solid(source, &fused)
    } else {
        fused
    };
    let area = tuc_area(source, &first, &prism(source, fixture, direction));
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

#[test]
fn BOPTucComplex__G2() {
    assert_tuc_area(
        "tests/boolean/boptuc_complex/G2",
        Point3::new(3., 3., 6.),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/A1_rr_7.brep"),
        Vector3::new(0., 0., 4.),
        false,
        96.,
    );
}

#[test]
fn BOPTucComplex__G3() {
    assert_tuc_area(
        "tests/boolean/boptuc_complex/G3",
        Point3::new(3., 3., 6.),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/A1_rr_7.brep"),
        Vector3::new(0., 0., -4.),
        true,
        0.,
    );
}

#[test]
fn BOPTucComplex__G4() {
    assert_tuc_area(
        "tests/boolean/boptuc_complex/G4",
        Point3::new(0., 0., 6.),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/C1_rr_8.brep"),
        Vector3::new(0., 0., -4.),
        true,
        0.,
    );
}

#[test]
fn BOPTucComplex__G5() {
    assert_tuc_area(
        "tests/boolean/boptuc_complex/G5",
        Point3::new(0., 0., 6.),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/C1_rr_8.brep"),
        Vector3::new(0., 0., 4.),
        true,
        96.,
    );
}

#[test]
fn BOPTucComplex__G6() {
    assert_tuc_area(
        "tests/boolean/boptuc_complex/G6",
        Point3::new(0., 0., 2.),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/E1_rr_7.brep"),
        Vector3::new(0., -4., 0.),
        true,
        96.,
    );
}

#[test]
fn BOPTucComplex__G7() {
    assert_tuc_area(
        "tests/boolean/boptuc_complex/G7",
        Point3::new(0., 0., 2.),
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/E1_rr_7.brep"),
        Vector3::new(0., 4., 0.),
        true,
        0.,
    );
}
