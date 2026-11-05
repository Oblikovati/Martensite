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
use martensite_tkmath::gp::Point2;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

const SOURCE: &str = "tests/blend/buildevol/A1";

fn fifth_edge(shape: &Shape) -> Edge {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).expect("source explode s E");
    topods::edge(edges.find_key(5).expect("source s_5 edge")).expect("source s_5 must be an edge")
}

fn source_box() -> Shape {
    BRepPrimAPIMakeBox::from_sizes(100., 100., 10.)
        .expect("source box s 100 100 10")
        .shape()
        .expect("source box result")
        .clone()
}

fn assert_source_area(shape: &Shape) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false)
        .expect("source checkprops result -s 23985.2");
    let expected = 23_985.2;
    assert!(
        (properties.mass() - expected).abs() / expected <= 0.01,
        "{SOURCE}: checkprops result -s {expected}; got {}",
        properties.mass()
    );
}

#[test]
fn Buildevol__A1() {
    let source = source_box();
    let edge = fifth_edge(&source);
    let radii = [
        Point2::new(0., 2.),
        Point2::new(1., 4.),
        Point2::new(2., 2.),
    ];
    let mut fillet = BRepFilletAPIMakeFillet::new(&source, FilletShape::Rational)
        .expect("source mkevol result s");
    fillet
        .add_with_pairs(&radii, &edge)
        .expect("source updatevol s_5 0 2 1 4 2 2");
    fillet
        .build(&ProgressRange::default())
        .expect("source buildevol");
    assert!(fillet.is_done(), "{SOURCE}: buildevol did not complete");
    let result = fillet.shape().expect("source result");
    let checker =
        BRepCheckAnalyzer::new(result, true, false, false).expect("source checkshape result");
    assert!(
        checker.is_valid().expect("source checkshape validity"),
        "{SOURCE}: invalid result"
    );
    assert_source_area(result);
}
