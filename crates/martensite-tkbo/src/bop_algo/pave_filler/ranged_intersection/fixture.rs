//! Frozen planar faces make source topology identical in root and direct section-stage controls.
use super::*;
use crate::bop_algo::Glue;
use crate::bop_algo::pave_filler::owned_progress_fixture::read_owned_progress_shape;
use crate::bop_ds::InterfFF;
use martensite_tkbrep::topods::Shape;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};
/// Construct source arguments and any explicitly requested intermediate stage; `intersection_case(24, false)`.
pub(super) fn intersection_case(kind: usize, parallel: bool) -> BOPAlgoPaveFiller {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.options_mut().set_run_parallel(parallel);
    filler.set_avoid_build_pcurve(kind == 7);
    if kind == 6 || kind == 21 {
        filler.set_glue(Glue::Full);
    }
    if kind == 8 {
        filler.set_is_primary(false);
    }
    if kind == 10 {
        filler.set_non_destructive(true);
    }
    add_intersection_arguments(&mut filler, kind);
    if kind >= 20 {
        prepare_section_stage(&mut filler, kind);
    }
    filler
}
fn add_intersection_arguments(filler: &mut BOPAlgoPaveFiller, kind: usize) {
    let arguments = match kind {
        0 => Vec::new(),
        1 | 2 | 20 => vec![
            point_vertex(Point3::default()),
            point_vertex(Point3::new(if kind == 2 { 0. } else { 10. }, 0., 0.)),
        ],
        3 | 8 => crossing_segments(),
        9 => vec![
            read_owned_progress_shape(
                include_bytes!("../degenerated/conical-face.brep"),
                ShapeEnum::Face,
            ),
            point_vertex(Point3::new(30., 30., 30.)),
        ],
        _ => plane_arguments(if kind == 5 || kind == 24 { 3 } else { 2 }),
    };
    for argument in arguments {
        filler.add_argument(&argument);
    }
}
fn point_vertex(point: Point3) -> Shape {
    BRepBuilderAPIMakeVertex::new(point)
        .unwrap()
        .vertex()
        .unwrap()
        .into_shape()
}
fn crossing_segments() -> Vec<Shape> {
    [
        [Point3::new(-1., 0., 0.), Point3::new(1., 0., 0.)],
        [Point3::new(0., -1., 0.), Point3::new(0., 1., 0.)],
    ]
    .map(|ends| {
        BRepBuilderAPIMakeEdge::from_points(ends[0], ends[1])
            .unwrap()
            .edge()
            .unwrap()
            .into_shape()
    })
    .to_vec()
}
fn plane_arguments(count: usize) -> Vec<Shape> {
    [
        include_bytes!("plane0.brep").as_slice(),
        include_bytes!("plane1.brep").as_slice(),
        include_bytes!("plane2.brep").as_slice(),
    ]
    .into_iter()
    .take(count)
    .map(|bytes| read_owned_progress_shape(bytes, ShapeEnum::Face))
    .collect()
}
fn prepare_section_stage(filler: &mut BOPAlgoPaveFiller, kind: usize) {
    filler.initialize().unwrap();
    if kind == 20 || kind == 21 {
        return;
    }
    if kind == 22 {
        let mut record = InterfFF::default();
        let structure = filler.data_structure.as_mut().unwrap();
        record.set_indices(
            structure.index(filler.arguments.first().unwrap()),
            structure.index(filler.arguments.last().unwrap()),
        );
        structure.interf_ff().append(record);
        return;
    }
    prepare_through_ef(filler);
    prepare_through_split_edges(filler);
    assert!(!filler.options().has_errors());
}
fn prepare_through_ef(filler: &mut BOPAlgoPaveFiller) {
    filler.prepare().unwrap();
    filler.perform_vv().unwrap();
    filler.perform_ve().unwrap();
    filler.update_pave_blocks_with_sd_vertices().unwrap();
    filler.perform_ee().unwrap();
    filler.update_pave_blocks_with_sd_vertices().unwrap();
    filler.perform_vf().unwrap();
    filler.update_pave_blocks_with_sd_vertices().unwrap();
    filler.perform_ef().unwrap();
    filler.update_after_ef().unwrap();
}
fn prepare_through_split_edges(filler: &mut BOPAlgoPaveFiller) {
    filler.repeat_intersection().unwrap();
    filler.force_interf_ee().unwrap();
    filler.force_interf_ef().unwrap();
    filler.perform_ff().unwrap();
    filler.update_after_ff().unwrap();
    filler.make_split_edges().unwrap();
    filler.update_pave_blocks_with_sd_vertices().unwrap();
}
