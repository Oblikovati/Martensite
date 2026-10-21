//! Whole surface-adaptor report, preserving original inputs and nested failure boundaries.
use super::brep_curve_fixture::{edge, projector};
use super::brep_face_fixture::face;
use super::brep_surface_print::ics_point;
use super::common::{Sink, attempt, dir, end, flag, int, pnt, real, show};
use super::geometry_records::common::Reader;
use martensite_tkernel::standard::Failure;
use martensite_tkgeomalgo::int_curve_surface::SamplePolyhedron;
use martensite_tkhlr::hlr_algo::{Projector, init_min_max};
use martensite_tkhlr::hlr_brep::{
    Curve, InterCSurf, SLProps, Surface, SurfaceTool, ThePolygonOfInterCSurf,
    ThePolyhedronOfInterCSurf,
};
use std::sync::Arc;

fn samples(tokens: &mut Sink, s: &Surface) -> Result<(), Failure> {
    int(tokens, SurfaceTool::nb_samples_u(s)?);
    int(tokens, SurfaceTool::nb_samples_v(s)?);
    Ok(())
}

fn surface_limits(s: &Surface) -> Result<[f64; 4], Failure> {
    Ok([
        SurfaceTool::first_u_parameter(s)?,
        SurfaceTool::last_u_parameter(s)?,
        SurfaceTool::first_v_parameter(s)?,
        SurfaceTool::last_v_parameter(s)?,
    ])
}

fn samples_in(tokens: &mut Sink, s: &Surface) -> Result<(), Failure> {
    let [u1, u2, v1, v2] = surface_limits(s)?;
    int(tokens, SurfaceTool::nb_samples_u_in(s, u1, u2)?);
    int(tokens, SurfaceTool::nb_samples_v_in(s, v1, v2)?);
    int(tokens, SurfaceTool::nb_samples_u_in(s, u1, (u1 + u2) / 3.)?);
    int(tokens, SurfaceTool::nb_samples_v_in(s, v1, (v1 + v2) / 3.)?);
    Ok(())
}

fn queries(tokens: &mut Sink, s: &Surface, tf: f64, tr: f64) {
    int(tokens, s.get_type() as i32);
    show(tokens, s.is_side(tf, tr), flag);
    attempt(tokens, |tokens| {
        let p = s.plane()?;
        pnt(tokens, p.location());
        dir(tokens, p.axis().direction());
        Ok(())
    });
    attempt(tokens, |tokens| samples(tokens, s));
    attempt(tokens, |tokens| samples_in(tokens, s));
}

fn slprops(tokens: &mut Sink, s: &Surface, u: f64, v: f64) {
    attempt(tokens, |tokens| {
        let mut props = SLProps::new(s, 2, 1e-9)?;
        props.set_parameters(u, v)?;
        pnt(tokens, props.value());
        let n = props.is_normal_defined()?;
        flag(tokens, n);
        if n {
            dir(tokens, props.normal()?);
        }
        curvature_report(tokens, &mut props)?;
        Ok(())
    });
}

fn curvature_report(tokens: &mut Sink, props: &mut SLProps<'_>) -> Result<(), Failure> {
    let c = props.is_curvature_defined()?;
    flag(tokens, c);
    if c {
        real(tokens, props.max_curvature()?);
        real(tokens, props.min_curvature()?);
    }
    Ok(())
}

fn print_inter(tokens: &mut Sink, i: &InterCSurf<'_>) -> Result<(), Failure> {
    flag(tokens, i.is_done());
    if !i.is_done() {
        return Ok(());
    }
    int(tokens, i.nb_points()?);
    for k in 1..=i.nb_points()? {
        ics_point(tokens, &i.point(k)?);
    }
    int(tokens, i.nb_segments()?);
    Ok(())
}

fn polyhedron_perform<'a>(
    tokens: &mut Sink,
    i: &mut InterCSurf<'a>,
    l: &martensite_tkmath::gp::Line3,
    s: &'a Surface,
    (pmin, pmax): (f64, f64),
) -> Result<(), Failure> {
    let [u1, u2, v1, v2] = surface_limits(s)?;
    let nu = SurfaceTool::nb_samples_u_in(s, u1, u2)?;
    let nv = SurfaceTool::nb_samples_v_in(s, v1, v2)?;
    let poly: ThePolyhedronOfInterCSurf =
        SamplePolyhedron::<SurfaceTool<'_>>::uniform(&s, nu, nv, [u1, v1, u2, v2])?;
    let polygon = ThePolygonOfInterCSurf::with_range(l, pmin, pmax, 3)?;
    real(tokens, poly.deflection_over_estimation());
    i.perform_polygon_polyhedron(l, &polygon, &s, &poly)
}

fn sight(tokens: &mut Sink, projector: &Projector, s: &Surface, r: &mut Reader) {
    let (x, y, mode, pmin, pmax) = (r.real(), r.real(), r.integer(), r.real(), r.real());
    attempt(tokens, |tokens| {
        let l = projector.shoot(x, y)?;
        let mut i = InterCSurf::new();
        if mode == 0 {
            i.perform(&l, &s)?;
        } else {
            polyhedron_perform(tokens, &mut i, &l, s, (pmin, pmax))?;
        }
        print_inter(tokens, &i)
    });
}

fn curve_above(
    tokens: &mut Sink,
    s: &Surface,
    e: &martensite_tkbrep::topods::Edge,
    projector: &Arc<Projector>,
    back: bool,
    tc: f64,
) -> Result<(), Failure> {
    let mut c = Curve::new();
    c.set_projector(Some(projector.clone()));
    c.set_curve(e)?;
    let (mut mn, mut mx) = ([0.; 16], [0.; 16]);
    init_min_max(1e100, &mut mn, &mut mx);
    attempt(tokens, |tokens| {
        c.update(&mut mn, &mut mx)?;
        flag(tokens, s.is_above(back, &c, tc)?);
        Ok(())
    });
    Ok(())
}

fn surface_body(
    tokens: &mut Sink,
    r: &mut Reader,
    projector: &Arc<Projector>,
) -> Result<(), Failure> {
    let f = face(r)?;
    let (tf, tr) = (r.real(), r.real());
    let (pu1, pv1, pu2, pv2) = (r.real(), r.real(), r.real(), r.real());
    let e = edge(r)?;
    let (back, tc) = (r.flag(), r.real());
    let mut s = Surface::new();
    s.set_projector(Some(projector.clone()));
    s.set_surface(&f)?;
    queries(tokens, &s, tf, tr);
    slprops(tokens, &s, pu1, pv1);
    slprops(tokens, &s, pu2, pv2);
    curve_above(tokens, &s, &e, projector, back, tc)?;
    for _ in 0..r.integer() {
        sight(tokens, projector, &s, r);
    }
    Ok(())
}

/// `projector face tolf toler u1 v1 u2 v2 edge back tolc nsight ...`: see
/// `hb_surface_case`.
pub fn surface_case(tokens: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(tokens, |tokens| {
        let p = Arc::new(projector(&mut r)?);
        surface_body(tokens, &mut r, &p)
    });
    end(tokens);
}
