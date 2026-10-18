//! The ExtremaPC2d and TangentialDeflection requests; the counterpart of
//! the implementation (same curve encoding and token order).
use super::common::{Reader, attempt, end, flag, int, pnt, pnt2d, real};
use super::fixture_input::GeometryFixtureInput;
use super::print::Output;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg2d::geom2d::{
    Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dCircle, Geom2dCurve, Geom2dEllipse,
    Geom2dHyperbola, Geom2dLine, Geom2dOffsetCurve, Geom2dParabola, Geom2dTrimmedCurve,
};
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkg2d::geom2d_eval::Geom2dEvalSineWaveCurve;
use martensite_tkgeombase::extrema_pc_core::{PcResult, SearchMode};
use martensite_tkgeombase::extrema_pc2d::{
    self as pc2d, Curve, DistanceFunction, Domain1D, ExtremaResult, GridEvaluator,
};
use martensite_tkmath::gp::{Axis22d, Point2};
use martensite_tkmath::math::Vector;

fn bezier(r: &mut impl GeometryFixtureInput) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let rational = r.flag();
    let n = r.integer();
    let mut p = Array1::<Point2>::with_bounds(1, n).unwrap();
    let mut w = Array1::<f64>::with_bounds(1, n).unwrap();
    for i in 1..=n {
        p[i] = r.pnt2d();
        w[i] = r.real();
    }
    let curve = if rational {
        Geom2dBezierCurve::with_weights(&p, &w)?
    } else {
        Geom2dBezierCurve::new(&p)?
    };
    Ok(Handle::new(curve).cast())
}

fn bspline(r: &mut impl GeometryFixtureInput) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let (rational, periodic) = (r.flag(), r.flag());
    let (degree, nk) = (r.integer(), r.integer());
    let mut k = Array1::<f64>::with_bounds(1, nk).unwrap();
    let mut m = Array1::<i32>::with_bounds(1, nk).unwrap();
    for i in 1..=nk {
        k[i] = r.real();
        m[i] = r.integer();
    }
    let np = r.integer();
    let mut p = Array1::<Point2>::with_bounds(1, np).unwrap();
    let mut w = Array1::<f64>::with_bounds(1, np).unwrap();
    for i in 1..=np {
        p[i] = r.pnt2d();
        w[i] = r.real();
    }
    let curve = if rational {
        Geom2dBSplineCurve::with_weights(&p, &w, &k, &m, degree, periodic)?
    } else {
        Geom2dBSplineCurve::new(&p, &k, &m, degree, periodic)?
    };
    Ok(Handle::new(curve).cast())
}

fn conic(kind: i32, r: &mut impl GeometryFixtureInput) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let a = r.ax2d();
    let sense = r.flag();
    let x = r.real();
    if kind == 1 {
        return Ok(Handle::new(Geom2dCircle::from_axis(a, x, sense)?).cast());
    }
    if kind == 4 {
        return Ok(Handle::new(Geom2dParabola::from_axis(a, x, sense)?).cast());
    }
    let y = r.real();
    if kind == 2 {
        return Ok(Handle::new(Geom2dEllipse::from_axis(a, x, y, sense)?).cast());
    }
    Ok(Handle::new(Geom2dHyperbola::from_axis(a, x, y, sense)?).cast())
}

/// One curve specification, as gb_curve2d reads it.
pub fn curve2d(r: &mut impl GeometryFixtureInput) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let kind = r.integer();
    match kind {
        0 => {
            let p = r.pnt2d();
            Ok(Handle::new(Geom2dLine::from_point(p, r.dir2d())).cast())
        }
        1..=4 => conic(kind, r),
        5 => bezier(r),
        6 => bspline(r),
        7 => {
            let (u1, u2) = (r.real(), r.real());
            let (sense, adjust) = (r.flag(), r.flag());
            let basis = curve2d(r)?;
            Ok(Handle::new(Geom2dTrimmedCurve::new(&basis, u1, u2, sense, adjust)?).cast())
        }
        8 => {
            let d = r.real();
            let no_check = r.flag();
            let basis = curve2d(r)?;
            Ok(Handle::new(Geom2dOffsetCurve::new(&basis, d, no_check)?).cast())
        }
        9 => {
            let (p, x, y) = (r.pnt2d(), r.dir2d(), r.dir2d());
            let frame = Axis22d::with_directions(p, x, y)?;
            Ok(Handle::new(Geom2dCircle::new(frame, r.real())?).cast())
        }
        14 => {
            let a = r.ax2d();
            let (x, y) = (r.real(), r.real());
            Ok(Handle::new(Geom2dEvalSineWaveCurve::new(a, x, y, r.real())?).cast())
        }
        _ => Err(Failure::with_type(
            "Standard_ProgramError",
            "unknown curve kind",
            "",
        )),
    }
}

