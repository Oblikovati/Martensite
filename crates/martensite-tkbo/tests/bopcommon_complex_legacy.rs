//! Native Rust translations of the complete-input `bopcommon_complex` command-host regressions.
//! Each case embeds its original BRep fixtures and checks the documented surface area.
#![allow(non_snake_case)]

use martensite_tkbo::brep_algo_api::BRepAlgoAPICommon;
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools,
    topods::{Shape, ShapeIterator},
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::{brep_builder_api::BRepBuilderAPIMakeShape, brep_gprop::BRepGProp};

pub(super) fn restored_shape(bytes: &[u8]) -> Shape {
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

fn common_case(source: &str, first: &[u8], second: &[u8], expected_area: f64) {
    let first = restored_shape(first);
    let second = restored_shape(second);
    let result = BRepAlgoAPICommon::from_shapes(&first, &second)
        .unwrap_or_else(|failure| panic!("{source}: bcommon failed: {failure:?}"));
    assert!(result.is_done(), "{source}: bcommon did not complete");
    let mut properties = GProps::default();
    BRepGProp::surface_properties(result.shape(), &mut properties, false, false)
        .unwrap_or_else(|failure| panic!("{source}: surface area failed: {failure:?}"));
    let area = properties.mass();
    if expected_area == 0.0 {
        assert!(
            area.abs() <= 1.0e-6,
            "{source}: expected empty area, got {area}"
        );
        return;
    }
    assert!(
        (area - expected_area).abs() / expected_area <= 0.01,
        "{source}: area {area}; expected {expected_area} within 1%"
    );
}

macro_rules! case {
    ($name:ident, $source:literal, $first:expr, $second:expr, $area:expr $(,)?) => {
        #[test]
        fn $name() {
            common_case($source, $first, $second, $area);
        }
    };
}

#[path = "bopcommon_complex_cases_0.rs"]
mod cases_0;
#[path = "bopcommon_complex_cases_1.rs"]
mod cases_1;
#[path = "bopcommon_prism_box_cases.rs"]
mod prism_box_cases;

fn compound_common_area(source: &str, bytes: &[u8], first: usize, second: usize) -> f64 {
    let assembly = restored_shape(bytes);
    let members: Vec<_> = ShapeIterator::new(&assembly, false, true).collect();
    assert_eq!(members.len(), 2, "{source}: expected two operands");
    let result = BRepAlgoAPICommon::from_shapes(&members[first], &members[second])
        .unwrap_or_else(|failure| panic!("{source}: bcommon failed: {failure:?}"));
    assert!(result.is_done(), "{source}: bcommon did not complete");
    let mut properties = GProps::default();
    BRepGProp::surface_properties(result.shape(), &mut properties, false, false)
        .unwrap_or_else(|failure| panic!("{source}: surface area failed: {failure:?}"));
    properties.mass()
}

fn assert_compound_common_area(
    source: &str,
    bytes: &[u8],
    first: usize,
    second: usize,
    expected_area: f64,
) {
    let area = compound_common_area(source, bytes, first, second);
    assert!(
        (area - expected_area).abs() / expected_area <= 0.01,
        "{source}: area {area}; expected {expected_area} within 1%"
    );
}

#[test]
fn BOPCommonComplex__E3() {
    assert_compound_common_area(
        "tests/boolean/bopcommon_complex/E3",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/BUC60928.brep"),
        0,
        1,
        30_514.4,
    );
}

#[test]
fn BOPCommonComplex__E4() {
    assert_compound_common_area(
        "tests/boolean/bopcommon_complex/E4",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/BUC60928.brep"),
        1,
        0,
        30_514.4,
    );
}

#[test]
fn BOPCommonComplex__F2() {
    assert_compound_common_area(
        "tests/boolean/bopcommon_complex/F2",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/buc60996.brep"),
        0,
        1,
        33_561.9,
    );
}

#[test]
fn BOPCommonComplex__F3() {
    assert_compound_common_area(
        "tests/boolean/bopcommon_complex/F3",
        include_bytes!("../../../external/martensite-test-data/dataset-7.9.0/brep/buc60996.brep"),
        1,
        0,
        33_561.9,
    );
}
