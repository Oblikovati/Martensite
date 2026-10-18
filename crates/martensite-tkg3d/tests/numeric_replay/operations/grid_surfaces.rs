//! Surface parameter-grid evaluation requests.
use super::adaptor3d_inputs::surface as a3_surface;
use super::common::{Reader, attempt, built_surface, end, flag, int, real, trsf};
use super::grid::{array2, curve_adaptor, null_curves, params};
use super::output::Output;
use super::shapes;
use martensite_tkernel::ncollection::{Array1, Array2};
use martensite_tkernel::standard::{Failure, Handle, RuntimeTyped};
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkg3d::geom::{
    GeomBSplineSurface, GeomBezierSurface, GeomConicalSurface, GeomCylindricalSurface,
    GeomOffsetSurface, GeomPlane, GeomSphericalSurface, GeomSurface, GeomSurfaceOfLinearExtrusion,
    GeomSurfaceOfRevolution, GeomToroidalSurface,
};
use martensite_tkg3d::geom_adaptor::{
    GeomAdaptorSurface, GeomAdaptorSurfaceOfLinearExtrusion, GeomAdaptorSurfaceOfRevolution,
    GeomAdaptorTransformedSurface,
};
use martensite_tkg3d::geom_grid_eval::{
    GeomGridEvalBSplineSurface, GeomGridEvalBezierSurface, GeomGridEvalCone, GeomGridEvalCylinder,
    GeomGridEvalOffsetSurface, GeomGridEvalOtherSurface, GeomGridEvalPlane, GeomGridEvalSphere,
    GeomGridEvalSurfD1, GeomGridEvalSurfD2, GeomGridEvalSurfD3, GeomGridEvalSurface,
    GeomGridEvalSurfaceOfExtrusion, GeomGridEvalSurfaceOfRevolution, GeomGridEvalTorus,
    evaluate_grid_d1_helper, evaluate_grid_d2_helper, evaluate_grid_d3_helper,
    evaluate_grid_dn_helper, evaluate_grid_helper,
};
use martensite_tkmath::gp::{Point3, Vector3};

type Params<'a> = (&'a Array1<'a, f64>, &'a Array1<'a, f64>);

