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
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::{brep_builder_api::BRepBuilderAPIMakeShape, brep_gprop::BRepGProp};

const SOURCE: &str = "tests/blend/simple/A2";
const AREA: f64 = 58_633.8;

fn source_box() -> Shape {
    BRepPrimAPIMakeBox::from_sizes(100., 100., 100.)
        .expect("source box s 100 100 100")
        .shape()
        .expect("source box result")
        .clone()
}

fn source_edges(shape: &Shape) -> [Edge; 3] {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).expect("source explode s E");
    [5, 6, 10].map(|index| {
        topods::edge(edges.find_key(index).expect("source edge index")).expect("source edge")
    })
}

fn source_blend(shape: &Shape) -> Shape {
    let mut blend = BRepFilletAPIMakeFillet::new(shape, FilletShape::Rational)
        .expect("source blend result s 10 s_5 10 s_6 10 s_10 setup");
    for edge in source_edges(shape) {
        blend
            .add_with_radius(10., &edge)
            .expect("source blend edge");
    }
    blend
        .build(&ProgressRange::default())
        .expect("source blend result build");
    assert!(blend.is_done(), "{SOURCE}: blend result incomplete");
    blend.shape().expect("source blend result").clone()
}

fn assert_source_area(shape: &Shape) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false)
        .expect("source checkprops result -s 58633.8");
    assert!(
        (properties.mass() - AREA).abs() / AREA <= 0.01,
        "{SOURCE}: checkprops result -s {AREA}; got {}",
        properties.mass()
    );
}

#[test]
fn BlendSimple__A2() {
    let result = source_blend(&source_box());
    assert_source_area(&result);
}