/// gb_pc_result: the result printer, for both dimensions.
pub fn pc_result<P>(output: &Output, result: &PcResult<P>, point: impl Fn(P))
where
    P: Copy,
{
    let index = |i: usize| {
        if i == PcResult::<P>::INVALID_INDEX {
            -1
        } else {
            i as i32
        }
    };
    int(output, result.status as i32);
    int(output, result.nb_ext() as i32);
    flag(output, result.is_done());
    flag(output, result.is_infinite());
    real(output, result.infinite_square_distance);
    int(output, index(result.min_index()));
    int(output, index(result.max_index()));
    real(output, result.min_square_distance());
    real(output, result.max_square_distance());
    for extremum in &result.extrema {
        real(output, extremum.parameter);
        point(extremum.point);
        real(output, extremum.square_distance);
        flag(output, extremum.is_minimum);
        flag(output, extremum.is_maximum);
    }
}

/// gb_mode2d / gb_mode3d.
pub fn mode(value: i32) -> SearchMode {
    match value {
        1 => SearchMode::Min,
        2 => SearchMode::Max,
        _ => SearchMode::MinMax,
    }
}

/// One query: `x y tol mode endpoints`.
pub struct Query2d {
    pub point: Point2,
    pub tol: f64,
    pub mode: SearchMode,
    pub ends: bool,
}

fn queries(r: &mut Reader) -> Vec<Query2d> {
    let n = r.integer();
    (0..n)
        .map(|_| {
            let point = r.pnt2d();
            let tol = r.real();
            let mode = mode(r.integer());
            Query2d {
                point,
                tol,
                mode,
                ends: r.flag(),
            }
        })
        .collect()
}

// The shared try block of the requests: a failure prints its type.
fn guarded(
    output: &Output,
    body: impl FnOnce(&Output, &mut Reader) -> Result<(), Failure>,
    values: &[f64],
) {
    let mut r = Reader::new(values);
    attempt(output, || body(output, &mut r));
    end(output);
}

fn curve_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let c = curve2d(r)?;
    let ctor = r.integer();
    let (mut u1, mut u2) = (0.0, 0.0);
    if ctor % 2 == 1 {
        u1 = r.real();
        u2 = r.real();
    }
    let adaptor = Geom2dAdaptorCurve::default();
    if ctor >= 2 {
        adaptor.load(&c)?;
    }
    let mut e: Curve = match ctor {
        0 => Curve::from_geom(&c)?,
        1 => Curve::from_geom_range(&c, u1, u2)?,
        2 => Curve::from_adaptor(&adaptor)?,
        _ => Curve::from_adaptor_range(&adaptor, u1, u2)?,
    };
    flag(output, e.is_initialized());
    for q in queries(r) {
        attempt(output, || {
            let result: &ExtremaResult = if q.ends {
                e.perform_with_endpoints(q.point, q.tol, q.mode)?
            } else {
                e.perform(q.point, q.tol, q.mode)?
            };
            pc_result(output, result, |point| pnt2d(output, point));
            Ok(())
        });
    }
    Ok(())
}
pub fn curve_case(output: &Output, values: &[f64]) {
    guarded(output, curve_body, values);
}

