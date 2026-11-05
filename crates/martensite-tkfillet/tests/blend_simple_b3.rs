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
use martensite_tkmath::{
    gp::{Axis1, Direction3, Point3, Transform3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::{brep_builder_api::BRepBuilderAPIMakeShape, brep_gprop::BRepGProp};

const SOURCE: &str = "tests/blend/simple/B3";
const AREA: f64 = 20_559.5;

fn source_cylinder() -> Shape {
    let mut source = BRepPrimAPIMakeCylinder::with_angle(50., 100., 90_f64.to_radians())
        .expect("source pcylinder s 50 100 90")
        .shape()
        .expect("source pcylinder result")
        .clone();
    let mut rotation = Transform3::default();
    rotation
        .set_rotation(
            Axis1::new(Point3::default(), Direction3::Z),
            270_f64.to_radians(),
        )
        .expect("source trotate s 0 0 0 0 0 1 270");
    source
        .move_by(&Location::new(rotation), false)
        .expect("source trotate result");
    source
}

fn source_edges(shape: &Shape) -> [Edge; 3] {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).expect("source explode s E");
    [1, 4, 5].map(|index| {
        topods::edge(edges.find_key(index).expect("source edge index"))
            .expect("source edge must be an edge")
    })
}

fn source_blend(shape: &Shape) -> Shape {
    let mut blend = BRepFilletAPIMakeFillet::new(shape, FilletShape::Rational)
        .expect("source blend result s 10 s_1 10 s_4 10 s_5 setup");
    for edge in source_edges(shape) {
        blend
            .add_with_radius(10., &edge)
            .expect("source blend result edge");
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
        .expect("source checkprops result -s 20559.5");
    assert!(
        (properties.mass() - AREA).abs() / AREA <= 0.01,
        "{SOURCE}: checkprops result -s {AREA}; got {}",
        properties.mass()
    );
}

#[test]
fn BlendSimple__B3() {
    let result = source_blend(&source_cylinder());
    assert_source_area(&result);
}
