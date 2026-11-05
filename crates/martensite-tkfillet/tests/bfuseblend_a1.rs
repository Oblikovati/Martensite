//! Native replay of `tests/blend/bfuseblend/A1`.
#![allow(non_snake_case)]

use martensite_tkbo::{
    bop_algo::BOPAlgoPaveFiller,
    brep_algo_api::{BRepAlgoAPIFuse, BRepAlgoAPISection},
};
use martensite_tkbrep::{
    brep::BRepBuilder,
    topexp::Explorer,
    topods::{self, Compound, Shape},
};
use martensite_tkernel::{message::ProgressRange, ncollection::List};
use martensite_tkfillet::{brep_fillet_api::BRepFilletAPIMakeFillet, chfi3d::FilletShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkgeombase::gprop::GProps;
use martensite_tkmath::{
    geom_abs::Shape as Continuity,
    gp::{Axis2, Direction3, Point3, Transform3},
};
use martensite_tkprim::brep_prim_api::{BRepPrimAPIMakeBox, BRepPrimAPIMakeCylinder};
use martensite_tktopalgo::{
    brep_builder_api::{BRepBuilderAPIMakeShape, BRepBuilderAPITransform},
    brep_check::BRepCheckAnalyzer,
    brep_gprop::BRepGProp,
};

const SOURCE: &str = "tests/blend/bfuseblend/A1";

fn source_shapes() -> (Shape, Shape) {
    let box_shape = BRepPrimAPIMakeBox::at_point(Point3::new(-15., -15., -15.), 30., 30., 15.)
        .unwrap()
        .shape()
        .unwrap()
        .clone();
    let axis = Axis2::new(Point3::new(0., 0., -0.01), Direction3::Z).unwrap();
    let cylinder = BRepPrimAPIMakeCylinder::with_axes(axis, 10., 15.)
        .shape()
        .unwrap()
        .clone();
    (scaled(&box_shape), scaled(&cylinder))
}

fn scaled(shape: &Shape) -> Shape {
    let mut transform = Transform3::default();
    transform.set_scale(Point3::default(), 10.).unwrap();
    BRepBuilderAPITransform::with_shape(shape, transform, false, false)
        .unwrap()
        .shape()
        .unwrap()
        .clone()
}

fn paved_fuse(first: &Shape, second: &Shape) -> (Shape, BOPAlgoPaveFiller) {
    let arguments: List<Shape> = [first.clone(), second.clone()].into_iter().collect();
    let mut filler = BOPAlgoPaveFiller::new();
    filler.set_arguments(&arguments);
    filler.options_mut().set_run_parallel(false);
    filler.perform();
    assert!(!filler.options().has_errors(), "{SOURCE}: bfillds");
    let mut fuse = BRepAlgoAPIFuse::from_shapes_with_filler(first, second, filler).unwrap();
    assert!(fuse.is_done(), "{SOURCE}: bfuseblend Boolean operation");
    let result = fuse.shape().clone();
    (
        result,
        fuse.take_ds_filler()
            .expect("{SOURCE}: fuse must preserve the DS filler"),
    )
}

fn blend_section_edges(
    fused: &Shape,
    first: &Shape,
    second: &Shape,
    filler: BOPAlgoPaveFiller,
) -> Shape {
    let section = BRepAlgoAPISection::from_shapes(first, second, Some(filler), true).unwrap();
    let section_shape = section.shape().clone();
    let builder = BRepBuilder::default();
    let mut result = Compound::default();
    builder.make_compound(&mut result);
    let mut solids = Explorer::new(fused, ShapeEnum::Solid, ShapeEnum::Shape).unwrap();
    while solids.more() {
        let solid = solids.current().clone();
        let mut fillet = BRepFilletAPIMakeFillet::new(&solid, FilletShape::Rational).unwrap();
        fillet.set_params(1e-2, 1e-4, 1e-5, 1e-4, 1e-5, 1e-3);
        fillet.set_continuity(Continuity::C1, 1e-2);
        let mut edges = Explorer::new(&section_shape, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
        while edges.more() {
            fillet
                .add_with_radius(10., &topods::edge(edges.current()).unwrap())
                .unwrap();
            edges.next().unwrap();
        }
        fillet.build(&ProgressRange::default()).unwrap();
        let output = if fillet.is_done() {
            fillet.shape().unwrap()
        } else {
            &solid
        };
        builder.add(&mut result, output).unwrap();
        solids.next().unwrap();
    }
    result.into_shape()
}

fn surface_area(shape: &Shape) -> f64 {
    let mut properties = GProps::default();
    BRepGProp::surface_properties(shape, &mut properties, false, false).unwrap();
    properties.mass()
}

#[test]
fn BFuseBlend__A1() {
    let (first, second) = source_shapes();
    let (fused, filler) = paved_fuse(&first, &second);
    let result = blend_section_edges(&fused, &first, &second, filler);
    assert!(
        BRepCheckAnalyzer::new(&result, true, false, false)
            .unwrap()
            .is_valid()
            .unwrap(),
        "{SOURCE}: result must be valid"
    );
    let area = surface_area(&result);
    assert!(
        (area - 451_533.).abs() <= 5_000.,
        "{SOURCE}: checkprops result -s 451533, got {area}"
    );
}
