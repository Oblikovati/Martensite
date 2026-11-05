//! Construction-owned source edges survive diagnostic DS index rebinding after a worker stop.
use super::*;
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};
pub(super) struct SplitCase {
    pub filler: BOPAlgoPaveFiller,
    pub originals: Vec<(i32, Edge)>,
}
pub(super) fn split_case(kind: usize, parallel: bool) -> SplitCase {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.options_mut().set_run_parallel(parallel);
    filler.set_non_destructive(kind == 5);
    let edges = source_edges(&mut filler, kind);
    filler.initialize().unwrap();
    let originals: Vec<_> = edges
        .into_iter()
        .map(|edge| (filler.data_structure().unwrap().index(&edge), edge))
        .collect();
    prepare_case(&mut filler, &originals, kind);
    SplitCase { filler, originals }
}
fn source_edges(filler: &mut BOPAlgoPaveFiller, kind: usize) -> Vec<Edge> {
    if kind == 0 || kind == 6 {
        for x in [0., 10.] {
            filler.add_argument(
                &BRepBuilderAPIMakeVertex::new(Point3::new(x, 0., 0.))
                    .unwrap()
                    .vertex()
                    .unwrap()
                    .into_shape(),
            );
        }
        return Vec::new();
    }
    let edges = if kind == 4 || kind == 5 {
        shared_vertex_edges()
    } else {
        separate_vertex_edges(kind)
    };
    for edge in &edges {
        filler.add_argument(&edge.clone().into_shape());
    }
    if kind == 1 || kind == 7 {
        filler.add_argument(
            &BRepBuilderAPIMakeVertex::new(Point3::new(10., 0., 0.))
                .unwrap()
                .vertex()
                .unwrap()
                .into_shape(),
        );
    }
    edges
}
fn shared_vertex_edges() -> Vec<Edge> {
    let first = BRepBuilderAPIMakeVertex::new(Point3::new(-1., 0., 0.))
        .unwrap()
        .vertex()
        .unwrap();
    let last = BRepBuilderAPIMakeVertex::new(Point3::new(1., 0., 0.))
        .unwrap()
        .vertex()
        .unwrap();
    (0..2)
        .map(|_| {
            BRepBuilderAPIMakeEdge::from_vertices(&first, &last)
                .unwrap()
                .edge()
                .unwrap()
        })
        .collect()
}
fn separate_vertex_edges(kind: usize) -> Vec<Edge> {
    let mut ends = vec![[Point3::new(-1., 0., 0.), Point3::new(1., 0., 0.)]];
    if kind == 2 {
        ends.push([Point3::new(0., -1., 0.), Point3::new(0., 1., 0.)]);
    }
    if kind == 3 {
        ends.push(ends[0]);
    }
    ends.into_iter()
        .map(|points| {
            BRepBuilderAPIMakeEdge::from_points(points[0], points[1])
                .unwrap()
                .edge()
                .unwrap()
        })
        .collect()
}
fn prepare_case(filler: &mut BOPAlgoPaveFiller, originals: &[(i32, Edge)], kind: usize) {
    if kind == 2 {
        filler.perform_ee().unwrap();
        filler.update_pave_blocks_with_sd_vertices().unwrap();
    } else if kind == 3 {
        filler.perform_vv().unwrap();
        filler.update_pave_blocks_with_sd_vertices().unwrap();
        filler.force_interf_ee().unwrap();
    } else {
        for (edge, _) in originals {
            filler
                .data_structure
                .as_mut()
                .unwrap()
                .change_pave_blocks(*edge)
                .unwrap();
        }
    }
    if kind == 4 || kind == 5 {
        filler.force_interf_ee().unwrap();
    }
    if kind == 6 {
        filler
            .data_structure
            .as_mut()
            .unwrap()
            .change_pave_blocks_pool()
            .extend([Vec::new(), Vec::new()]);
    }
    if kind == 7 {
        filler
            .data_structure
            .as_mut()
            .unwrap()
            .change_shape_info(originals[0].0)
            .unwrap()
            .set_flag(1);
    }
    assert!(!filler.options().has_errors());
}