/// The five members of every surface evaluator, with `Result` for the plane ones.
trait SurfaceGridEvaluator {
    fn grid(&self, uv: Params<'_>) -> Result<Array2<'static, Point3>, Failure>;
    fn grid_d1(&self, uv: Params<'_>) -> Result<Array2<'static, GeomGridEvalSurfD1>, Failure>;
    fn grid_d2(&self, uv: Params<'_>) -> Result<Array2<'static, GeomGridEvalSurfD2>, Failure>;
    fn grid_d3(&self, uv: Params<'_>) -> Result<Array2<'static, GeomGridEvalSurfD3>, Failure>;
    fn grid_dn(&self, uv: Params<'_>, n: (i32, i32)) -> Result<Array2<'static, Vector3>, Failure>;
}
macro_rules! surface_grid_evaluator {
    ($class:ty, $wrap:path) => {
        impl SurfaceGridEvaluator for $class {
            fn grid(&self, uv: Params<'_>) -> Result<Array2<'static, Point3>, Failure> {
                $wrap(self.evaluate_grid(uv.0, uv.1))
            }
            fn grid_d1(
                &self,
                uv: Params<'_>,
            ) -> Result<Array2<'static, GeomGridEvalSurfD1>, Failure> {
                $wrap(self.evaluate_grid_d1(uv.0, uv.1))
            }
            fn grid_d2(
                &self,
                uv: Params<'_>,
            ) -> Result<Array2<'static, GeomGridEvalSurfD2>, Failure> {
                $wrap(self.evaluate_grid_d2(uv.0, uv.1))
            }
            fn grid_d3(
                &self,
                uv: Params<'_>,
            ) -> Result<Array2<'static, GeomGridEvalSurfD3>, Failure> {
                $wrap(self.evaluate_grid_d3(uv.0, uv.1))
            }
            fn grid_dn(
                &self,
                uv: Params<'_>,
                n: (i32, i32),
            ) -> Result<Array2<'static, Vector3>, Failure> {
                $wrap(self.evaluate_grid_dn(uv.0, uv.1, n.0, n.1))
            }
        }
    };
}
use super::grid::same;
surface_grid_evaluator!(GeomGridEvalPlane, Ok);
surface_grid_evaluator!(GeomGridEvalCylinder, same);
surface_grid_evaluator!(GeomGridEvalSphere, same);
surface_grid_evaluator!(GeomGridEvalCone, same);
surface_grid_evaluator!(GeomGridEvalTorus, same);
surface_grid_evaluator!(GeomGridEvalBezierSurface, same);
surface_grid_evaluator!(GeomGridEvalBSplineSurface, same);
surface_grid_evaluator!(GeomGridEvalOffsetSurface, same);
surface_grid_evaluator!(GeomGridEvalSurfaceOfRevolution, same);
surface_grid_evaluator!(GeomGridEvalSurfaceOfExtrusion, same);
surface_grid_evaluator!(GeomGridEvalOtherSurface<'_>, same);
surface_grid_evaluator!(GeomGridEvalSurface<'_>, same);

/// The DN sample of the non-full summaries (g3_grid_sample_orders).
const SAMPLE_ORDERS: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (1, 1),
    (2, 1),
    (0, 3),
    (4, 0),
    (-1, 1),
    (0, 0),
];

/// g3_grid_surface_all: EvaluateGrid, D1, D2, D3, then DN for every order in -1..4 x -1..4
/// when `full`, the sample otherwise.
fn all(output: &Output, e: &dyn SurfaceGridEvaluator, uv: Params<'_>, full: bool) {
    array2(output, e.grid(uv));
    array2(output, e.grid_d1(uv));
    array2(output, e.grid_d2(uv));
    array2(output, e.grid_d3(uv));
    if full {
        for nu in -1..=4 {
            for nv in -1..=4 {
                array2(output, e.grid_dn(uv, (nu, nv)));
            }
        }
        return;
    }
    for o in SAMPLE_ORDERS {
        array2(output, e.grid_dn(uv, o));
    }
}

/// g3_grid_surface_dispatch: type, transformation, then the grids.
fn dispatch(output: &Output, e: &GeomGridEvalSurface<'_>, uv: Params<'_>, full: bool) {
    int(output, e.get_type() as i32);
    flag(output, e.has_transformation());
    if let Some(t) = e.get_transformation() {
        for i in 1..=3 {
            for j in 1..=4 {
                real(output, t.value(i, j).unwrap());
            }
        }
        int(output, t.form() as i32);
    }
    all(output, e, uv, full);
}

/// The specialized surface evaluators that take a typed handle and expose `geometry()`.
trait SpecializedSurface<G: RuntimeTyped>: SurfaceGridEvaluator + Sized {
    fn build(geometry: Handle<G>) -> Self;
    fn is_null(&self) -> bool;
}
macro_rules! specialized_surface {
    ($class:ty, $geom:ty) => {
        impl SpecializedSurface<$geom> for $class {
            fn build(geometry: Handle<$geom>) -> Self {
                <$class>::new(geometry)
            }
            fn is_null(&self) -> bool {
                self.geometry().is_null()
            }
        }
    };
}
specialized_surface!(GeomGridEvalPlane, GeomPlane);
specialized_surface!(GeomGridEvalCylinder, GeomCylindricalSurface);
specialized_surface!(GeomGridEvalSphere, GeomSphericalSurface);
specialized_surface!(GeomGridEvalCone, GeomConicalSurface);
specialized_surface!(GeomGridEvalTorus, GeomToroidalSurface);
specialized_surface!(GeomGridEvalBezierSurface, GeomBezierSurface);
specialized_surface!(GeomGridEvalBSplineSurface, GeomBSplineSurface);
specialized_surface!(GeomGridEvalOffsetSurface, GeomOffsetSurface);
specialized_surface!(GeomGridEvalSurfaceOfRevolution, GeomSurfaceOfRevolution);
specialized_surface!(GeomGridEvalSurfaceOfExtrusion, GeomSurfaceOfLinearExtrusion);

fn direct_as<G: RuntimeTyped, E: SpecializedSurface<G>>(
    output: &Output,
    s: &Handle<dyn GeomSurface>,
    uv: Params<'_>,
) -> bool {
    let g: Handle<G> = s.cast::<G>();
    if g.is_null() {
        return false;
    }
    let e = E::build(g);
    flag(output, e.is_null());
    all(output, &e, uv, true);
    true
}

/// g3_grid_surface_direct: the specialized evaluator of the surface's own class.
fn direct(output: &Output, s: &Handle<dyn GeomSurface>, uv: Params<'_>) {
    let _ = direct_as::<GeomPlane, GeomGridEvalPlane>(output, s, uv)
        || direct_as::<GeomCylindricalSurface, GeomGridEvalCylinder>(output, s, uv)
        || direct_as::<GeomSphericalSurface, GeomGridEvalSphere>(output, s, uv)
        || direct_as::<GeomConicalSurface, GeomGridEvalCone>(output, s, uv)
        || direct_as::<GeomToroidalSurface, GeomGridEvalTorus>(output, s, uv)
        || direct_as::<GeomBezierSurface, GeomGridEvalBezierSurface>(output, s, uv)
        || direct_as::<GeomBSplineSurface, GeomGridEvalBSplineSurface>(output, s, uv)
        || direct_as::<GeomOffsetSurface, GeomGridEvalOffsetSurface>(output, s, uv)
        || direct_as::<GeomSurfaceOfRevolution, GeomGridEvalSurfaceOfRevolution>(output, s, uv)
        || direct_as::<GeomSurfaceOfLinearExtrusion, GeomGridEvalSurfaceOfExtrusion>(output, s, uv);
}

/// g3_grid_surface: the dispatcher on the handle and on an adaptor, the specialized
/// evaluator and the fallback on the handle and on the adaptor.
pub fn grid_surface_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(s) = built_surface(output, &mut r) else {
        return;
    };
    let (u, v) = (params(&mut r), params(&mut r));
    let uv: Params<'_> = (&u, &v);
    dispatch(output, &GeomGridEvalSurface::from_surface(&s), uv, true);
    let adaptor = GeomAdaptorSurface::with_surface(&s);
    if let Err(error) = &adaptor {
        replay_print!(output, "{} ", error.exception_type());
    }
    if let Ok(a) = &adaptor {
        attempt(output, || {
            dispatch(output, &GeomGridEvalSurface::from_adaptor(a)?, uv, false);
            Ok(())
        });
    }
    direct(output, &s, uv);
    all(
        output,
        &GeomGridEvalOtherSurface::from_surface(s.clone()),
        uv,
        false,
    );
    if let Ok(a) = &adaptor {
        all(
            output,
            &GeomGridEvalOtherSurface::from_adaptor(a),
            uv,
            false,
        );
    }
    end(output);
}

/// One grid surface adaptor specification (g3_grid_surface_adaptor).
fn surface_adaptor(
    output: &Output,
    r: &mut Reader,
) -> Result<Handle<dyn Adaptor3dSurface>, Failure> {
    match r.integer() {
        0 => a3_surface(output, r),
        1 => {
            let d = r.dir();
            let c = curve_adaptor(output, r)?;
            Ok(
                Handle::new(GeomAdaptorSurfaceOfLinearExtrusion::with_curve_and_direction(&c, d))
                    .cast(),
            )
        }
        2 => {
            let ax = r.ax1();
            let c = curve_adaptor(output, r)?;
            Ok(Handle::new(GeomAdaptorSurfaceOfRevolution::with_curve_and_axis(&c, ax)?).cast())
        }
        3 => {
            let t = trsf(r);
            Ok(Handle::new(GeomAdaptorTransformedSurface::with_surface(
                &shapes::surface(output, r)?,
                t,
            )?)
            .cast())
        }
        _ => {
            let b: Vec<f64> = (0..6).map(|_| r.real()).collect();
            let t = trsf(r);
            let range = ((b[0], b[1], b[2], b[3]), (b[4], b[5]));
            let s = shapes::surface(output, r)?;
            Ok(Handle::new(GeomAdaptorTransformedSurface::with_range(&s, range, t)?).cast())
        }
    }
}

/// g3_grid_surface_adaptor: the dispatcher on any adaptor, then the fallback.
pub fn grid_surface_adaptor_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let a = match surface_adaptor(output, &mut r) {
        Ok(a) => a,
        Err(error) => {
            replay_println!(output, "{}", error.exception_type());
            return;
        }
    };
    let (u, v) = (params(&mut r), params(&mut r));
    let uv: Params<'_> = (&u, &v);
    let adaptor: &dyn Adaptor3dSurface = &*a;
    attempt(output, || {
        dispatch(
            output,
            &GeomGridEvalSurface::from_adaptor(adaptor)?,
            uv,
            true,
        );
        Ok(())
    });
    all(
        output,
        &GeomGridEvalOtherSurface::from_adaptor(adaptor),
        uv,
        false,
    );
    end(output);
}

fn null_as<G: RuntimeTyped, E: SpecializedSurface<G>>(output: &Output, uv: Params<'_>) {
    let e = E::build(Handle::default());
    flag(output, e.is_null());
    all(output, &e, uv, true);
}

/// g3_grid_null: every evaluator on null geometry, the dispatchers on null handles and
/// on empty adaptors (the surface one reports its type only).
pub fn grid_null_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (u, v) = (params(&mut r), params(&mut r));
    let uv: Params<'_> = (&u, &v);
    null_curves(output, &u);
    null_as::<GeomPlane, GeomGridEvalPlane>(output, uv);
    null_as::<GeomCylindricalSurface, GeomGridEvalCylinder>(output, uv);
    null_as::<GeomSphericalSurface, GeomGridEvalSphere>(output, uv);
    null_as::<GeomConicalSurface, GeomGridEvalCone>(output, uv);
    null_as::<GeomToroidalSurface, GeomGridEvalTorus>(output, uv);
    null_as::<GeomBezierSurface, GeomGridEvalBezierSurface>(output, uv);
    null_as::<GeomBSplineSurface, GeomGridEvalBSplineSurface>(output, uv);
    null_as::<GeomOffsetSurface, GeomGridEvalOffsetSurface>(output, uv);
    null_as::<GeomSurfaceOfRevolution, GeomGridEvalSurfaceOfRevolution>(output, uv);
    null_as::<GeomSurfaceOfLinearExtrusion, GeomGridEvalSurfaceOfExtrusion>(output, uv);
    dispatch(
        output,
        &GeomGridEvalSurface::from_surface(&Handle::default()),
        uv,
        true,
    );
    let a = GeomAdaptorSurface::default();
    let e = GeomGridEvalSurface::from_adaptor(&a).unwrap();
    int(output, e.get_type() as i32);
    flag(output, e.has_transformation());
    end(output);
}

