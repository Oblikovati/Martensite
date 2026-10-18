//! The Rust twin of the implementation.
//! Original Approx adaptor requests, with operation-local output and unchanged algorithms.
use super::{
    common::{self as p, Reader},
    geom_convert::{bspline, curve2d as print_curve2d},
    geom2d_convert::bspline as bspline2d,
    geometry_adaptor_fixtures::{adapted_curve, adapted_curve2d, adapted_surface, shape},
    geometry_record_fixtures::{curve, curve2d, surface},
    print::{Output, attempt, end},
};
use martensite_tkernel::{
    ncollection::Array1,
    standard::{Failure, Handle},
};
use martensite_tkg2d::{adaptor2d::Adaptor2dCurve2d, geom2d_adaptor::Geom2dAdaptorCurve};
use martensite_tkg3d::{
    adaptor3d::{Adaptor3dCurve, Adaptor3dSurface},
    geom_adaptor::{GeomAdaptorCurve, GeomAdaptorSurface},
};
use martensite_tkgeombase::approx::{
    Curve2d, Curve3d, CurveOnSurface, CurveOnSurfaceOptions, CurvilinearOptions,
    CurvilinearParameter, CurvlinFunc, SameParameter,
};
fn dump(output: &Output, text: &str) {
    output.write(format_args!("{} ", text.replace('\n', "|")));
}

pub fn curve3d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (tol, order) = (r.real(), shape(r.integer()));
    let (seg, deg) = (r.integer(), r.integer());
    attempt(output, || {
        let c = adapted_curve(&mut r)?;
        let a = Curve3d::new(&c, tol, order, seg, deg)?;
        p::flag(output, a.is_done());
        p::flag(output, a.has_result());
        bspline(output, &a.curve());
        p::real(output, a.max_error());
        if a.has_result() {
            let mut text = String::new();
            a.dump(&mut text)?;
            dump(output, &text);
        }
        Ok(())
    });
    end(output);
}

pub fn curve2d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (f, l, tu, tv) = (r.real(), r.real(), r.real(), r.real());
    let cont = shape(r.integer());
    let (deg, seg) = (r.integer(), r.integer());
    attempt(output, || {
        let c = adapted_curve2d(&mut r)?;
        let a = Curve2d::new(&c, f, l, tu, tv, cont, deg, seg)?;
        p::flag(output, a.is_done());
        p::flag(output, a.has_result());
        bspline2d(output, &a.curve());
        p::real(output, a.max_error_2d_u());
        p::real(output, a.max_error_2d_v());
        Ok(())
    });
    end(output);
}

fn cos_results(output: &Output, a: &CurveOnSurface) {
    p::flag(output, a.is_done());
    p::flag(output, a.has_result());
    bspline(output, &a.curve_3d());
    p::real(output, a.max_error_3d());
    bspline2d(output, &a.curve_2d());
    p::real(output, a.max_error_2d_u());
    p::real(output, a.max_error_2d_v());
}

pub fn cos_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let variant = r.integer();
    let (f, l, tol) = (r.real(), r.real(), r.real());
    let continuity = shape(r.integer());
    let (max_degree, max_segments) = (r.integer(), r.integer());
    let (only_3d, only_2d) = (r.flag(), r.flag());
    let options = CurveOnSurfaceOptions {
        max_segments,
        max_degree,
        continuity,
        only_3d,
        only_2d,
    };
    attempt(output, || {
        let c = adapted_curve2d(&mut r)?;
        let s = adapted_surface(&mut r)?;
        if variant == 0 {
            cos_results(output, &CurveOnSurface::new(&c, &s, f, l, tol, options)?);
        } else {
            let mut a = CurveOnSurface::without_perform(&c, &s, f, l, tol);
            cos_results(output, &a);
            a.perform(options)?;
            cos_results(output, &a);
        }
        Ok(())
    });
    end(output);
}

fn curvlin(r: &mut Reader<'_>, kind: i32, tol: f64) -> Result<CurvlinFunc, Failure> {
    if kind == 1 {
        return CurvlinFunc::from_curve(&adapted_curve(r)?, tol);
    }
    let c1 = adapted_curve2d(r)?;
    let s1 = adapted_surface(r)?;
    if kind == 2 {
        return CurvlinFunc::from_curve_on_surface(&c1, &s1, tol);
    }
    let c2 = adapted_curve2d(r)?;
    let s2 = adapted_surface(r)?;
    CurvlinFunc::from_two_curves_on_surfaces(&c1, &c2, &s1, &s2, tol)
}

