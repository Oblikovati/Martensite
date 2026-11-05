//! Native replay of the OCC25879 all-edge box fillet regression.
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
fn Modalg7__Bug25879() {
    let source = box_shape();
    let result = fillet_all_edges(&source);
    assert!(is_valid(&result));
}

fn box_shape() -> Shape {
    BRepPrimAPIMakeBox::from_sizes(10., 10., 10.)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

fn fillet_all_edges(source: &Shape) -> Shape {
    let edges = unique_edges(source);
    assert_eq!(edges.len(), 12);
    let mut builder = BRepFilletAPIMakeFillet::new(source, FilletShape::Rational).unwrap();
    for edge in &edges {
        builder.add_with_radius(1., edge).unwrap();
    }
    builder.build(&ProgressRange::default()).unwrap();
    assert!(builder.is_done());
    builder.shape().unwrap().clone()
}

fn unique_edges(shape: &Shape) -> Vec<Edge> {
    let mut mapped = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, ShapeEnum::Edge, &mut mapped).unwrap();
    (1..=mapped.extent())
        .map(|index| topods::edge(mapped.find_key(index).unwrap()).unwrap())
        .collect()
}

fn is_valid(shape: &Shape) -> bool {
    BRepCheckAnalyzer::new(shape, true, false, false)
        .unwrap()
        .is_valid()
        .unwrap()
}
