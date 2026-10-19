//! The GeomConvert conversion requests; the counterpart of
//! the implementation (same request layout, member order and
//! token stream).
// The printers run inside `attempt`, whose closure returns `Ok(printer(...))`.
#![allow(clippy::unit_arg)]
use super::super::common::{Reader, attempt, end, flag, int, pnt, real};
use super::super::geom2d_convert::{PARAMETERISATIONS, SHAPES};
use super::super::{geometry_record_fixtures as build, print::Output};
use super::{bezier, bezier_surface, bspline, bspline_surface, read_bspline, read_bspline_surface};
use martensite_tkernel::ncollection::{Array1, Array2, HArray1};
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::adaptor3d::{Adaptor3dCurve, Adaptor3dSurface};
use martensite_tkg3d::geom::{GeomBSplineCurve, GeomBezierSurface, GeomBoundedCurve};
use martensite_tkg3d::geom_adaptor::{GeomAdaptorCurve, GeomAdaptorSurface};
use martensite_tkgeombase::geom_convert::{
    self as gcv, ApproxCurve, ApproxSurface, BSplineCurveKnotSplitting, BSplineCurveToBezierCurve,
    BSplineSurfaceKnotSplitting, BSplineSurfaceToBezierSurface, CompBezierSurfacesToBSplineSurface,
    CompCurveToBSplineCurve, ConcatOutputs, HArrayOfBSplineCurves,
};

/// gcv_to_bspline.
pub fn to_bspline_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let p = PARAMETERISATIONS[r.integer() as usize];
    attempt(output, || {
        let c = build::curve(&mut r)?;
        Ok(bspline(output, &gcv::curve_to_bspline_curve(&c, p)?))
    });
    end(output);
}

/// gcv_surf_to_bspline.
pub fn surf_to_bspline_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || {
        let s = build::surface(&mut r)?;
        Ok(bspline_surface(
            output,
            &gcv::surface_to_bspline_surface(&s)?,
        ))
    });
    end(output);
}

/// gcv_split.
pub fn split_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let mode = r.integer();
    let (a, b, tol, same) = (r.real(), r.real(), r.real(), r.flag());
    attempt(output, || {
        let c = read_bspline(&mut r)?;
        if mode == 0 {
            return Ok(bspline(
                output,
                &gcv::split_bspline_curve_knots(&c, a as i32, b as i32, same)?,
            ));
        }
        Ok(bspline(
            output,
            &gcv::split_bspline_curve(&c, a, b, tol, same)?,
        ))
    });
    end(output);
}

/// gcv_split_surface.
pub fn split_surface_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let mode = r.integer();
    let (a, b, c, d, tol) = (r.real(), r.real(), r.real(), r.real(), r.real());
    let (usplit, su, sv) = (r.flag(), r.flag(), r.flag());
    attempt(output, || {
        let s = read_bspline_surface(&mut r)?;
        let x = match mode {
            0 => gcv::split_bspline_surface_knots(
                &s,
                (a as i32, b as i32),
                (c as i32, d as i32),
                (su, sv),
            )?,
            1 => {
                gcv::split_bspline_surface_one_direction_knots(&s, a as i32, b as i32, usplit, su)?
            }
            2 => gcv::split_bspline_surface(&s, (a, b), (c, d), tol, (su, sv))?,
            _ => gcv::split_bspline_surface_one_direction(&s, a, b, usplit, tol, su)?,
        };
        Ok(bspline_surface(output, &x))
    });
    end(output);
}

/// gcv_knot_split.
pub fn knot_split_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let range = r.integer();
    attempt(output, || {
        let s = BSplineCurveKnotSplitting::new(&read_bspline(&mut r)?, range)?;
        let n = s.nb_splits();
        int(output, n);
        for i in 0..=n + 1 {
            attempt(output, || Ok(int(output, s.split_value(i)?)));
        }
        let mut v: Array1<'static, i32> = Array1::with_bounds(1, n).unwrap();
        s.splitting(&mut v)?;
        (1..=n).for_each(|i| int(output, v[i]));
        Ok(())
    });
    end(output);
}

