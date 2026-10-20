//! Complete outlined topology operation, preserving native report and failure order.
use super::common::{Sink, attempt, end, flag, int};
use super::geometry_records::common::Reader;
use super::{
    brep_curve_fixture::projector, brep_record_print::edge_summary, brep_shape_fixture::shape,
};
use martensite_tkbrep::{
    topexp::{self, Explorer},
    topods::{self, Edge, Shape},
};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkhlr::hlr_topo_brep::{Data, MapOfShapeTool, OutLiner};

fn explored(source: &Shape, kind: ShapeEnum) -> Result<Vec<Shape>, Failure> {
    let mut shapes = Vec::new();
    let mut explorer = Explorer::new(source, kind, ShapeEnum::Shape)?;
    while explorer.more() {
        shapes.push(explorer.current().clone());
        explorer.next()?;
    }
    Ok(shapes)
}
fn shape_edges(tokens: &mut Sink, source: &Shape) -> Result<(), Failure> {
    let edges = explored(source, ShapeEnum::Edge)?;
    int(tokens, edges.len() as i32);
    for edge in &edges {
        int(tokens, edge.orientation() as i32);
        attempt(tokens, |tokens| edge_summary(tokens, &topods::edge(edge)?));
    }
    Ok(())
}
fn shape_topology(tokens: &mut Sink, source: &Shape) -> Result<(), Failure> {
    for face in explored(source, ShapeEnum::Face)? {
        let wires = explored(&face, ShapeEnum::Wire)?;
        for wire in &wires {
            int(tokens, explored(wire, ShapeEnum::Edge)?.len() as i32);
        }
        int(tokens, -(wires.len() as i32));
    }
    Ok(())
}
fn face_records(tokens: &mut Sink, records: &Data, source: &Shape) -> Result<(), Failure> {
    for face in explored(source, ShapeEnum::Face)? {
        let face = topods::face(&face)?;
        flag(tokens, records.face_has_int_l(&face));
        flag(tokens, records.face_has_out_l(&face));
        flag(tokens, records.face_has_iso_l(&face));
        attempt(tokens, |tokens| {
            int(tokens, records.face_int_l(&face)?.extent());
            int(tokens, records.face_out_l(&face)?.extent());
            int(tokens, records.face_iso_l(&face)?.extent());
            Ok(())
        });
    }
    Ok(())
}
fn vertex_records(tokens: &mut Sink, records: &Data, edge: &Edge) {
    let (first, last) = topexp::vertices(edge, false);
    for vertex in [first, last] {
        if !vertex.is_null() {
            flag(tokens, records.is_out_v(&vertex));
            flag(tokens, records.is_int_v(&vertex));
        }
    }
}
fn edge_records(tokens: &mut Sink, records: &Data, source: &Shape) -> Result<(), Failure> {
    for edge in explored(source, ShapeEnum::Edge)? {
        let edge = topods::edge(&edge)?;
        flag(tokens, records.edge_has_spl_e(&edge));
        attempt(tokens, |tokens| {
            int(tokens, records.edge_spl_e(&edge)?.extent());
            Ok(())
        });
        vertex_records(tokens, records, &edge);
    }
    Ok(())
}
fn outlined_shape(
    tokens: &mut Sink,
    reader: &mut Reader,
    view: &martensite_tkhlr::hlr_algo::Projector,
) -> Result<(), Failure> {
    let source = shape(reader)?;
    let nb_iso = reader.integer();
    let outliner = Handle::new(OutLiner::with_original(&source));
    let mut tools = MapOfShapeTool::default();
    outliner.fill(view, &mut tools, nb_iso)?;
    let outlined = outliner.out_lined_shape();
    flag(tokens, outlined.is_null());
    shape_topology(tokens, &outlined)?;
    shape_edges(tokens, &outlined)?;
    let records = outliner.data_structure();
    face_records(tokens, &records, &source)?;
    edge_records(tokens, &records, &source)
}
fn outliner_body(tokens: &mut Sink, reader: &mut Reader) -> Result<(), Failure> {
    let view = projector(reader)?;
    attempt(tokens, |tokens| outlined_shape(tokens, reader, &view));
    Ok(())
}
/// Replay projector, shape and isoline count; `outliner_case(tokens, fields)`.
pub fn outliner_case(tokens: &mut Sink, fields: &[f64]) {
    let mut reader = Reader::new(fields);
    attempt(tokens, |tokens| outliner_body(tokens, &mut reader));
    end(tokens);
}