fn zero() -> Vector3 {
    Vector3::new(0.0, 0.0, 0.0)
}

/// g3_grid_helpers: the GeomGridEval helper templates with fixed functors.
pub fn grid_helpers_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (u, v) = (params(&mut r), params(&mut r));
    array2(
        output,
        Ok(evaluate_grid_helper(&u, &v, |a, b| {
            Point3::new(a + b, a * b, a - b)
        })),
    );
    array2(
        output,
        Ok(evaluate_grid_d1_helper(&u, &v, |a, b| GeomGridEvalSurfD1 {
            point: Point3::new(a, b, 0.0),
            d1u: Vector3::new(1.0, 0.0, b),
            d1v: Vector3::new(0.0, 1.0, a),
        })),
    );
    array2(
        output,
        Ok(evaluate_grid_d2_helper(&u, &v, |a, b| GeomGridEvalSurfD2 {
            point: Point3::new(a, b, a * b),
            d1u: Vector3::new(1.0, 0.0, b),
            d1v: Vector3::new(0.0, 1.0, a),
            d2u: zero(),
            d2v: zero(),
            d2uv: Vector3::new(0.0, 0.0, 1.0),
        })),
    );
    array2(
        output,
        Ok(evaluate_grid_d3_helper(&u, &v, |a, b| GeomGridEvalSurfD3 {
            point: Point3::new(a, b, a * b * b),
            d1u: Vector3::new(1.0, 0.0, b * b),
            d1v: Vector3::new(0.0, 1.0, 2.0 * a * b),
            d2u: zero(),
            d2v: Vector3::new(0.0, 0.0, 2.0 * a),
            d2uv: Vector3::new(0.0, 0.0, 2.0 * b),
            d3u: zero(),
            d3v: zero(),
            d3uuv: zero(),
            d3uvv: Vector3::new(0.0, 0.0, 2.0),
        })),
    );
    array2(
        output,
        Ok(evaluate_grid_dn_helper(&u, &v, |a, b| {
            Vector3::new(a * a, b * b, a / (b + 3.0))
        })),
    );
    end(output);
}
