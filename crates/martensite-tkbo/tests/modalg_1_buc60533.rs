//! Supplemental fixed-order component receipts for the BUC60533 General Fuse result.
#![allow(non_snake_case)]

use martensite_tkbo::bop_algo::BOPAlgoBuilder;
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools,
    topods::{Shape, ShapeIterator},
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::{brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp};

const SOURCE: &str = "tests/bugs/modalg_1/buc60533";
const CURRENT_NATIVE_AREAS: [f64; 3] = [
    21_802_522.092_508_595,
    26_968_100.358_934_18,
    277_794_907.355_524_66,
];

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

fn surface_area(shape: &Shape) -> f64 {
    let mut properties = GProps::default();
    // These direct sprops receipts are fixed-order; the original checkprops uses adaptive epsilon.
    let estimate = BRepGProp::surface_properties_eps(shape, &mut properties, 1.0, false)
        .expect("fixed component surface area calculation");
    assert_eq!(
        estimate, 0.0,
        "fixed component receipt has no error estimate"
    );
    properties.mass()
}

fn assert_native_area(index: usize, shape: &Shape) {
    let expected = CURRENT_NATIVE_AREAS[index];
    let actual = surface_area(shape);
    // `checkprops -s` uses a one percent relative tolerance. Keep that source tolerance.
    assert!(
        (actual - expected).abs() / expected <= 0.01,
        "{SOURCE}: component {index} area {actual}; expected current native {expected}",
    );
}

#[test]
fn Modalg1__BUC60533() {
    let part = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/buc60533a.brep"
    ));
    let tool = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/buc60533b.brep"
    ));
    assert_valid("part input", &part);
    assert_valid("tool input", &tool);

    let mut builder = BOPAlgoBuilder::new();
    builder.add_argument(&part);
    builder.add_argument(&tool);
    builder.perform();
    assert!(!builder.options().has_errors(), "{SOURCE}: bbuild failed");
    assert_valid("result", builder.shape());

    let components: Vec<Shape> = ShapeIterator::new(builder.shape(), true, true).collect();
    assert_eq!(components.len(), 3, "{SOURCE}: bbuild component count");
    for (index, component) in components.iter().enumerate() {
        assert_native_area(index, component);
    }
}
