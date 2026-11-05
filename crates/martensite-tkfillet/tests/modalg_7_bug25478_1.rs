//! Native replay of the OCC25478 touching-fillet regression.
#![allow(non_snake_case)]

use martensite_tkbrep::{
    topexp,
    topods::{self, Edge, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkfillet::{brep_fillet_api::BRepFilletAPIMakeFillet, chfi3d::FilletShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer,
};

#[test]
fn Modalg7__Bug25478_1() {
    let source = BRepPrimAPIMakeBox::from_sizes(10., 10., 10.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let edges = box_edges(&source);
    let result = touching_fillets(&source, &edges);
    assert!(
        BRepCheckAnalyzer::new(&result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
}

fn box_edges(shape: &Shape) -> Vec<Edge> {
    let mut mapped = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut mapped).unwrap();
    (1..=mapped.extent())
        .map(|index| topods::edge(mapped.find_key(index).unwrap()).unwrap())
        .collect()
}

fn touching_fillets(source: &Shape, edges: &[Edge]) -> Shape {
    assert_eq!(edges.len(), 12);
    let mut fillet = BRepFilletAPIMakeFillet::new(source, FilletShape::Rational).unwrap();
    fillet.add_with_radius(5., &edges[0]).unwrap();
    fillet.add_with_radius(5., &edges[2]).unwrap();
    fillet.build(&ProgressRange::default()).unwrap();
    assert!(fillet.is_done());
    fillet.shape().unwrap().clone()
}
