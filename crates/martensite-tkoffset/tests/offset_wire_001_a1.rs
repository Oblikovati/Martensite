//! Reference replay of `tests/offset/wire_unclosed_outside_0_025/A1`.

use martensite_tkbrep::{
    brep::{BRepBuilder, BRepTool},
    brep_tools,
    topexp::{self, Explorer},
    topods::{self, Shape, Wire},
    toptools::{IndexedMapOfShape, MapOfShape},
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::geom_abs::JoinType;
use martensite_tkoffset::brep_offset_api::BRepOffsetAPIMakeOffset;
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeFace, BRepBuilderAPIMakeShape},
    brep_check::BRepCheckAnalyzer,
    brep_gprop::BRepGProp,
};

const SOURCE: &str = "tests/offset/wire_unclosed_outside_0_025/A1";
const OFFSET_FRACTION: f64 = 0.025;
const EXPECTED_LENGTH: f64 = 9.26702;

fn restored_shape(bytes: &[u8]) -> Shape {
    let header_end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .expect("A1 fixture: expected BRep header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[header_end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .expect("A1 fixture: expected readable wire");
    shape
}

fn count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut shapes = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut shapes).expect("A1 source shape traversal");
    shapes.extent()
}

fn length(shape: &Shape) -> f64 {
    let mut properties = GProps::default();
    BRepGProp::linear_properties(shape, &mut properties, false, false)
        .expect("A1 source linear properties");
    properties.mass()
}

fn max_tolerance(shape: &Shape) -> f64 {
    [ShapeEnum::Vertex, ShapeEnum::Edge]
        .into_iter()
        .map(|kind| BRepTool::max_tolerance(shape, kind).expect("A1 maximum tolerance"))
        .fold(0.0, f64::max)
}

fn alone_vertices(shape: &Shape) -> usize {
    let mut alone = MapOfShape::default();
    let mut vertices = Explorer::new(shape, ShapeEnum::Vertex, ShapeEnum::Shape)
        .expect("A1 checksection vertices");
    while vertices.more() {
        let vertex = vertices.current().clone();
        if !alone.add(vertex.clone()) {
            alone.remove(&vertex);
        }
        vertices.next().expect("A1 checksection vertex advance");
    }
    alone.extent()
}

fn assert_end_script_checks(result: &Shape, source_tolerance: f64) {
    let wires = Explorer::new(result, ShapeEnum::Wire, ShapeEnum::Shape).expect("A1 result wires");
    for wire_shape in wires {
        let wire = topods::wire(&wire_shape).expect("A1 result wire");
        assert!(
            max_tolerance(wire.as_ref()) <= source_tolerance,
            "{SOURCE}: result wire tolerance exceeds source tolerance"
        );
        assert_eq!(
            alone_vertices(wire.as_ref()),
            0,
            "{SOURCE}: checksection result wire"
        );
        let face = BRepBuilderAPIMakeFace::from_wire(&wire, false)
            .expect("A1 mkplane result wire")
            .face()
            .expect("A1 mkplane face");
        assert!(
            BRepCheckAnalyzer::new(face.as_ref(), true, false, false)
                .expect("A1 mkplane checkshape analyzer")
                .is_valid()
                .expect("A1 mkplane checkshape"),
            "{SOURCE}: mkplane result wire self-intersection"
        );
    }
}

/// The unclosed wire's positive offset preserves the source regression's measurements.
#[test]
#[allow(non_snake_case)]
fn OffsetWireUnclosedOutside__A1() {
    let source = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/offset_wire_001.brep"
    ));
    let source_tolerance = max_tolerance(&source);
    let wire = Wire::unchecked(source.clone());
    let mut offset = BRepOffsetAPIMakeOffset::from_wire(&wire, JoinType::Arc, false);
    offset
        .perform(OFFSET_FRACTION * length(&source), 0.0)
        .expect("A1 mkoffset");
    assert!(offset.is_done(), "{SOURCE}: mkoffset completion");
    let result = offset.shape().expect("A1 offset result");
    assert!(
        BRepCheckAnalyzer::new(result, true, false, false)
            .expect("A1 checkshape analyzer")
            .is_valid()
            .expect("A1 checkshape"),
        "{SOURCE}: checkshape result"
    );
    assert_eq!(count(result, ShapeEnum::Vertex), 24, "{SOURCE}: vertices");
    assert_eq!(count(result, ShapeEnum::Edge), 24, "{SOURCE}: edges");
    assert_eq!(count(result, ShapeEnum::Wire), 1, "{SOURCE}: wires");
    assert_end_script_checks(result, source_tolerance);
    assert!(
        (length(result) - EXPECTED_LENGTH).abs() <= 1.0e-5,
        "{SOURCE}: length {}; expected {EXPECTED_LENGTH}",
        length(result)
    );
}
