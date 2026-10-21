//! Whole edge record, flag/status and projected edge-conversion report.
//! Source: `tkhlr_probe_hlrbrep_records`::hb_edgedata_case.
use super::brep_curve_fixture::projector;
use super::brep_record_fixture::vertex_edge as edge_v;
use super::brep_record_print::{edge_flags, edge_summary};
use super::common::{Sink, attempt, end};
use super::geometry_records::common::Reader;
use martensite_tkernel::standard::Failure;
use martensite_tkhlr::{
    hlr_algo::{Projector, init_min_max},
    hlr_brep::{BCurveTool, Curve, EdgeData, make_edge, make_edge_3d},
};
use std::sync::Arc;

fn edge_conversions(tokens: &mut Sink, c: &Curve) -> Result<(), Failure> {
    let (a, b) = (
        BCurveTool::first_parameter(c.curve())?,
        BCurveTool::last_parameter(c.curve())?,
    );
    attempt(tokens, |tokens| edge_summary(tokens, &make_edge(c, a, b)?));
    attempt(tokens, |tokens| {
        edge_summary(tokens, &make_edge(c, a + (b - a) / 4., b - (b - a) / 3.)?)
    });
    attempt(tokens, |tokens| {
        edge_summary(tokens, &make_edge_3d(c, a, b)?)
    });
    attempt(tokens, |tokens| {
        edge_summary(tokens, &make_edge_3d(c, a + (b - a) / 4., b)?)
    });
    Ok(())
}

fn mutate_flags(d: &mut EdgeData, flags: [bool; 3]) {
    d.set_vertical(flags[0]);
    d.set_simple(flags[1]);
    d.set_auto_intersection_done(flags[2]);
    d.set_used(true);
    d.set_hide_count(4);
    d.set_selected(false);
    d.set_ver_at_sta(true);
    d.set_ver_at_end(flags[2]);
}

fn update_geometry(tokens: &mut Sink, d: &mut EdgeData, projector: Arc<Projector>) {
    let c = d.change_geometry();
    c.set_projector(Some(projector));
    let (mut mn, mut mx) = ([0.; 16], [0.; 16]);
    init_min_max(1e100, &mut mn, &mut mx);
    attempt(tokens, |tokens| {
        c.update(&mut mn, &mut mx)?;
        edge_conversions(tokens, c)
    });
}

fn edgedata_body(
    tokens: &mut Sink,
    r: &mut Reader,
    projector: Arc<Projector>,
) -> Result<(), Failure> {
    let edge = edge_v(r)?;
    let mut f = [false; 9];
    f.iter_mut().for_each(|b| *b = r.flag());
    let (s, ts, e, te) = (r.real(), r.real() as f32, r.real(), r.real() as f32);
    let mut d = EdgeData::new();
    edge_flags(tokens, &d, false);
    d.set(
        f[0], f[1], &edge, 3, 7, f[2], f[3], f[4], f[5], s, ts, e, te,
    )?;
    edge_flags(tokens, &d, true);
    mutate_flags(&mut d, [f[6], f[7], f[8]]);
    edge_flags(tokens, &d, true);
    update_geometry(tokens, &mut d, projector);
    Ok(())
}

/// `projector edge flags start tolstart end tolend`: see `hb_edgedata_case`.
pub fn edgedata_case(tokens: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(tokens, |tokens| {
        let p = Arc::new(projector(&mut r)?);
        attempt(tokens, |tokens| edgedata_body(tokens, &mut r, p));
        Ok(())
    });
    end(tokens);
}