/// gcv_knot_split_surface.
pub fn knot_split_surface_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (ur, vr) = (r.integer(), r.integer());
    attempt(output, || {
        let s = BSplineSurfaceKnotSplitting::new(&read_bspline_surface(&mut r)?, ur, vr)?;
        let nu = s.nb_u_splits();
        int(output, nu);
        let mut nv = 0;
        attempt(output, || {
            nv = s.nb_v_splits()?;
            Ok(int(output, nv))
        });
        for i in 0..=nu + 1 {
            attempt(output, || Ok(int(output, s.u_split_value(i)?)));
        }
        for i in 0..=nv + 1 {
            attempt(output, || Ok(int(output, s.v_split_value(i)?)));
        }
        let mut u: Array1<'static, i32> = Array1::with_bounds(1, nu).unwrap();
        let mut v: Array1<'static, i32> = Array1::with_bounds(1, nv).unwrap();
        s.splitting(&mut u, &mut v)?;
        (1..=nu).for_each(|i| int(output, u[i]));
        (1..=nv).for_each(|i| int(output, v[i]));
        Ok(())
    });
    end(output);
}

/// gcv_bezier.
pub fn bezier_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let mode = r.integer();
    let (u1, u2, tol) = (r.real(), r.real(), r.real());
    attempt(output, || {
        let c = read_bspline(&mut r)?;
        let s = if mode == 0 {
            BSplineCurveToBezierCurve::new(&c)?
        } else {
            BSplineCurveToBezierCurve::with_range(&c, u1, u2, tol)?
        };
        let n = s.nb_arcs()?;
        int(output, n);
        let mut k: Array1<'static, f64> = Array1::with_bounds(1, n + 1).unwrap();
        s.knots(&mut k)?;
        (1..=n + 1).for_each(|i| real(output, k[i]));
        for i in 0..=n + 1 {
            attempt(output, || Ok(bezier(output, &s.arc(i)?)));
        }
        let mut arcs = Array1::with_bounds(1, n).unwrap();
        s.arcs(&mut arcs)?;
        (1..=n).for_each(|i| bezier(output, &arcs[i]));
        Ok(())
    });
    end(output);
}

/// gcv_bezier_surface.
pub fn bezier_surface_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let mode = r.integer();
    let (u1, u2, v1, v2, tol) = (r.real(), r.real(), r.real(), r.real(), r.real());
    attempt(output, || {
        let s = read_bspline_surface(&mut r)?;
        let b = if mode == 0 {
            BSplineSurfaceToBezierSurface::new(&s)?
        } else {
            BSplineSurfaceToBezierSurface::with_range(&s, (u1, u2), (v1, v2), tol)?
        };
        let (nu, nv) = (b.nb_u_patches()?, b.nb_v_patches()?);
        int(output, nu);
        int(output, nv);
        let mut uk: Array1<'static, f64> = Array1::with_bounds(1, nu + 1).unwrap();
        let mut vk: Array1<'static, f64> = Array1::with_bounds(1, nv + 1).unwrap();
        b.u_knots(&mut uk)?;
        b.v_knots(&mut vk)?;
        (1..=nu + 1).for_each(|i| real(output, uk[i]));
        (1..=nv + 1).for_each(|i| real(output, vk[i]));
        for i in 0..=nu + 1 {
            attempt(output, || Ok(bezier_surface(output, &b.patch(i, 1)?)));
        }
        for j in 0..=nv + 1 {
            attempt(output, || Ok(bezier_surface(output, &b.patch(1, j)?)));
        }
        let mut p: Array2<'static, Handle<GeomBezierSurface>> =
            Array2::with_bounds(1, nu, 1, nv).unwrap();
        b.patches(&mut p)?;
        for i in 1..=nu {
            for j in 1..=nv {
                bezier_surface(output, p.value(i, j).unwrap());
            }
        }
        Ok(())
    });
    end(output);
}

