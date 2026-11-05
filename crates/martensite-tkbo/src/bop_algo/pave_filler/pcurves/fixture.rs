//! Intermediate stage inputs retain actual source handles rather than reconstructing output identity.
use super::*;
use crate::bop_algo::SectionAttribute;
use crate::bop_algo::pave_filler::owned_progress_fixture::read_owned_progress_shape;
use crate::bop_ds::{CommonBlock, InterfFF, Pave};
use martensite_tkbrep::topexp::Explorer;
use martensite_tkg2d::geom2d::Geom2dLine;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::gp::{Direction2, Point2, Point3};
use martensite_tktopalgo::brep_builder_api::{BRepBuilderAPIMakeEdge, BRepBuilderAPIMakeVertex};
use std::sync::Mutex;
/// Source inputs and stage state; `let case = pcurve_case(4, false)`.
pub(super) struct PCurveCase {
    pub filler: BOPAlgoPaveFiller,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
}
/// Build the explicit pre-stage state; `pcurve_case(8, true)` seeds repeated section pairs.
pub(super) fn pcurve_case(kind: usize, parallel: bool) -> PCurveCase {
    let mut filler = BOPAlgoPaveFiller::new();
    filler.options_mut().set_run_parallel(parallel);
    filler.set_avoid_build_pcurve(kind == 1);
    if kind == 2 {
        filler.set_section_attribute(&SectionAttribute::with_flags(true, false, false));
    }
    let mut case = PCurveCase {
        filler,
        edges: Vec::new(),
        faces: Vec::new(),
    };
    if kind == 0 {
        add_vertex_arguments(&mut case.filler);
    } else {
        add_pcurve_arguments(&mut case, kind);
    }
    case.filler.initialize().unwrap();
    if kind != 0 {
        prepare_face_tasks(&mut case, kind);
    }
    case
}
fn add_vertex_arguments(filler: &mut BOPAlgoPaveFiller) {
    for x in [0., 10.] {
        filler.add_argument(&point_vertex(Point3::new(x, 0., 0.)).into_shape());
    }
}
fn point_vertex(point: Point3) -> Vertex {
    BRepBuilderAPIMakeVertex::new(point)
        .unwrap()
        .vertex()
        .unwrap()
}
fn add_pcurve_arguments(case: &mut PCurveCase, kind: usize) {
    for _ in 0..if kind == 8 { 2 } else { 1 } {
        let shape = read_owned_progress_shape(
            include_bytes!("../face_face_store_fixture/cylinder.brep"),
            ShapeEnum::Face,
        );
        case.filler.add_argument(&shape);
        case.faces.push(Face::unchecked(shape));
    }
    for _ in 0..if kind == 7 { 4 } else { 1 } {
        let edge = source_edge(kind);
        case.filler.add_argument(&edge.clone().into_shape());
        case.edges.push(edge);
    }
}
fn source_edge(kind: usize) -> Edge {
    if kind != 10 {
        return BRepBuilderAPIMakeEdge::from_points(
            Point3::new(1., 0., -1.),
            Point3::new(1., 0., 1.),
        )
        .unwrap()
        .edge()
        .unwrap();
    }
    let builder = BRepBuilder::default();
    let mut edge = Edge::default();
    builder.make_edge(&mut edge).unwrap();
    let mut shape = edge.into_shape();
    for (z, orientation) in [(-1., Orientation::Forward), (1., Orientation::Reversed)] {
        builder
            .add(
                &mut shape,
                &point_vertex(Point3::new(1., 0., z)).oriented(orientation),
            )
            .unwrap();
    }
    Edge::unchecked(shape)
}
fn prepare_face_tasks(case: &mut PCurveCase, kind: usize) {
    let structure = case.filler.data_structure.as_mut().unwrap();
    let face = structure.index(&case.faces[0]);
    structure.change_face_info(face).unwrap();
    if kind == 3 {
        return;
    }
    let blocks: Vec<_> = case
        .edges
        .iter()
        .map(|edge| prepare_edge_block(structure, edge))
        .collect();
    insert_face_blocks(structure, face, &blocks, kind);
    if kind == 6 {
        prepare_common(case, &blocks[0]);
    }
    if kind == 8 {
        prepare_sections(case, &blocks[0]);
    }
    if kind == 9 {
        prepare_periodic_curve(case);
    }
}
fn prepare_edge_block(
    structure: &mut crate::bop_ds::DataStructure,
    edge: &Edge,
) -> PaveBlockHandle {
    let index = structure.index(edge);
    let block = structure.change_pave_blocks(index).unwrap()[0].clone();
    block.lock().unwrap().set_edge(index);
    block
}
fn insert_face_blocks(
    structure: &mut crate::bop_ds::DataStructure,
    face: i32,
    blocks: &[PaveBlockHandle],
    kind: usize,
) {
    let info = structure.change_face_info(face).unwrap();
    for block in blocks {
        if kind == 5 || kind == 6 {
            info.change_pave_blocks_on().add(block.clone());
        } else if kind != 8 {
            info.change_pave_blocks_in().add(block.clone());
        }
    }
}
fn seam_edge(face: &Face) -> Edge {
    let mut explorer = Explorer::new(face, ShapeEnum::Edge, ShapeEnum::Shape).unwrap();
    while explorer.more() {
        let edge = Edge::unchecked(explorer.current().clone());
        if BRepTool::is_closed_on_face(&edge, face).unwrap() {
            return edge;
        }
        explorer.next().unwrap();
    }
    panic!("fixture face {face:?}, expected cylindrical seam edge");
}
fn prepare_common(case: &mut PCurveCase, target: &PaveBlockHandle) {
    let seam = seam_edge(&case.faces[0]);
    let structure = case.filler.data_structure.as_mut().unwrap();
    let index = structure.index(&seam);
    let source = structure.change_pave_blocks(index).unwrap()[0].clone();
    let (first, last) = target.lock().unwrap().indices();
    source.lock().unwrap().set_pave1(Pave::new(first, -1.));
    source.lock().unwrap().set_pave2(Pave::new(last, 1.));
    let mut common = CommonBlock::default();
    common.add_pave_block(target.clone());
    common.add_pave_block(source);
    structure.set_common_block(target, &Arc::new(Mutex::new(common)));
}
fn prepare_sections(case: &mut PCurveCase, block: &PaveBlockHandle) {
    let structure = case.filler.data_structure.as_mut().unwrap();
    let first = structure.index(&case.faces[0]);
    let second = structure.index(&case.faces[1]);
    for _ in 0..2 {
        let mut record = InterfFF::default();
        record.set_indices(first, second);
        record
            .change_curves()
            .appended()
            .change_pave_blocks()
            .extend([block.clone(), block.clone()]);
        structure.interf_ff().append(record);
    }
}
fn prepare_periodic_curve(case: &PCurveCase) {
    let curve: Handle<dyn Geom2dCurve> = Handle::new(Geom2dLine::from_point(
        Point2::new(4. * std::f64::consts::TAU, -1.),
        Direction2::new(0., 1.).unwrap(),
    ))
    .cast();
    let builder = BRepBuilder::default();
    builder
        .update_edge_pcurve_on_face(&case.edges[0], &curve, &case.faces[0], 1e-7)
        .unwrap();
    builder
        .range_on_face(&case.edges[0], &case.faces[0], 0., 2.)
        .unwrap();
}
