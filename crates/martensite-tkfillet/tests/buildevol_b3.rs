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
    gp::{Axis1, Direction3, Point2, Point3, Transform3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCone;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

const SOURCE: &str = "tests/blend/buildevol/B3";

fn seventh_edge(shape: &Shape) -> Edge {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).expect("source explode s E");
    topods::edge(edges.find_key(7).expect("source s_7 edge")).expect("source s_7 must be an edge")
}

fn source_partial_cone() -> Shape {
    let mut source = BRepPrimAPIMakeCone::with_angle(50., 0., 120., 90_f64.to_radians())
        .expect("source pcone s 50 0 120 90")
        .shape()
        .expect("source partial cone result")
        .clone();
    let mut rotation = Transform3::default();
    rotation
        .set_rotation(
            Axis1::new(Point3::default(), Direction3::Z),
            90_f64.to_radians(),
        )
        .expect("source trotate s 0 0 0 0 0 1 90");
    source
        .move_by(&Location::new(rotation), false)
        .expect("source trotate result");
    source
}

fn assert_source_area(shape: &Shape) {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false)
        .expect("source checkprops result -s 12045.6");
    let expected = 12_045.6;
    assert!(
        (properties.mass() - expected).abs() / expected <= 0.01,
        "{SOURCE}: checkprops result -s {expected}; got {}",
        properties.mass()
    );
}

#[test]
fn Buildevol__B3() {
    let source = source_partial_cone();
    let edge = seventh_edge(&source);
    let radii = [
        Point2::new(0., 10.),
        Point2::new(1., 20.),
        Point2::new(2., 10.),
    ];
    let mut fillet = BRepFilletAPIMakeFillet::new(&source, FilletShape::Rational)
        .expect("source mkevol result s");
    fillet
        .add_with_pairs(&radii, &edge)
        .expect("source updatevol s_7 0 10 1 20 2 10");
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
