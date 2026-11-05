//! Native
#![allow(non_snake_case)]

use martensite_tkbo::bop_algo::{BOPAlgoPaveFiller, BOPAlgoSection};
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

use super::restored_shape;

fn first_shell(source: &str, shape: &Shape) -> Shape {
    let mut explorer = Explorer::new(shape, ShapeEnum::Shell, ShapeEnum::Shape)
        .unwrap_or_else(|failure| panic!("{source}: shell exploration failed: {failure:?}"));
    Iterator::next(&mut explorer).unwrap_or_else(|| panic!("{source}: expected first shell"))
}

fn section_length(source: &str, first: Shape, second: Shape) -> f64 {
    let arguments: List<Shape> = [first, second].into_iter().collect();
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
    properties.mass()
}

/// Replays `tests/boolean/bopsection/D7` without command-host at runtime.
#[test]
fn BOPSection__D7() {
    let source = "tests/boolean/bopsection/D7";
    let first = first_shell(
        source,
        &restored_shape(include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO909_object.brep"
        )),
    );
    let second = first_shell(
        source,
        &restored_shape(include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/CTO909_tool_4.brep"
        )),
    );
    let length = section_length(source, first, second);
    assert!(
        (length - 74.2717).abs() / 74.2717 <= 0.01,
        "{source}: length {length}"
    );
}
