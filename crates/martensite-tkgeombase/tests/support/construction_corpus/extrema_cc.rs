//! The Extrema curve-curve requests; the counterpart of
//! the implementation (same encoding and token order).
#![allow(clippy::unit_arg, clippy::type_complexity)]
use super::common::{Reader, attempt, end, flag, int, real};
use super::extrema_pc::{Space, Space2, Space3};
use super::print::Output;
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::extrema::{
    CCLocFOfLocECC, CurveTool, ExtCC, ExtCC2d, ExtremaCurveTool, GGenExtCC, GlobOptFuncCCC0,
    GlobOptFuncCCC1, GlobOptFuncCCC2, LocECC, LocateExtCC, POnCurve,
};
use martensite_tkmath::math::{Matrix, Vector};

type P<S> = <<S as Space>::Tool as ExtremaCurveTool>::Point;

/// The accessors of an ExtCC-like result.
trait CcView<S: Space> {
    fn done(&self) -> bool;
    fn parallel(&self) -> Result<bool, Failure>;
    fn count(&self) -> Result<i32, Failure>;
    fn sq(&self, n: i32) -> Result<f64, Failure>;
    fn pair(&self, n: i32) -> Result<(POnCurve<P<S>>, POnCurve<P<S>>), Failure>;
}
macro_rules! cc_view {
    ($t:ty, $s:ty) => {
        impl CcView<$s> for $t {
            fn done(&self) -> bool {
                self.is_done()
            }
            fn parallel(&self) -> Result<bool, Failure> {
                self.is_parallel()
            }
            fn count(&self) -> Result<i32, Failure> {
                self.nb_ext()
            }
            fn sq(&self, n: i32) -> Result<f64, Failure> {
                self.square_distance(n)
            }
            fn pair(&self, n: i32) -> Result<(POnCurve<P<$s>>, POnCurve<P<$s>>), Failure> {
                self.points(n)
            }
        }
    };
}
cc_view!(ExtCC<'_>, Space3);
cc_view!(ExtCC2d<'_>, Space2);
cc_view!(GGenExtCC<'_, CurveTool>, Space3);
cc_view!(
    GGenExtCC<'_, martensite_tkgeombase::extrema::Curve2dTool>,
    Space2
);

fn cc_result<S: Space>(output: &Output, e: &impl CcView<S>) {
    flag(output, e.done());
    attempt(output, || Ok(flag(output, e.parallel()?)));
    attempt(output, || {
        let n = e.count()?;
        int(output, n);
        for i in 1..=n {
            real(output, e.sq(i)?);
            attempt(output, || {
                let (a, b) = e.pair(i)?;
                S::pon(output, a);
                S::pon(output, b);
                Ok(())
            });
        }
        Ok(())
    });
    attempt(output, || Ok(real(output, e.sq(0)?)));
}

fn trimmed<S: Space>(output: &Output, d: [f64; 4], p: [P<S>; 4]) {
    for i in 0..4 {
        real(output, d[i]);
        S::pnt(output, p[i]);
    }
}

fn cc_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let a = Space3::adaptor(r)?;
    let b = Space3::adaptor(r)?;
    let (u1, u2, v1, v2) = (r.real(), r.real(), r.real(), r.real());
    let single = r.flag();
    attempt(output, || {
        let e = ExtCC::with_curves(&*a, &*b, 1.0e-10, 1.0e-10)?;
        cc_result::<Space3>(output, &e);
        if e.is_done() {
            let (d, p) = e.trimmed_square_distances();
            trimmed::<Space3>(output, d, p);
        }
        Ok(())
    });
    attempt(output, || {
        let e = ExtCC::with_ranges(&*a, &*b, (u1, u2), (v1, v2), 1.0e-10, 1.0e-10)?;
        cc_result::<Space3>(output, &e);
        if e.is_done() {
            let (d, p) = e.trimmed_square_distances();
            trimmed::<Space3>(output, d, p);
        }
        Ok(())
    });
    let mut e = ExtCC::new(1.0e-9, 1.0e-8);
    e.set_single_solution_flag(single);
    flag(output, e.get_single_solution_flag());
    attempt(output, || {
        e.initialize_ranges(&*a, &*b, (u1, u2), (v1, v2), (1.0e-9, 1.0e-8))?;
        e.perform()?;
        cc_result::<Space3>(output, &e);
        if e.is_done() {
            let (d, p) = e.trimmed_square_distances();
            trimmed::<Space3>(output, d, p);
        }
        e.perform()?;
        cc_result::<Space3>(output, &e);
        Ok(())
    });
    Ok(())
}

pub fn cc_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || cc_body(output, &mut r));
    end(output);
}

