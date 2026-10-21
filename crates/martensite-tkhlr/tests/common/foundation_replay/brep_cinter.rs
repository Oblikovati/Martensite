//! Whole projected-curve intersection report, preserving original query/error order.
use super::brep_curve_fixture::{edge, projector};
use super::brep_curve_print::domain;
use super::common::{Sink, attempt, end, flag, int, pnt2d, real, show};
use super::geometry_records::{common::Reader, intersection_report};
use martensite_tkernel::{ncollection::Array1, standard::Failure};
use martensite_tkhlr::{
    hlr_algo::init_min_max,
    hlr_brep::{CInter, Curve, CurveTool, TheProjPCurOfCInter},
};
use martensite_tkmath::gp::Point2;
use std::sync::Arc;

impl intersection_report::IntersectionEmitter for Sink {
    fn flag(&mut self, value: bool) {
        flag(self, value);
    }
    fn int(&mut self, value: i32) {
        int(self, value);
    }
    fn real(&mut self, value: f64) {
        real(self, value);
    }
    fn pnt2d(&mut self, value: Point2) {
        pnt2d(self, value);
    }
}

fn curve_tool(tokens: &mut Sink, c: &Curve) {
    show(tokens, CurveTool::nb_samples(c), int);
    attempt(tokens, |tokens| {
        let (first, last) = (
            CurveTool::first_parameter(c)?,
            CurveTool::last_parameter(c)?,
        );
        int(tokens, CurveTool::nb_samples_in(c, first, last)?);
        Ok(())
    });
    curve_intervals(tokens, c);
    int(tokens, CurveTool::the_type(c) as i32);
    flag(tokens, CurveTool::bezier(c).is_null());
    flag(tokens, CurveTool::bspline(c).is_null());
}

fn curve_intervals(tokens: &mut Sink, c: &Curve) {
    attempt(tokens, |tokens| {
        let n = CurveTool::nb_intervals(c)?;
        int(tokens, n);
        let mut tab = Array1::with_bounds(1, n + 1).expect("probe intervals");
        CurveTool::intervals(c, &mut tab)?;
        tab.iter().for_each(|v| real(tokens, *v));
        let (a, b) = CurveTool::get_interval(c, 1, &tab)?;
        real(tokens, a);
        real(tokens, b);
        Ok(())
    });
}

fn cinter(
    tokens: &mut Sink,
    c1: &Curve,
    c2: &Curve,
    tc: f64,
    t: f64,
    mode: i32,
) -> Result<(), Failure> {
    let mut i = CInter::new();
    let (d1, d2) = (i.compute_domain(c1, t)?, i.compute_domain(c2, t)?);
    domain(tokens, &d1)?;
    match mode {
        0 => i.perform(c1, &d1, c2, &d2, tc, t)?,
        1 => i.perform_between(c1, c2, tc, t)?,
        2 => i.perform_self_on(c1, &d1, tc, t)?,
        _ => i.perform_self(c1, tc, t)?,
    }
    intersection_report::intersection(tokens, &i)
}

fn updated_pair(r: &mut Reader) -> Result<(Curve, Curve), Failure> {
    let projector = Arc::new(projector(r)?);
    let (e1, e2) = (edge(r)?, edge(r)?);
    let (mut c1, mut c2) = (Curve::new(), Curve::new());
    c1.set_projector(Some(projector.clone()));
    c1.set_curve(&e1)?;
    c2.set_projector(Some(projector));
    c2.set_curve(&e2)?;
    Ok((c1, c2))
}

fn projected_middle(tokens: &mut Sink, c1: &Curve, c2: &Curve, t: f64) -> Result<(), Failure> {
    let (first, last) = (
        CurveTool::first_parameter(c1)?,
        CurveTool::last_parameter(c1)?,
    );
    let p = CurveTool::value(c1, (first + last) / 2.)?;
    real(tokens, TheProjPCurOfCInter::find_parameter(c2, p, t)?);
    Ok(())
}

fn updated_intersection(
    tokens: &mut Sink,
    c1: &mut Curve,
    c2: &mut Curve,
    tc: f64,
    t: f64,
    mode: i32,
) {
    let (mut mn, mut mx) = ([0.; 16], [0.; 16]);
    init_min_max(1e100, &mut mn, &mut mx);
    attempt(tokens, |tokens| {
        c1.update(&mut mn, &mut mx)?;
        c2.update(&mut mn, &mut mx)?;
        curve_tool(tokens, c1);
        curve_tool(tokens, c2);
        attempt(tokens, |tokens| projected_middle(tokens, c1, c2, t));
        attempt(tokens, |tokens| cinter(tokens, c1, c2, tc, t, mode));
        Ok(())
    });
}

/// Report both tools, point projection and one of four intersection modes;
/// `cinter_case(tokens, &[projector_fields, edge_fields, tolconf, tol, mode])`.
pub fn cinter_case(tokens: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(tokens, |tokens| {
        let (mut c1, mut c2) = updated_pair(&mut r)?;
        let (tc, t, mode) = (r.real(), r.real(), r.integer());
        updated_intersection(tokens, &mut c1, &mut c2, tc, t, mode);
        Ok(())
    });
    end(tokens);
}
