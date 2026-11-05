//! Native
#![allow(non_snake_case)]

use martensite_tkbo::bop_algo::{BOPAlgoCellsBuilder, BOPAlgoPaveFiller};
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools::{self, BRepToolsShapeSet},
    topods::{Shape, ShapeIterator},
    toptools::ShapeSet,
};
use martensite_tkernel::{message::ProgressRange, ncollection::List, storage::IoStream};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

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

fn fixture_members(source: &str) -> [Shape; 2] {
    let fixture = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/BUC60928.brep"
    ));
    let members: Vec<_> = ShapeIterator::new(&fixture, false, true).collect();
    assert_eq!(members.len(), 2, "{source}: expected two fixture members");
    [members[0].clone(), members[1].clone()]
}

fn cell_cut(source: &str, members: &[Shape; 2]) -> BOPAlgoCellsBuilder {
    let arguments: List<Shape> = members.iter().cloned().collect();
    let mut cells = BOPAlgoCellsBuilder::new();
    cells.set_arguments(&arguments);
    cells.options_mut().set_run_parallel(false);
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&arguments);
    filler.options_mut().set_run_parallel(false);
    filler.perform();
    assert!(!filler.options().has_errors(), "{source}: bfillds failed");
    cells.perform_with_filler(&mut filler);
    assert!(!cells.options().has_errors(), "{source}: bfillds failed");
    let take: List<Shape> = [members[0].clone()].into_iter().collect();
    let avoid: List<Shape> = [members[1].clone()].into_iter().collect();
    cells.add_to_result(&take, &avoid, 1, false).unwrap();
    cells
}

fn shape_count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut entries = BRepToolsShapeSet::default();
    entries.add(shape).expect("topology entry set");
    (1..=entries.nb_shapes())
        .filter(|index| entries.shape(*index).expect("topology entry").shape_type() == Ok(kind))
        .count()
}

fn assert_measurements(source: &str, shape: &Shape) {
    let mut surface = GProps::default();
    let mut volume = GProps::default();
    BRepGProp::surface_properties(shape, &mut surface, false, false).unwrap();
    BRepGProp::volume_properties(shape, &mut volume, false, false, false).unwrap();
    assert!(
        (surface.mass() - 190_481.).abs() / 190_481. <= 0.01,
        "{source}: area"
    );
    assert!(
        (volume.mass() - 1.23285e6).abs() / 1.23285e6 <= 0.01,
        "{source}: volume"
    );
}

/// Replays `tests/boolean/bopcut_complex/E5` without command-host at runtime.
#[test]
fn BOPCutComplex__E5() {
    let source = "tests/boolean/bopcut_complex/E5";
    let cells = cell_cut(source, &fixture_members(source));
    let result = cells.shape();
    assert_measurements(source, result);
    for (kind, expected) in [
        (ShapeEnum::Solid, 3),
        (ShapeEnum::Shell, 3),
        (ShapeEnum::Face, 14),
    ] {
        assert_eq!(
            shape_count(result, kind),
            expected,
            "{source}: {kind:?} count"
        );
    }
}
