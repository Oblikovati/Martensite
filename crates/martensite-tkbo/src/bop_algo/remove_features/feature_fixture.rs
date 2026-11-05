//! Prepared parent fixtures keep feature identity through construction history.
use super::super::helpers::{compound_of, explore};
use super::*;
use crate::brep_algo_api::{BRepAlgoAPIBooleanOperation, BRepAlgoAPIFuse};
use martensite_tkbrep::{topexp, topods::Shape, toptools::IndexedMapOfShape};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeBox;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;

pub(super) fn parent_box(corner: [f64; 3], size: [f64; 3]) -> Shape {
    BRepPrimAPIMakeBox::at_point(
        Point3::new(corner[0], corner[1], corner[2]),
        size[0],
        size[1],
        size[2],
    )
    .unwrap()
    .shape()
    .unwrap()
    .clone()
}
fn construction_faces(operation: &BRepAlgoAPIBooleanOperation, tool: &Shape) -> Vec<Shape> {
    let mut faces = Vec::new();
    let mut result = IndexedMapOfShape::default();
    topexp::map_shapes_of_type(operation.shape(), ShapeEnum::Face, &mut result).unwrap();
    for source in explore(tool, ShapeEnum::Face).unwrap() {
        faces.extend(
            operation
                .modified(&source)
                .unwrap()
                .into_iter()
                .filter(|image| result.contains(image)),
        );
        if result.contains(&source) {
            faces.push(source);
        }
    }
    faces
}
fn boss_fixture(count: usize) -> (Shape, Vec<Shape>) {
    let base = parent_box([0., 0., 0.], [10., 10., 5.]);
    let tools: Vec<_> = [[1., 1., 5.], [6., 6., 5.]]
        .into_iter()
        .take(count)
        .map(|corner| parent_box(corner, [2., 2., 2.]))
        .collect();
    let fuse = BRepAlgoAPIFuse::from_shapes(&base, &compound_of(&tools).unwrap()).unwrap();
    let features = tools
        .iter()
        .map(|tool| compound_of(&construction_faces(&fuse, tool)).unwrap())
        .collect();
    (fuse.shape().clone(), features)
}
pub(in crate::bop_algo::remove_features) fn parent_fixture(
    profile: usize,
    parallel: bool,
) -> BOPAlgoRemoveFeatures {
    let (source, features) = parent_geometry(profile);
    let mut remover = BOPAlgoRemoveFeatures::new();
    remover.set_shape(&source);
    remover.base.shape = source;
    remover.features = features;
    remover.base.options.set_run_parallel(parallel);
    remover.base.fill_history = profile != 7;
    remover
}
fn parent_geometry(profile: usize) -> (Shape, Vec<Shape>) {
    if matches!(profile, 6 | 7) {
        return boss_fixture(if profile == 6 { 1 } else { 2 });
    }
    let source = parent_box([0., 0., 0.], [2., 3., 4.]);
    match profile {
        0 => (source, Vec::new()),
        1 => (source, vec![compound_of(&[]).unwrap()]),
        2 => (
            source.clone(),
            vec![compound_of(&explore(&source, ShapeEnum::Face).unwrap()).unwrap()],
        ),
        3 => (
            source,
            vec![compound_of(&[]).unwrap(), compound_of(&[]).unwrap()],
        ),
        4 => separate_solids_fixture(source),
        5 => (
            source.clone(),
            vec![compound_of(&explore(&source, ShapeEnum::Face).unwrap()[..1]).unwrap()],
        ),
        other => panic!("parent profile={other}; expected 0..7"),
    }
}
fn separate_solids_fixture(first: Shape) -> (Shape, Vec<Shape>) {
    let second = parent_box([8., 0., 0.], [2., 3., 4.]);
    let features = [&first, &second]
        .map(|solid| compound_of(&explore(solid, ShapeEnum::Face).unwrap()).unwrap())
        .to_vec();
    (compound_of(&[first, second]).unwrap(), features)
}
