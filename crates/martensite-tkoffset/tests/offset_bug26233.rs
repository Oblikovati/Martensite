//! Reference replay of the bug26233 spherical-face thick-shell regression.

use martensite_tkbrep::{
    brep::BRepTool,
    topexp::{self, Explorer},
    topods::Shape,
    toptools::IndexedMapOfShape,
};
use martensite_tkernel::{message::ProgressRange, standard::Failure};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::geom_abs::JoinType;
use martensite_tkoffset::brep_offset::{BRepOffsetMakeOffset, MakeOffsetOptions};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSphere;
use martensite_tktopalgo::{
    brep_builder_api::BRepBuilderAPIMakeShape, brep_check::BRepCheckAnalyzer,
};

fn count(shape: &Shape, kind: ShapeEnum) -> Result<usize, Failure> {
    let mut found = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(shape, kind, &mut found)?;
    Ok(found.extent())
}

fn max_tolerance(shape: &Shape) -> Result<f64, Failure> {
    [ShapeEnum::Vertex, ShapeEnum::Edge, ShapeEnum::Face]
        .into_iter()
        .map(|kind| BRepTool::max_tolerance(shape, kind))
        .collect::<Result<Vec<_>, _>>()
        .map(|values| values.into_iter().fold(0., f64::max))
}

#[test]
#[allow(non_snake_case)]
fn bug26233() -> Result<(), Failure> {
    let sphere = BRepPrimAPIMakeSphere::new(100.0)?.shape()?.clone();
    let face = Explorer::new(&sphere, ShapeEnum::Face, ShapeEnum::Shape)?
        .current()
        .clone();
    let mut maker = BRepOffsetMakeOffset::new();
    maker.initialize(
        &face,
        10.0,
        1.0e-7,
        MakeOffsetOptions {
            join: JoinType::Intersection,
            thickening: true,
            ..MakeOffsetOptions::default()
        },
    );
    maker.make_offset_shape(&ProgressRange::default())?;
    assert!(maker.is_done(), "26233: thickshell did not complete");
    let result = maker.shape().clone();
    assert!(BRepCheckAnalyzer::new(&result, true, false, false)?.is_valid()?);
    assert_eq!(count(&result, ShapeEnum::Solid)?, 1);
    assert_eq!(count(&result, ShapeEnum::Shell)?, 2);
    assert_eq!(count(&result, ShapeEnum::Face)?, 2);
    assert_eq!(count(&result, ShapeEnum::Wire)?, 2);
    assert_eq!(count(&result, ShapeEnum::Edge)?, 6);
    assert_eq!(count(&result, ShapeEnum::Vertex)?, 4);
    assert!(max_tolerance(&result)? <= 2.0e-7, "26233: high tolerance");
    Ok(())
}
