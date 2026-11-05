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

const SOURCE: &str = "tests/blend/simple/K1";
const SCALE1: f64 = 10.;

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
    .expect("source restore CFI_l12__fiu.rle");
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

fn fifth_edge(shape: &Shape) -> Edge {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).expect("source explode s e");
    topods::edge(edges.find_key(5).expect("source s_5")).expect("source s_5 edge")
}

fn assert_source_area(shape: &Shape) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false)
        .expect("source checkprops result -s 72801.3");
    let expected = 72_801.3;
    assert!(
        (properties.mass() - expected).abs() / expected <= 0.01,
        "{SOURCE}: checkprops result -s {expected}; got {}",
        properties.mass()
    );
}

#[test]
fn BlendSimple__K1() {
    let fixture = restored_shape(include_bytes!(
        "../../../external/martensite-test-data/dataset-7.9.0/brep/CFI_l12__fiu.rle"
    ));
    let source = scaled_shape(&fixture);
    let edge = fifth_edge(&source);
    let mut blend = BRepFilletAPIMakeFillet::new(&source, FilletShape::Rational)
        .expect("source blend result s 0.5*SCALE1 s_5 setup");
    blend
        .add_with_radius(0.5 * SCALE1, &edge)
        .expect("source blend result s_5");
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
    assert_source_area(result);
}
