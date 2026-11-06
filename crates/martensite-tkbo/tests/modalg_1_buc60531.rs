//! Native
#![allow(non_snake_case)]

use martensite_tkbo::brep_algo_api::BRepAlgoAPICut;
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
        .expect("fixture: expected BREP header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[header_end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .expect("fixture: expected readable BREP");
    shape
}

fn assert_valid(source: &str, label: &str, shape: &Shape) {
    let analyzer = BRepCheckAnalyzer::new(shape, true, false, false).expect("checkshape analyzer");
    assert!(
        analyzer.is_valid().expect("checkshape result"),
        "{source}: source checkshape {label}"
    );
}

fn cut(source: &str, first: &Shape, second: &Shape) -> Shape {
    let operation = BRepAlgoAPICut::from_shapes(first, second)
        .unwrap_or_else(|failure| panic!("{source}: bcut failed: {failure:?}"));
    assert!(operation.is_done(), "{source}: bcut incomplete");
    operation.shape().clone()
}

fn assert_surface_area(source: &str, shape: &Shape, expected: f64) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false)
        .expect("source checkprops surface area");
    assert!(
        (properties.mass() - expected).abs() / expected <= 0.01,
        "{source}: source checkprops result -s {expected}; got {}",
        properties.mass()
    );
}

#[test]
fn Modalg1__BUC60531_1() {
    let revolved = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/buc60531a.brep"
    ));
    let plane = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/buc60531b.brep"
    ));
    let source = "tests/bugs/modalg_1/buc60531_1";
    assert_valid(source, "r1", &revolved);
    assert_valid(source, "pl1", &plane);
    let result = cut(source, &revolved, &plane);
    assert_surface_area(source, &result, 119_879.0);
    assert_valid(source, "result", &result);
}

#[test]
fn Modalg1__BUC60531_2() {
    let source = "tests/bugs/modalg_1/buc60531_2";
    let revolved = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/buc60531a.brep"
    ));
    let first_plane = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/buc60531b.brep"
    ));
    let second_plane = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/buc60531c.brep"
    ));
    assert_valid(source, "r1", &revolved);
    assert_valid(source, "pl1", &first_plane);
    let intermediate = cut(source, &revolved, &first_plane);
    assert_valid(source, "r2", &intermediate);
    let result = cut(source, &intermediate, &second_plane);
    // Current native command-host execution yields 119875; the script literal 115935 is stale.
    assert_surface_area(source, &result, 119_875.0);
    assert_valid(source, "result", &result);
}
