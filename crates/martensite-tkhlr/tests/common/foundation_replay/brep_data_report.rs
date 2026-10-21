//! Initialized records and interference reports with original query/failure ordering.
use super::common::{Sink, attempt, flag, int, real};
use super::{
    brep_face_record::face_boolean_flags,
    brep_record_print::edge_flags,
    hlr_algo::{print_interference_enums, print_intersection, print_minmax as minmax},
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::State;
use martensite_tkhlr::{
    hlr_algo::{Interference, Intersection},
    hlr_brep::{Data, EdgeData, FaceData, LocalGeometry2D},
};

pub(super) fn data_edge(tokens: &mut Sink, edge: &EdgeData, boxes: bool) {
    edge_flags(tokens, edge, true);
    if boxes {
        minmax(tokens, edge.min_max());
    }
}
pub(super) fn data_face(tokens: &mut Sink, face: &FaceData) -> Result<(), Failure> {
    face_boolean_flags(tokens, face);
    real(tokens, face.size());
    int(tokens, face.orientation() as i32);
    real(tokens, f64::from(face.tolerance()));
    let wires = face.wires().get().ok_or_else(|| Failure::new("no wires"))?;
    int(tokens, wires.nb_wires());
    minmax(tokens, &wires.min_max());
    for index in 1..=wires.nb_wires() {
        let block = wires.wire(index)?;
        wire_edges(tokens, block.get().ok_or_else(|| Failure::new("no edges"))?)?;
    }
    Ok(())
}
fn wire_edges(
    tokens: &mut Sink,
    block: &martensite_tkhlr::hlr_algo::EdgesBlock,
) -> Result<(), Failure> {
    int(tokens, block.nb_edges());
    minmax(tokens, &block.min_max());
    for edge in 1..=block.nb_edges() {
        int(tokens, block.edge(edge)?);
        int(tokens, block.orientation(edge)? as i32);
        for value in [
            block.out_line(edge)?,
            block.internal(edge)?,
            block.double(edge)?,
            block.iso_line(edge)?,
        ] {
            flag(tokens, value);
        }
    }
    Ok(())
}
fn local2d(tokens: &mut Sink, (tangent, normal, curvature): LocalGeometry2D) {
    real(tokens, tangent.x());
    real(tokens, tangent.y());
    real(tokens, normal.x());
    real(tokens, normal.y());
    real(tokens, curvature);
}
fn boundary(tokens: &mut Sink, item: &Interference, intersection: &Intersection) -> (i32, f64) {
    let (edge, parameter) = item.boundary().value_2d();
    int(tokens, edge);
    real(tokens, parameter);
    if intersection.state() == State::On {
        let (before, after) = item.boundary().state_3d();
        int(tokens, before as i32);
        int(tokens, after as i32);
    }
    (edge, parameter)
}
fn local_geometry(
    tokens: &mut Sink,
    records: &Data,
    intersection: &Intersection,
    edge: i32,
    parameter: f64,
) {
    attempt(tokens, |tokens| {
        local2d(
            tokens,
            records.local_le_geometry_2d(intersection.parameter())?,
        );
        Ok(())
    });
    attempt(tokens, |tokens| {
        local2d(tokens, records.local_fe_geometry_2d(edge, parameter)?);
        Ok(())
    });
}
pub(super) fn interference(tokens: &mut Sink, records: &Data, item: &Interference) {
    let intersection = item.intersection();
    int(tokens, intersection.orientation() as i32);
    print_intersection(tokens, intersection, false);
    int(tokens, intersection.state() as i32);
    let (edge, parameter) = boundary(tokens, item, intersection);
    print_interference_enums(tokens, item);
    local_geometry(tokens, records, intersection, edge, parameter);
}