fn cc2d_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let a = Space2::adaptor(r)?;
    let b = Space2::adaptor(r)?;
    let (u1, u2, v1, v2) = (r.real(), r.real(), r.real(), r.real());
    let single = r.flag();
    attempt(output, || {
        let e = ExtCC2d::with_curves(&*a, &*b, 1.0e-10, 1.0e-10)?;
        cc_result::<Space2>(output, &e);
        if e.is_done() {
            let (d, p) = e.trimmed_square_distances();
            trimmed::<Space2>(output, d, p);
        }
        Ok(())
    });
    attempt(output, || {
        let e = ExtCC2d::with_ranges(&*a, &*b, (u1, u2), (v1, v2), 1.0e-10, 1.0e-10)?;
        cc_result::<Space2>(output, &e);
        if e.is_done() {
            let (d, p) = e.trimmed_square_distances();
            trimmed::<Space2>(output, d, p);
        }
        Ok(())
    });
    let mut e = ExtCC2d::new();
    e.set_single_solution_flag(single);
    flag(output, e.get_single_solution_flag());
    attempt(output, || {
        e.initialize(&*b, v1, v2, (1.0e-9, 1.0e-8));
        e.perform(&*a, u1, u2)?;
        cc_result::<Space2>(output, &e);
        if e.is_done() {
            let (d, p) = e.trimmed_square_distances();
            trimmed::<Space2>(output, d, p);
        }
        e.perform(&*a, u1, u2)?;
        cc_result::<Space2>(output, &e);
        Ok(())
    });
    Ok(())
}

pub fn cc2d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || cc2d_body(output, &mut r));
    end(output);
}

fn ecc_body<S: Space>(output: &Output, r: &mut Reader) -> Result<(), Failure>
where
    for<'x> GGenExtCC<'x, S::Tool>: CcView<S>,
{
    let a = S::adaptor(r)?;
    let b = S::adaptor(r)?;
    let (u1, u2, v1, v2) = (r.real(), r.real(), r.real(), r.real());
    let single = r.flag();
    let tol = r.real();
    attempt(output, || {
        let mut e = GGenExtCC::<S::Tool>::with_curves(&*a, &*b)?;
        e.set_single_solution_flag(single);
        flag(output, e.get_single_solution_flag());
        e.set_tolerance(tol);
        e.perform()?;
        cc_result::<S>(output, &e);
        e.perform()?;
        cc_result::<S>(output, &e);
        Ok(())
    });
    attempt(output, || {
        let mut e = GGenExtCC::<S::Tool>::with_bounds(&*a, &*b, (u1, u2), (v1, v2));
        e.perform()?;
        cc_result::<S>(output, &e);
        Ok(())
    });
    let mut d = GGenExtCC::<S::Tool>::new();
    attempt(output, || Ok(flag(output, d.is_parallel()?)));
    d.set_params(&*a, &*b, (u1, u2), (v1, v2));
    attempt(output, || {
        d.perform()?;
        cc_result::<S>(output, &d);
        Ok(())
    });
    Ok(())
}

pub fn ecc_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || ecc_body::<Space3>(output, &mut r));
    end(output);
}
pub fn ecc2d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || ecc_body::<Space2>(output, &mut r));
    end(output);
}

fn locecc_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let a = Space3::adaptor(r)?;
    let b = Space3::adaptor(r)?;
    let n = r.integer();
    for _ in 0..n {
        let (u0, v0, tu, tv) = (r.real(), r.real(), r.real(), r.real());
        attempt(output, || {
            let e = LocateExtCC::new(&*a, &*b, u0, v0)?;
            flag(output, e.is_done());
            attempt(output, || Ok(real(output, e.square_distance()?)));
            attempt(output, || {
                let (p, s) = e.point()?;
                Space3::pon(output, p);
                Space3::pon(output, s);
                Ok(())
            });
            Ok(())
        });
        attempt(output, || {
            let e = LocECC::new::<CurveTool>(&*a, &*b, (u0, v0), (tu, tv))?;
            flag(output, e.is_done());
            attempt(output, || Ok(real(output, e.square_distance()?)));
            attempt(output, || {
                let (p, s) = e.point()?;
                Space3::pon(output, p);
                Space3::pon(output, s);
                Ok(())
            });
            Ok(())
        });
    }
    Ok(())
}

pub fn locecc_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || locecc_body(output, &mut r));
    end(output);
}

fn vec2(a: f64, b: f64) -> Vector<'static> {
    let mut v = Vector::with_bounds(1, 2).unwrap();
    v[1] = a;
    v[2] = b;
    v
}
fn matrix(output: &Output, m: &Matrix<'_>) {
    for i in 1..=2 {
        for j in 1..=2 {
            real(output, m[(i, j)]);
        }
    }
}

