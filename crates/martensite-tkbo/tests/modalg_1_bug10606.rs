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
        "../../../external/martensite-test-data/dataset-7.9.0/brep/OCC10604-1.brep"
    ));
    let second = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/OCC10606-2.brep"
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
        "../../../external/martensite-test-data/dataset-7.9.0/brep/OCC10604-1.brep"
    ));
    let second = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/OCC10606-2.brep"
    ));
    assert_valid(source, "first input", &first);
    assert_valid(source, "second input", &second);
    let result = BRepAlgoAPISection::from_shapes(&first, &second, None, true).expect("bopsection");
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

#[test]
fn Modalg1__OCC10606_1() {
    let source = "tests/bugs/modalg_1/bug10606_1";
    let result = boolean(source, Operation::Common);
    assert_valid(source, "result", &result);
    for (kind, count) in [
        (ShapeEnum::Vertex, 5),
        (ShapeEnum::Edge, 7),
        (ShapeEnum::Wire, 2),
        (ShapeEnum::Face, 2),
        (ShapeEnum::Shell, 1),
        (ShapeEnum::Solid, 1),
        (ShapeEnum::CompSolid, 0),
        (ShapeEnum::Compound, 1),
    ] {
        assert_eq!(
            topology_count(&result, kind),
            count,
            "{source}: {kind:?} count"
        );
    }
    let mut entries = BRepToolsShapeSet::default();
    entries.add(&result).expect("topology entry set");
    assert_eq!(entries.nb_shapes(), 19, "{source}: shape count");
    let mut properties = GProps::default();
    BRepGProp::surface_properties(&result, &mut properties, false, false).expect("surface area");
    assert!(
        (properties.mass() - 66.672).abs() / 66.672 <= 0.01,
        "{source}: area {}",
        properties.mass()
    );
}

#[test]
fn Modalg1__OCC10606_2() {
    let source = "tests/bugs/modalg_1/bug10606_2";
    let result = boolean(source, Operation::Fuse);
    assert_valid(source, "result", &result);
    for (kind, count) in [
        (ShapeEnum::Vertex, 5),
        (ShapeEnum::Edge, 7),
        (ShapeEnum::Wire, 4),
        (ShapeEnum::Face, 4),
        (ShapeEnum::Shell, 1),
        (ShapeEnum::Solid, 1),
        (ShapeEnum::CompSolid, 0),
        (ShapeEnum::Compound, 1),
    ] {
        assert_eq!(
            topology_count(&result, kind),
            count,
            "{source}: {kind:?} count"
        );
    }
    let mut entries = BRepToolsShapeSet::default();
    entries.add(&result).expect("topology entry set");
    assert_eq!(entries.nb_shapes(), 23, "{source}: shape count");
    let mut properties = GProps::default();
    BRepGProp::surface_properties(&result, &mut properties, false, false).expect("surface area");
    assert!(
        (properties.mass() - 1450.22).abs() / 1450.22 <= 0.01,
        "{source}: area {}",
        properties.mass()
    );
}

#[test]
fn Modalg1__OCC10606_3() {
    let source = "tests/bugs/modalg_1/bug10606_3";
    let result = boolean(source, Operation::Cut);
    assert_valid(source, "result", &result);
    for (kind, count) in [
        (ShapeEnum::Vertex, 5),
        (ShapeEnum::Edge, 7),
        (ShapeEnum::Wire, 3),
        (ShapeEnum::Face, 3),
        (ShapeEnum::Shell, 1),
        (ShapeEnum::Solid, 1),
        (ShapeEnum::CompSolid, 0),
        (ShapeEnum::Compound, 1),
    ] {
        assert_eq!(
            topology_count(&result, kind),
            count,
            "{source}: {kind:?} count"
        );
    }
    let mut entries = BRepToolsShapeSet::default();
    entries.add(&result).expect("topology entry set");
    assert_eq!(entries.nb_shapes(), 21, "{source}: shape count");
    let mut properties = GProps::default();
    BRepGProp::surface_properties(&result, &mut properties, false, false).expect("surface area");
    assert!(
        (properties.mass() - 761.355).abs() / 761.355 <= 0.01,
        "{source}: area {}",
        properties.mass()
    );
}

#[test]
fn Modalg1__OCC10606_4() {
    let source = "tests/bugs/modalg_1/bug10606_4";
    let result = boolean(source, Operation::Cut21);
    assert_valid(source, "result", &result);
    for (kind, count) in [
        (ShapeEnum::Vertex, 5),
        (ShapeEnum::Edge, 7),
        (ShapeEnum::Wire, 3),
        (ShapeEnum::Face, 3),
        (ShapeEnum::Shell, 1),
        (ShapeEnum::Solid, 1),
        (ShapeEnum::CompSolid, 0),
        (ShapeEnum::Compound, 1),
    ] {
        assert_eq!(
            topology_count(&result, kind),
            count,
            "{source}: {kind:?} count"
        );
    }
    let mut entries = BRepToolsShapeSet::default();
    entries.add(&result).expect("topology entry set");
    assert_eq!(entries.nb_shapes(), 21, "{source}: shape count");
    let mut properties = GProps::default();
    BRepGProp::surface_properties(&result, &mut properties, false, false).expect("surface area");
    assert!(
        (properties.mass() - 755.54).abs() / 755.54 <= 0.01,
        "{source}: area {}",
        properties.mass()
    );
}

#[test]
fn Modalg1__OCC10606_5() {
    let source = "tests/bugs/modalg_1/bug10606_5";
    let result = section(source);
    assert_valid(source, "result", &result);
    assert_eq!(
        topology_count(&result, ShapeEnum::Vertex),
        3,
        "{source}: vertex count"
    );
    assert_eq!(
        topology_count(&result, ShapeEnum::Edge),
        3,
        "{source}: edge count"
    );
    let mut entries = BRepToolsShapeSet::default();
    entries.add(&result).expect("topology entry set");
    assert_eq!(entries.nb_shapes(), 7, "{source}: shape count");
    let mut properties = GProps::default();
    BRepGProp::linear_properties(&result, &mut properties, false, false).expect("section length");
    assert!(
        (properties.mass() - 18.3861).abs() / 18.3861 <= 0.01,
        "{source}: length {}",
        properties.mass()
    );
}
