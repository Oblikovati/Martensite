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
use martensite_tkmath::gp::{Point3, Transform3};
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPITransform},
    brep_check::BRepCheckAnalyzer,
    brep_gprop::BRepGProp,
};

const SOURCE: &str = "tests/blend/tolblend_simple/A4";
const SCALE1: f64 = 5.;

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
    .expect("source restore CFI_5_b12fgg.rle");
    shape
}

fn scaled_shape(shape: &Shape) -> Shape {
    let mut scale = Transform3::default();
    scale
        .set_scale(Point3::default(), SCALE1)
        .expect("source tscale s 0 0 0 SCALE1");
    BRepBuilderAPITransform::with_shape(shape, scale, false, false)
        .expect("source tscale transform")
        .shape()
        .expect("source tscale result")
        .clone()
}

fn indexed_edges(shape: &Shape) -> Vec<Edge> {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).expect("source explode s E");
    (1..=edges.extent())
        .map(|index| {
            topods::edge(edges.find_key(index).expect("source indexed edge")).expect("source edge")
        })
        .collect()
}

fn result_area(shape: &Shape) -> f64 {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false)
        .expect("source checkprops result -s 4816.07");
    properties.mass()
}

#[test]
fn TolblendSimple__A4() {
    let fixture = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CFI_5_b12fgg.rle"
    ));
    let source = scaled_shape(&fixture);
    let edges = indexed_edges(&source);
    let mut blend = BRepFilletAPIMakeFillet::new(&source, FilletShape::Rational)
        .expect("source blend result setup");
    for (index, radius) in [(4, 0.5), (8, 0.4), (7, 0.6), (3, 0.3)] {
        blend
            .add_with_radius(radius * SCALE1, &edges[index - 1])
            .expect("source blend result edge");
    }
    blend
        .build(&ProgressRange::default())
        .expect("source blend result build");
    assert!(blend.is_done(), "{SOURCE}: blend result incomplete");
    let result = blend.shape().expect("source blend result");
    let checker =
        BRepCheckAnalyzer::new(result, true, false, false).expect("source checkshape result");
    assert!(
        checker.is_valid().expect("source checkshape validity"),
        "{SOURCE}: checkshape result"
    );
    let expected = 4_811.71;
    let actual = result_area(result);
    assert!(
        (actual - expected).abs() / expected <= 0.01,
        "{SOURCE}: checkprops result -s {expected}; got {actual}"
    );
}
