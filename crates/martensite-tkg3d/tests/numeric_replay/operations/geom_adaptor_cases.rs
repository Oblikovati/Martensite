//! Adaptor range, continuity and derivative requests.
use super::bspl::desc;
use super::common::{
    Reader, attempt, ax1, ax3, built_curve, built_surface, dir, end, flag, int, name, real, show,
    trsf,
};
use super::geom_adaptor::{
    curve_adaptor_summary, curve_intervals, curve_kinds, make_curve_summary, make_surface_summary,
    surface_adaptor_summary, surface_kinds,
};
use super::output::Output;
use super::shapes;
use super::swept::surface_desc;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::{
    Adaptor3dCurve, Adaptor3dCurveBase, Adaptor3dSurface, Adaptor3dSurfaceBase,
};
use martensite_tkg3d::geom::{
    GeomBSplineCurve, GeomBSplineSurface, GeomBezierCurve, GeomBezierSurface, GeomCurve,
    GeomOffsetCurve, GeomOffsetSurface, GeomSurface, GeomSurfaceOfLinearExtrusion,
    GeomSurfaceOfRevolution,
};
use martensite_tkg3d::geom_adaptor::{
    GeomAdaptorCurve, GeomAdaptorSurface, GeomAdaptorSurfaceOfLinearExtrusion,
    GeomAdaptorSurfaceOfRevolution, GeomAdaptorTransformedSurface,
};
use martensite_tkg3d::geom_eval::{RepCurveDesc, RepSurfaceDesc};
use martensite_tkmath::geom_abs::Shape;
use martensite_tkmath::gp::{Axis1, Direction3, Transform3};
use std::f64::consts::PI;

fn report<T>(output: &Output, built: Result<T, Failure>) -> Option<T> {
    match built {
        Ok(value) => Some(value),
        Err(error) => {
            replay_println!(output, "{}", error.exception_type());
            None
        }
    }
}

/// a3_base.
pub fn base_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let u = r.reals();
    let c: Handle<dyn Adaptor3dCurve> = Handle::new(Adaptor3dCurveBase::default()).cast();
    curve_adaptor_summary(output, &c, &u);
    attempt(output, || c.shallow_copy().map(|_| ()));
    attempt(output, || c.trim(0., 1., 1e-7).map(|_| ()));
    make_curve_summary(output, &*c, &u);
    let s: Handle<dyn Adaptor3dSurface> = Handle::new(Adaptor3dSurfaceBase::default()).cast();
    surface_adaptor_summary(output, &s, &u);
    attempt(output, || s.shallow_copy().map(|_| ()));
    attempt(output, || s.u_trim(0., 1., 1e-7).map(|_| ()));
    attempt(output, || s.v_trim(0., 1., 1e-7).map(|_| ()));
    make_surface_summary(output, &*s, &u);
    end(output);
}

// a3_geom_curve: 0 GeomAdaptorCurve(C) | 1 GeomAdaptorCurve(C, u1, u2) <curve>.
pub fn geom_curve(output: &Output, r: &mut Reader) -> Result<Handle<GeomAdaptorCurve>, Failure> {
    let kind = r.integer();
    if kind == 0 {
        return Ok(Handle::new(GeomAdaptorCurve::with_curve(&shapes::curve(
            output, r,
        )?)?));
    }
    let (u1, u2) = (r.real(), r.real());
    Ok(Handle::new(GeomAdaptorCurve::with_range(
        &shapes::curve(output, r)?,
        u1,
        u2,
    )?))
}

/// a3_curve.
pub fn curve_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(a) = report(output, geom_curve(output, &mut r)) else {
        return;
    };
    let u = r.reals();
    let (t1, t2) = (r.real(), r.real());
    let adaptor: Handle<dyn Adaptor3dCurve> = a.clone().cast();
    name(output, &a.curve());
    curve_adaptor_summary(output, &adaptor, &u);
    make_curve_summary(output, &*adaptor, &u);
    attempt(output, || {
        let t = a.trim(t1, t2, 1e-7)?;
        curve_adaptor_summary(output, &t, &u);
        make_curve_summary(output, &*t, &u);
        Ok(())
    });
    show(output, a.shallow_copy(), |c| {
        curve_adaptor_summary(output, &c, &u)
    });
    end(output);
}

