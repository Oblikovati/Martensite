//! Frozen owned boxes and retained construction handles cover the concrete volume policies.
use super::*;
use crate::bop_algo::pave_filler::owned_progress_fixture::read_owned_progress_shape;
use martensite_tkbrep::topods::Shape;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeVertex;
pub(super) struct VolumeCase {
    pub maker: BOPAlgoMakerVolume,
    pub sources: Vec<Shape>,
}
/// Profile ordinals encode kind*2+intersect; `volume_case(4, false)` uses one unintersected box.
pub(super) fn volume_case(profile: usize, parallel: bool) -> VolumeCase {
    let kind = profile / 2;
    let sources = volume_sources(kind);
    let mut maker = BOPAlgoMakerVolume::new();
    maker.set_intersect(profile % 2 == 1);
    maker.options_mut().set_run_parallel(parallel);
    maker.set_to_fill_history(kind != 6);
    maker.set_avoid_internal_shapes(kind == 5);
    maker.set_non_destructive(kind == 7);
    for source in &sources {
        maker.add_argument(source);
    }
    VolumeCase { maker, sources }
}
fn volume_sources(kind: usize) -> Vec<Shape> {
    if kind == 0 {
        return vec![];
    }
    if kind == 1 {
        return vec![vertex(0.)];
    }
    let mut sources = vec![read_owned_progress_shape(
        include_bytes!("../builder/entry_progress/box0.brep"),
        ShapeEnum::Solid,
    )];
    if kind == 3 {
        sources.push(read_owned_progress_shape(
            include_bytes!("../builder/entry_progress/box1.brep"),
            ShapeEnum::Solid,
        ));
    }
    if kind == 4 || kind == 5 {
        sources.push(vertex(1.));
    }
    sources
}
fn vertex(x: f64) -> Shape {
    BRepBuilderAPIMakeVertex::new(Point3::new(x, x, x))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}
/// Include volume-specific intermediate state with actual base incidence; `snapshot(&case, observed, queries)`.
pub(super) fn snapshot(case: &VolumeCase, observed: bool, queries: usize) -> String {
    let base = crate::bop_algo::builder::entry_progress::snapshot::builder_snapshot(
        &case.maker.builder,
        &case.sources,
        observed,
        queries,
    );
    format!(
        "{base};[{},{}]",
        case.maker.bounding_box().is_null(),
        case.maker.faces().len()
    )
}
