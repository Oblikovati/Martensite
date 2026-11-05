//! Owned fixtures use the unchanged line/plane inputs of the existing EF regression.
use super::*;
use crate::bop_algo::ProgressCancellation;
use martensite_tkernel::message::ProgressScope;
use martensite_tkmath::gp::{Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace};

/// Initializes the unchanged crossing arguments with an actual prepared context.
/// Example: `let filler = crossing_filler(false);`.
pub(super) fn crossing_filler(parallel: bool) -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::default();
    filler.add_argument(&crossing_edge());
    filler.add_argument(&crossing_face());
    filler.options.set_run_parallel(parallel);
    filler.initialize().unwrap();
    filler
}

/// Supplies a genuine edge for crossing and no-face controls.
/// Example: `filler.add_argument(&crossing_edge());`.
pub(super) fn crossing_edge() -> martensite_tkbrep::topods::Shape {
    BRepBuilderAPIMakeEdge::from_points(Point3::new(-1., 0., -1.), Point3::new(1., 0., 1.))
        .unwrap()
        .edge()
        .unwrap()
        .into_shape()
}

fn crossing_face() -> martensite_tkbrep::topods::Shape {
    BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-2., 2., -2., 2.))
        .unwrap()
        .face()
        .unwrap()
        .into_shape()
}

/// Returns genuine unexecuted tasks after actual shrunk-data and candidate preparation.
/// Example: `let (tasks, context) = prepared_crossing_tasks();`.
pub(super) fn prepared_crossing_tasks() -> (
    Vec<EdgeFaceTask>,
    martensite_tkernel::standard::Handle<crate::int_tools::Context>,
) {
    let mut filler = crossing_filler(false);
    filler
        .fill_shrunk_data(ShapeEnum::Edge, ShapeEnum::Face)
        .unwrap();
    filler
        .initialize_progress_pairs(ShapeEnum::Edge, ShapeEnum::Face)
        .unwrap();
    let tasks = prepare_fixture_tasks(&mut filler);
    (tasks, filler.context.clone())
}

fn prepare_fixture_tasks(filler: &mut BOPAlgoPaveFiller) -> Vec<EdgeFaceTask> {
    let scope = ProgressScope::default();
    let observed = ProgressCancellation::default();
    let boundary = super::progress::EdgeFaceBoundary {
        scope: &scope,
        observed: &observed,
    };
    filler.prepare_ef_tasks(&boundary).unwrap().unwrap()
}
