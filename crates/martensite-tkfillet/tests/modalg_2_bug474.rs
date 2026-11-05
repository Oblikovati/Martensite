//! Native replay of the OCC474 high-radius fillet regression.
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
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

const SOURCE: &str = "tests/bugs/modalg_2/bug474";

fn restored_source(bytes: &[u8]) -> Shape {
    let header_end = bytes
        .iter()
        .position(u8::is_ascii_whitespace)
        .expect("source OCC474 header");
    let mut shape = Shape::default();
    let mut stream = IoStream::from_bytes(bytes[header_end..].to_vec());
    brep_tools::read(
        &mut shape,
        &mut stream,
        &BRepBuilder::default(),
        &ProgressRange::default(),
    )
    .expect("source restore OCC474.brep");
    shape
}

fn sixteenth_edge(shape: &Shape) -> Edge {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).expect("source explode s e");
    topods::edge(edges.find_key(16).expect("source s_16")).expect("source s_16 edge")
}

fn source_blend(source: &Shape, edge: &Edge) -> Shape {
    let mut blend = BRepFilletAPIMakeFillet::new(source, FilletShape::Rational)
        .expect("source blend result setup");
    blend
        .add_with_radius(100., edge)
        .expect("source blend result 100 s_16");
    blend
        .build(&ProgressRange::default())
        .expect("source blend result build");
    assert!(blend.is_done(), "{SOURCE}: blend incomplete");
    blend.shape().expect("source blend result").clone()
}

fn assert_source_area(shape: &Shape) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false)
        .expect("source checkprops result -s 1.54883e+06");
    let expected = 1_548_830.;
    assert!(
        (properties.mass() - expected).abs() / expected <= 0.01,
        "{SOURCE}: expected surface {expected}, got {}",
        properties.mass()
    );
}

#[test]
fn Modalg2__Bug474() {
    let source = restored_source(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/OCC474.brep"
    ));
    let result = source_blend(&source, &sixteenth_edge(&source));
    let checker =
        BRepCheckAnalyzer::new(&result, true, false, false).expect("source checkshape result");
    assert!(
        checker.is_valid().expect("source checkshape validity"),
        "{SOURCE}: checkshape result"
    );
    assert_source_area(&result);
}
