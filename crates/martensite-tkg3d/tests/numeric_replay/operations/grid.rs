//! Curve parameter-grid request cases.
use super::adaptor3d_inputs::{adaptor2d, surface as a3_surface};
use super::common::{Reader, attempt, built_curve, end, flag, int, pnt, trsf, vec};
use super::output::Output;
use super::shapes;
use martensite_tkernel::ncollection::{Array1, Array2};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkg3d::adaptor3d::{
    Adaptor3dCurve, Adaptor3dCurveBase, Adaptor3dCurveOnSurface, Adaptor3dIsoCurve,
};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomBezierCurve, GeomCircle, GeomCurve, GeomEllipse, GeomHyperbola, GeomLine,
    GeomOffsetCurve, GeomParabola,
};
use martensite_tkg3d::geom_adaptor::{GeomAdaptorCurve, GeomAdaptorTransformedCurve};
use martensite_tkg3d::geom_grid_eval::{
    GeomGridEvalBSplineCurve, GeomGridEvalBezierCurve, GeomGridEvalCircle, GeomGridEvalCurve,
    GeomGridEvalCurveD1, GeomGridEvalCurveD2, GeomGridEvalCurveD3, GeomGridEvalEllipse,
    GeomGridEvalHyperbola, GeomGridEvalLine, GeomGridEvalOffsetCurve, GeomGridEvalOtherCurve,
    GeomGridEvalParabola, GeomGridEvalSurfD1, GeomGridEvalSurfD2, GeomGridEvalSurfD3,
};
use martensite_tkmath::geom_abs::IsoType;
use martensite_tkmath::gp::{Point3, Vector3};

/// `lower n u*n`, as g3_grid_params reads it.
pub fn params(r: &mut Reader) -> Array1<'static, f64> {
    let lower = r.integer();
    let u = r.reals();
    if u.is_empty() {
        return Array1::default();
    }
    let mut p: Array1<'static, f64> =
        Array1::with_bounds(lower, lower + u.len() as i32 - 1).unwrap();
    p.as_mut_slice().copy_from_slice(&u);
    p
}

/// One grid element, as the g3_grid_value overloads print it.
pub trait GridValue {
    fn print(&self, output: &Output);
}
impl GridValue for Point3 {
    fn print(&self, output: &Output) {
        pnt(output, *self);
    }
}
impl GridValue for Vector3 {
    fn print(&self, output: &Output) {
        vec(output, *self);
    }
}
impl GridValue for GeomGridEvalCurveD1 {
    fn print(&self, output: &Output) {
        pnt(output, self.point);
        vec(output, self.d1);
    }
}
impl GridValue for GeomGridEvalCurveD2 {
    fn print(&self, output: &Output) {
        pnt(output, self.point);
        [self.d1, self.d2]
            .into_iter()
            .for_each(|value| vec(output, value));
    }
}
impl GridValue for GeomGridEvalCurveD3 {
    fn print(&self, output: &Output) {
        pnt(output, self.point);
        [self.d1, self.d2, self.d3]
            .into_iter()
            .for_each(|value| vec(output, value));
    }
}
impl GridValue for GeomGridEvalSurfD1 {
    fn print(&self, output: &Output) {
        pnt(output, self.point);
        [self.d1u, self.d1v]
            .into_iter()
            .for_each(|value| vec(output, value));
    }
}
impl GridValue for GeomGridEvalSurfD2 {
    fn print(&self, output: &Output) {
        pnt(output, self.point);
        [self.d1u, self.d1v, self.d2u, self.d2v, self.d2uv]
            .into_iter()
            .for_each(|value| vec(output, value));
    }
}
impl GridValue for GeomGridEvalSurfD3 {
    fn print(&self, output: &Output) {
        pnt(output, self.point);
        let d = [
            self.d1u, self.d1v, self.d2u, self.d2v, self.d2uv, self.d3u, self.d3v, self.d3uuv,
            self.d3uvv,
        ];
        d.into_iter().for_each(|value| vec(output, value));
    }
}

/// g3_grid_array for a curve grid.
pub fn array<T: GridValue>(output: &Output, grid: Result<Array1<'static, T>, Failure>) {
    attempt(output, || {
        let grid = grid?;
        int(output, grid.lower());
        int(output, grid.len() as i32);
        grid.iter().for_each(|value| value.print(output));
        Ok(())
    });
}

/// g3_grid_array for a surface grid (row major, as Array2 is stored).
pub fn array2<T: GridValue>(output: &Output, grid: Result<Array2<'static, T>, Failure>) {
    attempt(output, || {
        let grid = grid?;
        int(output, grid.lower_row());
        int(output, grid.nb_rows() as i32);
        int(output, grid.lower_col());
        int(output, grid.nb_columns() as i32);
        grid.iter().for_each(|value| value.print(output));
        Ok(())
    });
}

