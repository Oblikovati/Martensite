//! Curve adaptor input decoding and summary encoding.
use super::common::{Reader, attempt, ax1, ax2, flag, int, pnt, real, show, vec};
use super::output::Output;
use super::shapes;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped, Transient};
use martensite_tkg2d::adaptor2d::{Adaptor2dCurve2d, Adaptor2dLine2d};
use martensite_tkg2d::geom2d::{
    Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dCircle, Geom2dCurve, Geom2dEllipse,
    Geom2dHyperbola, Geom2dLine, Geom2dParabola, Geom2dTrimmedCurve,
};
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkg3d::adaptor3d::{
    Adaptor3dCurve, Adaptor3dCurveOnSurface, Adaptor3dSurface, Adaptor3dSurfaceBase,
};
use martensite_tkg3d::geom_adaptor::{
    GeomAdaptorCurve, GeomAdaptorSurface, GeomAdaptorSurfaceOfLinearExtrusion,
};
use martensite_tkmath::geom_abs::{CurveType, Shape, SurfaceType};
use martensite_tkmath::gp::{Axis2d, Circle3, Direction2, Point2};

pub fn pnt2(r: &mut Reader) -> Point2 {
    let x = r.real();
    Point2::new(x, r.real())
}
pub fn dir2(r: &mut Reader) -> Direction2 {
    let x = r.real();
    Direction2::new(x, r.real()).expect("probe 2D direction")
}

/// One 2D curve specification (a3_curve2d).
pub fn curve2d(output: &Output, r: &mut Reader) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let kind = r.integer();
    match kind {
        0 => {
            let p = pnt2(r);
            Ok(Handle::new(Geom2dLine::from_point(p, dir2(r))).cast())
        }
        1..=4 => conic2d(kind, r),
        5 => bezier2d(output, r),
        6 => bspline2d(output, r),
        7 => {
            let (u1, u2) = (r.real(), r.real());
            let basis = curve2d(output, r)?;
            Ok(Handle::new(Geom2dTrimmedCurve::new(&basis, u1, u2, true, true)?).cast())
        }
        _ => panic!("2D curve kind {kind} has no builder"),
    }
}

fn conic2d(kind: i32, r: &mut Reader) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let p = pnt2(r);
    let axis = Axis2d::new(p, dir2(r));
    let (sense, x) = (r.flag(), r.real());
    match kind {
        1 => Ok(Handle::new(Geom2dCircle::from_axis(axis, x, sense)?).cast()),
        4 => Ok(Handle::new(Geom2dParabola::from_axis(axis, x, sense)?).cast()),
        2 => Ok(Handle::new(Geom2dEllipse::from_axis(axis, x, r.real(), sense)?).cast()),
        _ => Ok(Handle::new(Geom2dHyperbola::from_axis(axis, x, r.real(), sense)?).cast()),
    }
}

fn poles2d(
    _output: &Output,
    r: &mut Reader,
    n: i32,
) -> (Array1<'static, Point2>, Array1<'static, f64>) {
    let mut poles: Array1<'static, Point2> = Array1::with_bounds(1, n).unwrap();
    let mut weights: Array1<'static, f64> = Array1::with_bounds(1, n).unwrap();
    for i in 1..=n {
        poles.set_value(i, pnt2(r)).unwrap();
        weights.set_value(i, r.real()).unwrap();
    }
    (poles, weights)
}

fn bezier2d(output: &Output, r: &mut Reader) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let (rational, n) = (r.flag(), r.integer());
    let (poles, weights) = poles2d(output, r, n);
    let curve = match rational {
        true => Geom2dBezierCurve::with_weights(&poles, &weights)?,
        false => Geom2dBezierCurve::new(&poles)?,
    };
    Ok(Handle::new(curve).cast())
}

fn bspline2d(output: &Output, r: &mut Reader) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let (rational, periodic) = (r.flag(), r.flag());
    let (degree, nk) = (r.integer(), r.integer());
    let mut knots: Array1<'static, f64> = Array1::with_bounds(1, nk).unwrap();
    let mut mults: Array1<'static, i32> = Array1::with_bounds(1, nk).unwrap();
    for i in 1..=nk {
        knots.set_value(i, r.real()).unwrap();
        mults.set_value(i, r.integer()).unwrap();
    }
    let np = r.integer();
    let (poles, weights) = poles2d(output, r, np);
    let curve = match rational {
        true => {
            Geom2dBSplineCurve::with_weights(&poles, &weights, &knots, &mults, degree, periodic)?
        }
        false => Geom2dBSplineCurve::new(&poles, &knots, &mults, degree, periodic)?,
    };
    Ok(Handle::new(curve).cast())
}

