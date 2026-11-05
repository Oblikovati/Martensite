//! Native Rust translations of the BUC60623 bidirectional section regressions.
#![allow(non_snake_case)]

use martensite_tkbo::brep_algo_api::BRepAlgoAPISection;
use martensite_tkbrep::{brep::BRepBuilder, brep_tools, topods::Shape};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

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

fn assert_valid(source: &str, label: &str, shape: &Shape) {
    let valid = BRepCheckAnalyzer::new(shape, true, false, false)
        .expect("checkshape analyzer")
        .is_valid()
        .expect("checkshape result");
    assert!(valid, "{source}: {label} must be valid");
}

fn assert_section(source: &str, first: &Shape, second: &Shape) {
    assert_valid(source, "first input", first);
    assert_valid(source, "second input", second);
    let section = BRepAlgoAPISection::from_shapes(first, second, None, true)
        .unwrap_or_else(|failure| panic!("{source}: bsection failed: {failure:?}"));
    assert!(section.is_done(), "{source}: bsection incomplete");
    let result = section.shape();
    assert_valid(source, "result", result);
    let mut properties = GProps::default();
    BRepGProp::linear_properties(result, &mut properties, false, false)
        .expect("section length calculation");
    assert!(
        (properties.mass() - 1475.98).abs() / 1475.98 <= 0.01,
        "{source}: length {}; expected 1475.98",
        properties.mass()
    );
}

fn inputs() -> [Shape; 2] {
    [
        restored_shape(include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/buc60623a.brep"
        )),
        restored_shape(include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/buc60623b.brep"
        )),
    ]
}

#[test]
fn Modalg1__BUC60623_1() {
    let [first, second] = inputs();
    assert_section("tests/bugs/modalg_1/buc60623_1", &first, &second);
}

#[test]
fn Modalg1__BUC60623_2() {
    let [first, second] = inputs();
    assert_section("tests/bugs/modalg_1/buc60623_2", &second, &first);
}
