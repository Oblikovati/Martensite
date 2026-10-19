//! The GeomLib static member requests; the counterpart of
//! the implementation (same request layout and token stream).
// The printers run inside `attempt`, whose closure returns `Ok(printer(...))`.
#![allow(clippy::unit_arg)]
use super::common::{Reader, attempt, dir, end, flag, int, real};
use super::geom_convert::{UNSET, bspline_surface, curve, curve2d, read_bspline_surface, surface};
use super::geometry_record_fixtures as build;
use super::geometry_utilities_elementary::read_reals;
use super::print::Output;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkg2d::geom2d::Geom2dCurve;
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkg3d::adaptor3d::{Adaptor3dCurveOnSurface, Adaptor3dSurface};
use martensite_tkg3d::geom::{
    GeomBSplineSurface, GeomBezierSurface, GeomBoundedCurve, GeomBoundedSurface, GeomCurve,
    GeomSurface,
};
use martensite_tkg3d::geom_adaptor::{GeomAdaptorCurve, GeomAdaptorSurface};
use martensite_tkgeombase::geom_lib::{self as gl, IsoLine};
use martensite_tkmath::geom_abs::Shape;
use martensite_tkmath::gp::{Direction3, GeneralTransform2, Point2};

const SHAPES: [Shape; 7] = super::geom2d_convert::SHAPES;

pub(crate) fn curve_on_surface(
    c: &Handle<dyn Geom2dCurve>,
    s: &Handle<dyn GeomSurface>,
) -> Result<Adaptor3dCurveOnSurface, Failure> {
    let c2d: Handle<dyn Adaptor2dCurve2d> = Handle::new(Geom2dAdaptorCurve::with_curve(c)?).cast();
    let surf: Handle<dyn Adaptor3dSurface> =
        Handle::new(GeomAdaptorSurface::with_surface(s)?).cast();
    Adaptor3dCurveOnSurface::with_curve(&c2d, &surf)
}

/// gl_to3d.
pub fn to3d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let a = r.ax2();
    attempt(output, || {
        let c = build::curve2d(&mut r)?;
        Ok(curve(output, &gl::to_3d(a, &c)?))
    });
    end(output);
}

/// gl_gtransform.
pub fn gtransform_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let mut g = GeneralTransform2::default();
    for i in 1..=2 {
        for j in 1..=2 {
            g.set_value(i, j, r.real()).unwrap();
        }
    }
    let (t1, t2) = (r.real(), r.real());
    g.set_value(1, 3, t1).unwrap();
    g.set_value(2, 3, t2).unwrap();
    attempt(output, || {
        let c = build::curve2d(&mut r)?;
        Ok(curve2d(output, &gl::gtransform(&c, &g)?))
    });
    end(output);
}

/// gl_same_range.
pub fn same_range_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (tol, f, l, rf, rl) = (r.real(), r.real(), r.real(), r.real(), r.real());
    attempt(output, || {
        let c = build::curve2d(&mut r)?;
        let mut nc: Handle<dyn Geom2dCurve> = Handle::default();
        attempt(output, || {
            gl::same_range(tol, &c, (f, l), (rf, rl), &mut nc)
        });
        Ok(curve2d(output, &nc))
    });
    end(output);
}

/// gl_build_curve3d.
pub fn build_curve3d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (tol, f, l) = (r.real(), r.real(), r.real());
    let cont = SHAPES[r.integer() as usize];
    let (deg, seg) = (r.integer(), r.integer());
    attempt(output, || {
        let c = build::curve2d(&mut r)?;
        let s = build::surface(&mut r)?;
        let cos = curve_on_surface(&c, &s)?;
        let mut nc: Handle<dyn GeomCurve> = Handle::default();
        let (mut mx, mut av) = (UNSET, UNSET);
        attempt(output, || {
            (mx, av) = gl::build_curve3d(tol, &cos, (f, l), &mut nc, cont, deg, seg)?;
            Ok(())
        });
        curve(output, &nc);
        real(output, mx);
        Ok(real(output, av))
    });
    end(output);
}

/// gl_adjust.
pub fn adjust_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (p1, p2, t1, t2) = (r.pnt(), r.pnt(), r.vec(), r.vec());
    attempt(output, || {
        let mut c: Handle<dyn GeomBoundedCurve> = build::curve(&mut r)?.cast();
        attempt(output, || gl::adjust_extremity(&mut c, p1, p2, t1, t2));
        Ok(curve(output, &c.cast()))
    });
    end(output);
}