/// One 2D adaptor specification (a3_adaptor2d).
pub fn adaptor2d(output: &Output, r: &mut Reader) -> Result<Handle<dyn Adaptor2dCurve2d>, Failure> {
    match r.integer() {
        0 => Ok(Handle::new(Geom2dAdaptorCurve::with_curve(&curve2d(output, r)?)?).cast()),
        1 => {
            let (u1, u2) = (r.real(), r.real());
            Ok(Handle::new(Geom2dAdaptorCurve::with_range(
                &curve2d(output, r)?,
                u1,
                u2,
            )?)
            .cast())
        }
        _ => {
            let (p, d) = (pnt2(r), dir2(r));
            let (u1, u2) = (r.real(), r.real());
            Ok(Handle::new(Adaptor2dLine2d::new(p, d, u1, u2)).cast())
        }
    }
}

/// One surface adaptor specification (a3_surface).
pub fn surface(output: &Output, r: &mut Reader) -> Result<Handle<dyn Adaptor3dSurface>, Failure> {
    match r.integer() {
        0 => Ok(
            Handle::new(GeomAdaptorSurface::with_surface(&shapes::surface(
                output, r,
            )?)?)
            .cast(),
        ),
        1 => {
            let b: Vec<f64> = (0..6).map(|_| r.real()).collect();
            let range = ((b[0], b[1], b[2], b[3]), (b[4], b[5]));
            Ok(Handle::new(GeomAdaptorSurface::with_range(
                &shapes::surface(output, r)?,
                range,
            )?)
            .cast())
        }
        2 => Ok(Handle::new(Adaptor3dSurfaceBase::default()).cast()),
        _ => {
            let d = r.dir();
            let curve: Handle<dyn Adaptor3dCurve> =
                Handle::new(GeomAdaptorCurve::with_curve(&shapes::curve(output, r)?)?).cast();
            Ok(Handle::new(
                GeomAdaptorSurfaceOfLinearExtrusion::with_curve_and_direction(&curve, d),
            )
            .cast())
        }
    }
}

/// Reads a 2D adaptor and a surface adaptor, reporting a construction failure.
pub fn inputs(
    output: &Output,
    r: &mut Reader,
) -> Option<(Handle<dyn Adaptor2dCurve2d>, Handle<dyn Adaptor3dSurface>)> {
    let built = adaptor2d(output, r).and_then(|curve| Ok((curve, surface(output, r)?)));
    match built {
        Ok(pair) => Some(pair),
        Err(error) => {
            replay_println!(output, "{}", error.exception_type());
            None
        }
    }
}

pub fn circ(output: &Output, c: Circle3) {
    ax2(output, c.position());
    real(output, c.radius());
}

fn shape_of(index: i32) -> Shape {
    [
        Shape::C0,
        Shape::G1,
        Shape::C1,
        Shape::G2,
        Shape::C2,
        Shape::C3,
        Shape::CN,
    ][index as usize]
}

/// NbIntervals and Intervals for every continuity (a3_curve_intervals).
pub fn curve_intervals(output: &Output, a: &dyn Adaptor3dCurve) {
    for s in 0..=6 {
        let n = match a.nb_intervals(shape_of(s)) {
            Ok(n) => n,
            Err(e) => {
                replay_print!(output, "{} ", e.exception_type());
                continue;
            }
        };
        int(output, n);
        let mut t: Array1<'static, f64> = Array1::with_bounds(1, n + 1).unwrap();
        attempt(output, || {
            a.intervals(&mut t, shape_of(s))?;
            t.as_slice().iter().for_each(|v| real(output, *v));
            Ok(())
        });
    }
}

/// Whether a spline query would dereference a null 2D spline natively (a3_null_spline):
/// a curve on a plane whose 2D Geom2dAdaptorCurve is of another kind.
fn null_spline(_output: &Output, a: &dyn Adaptor3dCurve, kind: CurveType) -> bool {
    let transient: &dyn Transient = a;
    let Some(cos) = Adaptor3dCurveOnSurface::project_from(transient) else {
        return false;
    };
    let (curve, surface) = (cos.get_curve(), cos.get_surface());
    if surface.get_type().ok() != Some(SurfaceType::Plane) {
        return false;
    }
    curve.get_type().ok() != Some(kind) && curve.cast::<Adaptor2dLine2d>().is_null()
}

