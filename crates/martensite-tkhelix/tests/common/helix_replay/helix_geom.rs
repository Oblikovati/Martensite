//! HelixGeom probe operations of the helix curve and the tools (the builders are in helix_geom_builders.rs).
use super::common::{Reader, Sink, attempt, end, flag, int, pnt, real, xyz};
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dCurveBase};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomCircle, GeomCurve};
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkhelix::helix_geom::{HelixGeomHelixCurve, HelixGeomTools};
use martensite_tkmath::geom_abs::Shape;
use martensite_tkmath::gp::{Axis2, Vector3};

/// `hx_shape`: the GeomAbs_Shape of a code.
pub fn shape(code: i32) -> Shape {
    match code {
        0 => Shape::C0,
        1 => Shape::G1,
        2 => Shape::C1,
        3 => Shape::G2,
        4 => Shape::C2,
        5 => Shape::C3,
        _ => Shape::CN,
    }
}

/// The helix parameters `t1 t2 pitch r taper cw` of a request.
pub struct HelixFields {
    pub t1: f64,
    pub t2: f64,
    pub pitch: f64,
    pub r_start: f64,
    pub taper: f64,
    pub cw: bool,
}

/// Reads `t1 t2 pitch r taper cw`.
pub fn helix_fields(r: &mut Reader<'_>) -> HelixFields {
    let (t1, t2, pitch) = (r.real(), r.real(), r.real());
    let (r_start, taper, cw) = (r.real(), r.real(), r.flag());
    HelixFields {
        t1,
        t2,
        pitch,
        r_start,
        taper,
        cw,
    }
}

fn vec(out: &mut Sink, v: Vector3) {
    xyz(out, *v.xyz());
}

/// `hx_bspline`: null flag, degree, periodic, rational, poles (weights), knots and mults.
pub fn bspline(out: &mut Sink, c: &Handle<GeomBSplineCurve>) -> Result<(), Failure> {
    flag(out, c.is_null());
    let Some(c) = c.get() else { return Ok(()) };
    int(out, c.degree());
    flag(out, c.is_periodic());
    flag(out, c.is_rational());
    int(out, c.nb_poles());
    for i in 1..=c.nb_poles() {
        pnt(out, c.pole(i)?);
        if c.is_rational() {
            real(out, c.weight(i)?);
        }
    }
    int(out, c.nb_knots());
    for i in 1..=c.nb_knots() {
        real(out, c.knot(i)?);
        int(out, c.multiplicity(i)?);
    }
    Ok(())
}

/// `hx_curve`: the B-spline flag, the range and the B-spline data of a builder curve.
pub fn curve(out: &mut Sink, c: &Handle<dyn GeomCurve>) -> Result<(), Failure> {
    let b: Handle<GeomBSplineCurve> = c.cast();
    flag(out, !b.is_null());
    if let Some(c) = c.get() {
        real(out, c.first_parameter());
        real(out, c.last_parameter());
    }
    bspline(out, &b)
}

// hx_evaluate: D0, D1, D2, D3 (not implemented) and DN(u, n).
fn evaluate(out: &mut Sink, h: &HelixGeomHelixCurve, u: f64, n: i32) {
    attempt(out, |out| {
        pnt(out, h.value(u)?);
        Ok(())
    });
    attempt(out, |out| {
        let (p, v1) = h.d1(u)?;
        pnt(out, p);
        vec(out, v1);
        Ok(())
    });
    attempt(out, |out| {
        let (p, v1, v2) = h.d2(u)?;
        pnt(out, p);
        vec(out, v1);
        vec(out, v2);
        Ok(())
    });
    attempt(out, |out| {
        pnt(out, h.d3(u)?.0);
        Ok(())
    });
    attempt(out, |out| {
        vec(out, h.dn(u, n)?);
        Ok(())
    });
}

