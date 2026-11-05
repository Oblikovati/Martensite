//! Native
#![allow(non_snake_case)]

use martensite_tkbo::bop_algo::{BOPAlgoPaveFiller, BOPAlgoSection};
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools,
    topods::{Shape, Solid},
};
use martensite_tkernel::{message::ProgressRange, ncollection::List, storage::IoStream};
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

fn restored_shape(bytes: &[u8]) -> Shape {
    let end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .expect("BRep fixture header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .expect("BRep fixture read");
    shape
}

fn solid_from_shell(shell: Shape) -> Shape {
    let builder = BRepBuilder::default();
    let mut solid = Solid::default();
    builder.make_solid(&mut solid);
    builder.add(&mut solid, &shell).expect("source solid shell");
    solid.into_shape()
}

/// Replays `tests/boolean/bopsection/C6` without command-host at runtime.
#[test]
fn BOPSection__C6() {
    let source = "tests/boolean/bopsection/C6";
    let first = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_pro10095a.rle"
    ));
    let shell = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO904_pro10095b.rle"
    ));
    let arguments: List<Shape> = [first, solid_from_shell(shell)].into_iter().collect();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&arguments);
    filler.options_mut().set_run_parallel(false);
    filler.perform();
    assert!(
        !filler.options().has_errors(),
        "{source}: bop preparation failed"
    );
    let mut section = BOPAlgoSection::new();
    section.set_arguments(&arguments);
    section.options_mut().set_run_parallel(false);
    section.perform_with_filler(&mut filler);
    assert!(
        !section.options().has_errors(),
        "{source}: bopsection failed"
    );
    let mut properties = GProps::default();
    BRepGProp::linear_properties(section.shape(), &mut properties, false, false)
        .unwrap_or_else(|failure| panic!("{source}: length failed: {failure:?}"));
    let length = properties.mass();
    assert!(
        (length - 1244.52).abs() / 1244.52 <= 0.01,
        "{source}: length {length}"
    );
}
