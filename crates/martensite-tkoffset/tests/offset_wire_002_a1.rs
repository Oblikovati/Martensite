//! Reference replay of `tests/offset/wire_closed_inside_0_005/A1`.

use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools, topexp,
    topods::{Shape, Wire},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkoffset::brep_offset_api::BRepOffsetAPIMakeOffset;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

const SOURCE: &str = "tests/offset/wire_closed_inside_0_005/A1";
const EXPECTED_LENGTH: f64 = 232.96;
const OFFSET_FRACTION: f64 = -0.005;

fn restored_shape(bytes: &[u8]) -> Shape {
    let header_end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .expect("A1 fixture header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[header_end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .expect("A1 fixture must restore");
    shape
}

fn count(shape: &Shape, kind: ShapeEnum) -> usize {
    let mut shapes = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut shapes).expect("A1 source explorer");
    shapes.extent()
}

fn source_length(shape: &Shape) -> f64 {
    let mut properties = GProps::default();
    BRepGProp::linear_properties(shape, &mut properties, false, false).expect("A1 source lprops");
    properties.mass()
}

#[test]
#[allow(non_snake_case)]
fn OffsetWireClosedInside__A1() {
    let source = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/offset_wire_002.brep"
    ));
    let wire = Wire::unchecked(source.clone());
    let mut offset = BRepOffsetAPIMakeOffset::from_wire(
        &wire,
        martensite_tkmath::geom_abs::JoinType::Arc,
        false,
    );
    offset
        .perform(OFFSET_FRACTION * source_length(&source), 0.0)
        .expect("A1 mkoffset");
    assert!(offset.is_done(), "{SOURCE}: mkoffset completion");
    let result = offset.shape().expect("A1 result");
    assert!(
        BRepCheckAnalyzer::new(result, true, false, false)
            .expect("A1 checkshape analyzer")
            .is_valid()
            .expect("A1 checkshape")
    );
    assert_eq!(
        count(result, ShapeEnum::Vertex),
        5,
        "{SOURCE}: vertex count"
    );
    assert_eq!(count(result, ShapeEnum::Edge), 5, "{SOURCE}: edge count");
    assert_eq!(count(result, ShapeEnum::Wire), 1, "{SOURCE}: wire count");
    assert!(
        (source_length(result) - EXPECTED_LENGTH).abs() <= 1.0e-2,
        "{SOURCE}: checkprops result -l {EXPECTED_LENGTH}"
    );
}