// hx_queries: range, continuity, intervals into [lo, up] and the raising members.
fn queries(out: &mut Sink, h: &HelixGeomHelixCurve, lo: i32, up: i32) -> Result<(), Failure> {
    real(out, h.first_parameter()?);
    real(out, h.last_parameter()?);
    int(out, h.continuity()? as i32);
    int(out, h.nb_intervals(Shape::C2)?);
    attempt(out, |out| {
        let mut t: Array1<'static, f64> =
            Array1::with_bounds(lo, up).map_err(|e| e.into_failure("NCollection_Array1"))?;
        t.init(-1.);
        h.intervals(&mut t, Shape::C0)?;
        t.iter().for_each(|x| real(out, *x));
        Ok(())
    });
    attempt(out, |out| h.resolution(1.).map(|value| real(out, value)));
    attempt(out, |out| h.is_closed().map(|value| flag(out, value)));
    attempt(out, |out| h.is_periodic().map(|value| flag(out, value)));
    attempt(out, |out| h.period().map(|value| real(out, value)));
    Ok(())
}

/// `nloads (mode helix)... lo up n nu (u)...`: loads, queries and evaluations.
pub fn curve_case(out: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    let h = HelixGeomHelixCurve::new();
    for _ in 0..r.integer() {
        let mode = r.integer();
        let f = helix_fields(&mut r);
        if mode == 0 {
            attempt(out, |_| {
                h.load(f.t1, f.t2, f.pitch, f.r_start, f.taper, f.cw)
            });
        } else {
            attempt(out, |_| h.load_current());
        }
    }
    let (lo, up, n, nu) = (r.integer(), r.integer(), r.integer(), r.integer());
    attempt(out, |out| queries(out, &h, lo, up));
    for _ in 0..nu {
        evaluate(out, &h, r.real(), n);
    }
    end(out);
}

// hx_adaptor: 0 a helix, 1 a GeomAdaptor_Curve on a circle, 2 the base class.
fn adaptor(r: &mut Reader<'_>) -> Result<Handle<dyn Adaptor3dCurve>, Failure> {
    let kind = r.integer();
    if kind == 0 {
        let f = helix_fields(r);
        let h = HelixGeomHelixCurve::new();
        h.load(f.t1, f.t2, f.pitch, f.r_start, f.taper, f.cw)?;
        return Ok(h.to_adaptor());
    }
    if kind == 1 {
        let (radius, first, last) = (r.real(), r.real(), r.real());
        let circle: Handle<dyn GeomCurve> =
            Handle::new(GeomCircle::new(Axis2::default(), radius)?).cast();
        return Ok(Handle::new(GeomAdaptorCurve::with_range(&circle, first, last)?).cast());
    }
    Ok(Handle::new(Adaptor3dCurveBase::default()).cast())
}

/// `adaptor tol cont maxseg maxdeg`: HelixGeom_Tools::ApprCurve3D.
pub fn appr_curve3d_case(out: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(out, |out| {
        let a = adaptor(&mut r)?;
        let (tol, cont, seg, deg) = (r.real(), r.integer(), r.integer(), r.integer());
        let (mut b, mut err): (Handle<GeomBSplineCurve>, f64) = (Handle::default(), -7.);
        let code = HelixGeomTools::appr_curve_3d(&a, tol, shape(cont), seg, deg, &mut b, &mut err)?;
        int(out, code);
        real(out, err);
        bspline(out, &b)
    });
    end(out);
}

/// `helix tol`: HelixGeom_Tools::ApprHelix.
pub fn appr_helix_case(out: &mut Sink, values: &[f64]) {
    let mut r = Reader::new(values);
    let f = helix_fields(&mut r);
    let tol = r.real();
    attempt(out, |out| {
        let (mut b, mut err): (Handle<GeomBSplineCurve>, f64) = (Handle::default(), -7.);
        let code = HelixGeomTools::appr_helix(
            f.t1, f.t2, f.pitch, f.r_start, f.taper, f.cw, tol, &mut b, &mut err,
        )?;
        int(out, code);
        real(out, err);
        bspline(out, &b)
    });
    end(out);
}