/// The kind queries (a3_curve_kinds).
pub fn curve_kinds(output: &Output, a: &dyn Adaptor3dCurve) {
    show(output, a.get_type(), |k| int(output, k as i32));
    show(output, a.line(), |l| ax1(output, l.position()));
    show(output, a.circle(), |value| circ(output, value));
    show(output, a.ellipse(), |e| {
        ax2(output, e.position());
        real(output, e.major_radius());
        real(output, e.minor_radius());
    });
    show(output, a.hyperbola(), |h| {
        ax2(output, h.position());
        real(output, h.major_radius());
        real(output, h.minor_radius());
    });
    show(output, a.parabola(), |p| {
        ax2(output, p.position());
        real(output, p.focal());
    });
    show(output, a.degree(), |value| int(output, value));
    show(output, a.is_rational(), |value| flag(output, value));
    show(output, a.nb_poles(), |value| int(output, value));
    show(output, a.nb_knots(), |value| int(output, value));
    attempt(output, || {
        if null_spline(output, a, CurveType::BezierCurve) {
            return Ok(());
        }
        let b = a.bezier()?;
        super::common::name(output, &b);
        if let Some(b) = b.get() {
            flag(output, b.is_rational());
            for i in 1..=b.nb_poles() {
                pnt(output, b.pole(i)?);
                real(output, b.weight(i)?);
            }
        }
        Ok(())
    });
    attempt(output, || {
        if null_spline(output, a, CurveType::BSplineCurve) {
            return Ok(());
        }
        let b = a.bspline()?;
        super::common::name(output, &b);
        if let Some(b) = b.get() {
            use martensite_tkg3d::geom::GeomCurve;
            flag(output, b.is_rational());
            flag(output, b.is_periodic());
            int(output, b.degree());
            for i in 1..=b.nb_knots() {
                real(output, b.knot(i)?);
                int(output, b.multiplicity(i)?);
            }
            for i in 1..=b.nb_poles() {
                pnt(output, b.pole(i)?);
                real(output, b.weight(i)?);
            }
        }
        Ok(())
    });
}

/// The evaluators at one parameter (a3_curve_evaluations).
pub fn curve_evaluations(output: &Output, a: &dyn Adaptor3dCurve, u: f64) {
    show(output, a.eval_d0(u), |value| pnt(output, value));
    show(output, a.eval_d1(u), |e| {
        pnt(output, e.point);
        vec(output, e.d1);
    });
    show(output, a.eval_d2(u), |e| {
        pnt(output, e.point);
        vec(output, e.d1);
        vec(output, e.d2);
    });
    show(output, a.eval_d3(u), |e| {
        pnt(output, e.point);
        vec(output, e.d1);
        vec(output, e.d2);
        vec(output, e.d3);
    });
    for n in 0..=4 {
        show(output, a.eval_dn(u, n), |value| vec(output, value));
    }
    show(output, a.value(u), |value| pnt(output, value));
    show(output, a.d0(u), |value| pnt(output, value));
    show(output, a.d1(u), |(p, v)| {
        pnt(output, p);
        vec(output, v);
    });
    show(output, a.d2(u), |(p, v1, v2)| {
        pnt(output, p);
        vec(output, v1);
        vec(output, v2);
    });
    show(output, a.d3(u), |(p, v1, v2, v3)| {
        pnt(output, p);
        vec(output, v1);
        vec(output, v2);
        vec(output, v3);
    });
    show(output, a.dn(u, 2), |value| vec(output, value));
}

/// The curve-level summary (a3_curve_summary).
pub fn curve_summary(output: &Output, a: &dyn Adaptor3dCurve, params: &[f64]) {
    replay_print!(output, "{} ", a.dynamic_type().name());
    show(output, a.first_parameter(), |value| real(output, value));
    show(output, a.last_parameter(), |value| real(output, value));
    show(output, a.continuity(), |c| int(output, c as i32));
    curve_intervals(output, a);
    curve_intervals(output, a);
    show(output, a.is_closed(), |value| flag(output, value));
    show(output, a.is_periodic(), |value| flag(output, value));
    show(output, a.period(), |value| real(output, value));
    for r in [1e-7, 1e-3, 0.5] {
        show(output, a.resolution(r), |value| real(output, value));
    }
    curve_kinds(output, a);
    attempt(output, || {
        curve_evaluations(output, a, a.first_parameter()?);
        Ok(())
    });
    attempt(output, || {
        curve_evaluations(output, a, a.last_parameter()?);
        Ok(())
    });
    for u in params {
        curve_evaluations(output, a, *u);
    }
}