/// The five members of every curve evaluator, with `Result` for the infallible line ones.
trait CurveGridEvaluator {
    fn grid(&self, p: &Array1<'_, f64>) -> Result<Array1<'static, Point3>, Failure>;
    fn grid_d1(&self, p: &Array1<'_, f64>)
    -> Result<Array1<'static, GeomGridEvalCurveD1>, Failure>;
    fn grid_d2(&self, p: &Array1<'_, f64>)
    -> Result<Array1<'static, GeomGridEvalCurveD2>, Failure>;
    fn grid_d3(&self, p: &Array1<'_, f64>)
    -> Result<Array1<'static, GeomGridEvalCurveD3>, Failure>;
    fn grid_dn(&self, p: &Array1<'_, f64>, n: i32) -> Result<Array1<'static, Vector3>, Failure>;
}
macro_rules! curve_grid_evaluator {
    ($class:ty, $wrap:path) => {
        impl CurveGridEvaluator for $class {
            fn grid(&self, p: &Array1<'_, f64>) -> Result<Array1<'static, Point3>, Failure> {
                $wrap(self.evaluate_grid(p))
            }
            fn grid_d1(
                &self,
                p: &Array1<'_, f64>,
            ) -> Result<Array1<'static, GeomGridEvalCurveD1>, Failure> {
                $wrap(self.evaluate_grid_d1(p))
            }
            fn grid_d2(
                &self,
                p: &Array1<'_, f64>,
            ) -> Result<Array1<'static, GeomGridEvalCurveD2>, Failure> {
                $wrap(self.evaluate_grid_d2(p))
            }
            fn grid_d3(
                &self,
                p: &Array1<'_, f64>,
            ) -> Result<Array1<'static, GeomGridEvalCurveD3>, Failure> {
                $wrap(self.evaluate_grid_d3(p))
            }
            fn grid_dn(
                &self,
                p: &Array1<'_, f64>,
                n: i32,
            ) -> Result<Array1<'static, Vector3>, Failure> {
                $wrap(self.evaluate_grid_dn(p, n))
            }
        }
    };
}
pub fn same<T>(value: Result<T, Failure>) -> Result<T, Failure> {
    value
}
curve_grid_evaluator!(GeomGridEvalLine, Ok);
curve_grid_evaluator!(GeomGridEvalCircle, same);
curve_grid_evaluator!(GeomGridEvalEllipse, same);
curve_grid_evaluator!(GeomGridEvalHyperbola, same);
curve_grid_evaluator!(GeomGridEvalParabola, same);
curve_grid_evaluator!(GeomGridEvalBezierCurve, same);
curve_grid_evaluator!(GeomGridEvalBSplineCurve, same);
curve_grid_evaluator!(GeomGridEvalOffsetCurve, same);
curve_grid_evaluator!(GeomGridEvalOtherCurve<'_>, same);
curve_grid_evaluator!(GeomGridEvalCurve<'_>, same);

/// g3_grid_curve_all: EvaluateGrid, D1, D2, D3, then DN for N = -1..7.
fn all(output: &Output, e: &dyn CurveGridEvaluator, p: &Array1<'_, f64>) {
    array(output, e.grid(p));
    array(output, e.grid_d1(p));
    array(output, e.grid_d2(p));
    array(output, e.grid_d3(p));
    for n in -1..=7 {
        array(output, e.grid_dn(p, n));
    }
}

fn dispatch(output: &Output, e: &GeomGridEvalCurve<'_>, p: &Array1<'_, f64>) {
    int(output, e.get_type() as i32);
    all(output, e, p);
}

/// The specialized curve evaluators that take a typed handle and expose `geometry()`.
trait SpecializedCurve<G: RuntimeTyped>: CurveGridEvaluator + Sized {
    fn build(geometry: Handle<G>) -> Self;
    fn is_null(&self) -> bool;
}
macro_rules! specialized_curve {
    ($class:ty, $geom:ty) => {
        impl SpecializedCurve<$geom> for $class {
            fn build(geometry: Handle<$geom>) -> Self {
                <$class>::new(geometry)
            }
            fn is_null(&self) -> bool {
                self.geometry().is_null()
            }
        }
    };
}
specialized_curve!(GeomGridEvalLine, GeomLine);
specialized_curve!(GeomGridEvalCircle, GeomCircle);
specialized_curve!(GeomGridEvalEllipse, GeomEllipse);
specialized_curve!(GeomGridEvalHyperbola, GeomHyperbola);
specialized_curve!(GeomGridEvalParabola, GeomParabola);
specialized_curve!(GeomGridEvalBezierCurve, GeomBezierCurve);
specialized_curve!(GeomGridEvalBSplineCurve, GeomBSplineCurve);
specialized_curve!(GeomGridEvalOffsetCurve, GeomOffsetCurve);

fn direct_as<G: RuntimeTyped, E: SpecializedCurve<G>>(
    output: &Output,
    c: &Handle<dyn GeomCurve>,
    p: &Array1<'_, f64>,
) -> bool {
    let g: Handle<G> = c.cast::<G>();
    if g.is_null() {
        return false;
    }
    let e = E::build(g);
    flag(output, e.is_null());
    all(output, &e, p);
    true
}

/// g3_grid_curve_direct: the specialized evaluator of the curve's own class.
fn direct(output: &Output, c: &Handle<dyn GeomCurve>, p: &Array1<'_, f64>) {
    let _ = direct_as::<GeomLine, GeomGridEvalLine>(output, c, p)
        || direct_as::<GeomCircle, GeomGridEvalCircle>(output, c, p)
        || direct_as::<GeomEllipse, GeomGridEvalEllipse>(output, c, p)
        || direct_as::<GeomHyperbola, GeomGridEvalHyperbola>(output, c, p)
        || direct_as::<GeomParabola, GeomGridEvalParabola>(output, c, p)
        || direct_as::<GeomBezierCurve, GeomGridEvalBezierCurve>(output, c, p)
        || direct_as::<GeomBSplineCurve, GeomGridEvalBSplineCurve>(output, c, p)
        || direct_as::<GeomOffsetCurve, GeomGridEvalOffsetCurve>(output, c, p);
}

/// g3_grid: the dispatcher on the handle and on an adaptor, the specialized evaluator
/// and the fallback.
pub fn grid_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built_curve(output, &mut r) else {
        return;
    };
    let p = params(&mut r);
    dispatch(output, &GeomGridEvalCurve::from_curve(&c), &p);
    let a = GeomAdaptorCurve::with_curve(&c).unwrap();
    dispatch(output, &GeomGridEvalCurve::from_adaptor(&a).unwrap(), &p);
    direct(output, &c, &p);
    all(output, &GeomGridEvalOtherCurve::new(&a), &p);
    end(output);
}

/// One grid curve adaptor specification (g3_grid_curve_adaptor).
pub fn curve_adaptor(
    output: &Output,
    r: &mut Reader,
) -> Result<Handle<dyn Adaptor3dCurve>, Failure> {
    match r.integer() {
        0 => Ok(Handle::new(GeomAdaptorCurve::with_curve(&shapes::curve(output, r)?)?).cast()),
        1 => {
            let (u1, u2) = (r.real(), r.real());
            Ok(Handle::new(GeomAdaptorCurve::with_range(
                &shapes::curve(output, r)?,
                u1,
                u2,
            )?)
            .cast())
        }
        2 => Ok(Handle::new(Adaptor3dCurveBase::default()).cast()),
        3 => {
            let t = trsf(r);
            Ok(Handle::new(GeomAdaptorTransformedCurve::with_curve(
                &shapes::curve(output, r)?,
                t,
            )?)
            .cast())
        }
        4 => {
            let iso = [IsoType::IsoU, IsoType::IsoV, IsoType::NoneIso][r.integer() as usize];
            let w = r.real();
            Ok(Handle::new(Adaptor3dIsoCurve::with_iso(
                &a3_surface(output, r)?,
                iso,
                w,
            )?)
            .cast())
        }
        _ => {
            let c = adaptor2d(output, r)?;
            Ok(Handle::new(Adaptor3dCurveOnSurface::with_curve(
                &c,
                &a3_surface(output, r)?,
            )?)
            .cast())
        }
    }
}

/// g3_grid_adaptor: the dispatcher on any adaptor, then the fallback.
pub fn grid_adaptor_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let a = match curve_adaptor(output, &mut r) {
        Ok(a) => a,
        Err(error) => {
            replay_println!(output, "{}", error.exception_type());
            return;
        }
    };
    let p = params(&mut r);
    let adaptor: &dyn Adaptor3dCurve = &*a;
    attempt(output, || {
        dispatch(output, &GeomGridEvalCurve::from_adaptor(adaptor)?, &p);
        Ok(())
    });
    all(output, &GeomGridEvalOtherCurve::new(adaptor), &p);
    end(output);
}

fn null_as<G: RuntimeTyped, E: SpecializedCurve<G>>(output: &Output, p: &Array1<'_, f64>) {
    let e = E::build(Handle::default());
    flag(output, e.is_null());
    all(output, &e, p);
}

/// The curve half of g3_grid_null: every evaluator on a null curve, the dispatcher on a
/// null handle and on an empty adaptor.
pub fn null_curves(output: &Output, p: &Array1<'_, f64>) {
    null_as::<GeomLine, GeomGridEvalLine>(output, p);
    null_as::<GeomCircle, GeomGridEvalCircle>(output, p);
    null_as::<GeomEllipse, GeomGridEvalEllipse>(output, p);
    null_as::<GeomHyperbola, GeomGridEvalHyperbola>(output, p);
    null_as::<GeomParabola, GeomGridEvalParabola>(output, p);
    null_as::<GeomBezierCurve, GeomGridEvalBezierCurve>(output, p);
    null_as::<GeomBSplineCurve, GeomGridEvalBSplineCurve>(output, p);
    null_as::<GeomOffsetCurve, GeomGridEvalOffsetCurve>(output, p);
    dispatch(
        output,
        &GeomGridEvalCurve::from_curve(&Handle::default()),
        p,
    );
    let a = GeomAdaptorCurve::default();
    dispatch(output, &GeomGridEvalCurve::from_adaptor(&a).unwrap(), p);
}
