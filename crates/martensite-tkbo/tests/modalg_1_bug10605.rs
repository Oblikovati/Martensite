//! Native
#![allow(non_snake_case)]

use martensite_tkbo::{
    bop_algo::{BOPAlgoBOP, Operation},
    brep_algo_api::BRepAlgoAPISection,
};
use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools::{self, BRepToolsShapeSet},
    topods::Shape,
    toptools::ShapeSet,
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

fn restored_shape(bytes: &[u8]) -> Shape {
    let header_end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .expect("BRep fixture header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[header_end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .expect("BRep fixture read");
    shape
}

fn assert_valid(source: &str, label: &str, shape: &Shape) {
    let analyzer = BRepCheckAnalyzer::new(shape, true, false, false).expect("checkshape analyzer");
    assert!(
        analyzer.is_valid().expect("checkshape result"),
        "{source}: {label} must be valid"
    );
}

fn boolean(source: &str, operation: Operation) -> Shape {
    let first = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/OCC10605-1.brep"
    ));
    let second = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/OCC10605-3.brep"
    ));
    assert_valid(source, "first input", &first);
    assert_valid(source, "second input", &second);
    let mut runner = BOPAlgoBOP::new();
    runner.add_argument(&first);
    runner.add_tool(&second);
    runner.set_operation(operation);
    runner.options_mut().set_run_parallel(false);
    runner.perform();
    assert!(
        !runner.options().has_errors(),
        "{source}: Boolean operation failed"
    );
    runner.shape().clone()
}

fn section(source: &str) -> Shape {
    let first = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/OCC10605-1.brep"
    ));
    let second = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/OCC10605-3.brep"
    ));
    assert_valid(source, "first input", &first);
    assert_valid(source, "second input", &second);
    let result = BRepAlgoAPISection::from_shapes(&first, &second, None, true)
        .unwrap_or_else(|failure| panic!("{source}: bopsection failed: {failure:?}"));
    assert!(result.is_done(), "{source}: bopsection incomplete");
    result.shape().clone()
}

fn topology_count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut entries = BRepToolsShapeSet::default();
    entries.add(shape).expect("topology entry set");
    (1..=entries.nb_shapes())
        .filter(|index| {
            entries
                .shape(*index)
                .expect("topology entry")
                .shape_type()
                .expect("topology type")
                == kind
        })
        .count()
}

fn assert_topology(source: &str, result: &Shape, expected: &[(ShapeEnum, usize)], total: usize) {
    for (kind, count) in expected {
        assert_eq!(
            topology_count(result, *kind),
            *count,
            "{source}: {kind:?} count"
        );
    }
    let mut entries = BRepToolsShapeSet::default();
    entries.add(result).expect("topology entry set");
    assert_eq!(entries.nb_shapes(), total as i32, "{source}: shape count");
}

fn assert_area(source: &str, result: &Shape, expected: f64) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(result, &mut properties, false, false).expect("surface area");
    assert!(
        (properties.mass() - expected).abs() / expected <= 0.01,
        "{source}: area {}",
        properties.mass()
    );
}

#[test]
fn Modalg1__OCC10605_1() {
    let source = "tests/bugs/modalg_1/bug10605_1";
    let result = boolean(source, Operation::Common);
    assert_valid(source, "result", &result);
    assert_topology(
        source,
        &result,
        &[
            (ShapeEnum::Vertex, 10),
            (ShapeEnum::Edge, 15),
            (ShapeEnum::Wire, 7),
            (ShapeEnum::Face, 7),
            (ShapeEnum::Shell, 1),
            (ShapeEnum::Solid, 1),
            (ShapeEnum::CompSolid, 0),
            (ShapeEnum::Compound, 1),
        ],
        42,
    );
    assert_area(source, &result, 233.798);
}

#[test]
fn Modalg1__OCC10605_2() {
    let source = "tests/bugs/modalg_1/bug10605_2";
    let result = boolean(source, Operation::Fuse);
    assert_valid(source, "result", &result);
    assert_topology(
        source,
        &result,
        &[
            (ShapeEnum::Vertex, 15),
            (ShapeEnum::Edge, 22),
            (ShapeEnum::Wire, 10),
            (ShapeEnum::Face, 9),
            (ShapeEnum::Shell, 1),
            (ShapeEnum::Solid, 1),
            (ShapeEnum::CompSolid, 0),
            (ShapeEnum::Compound, 1),
        ],
        59,
    );
    assert_area(source, &result, 838.752);
}

#[test]
fn Modalg1__OCC10605_3() {
    let source = "tests/bugs/modalg_1/bug10605_3";
    let result = boolean(source, Operation::Cut);
    assert_valid(source, "result", &result);
    assert_topology(
        source,
        &result,
        &[
            (ShapeEnum::Vertex, 13),
            (ShapeEnum::Edge, 19),
            (ShapeEnum::Wire, 8),
            (ShapeEnum::Face, 7),
            (ShapeEnum::Shell, 1),
            (ShapeEnum::Solid, 1),
            (ShapeEnum::CompSolid, 0),
            (ShapeEnum::Compound, 1),
        ],
        50,
    );
    assert_area(source, &result, 779.887);
}

#[test]
fn Modalg1__OCC10605_4() {
    let source = "tests/bugs/modalg_1/bug10605_4";
    let result = boolean(source, Operation::Cut21);
    assert_valid(source, "result", &result);
    assert_topology(
        source,
        &result,
        &[
            (ShapeEnum::Vertex, 12),
            (ShapeEnum::Edge, 18),
            (ShapeEnum::Wire, 9),
            (ShapeEnum::Face, 9),
            (ShapeEnum::Shell, 2),
            (ShapeEnum::Solid, 2),
            (ShapeEnum::CompSolid, 0),
            (ShapeEnum::Compound, 1),
        ],
        53,
    );
    assert_area(source, &result, 292.663);
}

#[test]
fn Modalg1__OCC10605_5() {
    let source = "tests/bugs/modalg_1/bug10605_5";
    let result = section(source);
    assert_valid(source, "result", &result);
    assert_topology(
        source,
        &result,
        &[
            (ShapeEnum::Vertex, 9),
            (ShapeEnum::Edge, 9),
            (ShapeEnum::Wire, 0),
            (ShapeEnum::Face, 0),
            (ShapeEnum::Shell, 0),
            (ShapeEnum::Solid, 0),
            (ShapeEnum::CompSolid, 0),
            (ShapeEnum::Compound, 1),
        ],
        19,
    );
    let mut properties = GProps::default();
    BRepGProp::linear_properties(&result, &mut properties, false, false).expect("section length");
    assert!(
        (properties.mass() - 59.8334).abs() / 59.8334 <= 0.01,
        "{source}: length {}",
        properties.mass()
    );
}
