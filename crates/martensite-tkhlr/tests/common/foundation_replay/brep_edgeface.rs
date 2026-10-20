//! Whole edge-face curvature/UV report with original query/error boundaries.
//! Source: `tkhlr_probe_hlrbrep_records`::hb_edgeface_case.
use super::brep_curve_fixture::projector;
use super::brep_face_fixture::face;
use super::brep_record_fixture::vertex_edge as edge_v;
use super::common::{Sink, attempt, end, flag, real, show};
use super::geometry_records::common::Reader;
use martensite_tkbrep::{
    topexp::Explorer,
    topods::{self, Edge},
};
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkhlr::{
    hlr_algo::Projector,
    hlr_brep::{BCurveTool, Curve, EdgeFaceTool, Surface, poly_hlr_angle_and_deflection},
};
use std::sync::Arc;

fn uv_of(
    tokens: &mut Sink,
    e: &Edge,
    s: &Surface,
    projector: &Arc<Projector>,
    par: f64,
) -> Result<(), Failure> {
    let mut c = Curve::new();
    c.set_projector(Some(projector.clone()));
    c.set_curve(e)?;
    let (a, b) = (
        BCurveTool::first_parameter(c.curve())?,
        BCurveTool::last_parameter(c.curve())?,
    );
    attempt(tokens, |tokens| {
        let uv = EdgeFaceTool::uv_point(a + (b - a) * par, &c, s)?;
        flag(tokens, uv.is_some());
        let (uu, vv) = uv.unwrap_or((-1., -1.));
        real(tokens, uu);
        real(tokens, vv);
        Ok(())
    });
    Ok(())
}

fn edgeface_body(
    tokens: &mut Sink,
    r: &mut Reader,
    projector: Arc<Projector>,
) -> Result<(), Failure> {
    let f = face(r)?;
    let (u, v, d, par) = (r.real(), r.real(), r.dir(), r.real());
    let free = edge_v(r)?;
    let mut s = Surface::new();
    s.set_projector(Some(projector.clone()));
    s.set_surface(&f)?;
    show(tokens, EdgeFaceTool::curvature_value(&s, u, v, d), real);
    let ex = Explorer::new(&f, ShapeEnum::Edge, ShapeEnum::Shape)?;
    let first = topods::edge(ex.current())?;
    for e in [first, free] {
        uv_of(tokens, &e, &s, &projector, par)?;
    }
    let (oa, od) = poly_hlr_angle_and_deflection(par);
    real(tokens, oa);
    real(tokens, od);
    Ok(())
}

/// `projector face u v dir par edge`: see `hb_edgeface_case`.
pub fn edgeface_case(tokens: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(tokens, |tokens| {
        let p = Arc::new(projector(&mut r)?);
        attempt(tokens, |tokens| edgeface_body(tokens, &mut r, p));
        Ok(())
    });
    end(tokens);
}
