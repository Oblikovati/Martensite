//! Native regression for `tests/boolean/history/A7`.
#![allow(non_snake_case)]

use martensite_tkbo::brep_algo_api::BRepAlgoAPIFuse;
use martensite_tkbrep::{
    topexp::{self, Explorer},
    topods::{self, Edge, Shape},
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::message::ProgressRange;
use martensite_tkfillet::{brep_fillet_api::BRepFilletAPIMakeFillet, chfi3d::FilletShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::{geom_abs::Shape as Continuity, gp::Point3};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer, brep_gprop::BRepGProp,
};

fn box_at(corner: Point3, side: f64) -> Shape {
    BRepPrimAPIMakeBox::at_point(corner, side, side, side)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

fn generated_edges(fuse: &BRepAlgoAPIFuse, source: &Shape) -> Vec<Edge> {
    let mut edges = IndexedMapOfShape::default();
    let mut faces = Explorer::new(source, ShapeEnum::Face, ShapeEnum::Shape).unwrap();
    while faces.more() {
        for generated in fuse.generated(faces.current()).unwrap() {
            topexp::map_shapes_of_type(&generated, ShapeEnum::Edge, &mut edges).unwrap();
        }
        faces.next().unwrap();
    }
    edges
        .iter()
        .map(|shape| topods::edge(shape).unwrap())
        .collect()
}

fn surface_area(shape: &Shape) -> f64 {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false).unwrap();
    properties.mass()
}

fn volume(shape: &Shape) -> f64 {
    let mut properties = GProps::default();
    BRepGProp::volume_properties(shape, &mut properties, false, false, false).unwrap();
    properties.mass()
}

fn assert_topology(shape: &Shape) {
    let mut map = IndexedMapOfShape::default();
    topexp::map_shapes(shape, &mut map, true, true);
    let count = |kind| {
        map.iter()
            .filter(|item| item.shape_type() == Ok(kind))
            .count()
    };
    assert_eq!(count(ShapeEnum::Vertex), 26);
    assert_eq!(count(ShapeEnum::Edge), 42);
    assert_eq!(count(ShapeEnum::Wire), 18);
    assert_eq!(count(ShapeEnum::Face), 18);
    assert_eq!(count(ShapeEnum::Shell), 1);
    assert_eq!(count(ShapeEnum::Solid), 1);
}

// Direct translation of the generated-edge history and `blend` calls in history/A7.
#[test]
fn BooleanHistory__A7_FuseGeneratedEdgesBlended() {
    let first = box_at(Point3::new(0.0, 0.0, 0.0), 10.0);
    let second = box_at(Point3::new(5.0, 5.0, 5.0), 10.0);
    let mut fuse = BRepAlgoAPIFuse::from_shapes(&first, &second).unwrap();
    fuse.set_run_parallel(false);
    fuse.build().unwrap();
    assert!(!fuse.has_errors());

    let edges = generated_edges(&fuse, &first);
    assert!(!edges.is_empty(), "fuse history must generate blend edges");
    let fused = fuse.shape().clone();
    let mut blend = BRepFilletAPIMakeFillet::new(&fused, FilletShape::Rational).unwrap();
    blend.set_params(1e-2, 1e-4, 1e-5, 1e-4, 1e-5, 1e-3);
    blend.set_continuity(Continuity::C1, 1e-2);
    for edge in &edges {
        blend.add_with_radius(2.0, edge).unwrap();
    }
    blend.build(&ProgressRange::default()).unwrap();
    assert!(blend.is_done(), "blend operation failed");
    let result = blend.shape().unwrap();

    assert!(
        BRepCheckAnalyzer::new(result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    assert!((surface_area(result) - 1027.65).abs() <= 1e-2);
    assert!((volume(result) - 1905.35).abs() <= 1e-2);
    assert_topology(result);
}
