//! Native replay of the OCC21427 attached-solid Boolean cut regression.
#![allow(non_snake_case)]

use martensite_tkbo::bop_algo::{BOPAlgoBOP, Operation};
use martensite_tkbrep::{brep::BRepBuilder, brep_tools, topods::Shape};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::{brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp};

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

fn assert_valid(source: &str, shape: &Shape) {
    let valid = BRepCheckAnalyzer::new(shape, true, false, false)
        .expect("checkshape analyzer")
        .is_valid()
        .expect("checkshape result");
    assert!(valid, "{source}: checkshape result");
}

#[test]
fn Modalg6__Bug21427() {
    let source = "tests/bugs/modalg_6/bug21427";
    let shape = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/bug21427a.brep"
    ));
    let tool = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/bug21427b.brep"
    ));
    let mut cut = BOPAlgoBOP::new();
    cut.add_argument(&shape);
    cut.add_tool(&tool);
    cut.set_operation(Operation::Cut);
    cut.options_mut().set_run_parallel(false);
    cut.perform();
    assert!(!cut.options().has_errors(), "{source}: bcut failed");
    let result = cut.shape();
    assert_valid(source, result);
    let mut properties = GProps::default();
    BRepGProp::surface_properties(result, &mut properties, false, false)
        .expect("surface properties");
    let expected = 1.49188e7;
    assert!(
        (properties.mass() - expected).abs() / expected <= 0.01,
        "{source}: checkprops -s {}; expected {expected} within 1%",
        properties.mass()
    );
}