// a3_empty_curve.
fn empty_curve(output: &Output, a: &Handle<GeomAdaptorCurve>) {
    name(output, &a.curve());
    real(output, a.first_parameter().unwrap());
    real(output, a.last_parameter().unwrap());
    int(output, a.get_type().unwrap() as i32);
    show(output, a.continuity(), |s| int(output, s as i32));
    curve_intervals(output, &**a);
    for r in [1e-3, 4.0] {
        show(output, a.resolution(r), |value| real(output, value));
    }
    curve_kinds(output, &**a);
    attempt(output, || a.trim(0., 1., 1e-7).map(|_| ()));
    show(output, a.shallow_copy(), |c| {
        real(output, c.last_parameter().unwrap())
    });
    make_curve_summary(output, &**a, &[]);
}

/// a3_curve_load.
pub fn curve_load_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c1) = built_curve(output, &mut r) else {
        return;
    };
    let c2 = shapes::curve(output, &mut r).unwrap();
    let u = r.reals();
    let (u1, u2) = (r.real(), r.real());
    let a = Handle::new(GeomAdaptorCurve::default());
    let dyn_a: Handle<dyn Adaptor3dCurve> = a.clone().cast();
    empty_curve(output, &a);
    attempt(output, || a.load(&Handle::default()));
    attempt(output, || a.load_range(&Handle::default(), 0., 1.));
    attempt(output, || {
        a.load(&c1)?;
        name(output, &a.curve());
        curve_adaptor_summary(output, &dyn_a, &u);
        Ok(())
    });
    attempt(output, || {
        a.load_range(&c1, u1, u2)?;
        curve_adaptor_summary(output, &dyn_a, &u);
        Ok(())
    });
    attempt(output, || a.load_range(&c1, u2 + 1., u2));
    real(output, a.first_parameter().unwrap());
    real(output, a.last_parameter().unwrap());
    attempt(output, || {
        a.load_range(&c2, u1, u2)?;
        name(output, &a.curve());
        curve_adaptor_summary(output, &dyn_a, &u);
        Ok(())
    });
    // `a->Reset()`: `Handle::reset` is `a.reset(new ...)`, so dereference explicitly.
    (*a).reset();
    empty_curve(output, &a);
    attempt(output, || {
        a.load(&c2)?;
        curve_adaptor_summary(output, &dyn_a, &u);
        Ok(())
    });
    end(output);
}

// a3_set_curve_rep: set (or clear, for a null descriptor) the representation.
fn set_curve_rep(
    _output: &Output,
    c: &Handle<dyn GeomCurve>,
    d: &Handle<dyn RepCurveDesc>,
) -> Result<(), Failure> {
    macro_rules! apply {
        ($class:ty) => {
            if let Some(curve) = c.cast::<$class>().get() {
                if d.is_null() {
                    curve.clear_eval_representation();
                    return Ok(());
                }
                return curve.set_eval_representation(d.clone());
            }
        };
    }
    apply!(GeomBezierCurve);
    apply!(GeomBSplineCurve);
    apply!(GeomOffsetCurve);
    Ok(())
}

/// a3_curve_evalrep.
pub fn curve_evalrep_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built_curve(output, &mut r) else {
        return;
    };
    let rep = shapes::curve(output, &mut r).unwrap();
    let d = desc(output, &mut r, rep);
    let u = r.reals();
    let (u1, u2) = (r.real(), r.real());
    attempt(output, || set_curve_rep(output, &c, &d));
    let a = Handle::new(GeomAdaptorCurve::with_range(&c, u1, u2).unwrap());
    let dyn_a: Handle<dyn Adaptor3dCurve> = a.clone().cast();
    curve_adaptor_summary(output, &dyn_a, &u);
    set_curve_rep(output, &c, &Handle::default()).unwrap();
    curve_adaptor_summary(output, &dyn_a, &u);
    a.load_range(&c, u1, u2).unwrap();
    curve_adaptor_summary(output, &dyn_a, &u);
    a.load_range(&c.copy().unwrap().cast(), u1, u2).unwrap();
    curve_adaptor_summary(output, &dyn_a, &u);
    end(output);
}

include!("geom_adaptor_cases/surface_cases.rs");
