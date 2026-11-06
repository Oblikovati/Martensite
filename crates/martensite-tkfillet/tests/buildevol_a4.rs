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
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

const SOURCE: &str = "tests/blend/buildevol/A4";

fn ninth_edge(shape: &Shape) -> Edge {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).expect("source explode s E");
    topods::edge(edges.find_key(9).expect("source s_9 edge")).expect("source s_9 must be an edge")
}

fn source_partial_cylinder() -> Shape {
    let mut source = BRepPrimAPIMakeCylinder::with_angle(50., 100., 90_f64.to_radians())
        .expect("source pcylinder s 50 100 90")
        .shape()
        .expect("source partial cylinder result")
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
        .expect("source checkprops result -s 21068.3");
    let expected = 21_068.3;
    assert!(
        (properties.mass() - expected).abs() / expected <= 0.01,
        "{SOURCE}: checkprops result -s {expected}; got {}",
        properties.mass()
    );
}

#[test]
fn Buildevol__A4() {
    let source = source_partial_cylinder();
    let edge = ninth_edge(&source);
    let radii = [
        Point2::new(0., 10.),
        Point2::new(1., 20.),
        Point2::new(2., 10.),
    ];
    let mut fillet = BRepFilletAPIMakeFillet::new(&source, FilletShape::Rational)
        .expect("source mkevol result s");
    fillet
        .add_with_pairs(&radii, &edge)
        .expect("source updatevol s_9 0 10 1 20 2 10");
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