fn grid_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let c = curve2d(r)?;
    let adaptor = Geom2dAdaptorCurve::with_curve(&c)?;
    let v = r.reals();
    let mut params: Vector<'static> = Vector::with_size(v.len());
    for (i, x) in v.iter().enumerate() {
        params[i as i32] = *x;
    }
    let dmin = r.real();
    let d = Domain1D::new(dmin, r.real());
    let mut e = GridEvaluator::default();
    e.set_params(&params);
    for q in queries(r) {
        pc_result(
            output,
            e.perform(&adaptor, &q.point, &d, q.tol, q.mode),
            |point| pnt2d(output, point),
        );
    }
    flag(output, GridEvaluator::is_closed_domain(&adaptor, &d));
    attempt(output, || {
        let a = GridEvaluator::build_curve_aware_params(&adaptor, &d)?;
        vector(output, &a);
        Ok(())
    });
    Ok(())
}
pub fn grid_case(output: &Output, values: &[f64]) {
    guarded(output, grid_body, values);
}

/// A math_Vector as its size and values.
pub fn vector(output: &Output, v: &Vector<'_>) {
    int(output, v.size() as i32);
    v.array1().iter().for_each(|x| real(output, *x));
}

pub fn uniform_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (a, b) = (r.real(), r.real());
    let n = r.integer();
    attempt(output, || {
        vector(
            output,
            &GridEvaluator::build_uniform_params(a, b, n as usize)?,
        );
        Ok(())
    });
    end(output);
}

fn distance_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let c = curve2d(r)?;
    let adaptor = Geom2dAdaptorCurve::with_curve(&c)?;
    let p = r.pnt2d();
    let f = DistanceFunction::new(&adaptor, p);
    let u = r.reals();
    real(output, f.first_parameter()?);
    real(output, f.last_parameter()?);
    pnt2d(output, *f.point());
    for x in u {
        attempt(output, || {
            real(output, f.value(x)?);
            let (v, dv) = f.values(x)?;
            real(output, v);
            real(output, dv);
            real(output, f.derivative(x)?);
            Ok(())
        });
    }
    Ok(())
}
pub fn distance_case(output: &Output, values: &[f64]) {
    guarded(output, distance_body, values);
}

/// The adaptor as the AddEndpointExtrema evaluator of the endpoint request.
struct AdaptorValue<'a>(&'a dyn Adaptor2dCurve2d);
impl pc2d::CurveEvaluator<Point2> for AdaptorValue<'_> {
    type Error = Failure;
    fn evaluate(&self, u: f64) -> Result<Point2, Failure> {
        self.0.value(u)
    }
}

fn endpoints_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let c = curve2d(r)?;
    let adaptor = Geom2dAdaptorCurve::with_curve(&c)?;
    let dmin = r.real();
    let d = Domain1D::new(dmin, r.real());
    let p = r.pnt2d();
    let m = mode(r.integer());
    let closed = r.flag();
    let mut result = ExtremaResult::default();
    pc2d::add_endpoint_extrema(&mut result, &p, &d, &AdaptorValue(&adaptor), m, closed)?;
    pc_result(output, &result, |point| pnt2d(output, point));
    Ok(())
}
pub fn endpoints_case(output: &Output, values: &[f64]) {
    guarded(output, endpoints_body, values);
}

fn td2d_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let c = curve2d(r)?;
    let adaptor = Geom2dAdaptorCurve::with_curve(&c)?;
    let ranged = r.flag();
    let range = if ranged {
        Some((r.real(), r.real()))
    } else {
        None
    };
    let criteria = [r.real(), r.real(), r.real(), r.real(), r.real()];
    attempt(output, || {
        let (params, points) = martensite_tkgeombase::probe_support::tangential_deflection_2d(
            &adaptor, range, criteria,
        )?;
        int(output, params.len() as i32);
        for (u, p) in params.iter().zip(&points) {
            real(output, *u);
            pnt(output, *p);
        }
        Ok(())
    });
    attempt(output, || {
        let values = <martensite_tkgeombase::extrema::Curve2dTool as martensite_tkgeombase::extrema::ExtremaCurveTool>::defl_curv_intervals(&adaptor)?;
        int(output, values.len() as i32);
        values.iter().for_each(|x| real(output, *x));
        Ok(())
    });
    Ok(())
}
pub fn td2d_case(output: &Output, values: &[f64]) {
    guarded(output, td2d_body, values);
}
