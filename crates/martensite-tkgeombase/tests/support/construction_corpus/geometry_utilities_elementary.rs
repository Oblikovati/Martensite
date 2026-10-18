//! Original elementary GeomLib families share typed readers and explicit local output.
#![allow(clippy::unit_arg)]
use super::common::{Reader, attempt, ax2, dir, end, flag, int, pnt, real};
use super::{fixture_input::GeometryFixtureInput, numeric_protocol::UNSET, print::Output};
use martensite_tkernel::ncollection::{Array1, LinearVector};
use martensite_tkgeombase::geom_lib::{self as gl, LogSample, PolyFunc};
use martensite_tkmath::gp::{Axis2, Direction3, Point3};
use martensite_tkmath::math::{Function, FunctionWithDerivative, Vector};

/// gl_inertia.
pub fn inertia_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (tol, n) = (r.real(), r.integer());
    let mut p: Array1<'static, Point3> = Array1::with_bounds(1, n).unwrap();
    (1..=n).for_each(|i| p[i] = r.pnt());
    let (mut b, mut x, mut y) = (
        Point3::default(),
        Direction3::default(),
        Direction3::default(),
    );
    let mut gaps = [UNSET; 3];
    attempt(output, || {
        let axes = gl::inertia(&p)?;
        (b, x, y, gaps) = (
            axes.bary,
            axes.x_dir,
            axes.y_dir,
            [axes.x_gap, axes.y_gap, axes.z_gap],
        );
        Ok(())
    });
    pnt(output, b);
    dir(output, x);
    dir(output, y);
    gaps.into_iter().for_each(|value| real(output, value));
    let (mut a, mut singular) = (Axis2::default(), false);
    attempt(output, || {
        let axe = gl::axe_of_inertia(&p, tol)?;
        (a, singular) = (axe.axe, axe.is_singular);
        Ok(())
    });
    ax2(output, a);
    flag(output, singular);
    end(output);
}

fn reals(output: &Output, a: &Array1<'_, f64>) {
    int(output, a.lower());
    int(output, a.upper());
    a.iter().for_each(|v| real(output, *v));
}

pub fn read_reals(r: &mut impl GeometryFixtureInput) -> Array1<'static, f64> {
    let (lower, n) = (r.integer(), r.integer());
    if n == 0 {
        return Array1::default();
    }
    let mut a = Array1::with_bounds(lower, lower + n - 1).unwrap();
    (lower..lower + n).for_each(|i| a[i] = r.real());
    a
}

/// gl_arrays.
pub fn arrays_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (mode, count) = (r.integer(), r.integer());
    let a = read_reals(&mut r);
    if mode == 2 {
        let b = read_reals(&mut r);
        let (conf, adjust) = (r.real(), r.flag());
        let mut f: LinearVector<f64> = LinearVector::default();
        f.append(-1.0);
        gl::fuse_intervals(&a, &b, &mut f, conf, adjust);
        int(output, f.len() as i32);
        f.iter().for_each(|v| real(output, *v));
        return end(output);
    }
    let mut out: Array1<'static, f64> = Array1::default();
    attempt(output, || {
        if mode == 0 {
            gl::remove_points_from_array(count, &a, &mut out)?;
        } else {
            gl::densify_array1_of_real(count, &a, &mut out)?;
        }
        Ok(reals(output, &out))
    });
    end(output);
}

/// gl_log_sample.
pub fn log_sample_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (a, b, n) = (r.real(), r.real(), r.integer());
    let s = LogSample::new(a, b, n);
    for i in 0..=n + 1 {
        attempt(output, || Ok(real(output, s.get_parameter(i)?)));
    }
    end(output);
}

/// gl_poly_func.
pub fn poly_func_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (lower, n) = (r.integer(), r.integer());
    let mut c: Vector<'static> = Vector::with_bounds(lower, lower + n - 1).unwrap();
    (lower..lower + n).for_each(|i| c[i] = r.real());
    let x = r.real();
    attempt(output, || {
        let mut f = PolyFunc::new(&c)?;
        let (mut v, mut d) = (UNSET, UNSET);
        let ok = f.value(x, &mut v);
        f.take_failure()?;
        flag(output, ok);
        real(output, v);
        let ok = f.derivative(x, &mut d);
        f.take_failure()?;
        flag(output, ok);
        real(output, d);
        (v, d) = (UNSET, UNSET);
        let ok = f.values(x, &mut v, &mut d);
        f.take_failure()?;
        flag(output, ok);
        real(output, v);
        Ok(real(output, d))
    });
    end(output);
}
