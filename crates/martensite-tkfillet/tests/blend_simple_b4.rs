//! Native
#![allow(non_snake_case)]

use martensite_tkbrep::{
    topexp,
    topods::{self, Edge, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkfillet::{brep_fillet_api::BRepFilletAPIMakeFillet, chfi3d::FilletShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::{brep_builder_api::BRepBuilderAPIMakeShape, brep_gprop::BRepGProp};

const SOURCE: &str = "tests/blend/simple/B4";
const AREA: f64 = 44_956.6;

fn source_cylinder() -> Shape {
    BRepPrimAPIMakeCylinder::with_angle(50., 100., 270_f64.to_radians())
        .expect("source pcylinder s 50 100 270")
        .shape()
        .expect("source pcylinder result")
        .clone()
}

fn ninth_edge(shape: &Shape) -> Edge {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).expect("source explode s E");
    topods::edge(edges.find_key(9).expect("source s_9 edge")).expect("source s_9 must be an edge")
}

fn source_blend(shape: &Shape) -> Shape {
    let edge = ninth_edge(shape);
    let mut blend = BRepFilletAPIMakeFillet::new(shape, FilletShape::Rational)
        .expect("source blend result s 10 s_9 setup");
    blend
        .add_with_radius(10., &edge)
        .expect("source blend result s_9");
    blend
        .build(&ProgressRange::default())
        .expect("source blend result build");
    assert!(blend.is_done(), "{SOURCE}: blend result incomplete");
    blend.shape().expect("source blend result").clone()
}

fn assert_source_area(shape: &Shape) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false)
        .expect("source checkprops result -s 44956.6");
    assert!(
        (properties.mass() - AREA).abs() / AREA <= 0.01,
        "{SOURCE}: checkprops result -s {AREA}; got {}",
        properties.mass()
    );
}

#[test]
fn BlendSimple__B4() {
    let result = source_blend(&source_cylinder());
    assert_source_area(&result);
}
