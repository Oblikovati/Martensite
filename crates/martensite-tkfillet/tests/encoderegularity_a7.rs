//! Native
#![allow(non_snake_case)]

use martensite_tkbrep::{
    brep::BRepBuilder,
    brep_tools, topexp,
    topods::{self, Edge, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::{message::ProgressRange, storage::IoStream};
use martensite_tkfillet::{brep_fillet_api::BRepFilletAPIMakeFillet, chfi3d::FilletShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_gprop::BRepGProp, brep_lib::encode_regularity,
};

const SOURCE: &str = "tests/blend/encoderegularity/A7";
const ANGLE_DEGREES: f64 = 3.;
const RADIUS: f64 = 7.;
const EXPECTED_AREA: f64 = 17_179.7;

fn restored_shape(bytes: &[u8]) -> Shape {
    let header_end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .expect("BREP fixture header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[header_end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .expect("source restore CFI901_congefke.rle");
    shape
}

fn source_edge(shape: &Shape, index: usize) -> Edge {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).expect("source explode m E");
    topods::edge(edges.find_key(index).expect("source m edge index")).expect("source m edge type")
}

fn source_requests() -> [usize; 17] {
    [
        1, 5, 7, 8, 9, 10, 11, 12, 14, 16, 17, 18, 19, 20, 22, 23, 24,
    ]
}

fn source_area(shape: &Shape) -> f64 {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false)
        .expect("source checkprops result -s 17179.7");
    properties.mass()
}

#[test]
fn EncodeRegularity__A7() {
    let source = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CFI901_congefke.rle"
    ));
    encode_regularity(&source, ANGLE_DEGREES.to_radians()).expect("source encoderegularity m 3");
    let mut blend = BRepFilletAPIMakeFillet::new(&source, FilletShape::Rational)
        .expect("source blend result m setup");
    for index in source_requests() {
        blend
            .add_with_radius(RADIUS, &source_edge(&source, index))
            .expect("source blend result m 7 m_edge");
    }
    blend
        .build(&ProgressRange::default())
        .expect("source blend result m build");
    assert!(
        blend.is_done(),
        "{SOURCE}: source blend result must complete"
    );
    let actual = source_area(blend.shape().expect("source blend result"));
    assert!(
        (actual - EXPECTED_AREA).abs() / EXPECTED_AREA <= 0.01,
        "{SOURCE}: checkprops result -s {EXPECTED_AREA}; got {actual}"
    );
}
