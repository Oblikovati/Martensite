//! The same owned conical-face bytes and retained source identities prepare both stage witnesses.
use super::*;
use crate::bop_algo::pave_filler::owned_progress_fixture::read_owned_progress_shape;
use martensite_tkbrep::{topexp::Explorer, topods::Shape};
use martensite_tkmath::gp::Point3;
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};
/// Retained source handles define identities; `let case = degenerate_case(2, false)`.
pub(super) struct DegenerateCase {
    pub filler: BOPAlgoPaveFiller,
    pub original: i32,
    pub owner: i32,
    pub pole: i32,
    pub mapped: i32,
    pub source: Edge,
}
/// Prepare a stage input without invoking the stage; `degenerate_case(4, true)`.
pub(super) fn degenerate_case(kind: usize, parallel: bool) -> DegenerateCase {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.options_mut().set_run_parallel(parallel);
    let (argument, source) = source_argument(kind);
    filler.add_argument(&argument);
    filler.add_argument(&point_vertex(Point3::new(30., 30., 30.)).into_shape());
    filler.initialize().unwrap();
    let mut case = empty_degenerate_case(filler, source);
    if kind >= 2 {
        prepare_degenerated(&mut case, kind);
    }
    case
}
fn empty_degenerate_case(filler: BOPAlgoPaveFiller, source: Edge) -> DegenerateCase {
    DegenerateCase {
        filler,
        original: -1,
        owner: -1,
        pole: -1,
        mapped: -1,
        source,
    }
}
fn source_argument(kind: usize) -> (Shape, Edge) {
    if kind == 0 {
        return (
            point_vertex(Point3::default()).into_shape(),
            Edge::default(),
        );
    }
    if kind == 1 {
        return ordinary_edge_argument();
    }
    let face = read_owned_progress_shape(include_bytes!("conical-face.brep"), ShapeEnum::Face);
    let edge = degenerated_edge(&face);
    let argument = if kind == 2 || kind == 6 {
        edge.clone().into_shape()
    } else {
        face
    };
    (argument, edge)
}
fn ordinary_edge_argument() -> (Shape, Edge) {
    let edge = BRepBuilderAPIMakeEdge::from_points(Point3::default(), Point3::new(2., 0., 0.))
        .unwrap()
        .edge()
        .unwrap();
    (edge.into_shape(), Edge::default())
}
fn degenerated_edge(shape: &Shape) -> Edge {
    let mut explorer = Explorer::new(shape, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let edge = Edge::unchecked(explorer.current().clone());
        if BRepTool::degenerated(&edge).unwrap() {
            return edge;
        }
        explorer.next().unwrap();
    }
    panic!("fixture shape {shape:?}, expected a degenerated edge");
}
fn prepare_degenerated(case: &mut DegenerateCase, kind: usize) {
    let structure = case.filler.structure_mut().unwrap();
    case.original = structure.index(&case.source);
    let info = structure.shape_info(case.original).unwrap();
    case.owner = info.flag();
    case.pole = info.sub_shapes()[0];
    case.mapped = case.pole;
    if kind == 5 || kind == 6 {
        seed_pole_representative(case);
    }
    if (3..=5).contains(&kind) {
        prepare_face_blocks(case, kind);
    }
    assert!(!case.filler.options().has_errors());
}
fn seed_pole_representative(case: &mut DegenerateCase) {
    let structure = case.filler.structure_mut().unwrap();
    let pole = Vertex::unchecked(structure.shape(case.pole).unwrap().clone());
    let mut info = ShapeInfo::default();
    info.set_shape_type(ShapeEnum::Vertex);
    info.set_shape(&point_vertex(BRepTool::pnt(&pole).unwrap()).into_shape());
    case.mapped = structure.append_shape_info(&info);
    structure.add_shape_sd(case.pole, case.mapped);
}
fn prepare_face_blocks(case: &mut DegenerateCase, kind: usize) {
    let structure = case.filler.structure_mut().unwrap();
    for index in 0..structure.nb_source_shapes() {
        if structure.shape_info(index).unwrap().shape_type() == ShapeEnum::Edge {
            structure.change_pave_blocks(index).unwrap();
        }
    }
    if kind == 4 {
        structure.change_face_info(case.owner).unwrap();
        structure.change_pave_blocks(case.original).unwrap().clear();
    }
}
fn point_vertex(point: Point3) -> Vertex {
    BRepBuilderAPIMakeVertex::new(point)
        .unwrap()
        .vertex()
        .unwrap()
}