/// gl_extend_curve.
pub fn extend_curve_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (cont, after, p) = (r.integer(), r.flag(), r.pnt());
    attempt(output, || {
        let mut c: Handle<dyn GeomBoundedCurve> = build::curve(&mut r)?.cast();
        attempt(output, || gl::extend_curve_to_point(&mut c, p, cont, after));
        Ok(curve(output, &c.cast()))
    });
    end(output);
}

/// gl_extend_surf.
pub fn extend_surf_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (len, cont, in_u, after) = (r.real(), r.integer(), r.flag(), r.flag());
    attempt(output, || {
        let mut s: Handle<dyn GeomBoundedSurface> = build::surface(&mut r)?.cast();
        attempt(output, || {
            gl::extend_surf_by_length(&mut s, len, cont, in_u, after)
        });
        Ok(surface(output, &s.cast()))
    });
    end(output);
}

/// gl_eval_max.
pub fn eval_max_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (mode, tol) = (r.integer(), r.real());
    let p = read_reals(&mut r);
    attempt(output, || {
        let a = build::curve(&mut r)?;
        let b = build::curve(&mut r)?;
        let (ca, cb) = (
            GeomAdaptorCurve::with_curve(&a)?,
            GeomAdaptorCurve::with_curve(&b)?,
        );
        let mut d = UNSET;
        attempt(output, || {
            d = if mode == 0 {
                gl::eval_max_parametric_distance(&ca, &cb, tol, &p)?
            } else {
                gl::eval_max_distance_along_parameter(&ca, &cb, tol, &p)?
            };
            Ok(())
        });
        Ok(real(output, d))
    });
    end(output);
}

/// gl_cancel_denom.
pub fn cancel_denom_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (ud, vd) = (r.flag(), r.flag());
    attempt(output, || {
        let mut s = read_bspline_surface(&mut r)?;
        attempt(output, || gl::cancel_denominator_derivative(&mut s, ud, vd));
        Ok(bspline_surface(output, &s))
    });
    end(output);
}

/// gl_norm_estim.
pub fn norm_estim_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (u, v, tol) = (r.real(), r.real(), r.real());
    attempt(output, || {
        let s = build::surface(&mut r)?;
        let mut n = Direction3::default();
        int(output, gl::norm_estim(&s, Point2::new(u, v), tol, &mut n)?);
        Ok(dir(output, n))
    });
    end(output);
}

/// gl_is_closed.
pub fn is_closed_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let mode = r.integer();
    let (p1, p2, tol) = (r.real(), r.real(), r.real());
    attempt(output, || {
        let s = build::surface(&mut r)?;
        if mode == 0 {
            let (u, v) = gl::is_closed(&s, tol)?;
            flag(output, u);
            return Ok(flag(output, v));
        }
        if mode <= 2 {
            let b: Handle<GeomBSplineSurface> = s.cast();
            let closed = if mode == 1 {
                gl::is_bspl_u_closed(&b, p1, p2, tol)?
            } else {
                gl::is_bspl_v_closed(&b, p1, p2, tol)?
            };
            return Ok(flag(output, closed));
        }
        let b: Handle<GeomBezierSurface> = s.cast();
        let closed = if mode == 3 {
            gl::is_bz_u_closed(&b, p1, p2, tol)?
        } else {
            gl::is_bz_v_closed(&b, p1, p2, tol)?
        };
        Ok(flag(output, closed))
    });
    end(output);
}

/// gl_iso_line.
pub fn iso_line_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (f, l, tol, explicit) = (r.real(), r.real(), r.real(), r.flag());
    let given = IsoLine {
        is_u: r.flag(),
        param: r.real(),
        is_forward: r.flag(),
    };
    attempt(output, || {
        let c = build::curve2d(&mut r)?;
        let s = build::surface(&mut r)?;
        let c2d: Handle<dyn Adaptor2dCurve2d> =
            Handle::new(Geom2dAdaptorCurve::with_range(&c, f, l)?).cast();
        let sa: Handle<dyn Adaptor3dSurface> =
            Handle::new(GeomAdaptorSurface::with_surface(&s)?).cast();
        let detected = gl::is_iso_line(&c2d)?;
        let shown = detected.unwrap_or(IsoLine {
            is_u: false,
            param: UNSET,
            is_forward: false,
        });
        flag(output, detected.is_some());
        flag(output, shown.is_u);
        real(output, shown.param);
        flag(output, shown.is_forward);
        let iso = if explicit { given } else { shown };
        if detected.is_some() || explicit {
            attempt(output, || {
                Ok(curve(
                    output,
                    &gl::build_c3d_on_iso_line(&c2d, &sa, (f, l), tol, iso)?,
                ))
            });
        }
        Ok(())
    });
    end(output);
}
