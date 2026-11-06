//! Native
#![allow(non_snake_case)]

use martensite_tkbo::brep_algo_api::BRepAlgoAPISection;
use martensite_tkbrep::{brep::BRepBuilder, brep_tools, topods::Shape};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

const SOURCE: &str = "tests/bugs/modalg_1/buc60669";

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

fn assert_valid(label: &str, shape: &Shape) {
    let valid = BRepCheckAnalyzer::new(shape, true, false, false)
        .expect("checkshape analyzer")
        .is_valid()
        .expect("checkshape result");
    assert!(valid, "{SOURCE}: {label} must be valid");
}

#[test]
fn Modalg1__BUC60669() {
    let first = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/buc60669a.brep"
    ));
    let second = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/buc60669b.brep"
    ));
    assert_valid("first input", &first);
    assert_valid("second input", &second);
    let section = BRepAlgoAPISection::from_shapes(&first, &second, None, true)
        .unwrap_or_else(|failure| panic!("{SOURCE}: bsection failed: {failure:?}"));
    assert!(section.is_done(), "{SOURCE}: bsection incomplete");
    let result = section.shape();
    assert_valid("result", result);
    let mut properties = GProps::default();
    BRepGProp::linear_properties(result, &mut properties, false, false)
        .expect("section length calculation");
    let expected = 104.754;
    assert!(
        (properties.mass() - expected).abs() / expected <= 0.01,
        "{SOURCE}: length {}; expected {expected}",
        properties.mass()
    );
}
