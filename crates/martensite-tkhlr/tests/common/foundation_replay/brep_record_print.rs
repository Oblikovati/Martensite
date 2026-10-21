//! One ordered edge summary/record report for owned Sink and legacy stdout.
use super::common::{Sink, attempt, flag, int, pnt, real};
use martensite_tkbrep::{
    brep::BRepTool,
    brep_adaptor::BRepAdaptorCurve,
    topexp,
    topods::{self, Edge, Shape},
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::{adaptor3d::Adaptor3dCurve, top_abs::ShapeEnum};
use martensite_tkhlr::hlr_brep::EdgeData;
use martensite_tkmath::gp::Point3;

pub(crate) trait RecordEmitter {
    fn flag(&mut self, value: bool);
    fn int(&mut self, value: i32);
    fn real(&mut self, value: f64);
    fn pnt(&mut self, value: Point3);
    fn attempt(&mut self, body: impl FnOnce(&mut Self) -> Result<(), Failure>);
}
impl RecordEmitter for Sink {
    fn flag(&mut self, value: bool) {
        flag(self, value);
    }
    fn int(&mut self, value: i32) {
        int(self, value);
    }
    fn real(&mut self, value: f64) {
        real(self, value);
    }
    fn pnt(&mut self, value: Point3) {
        pnt(self, value);
    }
    fn attempt(&mut self, body: impl FnOnce(&mut Self) -> Result<(), Failure>) {
        attempt(self, body);
    }
}

/// Report vertices and sampled geometry with the original inner failure boundary;
/// `edge_summary(emit, &edge)`.
pub fn edge_summary(emit: &mut impl RecordEmitter, edge: &Edge) -> Result<(), Failure> {
    emit.flag(edge.is_null());
    if edge.is_null() {
        return Ok(());
    }
    let (first, last) = topexp::vertices(edge, false);
    for vertex in [first, last] {
        emit.flag(vertex.is_null());
        if !vertex.is_null() {
            emit.pnt(BRepTool::pnt(&vertex)?);
        }
    }
    emit.attempt(|emit| curve_summary(emit, edge));
    Ok(())
}
fn curve_summary(emit: &mut impl RecordEmitter, edge: &Edge) -> Result<(), Failure> {
    let curve = BRepAdaptorCurve::with_edge(edge)?;
    let (first, last) = (curve.first_parameter()?, curve.last_parameter()?);
    emit.real(first);
    emit.real(last);
    emit.int(curve.get_type()? as i32);
    for parameter in [first, (first + last) / 2., last] {
        emit.pnt(curve.value(parameter)?);
    }
    Ok(())
}

fn edge_boolean_flags(emit: &mut impl RecordEmitter, record: &EdgeData) {
    for value in [
        record.selected(),
        record.rg1_line(),
        record.rgn_line(),
        record.vertical(),
        record.simple(),
        record.out_lv_sta(),
        record.out_lv_end(),
        record.cut_at_sta(),
        record.cut_at_end(),
        record.ver_at_sta(),
        record.ver_at_end(),
        record.auto_intersection_done(),
        record.used(),
    ] {
        emit.flag(value);
    }
}

// Native vertex indices, tolerance and status are uninitialized before Set;
// preserve the explicit full=false protocol instead of reading those values.
/// Report initialized flags and optionally initialized bounds; `edge_flags(emit, &record, false)`.
pub fn edge_flags(emit: &mut impl RecordEmitter, record: &EdgeData, full: bool) {
    edge_boolean_flags(emit, record);
    emit.int(record.hide_count());
    if !full {
        return;
    }
    emit.int(record.v_sta());
    emit.int(record.v_end());
    emit.real(f64::from(record.tolerance()));
    let (start, start_tol, end, end_tol) = record.status().bounds();
    emit.real(start);
    emit.real(f64::from(start_tol));
    emit.real(end);
    emit.real(f64::from(end_tol));
}

/// Report the original compound edge order and failure boundaries; `compound(emit, &shape)`.
pub fn compound(emit: &mut impl RecordEmitter, result: &Shape) -> Result<(), Failure> {
    emit.flag(result.is_null());
    if result.is_null() {
        return Ok(());
    }
    let edges: Vec<Shape> =
        topexp::Explorer::new(result, ShapeEnum::Edge, ShapeEnum::Shape)?.collect();
    emit.int(edges.len() as i32);
    for edge in &edges {
        emit.attempt(|emit| edge_summary(emit, &topods::edge(edge)?));
    }
    Ok(())
}
