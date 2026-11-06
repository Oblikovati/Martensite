//! Whole public-entry fixtures reuse constructed identities rather than coordinate matching.
use crate::bop_algo::remove_features::{
    BOPAlgoRemoveFeatures, feature::fixture::parent_fixture, helpers::compound_of,
};
use crate::brep_algo_api::BRepAlgoAPIDefeaturing;
use martensite_tkbrep::{
    brep::BRepBuilder,
    topods::{CompSolid, Shape},
};
use martensite_tkernel::ncollection::List;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
pub(super) fn whole_entry_inputs(profile: usize) -> (Shape, List<Shape>) {
    let prepared = match profile {
        4 | 9 | 10 => 2,
        5 => 5,
        6 => 6,
        7 | 8 => 7,
        0..=3 => 0,
        other => panic!("entry profile={other}; expected 0..10"),
    };
    let remover = parent_fixture(prepared, false);
    let mut source = remover.input_shape().clone();
    source = entry_source_kind(profile, source);
    (source, remover.features.into_iter().collect())
}
fn entry_source_kind(profile: usize, source: Shape) -> Shape {
    match profile {
        0 => Shape::default(),
        2 => entry_vertex([1., 2., 3.]),
        3 => compound_of(&[]).unwrap(),
        9 => compound_of(&[source, entry_vertex([8., 9., 10.])]).unwrap(),
        10 => entry_compsolid(&source),
        _ => source,
    }
}
fn entry_vertex(point: [f64; 3]) -> Shape {
    BRepBuilderAPIMakeVertex::new(Point3::new(point[0], point[1], point[2]))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}
fn entry_compsolid(solid: &Shape) -> Shape {
    let builder = BRepBuilder::default();
    let mut body = CompSolid::default();
    builder.make_comp_solid(&mut body);
    let mut result = body.into_shape();
    builder.add(&mut result, solid).unwrap();
    result
}
pub(super) fn whole_entry_facade(
    profile: usize,
    parallel: bool,
) -> (BRepAlgoAPIDefeaturing, Shape) {
    let (source, requested) = whole_entry_inputs(profile);
    let mut api = BRepAlgoAPIDefeaturing::new();
    api.set_shape(&source);
    api.add_faces_to_remove(&requested);
    api.set_to_fill_history(profile != 7);
    api.set_run_parallel(parallel);
    (api, source)
}
pub(super) fn whole_entry_receiver(profile: usize) -> BOPAlgoRemoveFeatures {
    let (source, requested) = whole_entry_inputs(profile);
    let mut remover = BOPAlgoRemoveFeatures::new();
    remover.set_shape(&source);
    remover.add_faces_to_remove(&requested);
    remover.set_to_fill_history(profile != 7);
    remover
}