fn fcc_step(output: &Output, f: &mut CCLocFOfLocECC<'_>, uv: &Vector<'_>) {
    let mut m = Matrix::filled(1, 2, 1, 2, 0.0).unwrap();
    attempt(output, || {
        let mut v = vec2(0.0, 0.0);
        flag(output, f.value(uv, &mut v)?);
        real(output, v[1]);
        real(output, v[2]);
        Ok(())
    });
    attempt(output, || {
        flag(output, f.derivatives(uv, &mut m)?);
        matrix(output, &m);
        Ok(())
    });
    attempt(output, || {
        let mut w = vec2(0.0, 0.0);
        flag(output, f.values(uv, &mut w, &mut m)?);
        real(output, w[1]);
        real(output, w[2]);
        matrix(output, &m);
        Ok(())
    });
    int(output, f.get_state_number());
}

fn fcc_body(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let a = Space3::adaptor(r)?;
    let b = Space3::adaptor(r)?;
    let tol = r.real();
    let n = r.integer();
    let mut f = CCLocFOfLocECC::with_curves(&*a, &*b, tol)?;
    int(output, f.nb_variables());
    int(output, f.nb_equations());
    real(output, f.tolerance());
    attempt(output, || Ok(real(output, f.search_of_tolerance(&*a)?)));
    attempt(output, || Ok(real(output, f.search_of_tolerance(&*b)?)));
    let same = |c: Option<&dyn martensite_tkg3d::adaptor3d::Adaptor3dCurve>,
                t: &dyn martensite_tkg3d::adaptor3d::Adaptor3dCurve| {
        c.is_some_and(|c| std::ptr::addr_eq(c, t))
    };
    flag(output, same(f.curve_ptr(1)?, &*a));
    flag(output, same(f.curve_ptr(2)?, &*b));
    attempt(output, || f.curve_ptr(3).map(|_| ()));
    for _ in 0..n {
        let uv = vec2(r.real(), r.real());
        fcc_step(output, &mut f, &uv);
    }
    int(output, f.nb_ext());
    for i in 0..=f.nb_ext() + 1 {
        attempt(output, || {
            real(output, f.square_distance(i)?);
            let (p, s) = f.points(i)?;
            Space3::pon(output, p);
            Space3::pon(output, s);
            Ok(())
        });
    }
    f.set_tolerance(2.0 * tol);
    real(output, f.tolerance());
    attempt(output, || f.set_curve(3, &*a));
    f.set_curve(1, &*b)?;
    flag(output, same(f.curve_ptr(1)?, &*b));
    f.set_curve(2, &*b)?;
    attempt(output, || {
        let mut v = vec2(0.0, 0.0);
        flag(output, f.value(&vec2(0.25, 0.5), &mut v)?);
        real(output, v[1]);
        real(output, v[2]);
        Ok(())
    });
    let g = CCLocFOfLocECC::new(1.0e-10);
    real(output, g.tolerance());
    attempt(output, || Ok(real(output, g.search_of_tolerance(&*a)?)));
    Ok(())
}

pub fn fcc_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || fcc_body(output, &mut r));
    end(output);
}

fn gofcc_body<S: Space>(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let a = S::adaptor(r)?;
    let b = S::adaptor(r)?;
    let n = r.integer();
    let f0 = GlobOptFuncCCC0::<S::Tool>::new(&*a, &*b);
    let f1 = GlobOptFuncCCC1::<S::Tool>::new(&*a, &*b);
    let f2 = GlobOptFuncCCC2::<S::Tool>::new(&*a, &*b);
    int(output, f0.nb_variables());
    int(output, f1.nb_variables());
    int(output, f2.nb_variables());
    for _ in 0..n {
        let x = vec2(r.real(), r.real());
        let mut g = vec2(0.0, 0.0);
        let mut h = Matrix::filled(1, 2, 1, 2, 0.0).unwrap();
        let mut v = 0.0;
        attempt(output, || {
            flag(output, f0.value(&x, &mut v)?);
            real(output, v);
            Ok(())
        });
        attempt(output, || {
            v = 0.0;
            flag(output, f1.value(&x, &mut v)?);
            real(output, v);
            Ok(())
        });
        attempt(output, || {
            flag(output, f1.gradient(&x, &mut g)?);
            real(output, g[1]);
            real(output, g[2]);
            Ok(())
        });
        attempt(output, || {
            v = 0.0;
            flag(output, f1.values(&x, &mut v, &mut g)?);
            real(output, v);
            Ok(())
        });
        attempt(output, || {
            v = 0.0;
            flag(output, f2.values(&x, &mut v, &mut g)?);
            real(output, v);
            real(output, g[1]);
            real(output, g[2]);
            Ok(())
        });
        attempt(output, || {
            v = 0.0;
            flag(output, f2.values_hessian(&x, &mut v, &mut g, &mut h)?);
            real(output, v);
            matrix(output, &h);
            Ok(())
        });
    }
    Ok(())
}

pub fn gofcc_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || gofcc_body::<Space3>(output, &mut r));
    end(output);
}
pub fn gofcc2d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || gofcc_body::<Space2>(output, &mut r));
    end(output);
}