fn curvlin_command(
    output: &Output,
    r: &mut Reader<'_>,
    f: &CurvlinFunc,
    kind: i32,
    c3d: &dyn Adaptor3dCurve,
) -> Result<(), Failure> {
    match r.integer() {
        1 => [f.first_parameter(), f.last_parameter(), f.get_length()]
            .into_iter()
            .for_each(|value| p::real(output, value)),
        2 => {
            let s = shape(r.integer());
            let n = f.nb_intervals(s)?;
            p::int(output, n);
            let mut t = Array1::with_bounds(1, n + 1).unwrap();
            f.intervals(&mut t, s)?;
            t.iter().for_each(|&v| p::real(output, v));
        }
        3 => p::real(output, f.get_s_parameter(r.real())?),
        4 => {
            let (s, order) = (r.real(), r.integer());
            let mut res = Array1::with_bounds(0, 2 * kind).unwrap();
            res.init(-7.);
            let done = match kind {
                1 => f.eval_case1(s, order, &mut res)?,
                2 => f.eval_case2(s, order, &mut res)?,
                _ => f.eval_case3(s, order, &mut res)?,
            };
            p::flag(output, done);
            res.iter().for_each(|&v| p::real(output, v));
        }
        5 => {
            let (a, b, t) = (r.real(), r.real(), r.real());
            f.trim(a, b, t)?;
            p::real(output, f.first_parameter());
            p::real(output, f.last_parameter());
        }
        6 => {
            f.length()?;
            p::real(output, f.get_length());
        }
        7 => f.set_tol(r.real()),
        8 => p::real(output, f.get_u_parameter(c3d, r.real(), 1)?),
        _ => {
            let (a, b) = (r.real(), r.real());
            p::real(output, f.length_of(c3d, a, b)?);
        }
    }
    Ok(())
}

pub fn curvlin_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (kind, tol) = (r.integer(), r.real());
    attempt(output, || {
        let f = curvlin(&mut r, kind, tol)?;
        let c3d = adapted_curve(&mut r)?;
        let c3d = c3d.get().expect("probe curve");
        for _ in 0..r.integer() {
            attempt(output, || curvlin_command(output, &mut r, &f, kind, c3d));
        }
        Ok(())
    });
    end(output);
}

fn curvilinear_results(
    output: &Output,
    a: &CurvilinearParameter,
    kind: i32,
) -> Result<(), Failure> {
    p::flag(output, a.is_done());
    p::flag(output, a.has_result());
    bspline(output, &a.curve_3d());
    p::real(output, a.max_error_3d());
    bspline2d(output, &a.curve_2d1());
    p::real(output, a.max_error_2d1());
    bspline2d(output, &a.curve_2d2());
    // Preserve the existing probe omission: the retained case-2 constructor leaves this field uninitialized.
    if kind != 2 {
        p::real(output, a.max_error_2d2());
    }
    let mut text = String::new();
    a.dump(&mut text).expect("string");
    dump(output, &text);
    Ok(())
}

pub fn curvilinear_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (kind, tol) = (r.integer(), r.real());
    let order = shape(r.integer());
    let (max_degree, max_segments) = (r.integer(), r.integer());
    let options = CurvilinearOptions {
        order,
        max_degree,
        max_segments,
    };
    attempt(output, || {
        let a = match kind {
            1 => CurvilinearParameter::from_curve(&adapted_curve(&mut r)?, tol, options)?,
            2 => {
                let c = adapted_curve2d(&mut r)?;
                let s = adapted_surface(&mut r)?;
                CurvilinearParameter::from_curve_on_surface(&c, &s, tol, options)?
            }
            _ => {
                let c1 = adapted_curve2d(&mut r)?;
                let s1 = adapted_surface(&mut r)?;
                let c2 = adapted_curve2d(&mut r)?;
                let s2 = adapted_surface(&mut r)?;
                CurvilinearParameter::from_two_curves_on_surfaces(
                    [&c1, &c2],
                    [&s1, &s2],
                    tol,
                    options,
                )?
            }
        };
        curvilinear_results(output, &a, kind)
    });
    end(output);
}

fn same_results(output: &Output, a: &SameParameter) -> Result<(), Failure> {
    p::flag(output, a.is_done());
    p::flag(output, a.is_same_parameter());
    p::real(output, a.tol_reached());
    print_curve2d(output, &a.curve_2d());
    let cos = a.curve_on_surface();
    p::flag(output, cos.is_null());
    let Some(cos) = cos.get() else {
        return Ok(());
    };
    let (f, l) = (cos.first_parameter()?, cos.last_parameter()?);
    p::real(output, f);
    p::real(output, l);
    for i in 0..=4 {
        p::pnt(output, cos.value(f + (l - f) * f64::from(i) / 4.)?);
    }
    Ok(())
}

pub fn same_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (variant, tol) = (r.integer(), r.real());
    attempt(output, || {
        let c3 = curve(&mut r)?;
        let c2 = curve2d(&mut r)?;
        let s = surface(&mut r)?;
        let a3: Handle<dyn Adaptor3dCurve> = Handle::new(GeomAdaptorCurve::with_curve(&c3)?).cast();
        let a_s: Handle<dyn Adaptor3dSurface> =
            Handle::new(GeomAdaptorSurface::with_surface(&s)?).cast();
        let a = match variant {
            0 => SameParameter::from_geometry(&c3, &c2, &s, tol)?,
            1 => SameParameter::from_adaptors_and_pcurve(&a3, &c2, &a_s, tol)?,
            _ => {
                let a2: Handle<dyn Adaptor2dCurve2d> =
                    Handle::new(Geom2dAdaptorCurve::with_curve(&c2)?).cast();
                SameParameter::from_adaptors(&a3, &a2, &a_s, tol)?
            }
        };
        same_results(output, &a)
    });
    end(output);
}
