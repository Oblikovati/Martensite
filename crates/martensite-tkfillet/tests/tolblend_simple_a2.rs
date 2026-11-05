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
use martensite_tkmath::gp::{Point3, Transform3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeWedge;
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPITransform},
    brep_check::BRepCheckAnalyzer,
    brep_gprop::BRepGProp,
};

const SOURCE: &str = "tests/blend/tolblend_simple/A2";
const SCALE1: f64 = 5.;

fn scaled_source_wedge() -> Shape {
    let mut maker = BRepPrimAPIMakeWedge::with_limits(10., 7., 10., [5., 5., 5., 5.])
        .expect("source wedge s 10 7 10 5 5 5 5");
    let source = maker.shape().expect("source wedge s");
    let mut scale = Transform3::default();
    scale
        .set_scale(Point3::default(), SCALE1)
        .expect("source tscale s 0 0 0 SCALE1");
    BRepBuilderAPITransform::with_shape(source, scale, false, false)
        .expect("source tscale result")
        .shape()
        .expect("source scaled solid")
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
        .expect("source checkprops result -s 6734.43");
    properties.mass()
}

#[test]
fn TolblendSimple__A2() {
    let source = scaled_source_wedge();
    let edges = indexed_edges(&source);
    let mut blend = BRepFilletAPIMakeFillet::new(&source, FilletShape::Rational)
        .expect("source blend result setup");
    for (index, radius) in [(2, 0.2), (3, 0.3), (5, 0.4), (6, 0.5)] {
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
    let expected = 6_734.43;
    let actual = result_area(result);
    assert!(
        (actual - expected).abs() / expected <= 0.01,
        "{SOURCE}: checkprops result -s {expected}; got {actual}"
    );
}
