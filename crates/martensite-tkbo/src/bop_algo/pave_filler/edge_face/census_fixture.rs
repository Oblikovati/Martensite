//! Exact procedural inputs from the retained 24-cell native census.
use super::BOPAlgoPaveFiller;
use crate::bop_algo::Glue;
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::gp::{Plane3, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeFace};

pub(super) fn ef_census_receiver(profile: &str, parallel: bool) -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::default();
    filler.add_argument(&census_edge((-1., 1., 0.)));
    append_census_arguments(&mut filler, profile);
    filler.options_mut().set_run_parallel(parallel);
    filler
        .options_mut()
        .set_fuzzy_value(if profile == "tail" { 0.0004 } else { 0.000001 });
    if profile == "glue_full" {
        filler.set_glue(Glue::Full);
    }
    filler.initialize().unwrap();
    filler
}

fn append_census_arguments(filler: &mut BOPAlgoPaveFiller, profile: &str) {
    if profile == "none" {
        filler.add_argument(&census_edge((-1., 1., 0.)));
        return;
    }
    assert!(
        ["cross", "tail", "glue_full"].contains(&profile),
        "EF profile {profile:?}; expected none/cross/tail/glue_full"
    );
    if profile == "tail" {
        filler.add_argument(&census_edge((-0.99975, 1.00025, 0.00025)));
        filler.add_argument(&census_edge((-0.9995, 1.0005, 0.0005)));
    }
    let face = BRepBuilderAPIMakeFace::from_plane_bounds(Plane3::default(), (-2., 2., -2., 2.))
        .unwrap()
        .face()
        .unwrap()
        .into_shape();
    filler.add_argument(&face);
}

fn census_edge((first, last, y): (f64, f64, f64)) -> Shape {
    BRepBuilderAPIMakeEdge::from_points(Point3::new(first, y, -1.), Point3::new(last, y, 1.))
        .unwrap()
        .edge()
        .unwrap()
        .into_shape()
}
