//! Native Rust translations of the pro15203 face-selection section regressions.
#![allow(non_snake_case)]

use martensite_tkbo::bop_algo::{BOPAlgoPaveFiller, BOPAlgoSection};
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkernel::ncollection::List;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::BRepGProp;

use super::restored_shape;

fn first_face(source: &str, shape: &Shape) -> Shape {
    let mut explorer = Explorer::new(shape, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap_or_else(|failure| panic!("{source}: face exploration failed: {failure:?}"));
    Iterator::next(&mut explorer).unwrap_or_else(|| panic!("{source}: expected first face"))
}

fn pro15203_section(source: &str) {
    let (first, second) = pro15203_faces(source);
    let length = section_length(source, first, second);
    assert!(
        (length - 24.3614).abs() / 24.3614 <= 0.01,
        "{source}: length {length}"
    );
}

fn pro15203_faces(source: &str) -> (Shape, Shape) {
    let first = first_face(
        source,
        &restored_shape(include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/pro15203a.rle"
        )),
    );
    let second = first_face(
        source,
        &restored_shape(include_bytes!(
            "../../../external/martensite-test-data/dataset-7.9.0/brep/pro15203c.rle"
        )),
    );
    (first, second)
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

#[test]
fn BOPSection__C7() {
    pro15203_section("tests/boolean/bopsection/C7");
}

#[test]
fn BOPSection__E1() {
    pro15203_section("tests/boolean/bopsection/E1");
}