/// gcv_comp.
pub fn comp_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let p = PARAMETERISATIONS[r.integer() as usize];
    let (with_first, n) = (r.flag(), r.integer());
    attempt(output, || {
        let mut comp: Option<CompCurveToBSplineCurve> = None;
        for i in 0..n {
            let (tol, after, ratio, min_m) = (r.real(), r.flag(), r.flag(), r.integer());
            let c: Handle<dyn GeomBoundedCurve> = build::curve(&mut r)?.cast();
            if i == 0 && with_first {
                comp = Some(CompCurveToBSplineCurve::with_curve(&c, p)?);
                continue;
            }
            let comp = comp.get_or_insert_with(|| CompCurveToBSplineCurve::new(p));
            attempt(output, || {
                Ok(flag(output, comp.add(&c, tol, after, ratio, min_m)?))
            });
        }
        let comp = comp
            .as_mut()
            .expect("the probe corpus adds at least one curve");
        bspline(output, &comp.bspline_curve());
        comp.clear();
        Ok(flag(output, comp.bspline_curve().is_null()))
    });
    end(output);
}

/// gb_gcv_comp_bezier_result.
fn comp_bezier_result(output: &Output, c: &CompBezierSurfacesToBSplineSurface) {
    flag(output, c.is_done());
    [
        c.nb_u_knots(),
        c.nb_u_poles(),
        c.nb_v_knots(),
        c.nb_v_poles(),
    ]
    .into_iter()
    .for_each(|value| int(output, value));
    int(output, c.u_degree());
    int(output, c.v_degree());
    let poles = c.poles();
    let p = poles.get().unwrap().array2();
    for i in p.lower_row()..=p.upper_row() {
        for j in p.lower_col()..=p.upper_col() {
            pnt(output, *p.value(i, j).unwrap());
        }
    }
    for k in [c.u_knots(), c.v_knots()] {
        k.get()
            .unwrap()
            .array1()
            .iter()
            .for_each(|v| real(output, *v));
    }
    for m in [c.u_multiplicities(), c.v_multiplicities()] {
        m.get()
            .unwrap()
            .array1()
            .iter()
            .for_each(|v| int(output, *v));
    }
}

/// gcv_comp_bezier.
pub fn comp_bezier_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (mode, tol, rk) = (r.integer(), r.real(), r.flag());
    let (uc, vc) = (SHAPES[r.integer() as usize], SHAPES[r.integer() as usize]);
    let (nu, nv) = (r.integer(), r.integer());
    let mut uk: Array1<'static, f64> = Array1::with_bounds(1, nu + 1).unwrap();
    let mut vk: Array1<'static, f64> = Array1::with_bounds(1, nv + 1).unwrap();
    (1..=nu + 1).for_each(|i| uk[i] = r.real());
    (1..=nv + 1).for_each(|i| vk[i] = r.real());
    let mut b: Array2<'static, Handle<GeomBezierSurface>> =
        Array2::with_bounds(1, nu, 1, nv).unwrap();
    for i in 1..=nu {
        for j in 1..=nv {
            b.set_value(i, j, build::surface(&mut r).unwrap().cast())
                .unwrap();
        }
    }
    attempt(output, || {
        let c = match mode {
            0 => CompBezierSurfacesToBSplineSurface::new(&b)?,
            1 => CompBezierSurfacesToBSplineSurface::with_tolerance(&b, tol, rk)?,
            _ => CompBezierSurfacesToBSplineSurface::with_knots(&b, &uk, &vk, (uc, vc), tol)?,
        };
        Ok(comp_bezier_result(output, &c))
    });
    for i in 1..=nu {
        for j in 1..=nv {
            bezier_surface(output, b.value(i, j).unwrap());
        }
    }
    end(output);
}

