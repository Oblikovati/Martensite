//! Native replay of the OCC25478 touching-fillet cut regression.
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
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer,
};

#[test]
fn Modalg7__Bug25478_2() {
    let cut = source_cut();
    let edges = edges(&cut);
    let result = touching_fillets(&cut, &edges);
    assert!(
        BRepCheckAnalyzer::new(&result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
}

fn source_cut() -> Shape {
    let object = BRepPrimAPIMakeBox::from_sizes(10., 10., 10.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let tool = BRepPrimAPIMakeBox::at_point(Point3::new(5., 5., -1.), 10., 10., 12.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let cut = BRepAlgoAPICut::from_shapes(&object, &tool).unwrap();
    assert!(cut.is_done());
    cut.shape().clone()
}

fn edges(shape: &Shape) -> Vec<Edge> {
    let mut mapped = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut mapped).unwrap();
    (1..=mapped.extent())
        .map(|index| topods::edge(mapped.find_key(index).unwrap()).unwrap())
        .collect()
}

fn touching_fillets(source: &Shape, edges: &[Edge]) -> Shape {
    assert!(edges.len() >= 18);
    let mut fillet = BRepFilletAPIMakeFillet::new(source, FilletShape::Rational).unwrap();
    for index in [12, 16, 17] {
        fillet.add_with_radius(2.5, &edges[index]).unwrap();
    }
    fillet.build(&ProgressRange::default()).unwrap();
    assert!(fillet.is_done());
    fillet.shape().unwrap().clone()
}
