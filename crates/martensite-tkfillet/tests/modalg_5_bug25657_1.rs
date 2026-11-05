//! Regression of `tests/bugs/modalg_5/bug25657_1`.
#![allow(non_snake_case)]

use martensite_tkbo::brep_algo_api::BRepAlgoAPICut;
use martensite_tkbrep::{
    topexp,
    topods::{self, Edge, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkfillet::{brep_fillet_api::BRepFilletAPIMakeFillet, chfi3d::FilletShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Transform3, Vector3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPITransform},
    brep_check::BRepCheckAnalyzer,
};

const SOURCE: &str = "tests/bugs/modalg_5/bug25657_1";

fn translated_cylinder() -> Shape {
    let cylinder = BRepPrimAPIMakeCylinder::new(206., 120.)
        .shape()
        .unwrap()
        .clone();
    let mut location = Transform3::default();
    location.set_translation_part(Vector3::new(100., 100., 120.));
    BRepBuilderAPITransform::with_shape(&cylinder, location, false, false)
        .expect("source ttranslate a2 100 100 120")
        .shape()
        .expect("source translated a2")
        .clone()
}

fn ninth_edge(shape: &Shape) -> Edge {
    let mut edges = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut edges).expect("source explode a e");
    topods::edge(edges.find_key(9).expect("source a_9")).expect("source a_9 edge")
}

#[test]
fn Modalg5__Bug25657_1() {
    let first = BRepPrimAPIMakeCylinder::new(140., 220.)
        .shape()
        .unwrap()
        .clone();
    let cutter = translated_cylinder();
    let cut = BRepAlgoAPICut::from_shapes(&first, &cutter).expect("source bcut a a1 a2");
    assert!(cut.is_done(), "{SOURCE}: source bcut incomplete");
    let source = cut.shape().clone();
    let edge = ninth_edge(&source);
    let mut blend = BRepFilletAPIMakeFillet::new(&source, FilletShape::Rational)
        .expect("source blend result a 20 a_9 setup");
    blend
        .add_with_radius(20., &edge)
        .expect("source blend result a_9");
    blend
        .build(&ProgressRange::default())
        .expect("source blend result build");
    assert!(blend.is_done(), "{SOURCE}: source blend result incomplete");
    let result = blend.shape().expect("source blend result");
    let check =
        BRepCheckAnalyzer::new(result, true, false, false).expect("source checkshape result");
    assert!(
        check.is_valid().expect("source checkshape result"),
        "{SOURCE}: invalid blend result"
    );
}