/// gcv_concat.
pub fn concat_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let kind = r.integer();
    let mut closed = r.flag();
    let (ctol, ang) = (r.real(), r.real());
    let nt = r.integer();
    let mut tols: Array1<'static, f64> = Array1::with_bounds(0, nt - 1).unwrap();
    (0..nt).for_each(|i| tols[i] = r.real());
    let nc = r.integer();
    let mut curves: Array1<'static, Handle<GeomBSplineCurve>> =
        Array1::with_bounds(0, nc - 1).unwrap();
    (0..nc).for_each(|i| curves[i] = read_bspline(&mut r).unwrap());
    let mut idx: Handle<HArray1<i32>> = Handle::default();
    let mut res: HArrayOfBSplineCurves = Handle::default();
    attempt(output, || match kind {
        0 => gcv::concat_g1(&mut curves, &tols, &mut res, &mut closed, ctol),
        1 => gcv::concat_c1(&mut curves, &tols, &mut idx, &mut res, &mut closed, ctol),
        _ => {
            let outputs = ConcatOutputs {
                indices: &mut idx,
                concatenated: &mut res,
            };
            gcv::concat_c1_with_angular(&mut curves, &tols, outputs, &mut closed, ctol, ang)
        }
    });
    flag(output, closed);
    match idx.get() {
        None => int(output, -1),
        Some(t) => {
            let t = t.array1();
            int(output, t.lower());
            int(output, t.upper());
            (t.lower()..=t.upper()).for_each(|i| int(output, t[i]));
        }
    }
    curve_table(output, &res);
    (0..nc).for_each(|i| bspline(output, &curves[i]));
    end(output);
}

// The bounds and curves of a B-spline table, -1 when null.
fn curve_table(output: &Output, table: &HArrayOfBSplineCurves) {
    let Some(t) = table.get() else {
        return int(output, -1);
    };
    let t = t.array1();
    int(output, t.lower());
    int(output, t.upper());
    (t.lower()..=t.upper()).for_each(|i| bspline(output, &t[i]));
}

/// gcv_c0c1.
pub fn c0c1_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let mode = r.integer();
    let (tol, ang) = (r.real(), r.real());
    attempt(output, || {
        let mut c = read_bspline(&mut r)?;
        if mode == 0 {
            gcv::c0_bspline_to_c1_bspline_curve(&mut c, tol, ang)?;
            return Ok(bspline(output, &c));
        }
        let mut t: HArrayOfBSplineCurves = Handle::default();
        attempt(output, || {
            if mode == 1 {
                gcv::c0_bspline_to_array_of_c1_bspline_curve(&c, &mut t, tol)
            } else {
                gcv::c0_bspline_to_array_of_c1_bspline_curve_with_angular(&c, &mut t, ang, tol)
            }
        });
        Ok(curve_table(output, &t))
    });
    end(output);
}

/// gcv_approx_curve.
pub fn approx_curve_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (via_adaptor, ranged) = (r.flag(), r.flag());
    let (u1, u2, tol) = (r.real(), r.real(), r.real());
    let order = SHAPES[r.integer() as usize];
    let (seg, deg) = (r.integer(), r.integer());
    attempt(output, || {
        let c = build::curve(&mut r)?;
        let a = if via_adaptor {
            let adaptor = if ranged {
                GeomAdaptorCurve::with_range(&c, u1, u2)?
            } else {
                GeomAdaptorCurve::with_curve(&c)?
            };
            let h: Handle<dyn Adaptor3dCurve> = Handle::new(adaptor).cast();
            ApproxCurve::from_adaptor(&h, tol, order, seg, deg)?
        } else {
            ApproxCurve::from_curve(&c, tol, order, seg, deg)?
        };
        flag(output, a.is_done());
        flag(output, a.has_result());
        real(output, a.max_error());
        Ok(bspline(output, &a.curve()))
    });
    end(output);
}

/// gcv_approx_surface.
pub fn approx_surface_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (via_adaptor, tol) = (r.flag(), r.real());
    let (uc, vc) = (SHAPES[r.integer() as usize], SHAPES[r.integer() as usize]);
    let (du, dv, seg, precis) = (r.integer(), r.integer(), r.integer(), r.integer());
    attempt(output, || {
        let s = build::surface(&mut r)?;
        let a = if via_adaptor {
            let h: Handle<dyn Adaptor3dSurface> =
                Handle::new(GeomAdaptorSurface::with_surface(&s)?).cast();
            ApproxSurface::from_adaptor(&h, tol, (uc, vc), (du, dv), seg, precis)?
        } else {
            ApproxSurface::from_surface(&s, tol, (uc, vc), (du, dv), seg, precis)?
        };
        flag(output, a.is_done());
        flag(output, a.has_result());
        real(output, a.max_error());
        Ok(bspline_surface(output, &a.surface()))
    });
    end(output);
}
