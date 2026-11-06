//! Owned source handles retain argument/tool roles across all four Boolean operations.
use super::*;
use crate::bop_algo::pave_filler::owned_progress_fixture::read_owned_progress_shape;
use martensite_tkbrep::{brep::BRepBuilder, topods::Compound};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};
pub(super) struct BooleanCase {
    pub bop: BOPAlgoBOP,
    pub sources: Vec<Shape>,
}
/// Profile ordinals retain kind*4+operation; `boolean_case(21, false)` fuses overlapping boxes.
pub(super) fn boolean_case(profile: usize, parallel: bool) -> BooleanCase {
    let kind = profile / 4;
    let sources = boolean_sources(kind);
    let operations = [
        Operation::Common,
        Operation::Fuse,
        Operation::Cut,
        Operation::Cut21,
    ];
    let mut bop = BOPAlgoBOP::new();
    bop.set_operation(if kind == 10 {
        Operation::Unknown
    } else {
        operations[profile % 4]
    });
    bop.options_mut().set_run_parallel(parallel);
    bop.set_to_fill_history(kind != 11);
    bop.set_non_destructive(kind == 12);
    if kind == 13 {
        bop.options_mut().set_fuzzy_value(1e-5);
        bop.options_mut().set_use_obb(true);
    }
    if let Some(first) = sources.first() {
        bop.add_argument(first);
        if kind == 9 {
            bop.add_argument(&sources[1]);
        }
        bop.add_tool(sources.last().unwrap());
    }
    BooleanCase { bop, sources }
}
fn boolean_sources(kind: usize) -> Vec<Shape> {
    match kind {
        0 => vec![],
        1 | 2 | 10 => vec![vertex(0.), vertex(if kind == 2 { 0. } else { 10. })],
        3 | 13 => vec![
            edge([-1., 0., 0.], [1., 0., 0.]),
            edge([0., -1., 0.], [0., 1., 0.]),
        ],
        4 => vec![
            read_owned_progress_shape(
                include_bytes!("../pave_filler/ranged_intersection/plane0.brep"),
                ShapeEnum::Face,
            ),
            read_owned_progress_shape(
                include_bytes!("../pave_filler/ranged_intersection/plane1.brep"),
                ShapeEnum::Face,
            ),
        ],
        6 => vec![empty(), empty()],
        7 => vec![empty(), box_shape(1)],
        8 => vec![box_shape(0), empty()],
        9 => vec![box_shape(0), box_shape(1), empty()],
        14 => vec![open_box(), box_shape(1)],
        15 => vec![open_box(), open_box()],
        _ => vec![box_shape(0), box_shape(1)],
    }
}
fn box_shape(index: usize) -> Shape {
    let bytes: &[u8] = if index == 0 {
        include_bytes!("../builder/entry_progress/box0.brep")
    } else {
        include_bytes!("../builder/entry_progress/box1.brep")
    };
    read_owned_progress_shape(bytes, ShapeEnum::Solid)
}
fn open_box() -> Shape {
    read_owned_progress_shape(
        include_bytes!("progress_fixture/openbox0.brep"),
        ShapeEnum::Solid,
    )
}
fn empty() -> Shape {
    let mut shape = Compound::default();
    BRepBuilder::default().make_compound(&mut shape);
    shape.into_shape()
}
fn vertex(x: f64) -> Shape {
    BRepBuilderAPIMakeVertex::new(Point3::new(x, 0., 0.))
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}
fn edge(first: [f64; 3], last: [f64; 3]) -> Shape {
    BRepBuilderAPIMakeEdge::from_points(
        Point3::new(first[0], first[1], first[2]),
        Point3::new(last[0], last[1], last[2]),
    )
    .unwrap()
    .edge()
    .unwrap()
    .into_shape()
}
/// Observe actual final state and retained split selection; `snapshot(&case, observed, queries)`.
pub(super) fn snapshot(case: &BooleanCase, observed: bool, queries: usize) -> String {
    let base = crate::bop_algo::builder::entry_progress::snapshot::builder_snapshot(
        case.bop.builder(),
        &case.sources,
        observed,
        queries,
    );
    let dims = case.bop.dims();
    let rc = &case.bop.rc;
    format!(
        "{base};[{},{},{},{}]",
        dims[0],
        dims[1],
        rc.is_null(),
        if rc.is_null() {
            -1
        } else {
            rc.shape_type().unwrap() as i32
        }
    )
}
