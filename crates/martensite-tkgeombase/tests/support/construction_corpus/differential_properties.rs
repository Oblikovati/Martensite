//! The LProp and GeomLProp probe; the counterpart of the implementation.
use super::geometry_fixtures as build;
use super::print::Output;

use super::common::{self as p, Reader, attempt, end};
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkg3d::geom_adaptor::GeomAdaptorCurve;
use martensite_tkgeombase::geom_lprop::{self, CLPropsBase, CurAndInf2d, SLProps};
use martensite_tkgeombase::lprop::CurAndInf;
use martensite_tkgeombase::lprop::curve_access::{CurveAccess, DirectAccess, PropsVector};
use martensite_tkmath::gp::{Direction2, Direction3, Point2, Point3, Vector2, Vector3};

/// The overloaded gb_lp printers.
pub trait Out {
    fn out(self, output: &Output);
}
impl Out for Point3 {
    fn out(self, output: &Output) {
        p::pnt(output, self);
    }
}
impl Out for Point2 {
    fn out(self, output: &Output) {
        p::pnt2d(output, self);
    }
}
impl Out for Vector3 {
    fn out(self, output: &Output) {
        p::vec(output, self);
    }
}
impl Out for Vector2 {
    fn out(self, output: &Output) {
        p::vec2d(output, self);
    }
}
impl Out for Direction3 {
    fn out(self, output: &Output) {
        p::dir(output, self);
    }
}
impl Out for Direction2 {
    fn out(self, output: &Output) {
        p::dir2d(output, self);
    }
}

/// A printable Result, or the exception type (the body of fallible result capture).
fn out<T: Out>(output: &Output, value: Result<T, Failure>) {
    p::show(output, value, |value| value.out(output));
}

type Props<V, C> = CLPropsBase<V, C, DirectAccess>;

/// gb_lp_clprops_members.
fn clprops_members<V, C>(output: &Output, props: &mut Props<V, C>, mode: i32)
where
    V: PropsVector + Out,
    V::Point: Out,
    V::Direction: Out,
    DirectAccess: CurveAccess<C, V>,
{
    if mode == 1 {
        out(output, props.centre_of_curvature());
        out(output, props.normal());
        p::show(output, props.curvature(), |value| p::real(output, value));
        out(output, props.centre_of_curvature());
        out(output, props.tangent());
        out(output, props.d3());
        props.value().out(output);
        return;
    }
    props.value().out(output);
    p::show(output, props.is_tangent_defined(), |value| {
        p::flag(output, value)
    });
    out(output, props.tangent());
    p::show(output, props.curvature(), |value| p::real(output, value));
    out(output, props.normal());
    out(output, props.centre_of_curvature());
    out(output, props.d1());
    out(output, props.d2());
    out(output, props.d3());
    props.value().out(output);
}

/// gb_lp_clprops_run.
fn clprops_run<V, C>(output: &Output, curve: C, r: &mut Reader<'_>)
where
    V: PropsVector + Out,
    V::Point: Out,
    V::Direction: Out,
    C: Clone + Default,
    DirectAccess: CurveAccess<C, V>,
{
    let (mode, n, tol, np) = (r.integer(), r.integer(), r.real(), r.integer());
    let mut shared: Option<Props<V, C>> = None;
    if mode == 2 {
        match Props::<V, C>::new(curve.clone(), n, tol) {
            Ok(props) => shared = Some(props),
            Err(error) => {
                output.write(format_args!("{} ", error.exception_type()));
                return end(output);
            }
        }
    }
    for _ in 0..np {
        let u = r.real();
        if let Some(props) = shared.as_mut() {
            attempt(output, || props.set_parameter(u));
            p::show(output, props.curvature(), |value| p::real(output, value));
            out(output, props.centre_of_curvature());
            out(output, props.tangent());
            out(output, props.d1());
            continue;
        }
        if mode == 3 {
            attempt(output, || {
                let mut props = Props::<V, C>::without_curve(n, tol)?;
                p::flag(output, props.is_tangent_defined()?);
                out(output, props.tangent());
                props.set_curve(curve.clone());
                props.set_parameter(u)?;
                clprops_members(output, &mut props, 0);
                Ok(())
            });
            continue;
        }
        attempt(output, || {
            let mut props = Props::<V, C>::with_parameter(curve.clone(), u, n, tol)?;
            clprops_members(output, &mut props, mode);
            Ok(())
        });
    }
    end(output);
}

/// Build a curve or print the construction exception and end the line.
fn built<T>(output: &Output, value: Result<T, Failure>) -> Option<T> {
    match value {
        Ok(value) => Some(value),
        Err(error) => {
            output.write(format_args!("{}\n", error.exception_type()));
            None
        }
    }
}

pub fn cl2d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(curve) = built(output, build::curve2d(&mut r)) else {
        return;
    };
    clprops_run::<Vector2, _>(output, curve, &mut r);
}

pub fn cl3d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(curve) = built(output, build::curve3d(&mut r)) else {
        return;
    };
    clprops_run::<Vector3, _>(output, curve, &mut r);
}

pub fn cl3d_adaptor_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let adaptor = build::curve3d(&mut r).and_then(|c| GeomAdaptorCurve::with_curve(&c));
    let Some(adaptor) = built(output, adaptor) else {
        return;
    };
    let curve: Handle<dyn Adaptor3dCurve> = Handle::new(adaptor).cast();
    clprops_run::<Vector3, _>(output, curve, &mut r);
}

/// gb_lp_slprops_members.
fn slprops_members(output: &Output, props: &mut SLProps, mode: i32) {
    p::pnt(output, props.value());
    if mode == 1 {
        curvature_getters(output, props);
    }
    out(output, props.tangent_u());
    out(output, props.tangent_v());
    p::show(output, props.is_tangent_u_defined(), |value| {
        p::flag(output, value)
    });
    p::show(output, props.is_tangent_v_defined(), |value| {
        p::flag(output, value)
    });
    p::show(output, props.is_normal_defined(), |value| {
        p::flag(output, value)
    });
    out(output, props.normal());
    p::show(output, props.is_curvature_defined(), |value| {
        p::flag(output, value)
    });
    curvature_getters(output, props);
    p::show(output, props.curvature_directions(), |(max, min)| {
        p::dir(output, max);
        p::dir(output, min);
    });
    out(output, props.tangent_u());
    out(output, props.tangent_v());
    out(output, props.d1u());
    out(output, props.d1v());
    out(output, props.d2u());
    out(output, props.d2v());
    out(output, props.duv());
}

fn curvature_getters(output: &Output, props: &mut SLProps) {
    p::show(output, props.max_curvature(), |value| {
        p::real(output, value)
    });
    p::show(output, props.min_curvature(), |value| {
        p::real(output, value)
    });
    p::show(output, props.mean_curvature(), |value| {
        p::real(output, value)
    });
    p::show(output, props.gaussian_curvature(), |value| {
        p::real(output, value)
    });
    p::show(output, props.is_umbilic(), |value| p::flag(output, value));
}

pub fn sl_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(surface) = built(output, build::surface(&mut r)) else {
        return;
    };
    let (mode, n, tol, np) = (r.integer(), r.integer(), r.real(), r.integer());
    let mut shared: Option<SLProps> = None;
    if mode == 2 {
        match SLProps::new(surface.clone(), n, tol) {
            Ok(props) => shared = Some(props),
            Err(error) => {
                output.write(format_args!("{} ", error.exception_type()));
                return end(output);
            }
        }
    }
    for _ in 0..np {
        let (u, v) = (r.real(), r.real());
        if let Some(props) = shared.as_mut() {
            attempt(output, || {
                props.set_parameters(u, v)?;
                slprops_members(output, props, 1);
                Ok(())
            });
            continue;
        }
        if mode == 3 {
            attempt(output, || {
                let mut props = SLProps::without_surface(n, tol)?;
                p::flag(output, props.is_normal_defined()?);
                p::flag(output, props.is_curvature_defined()?);
                props.set_surface(surface.clone());
                props.set_parameters(u, v)?;
                slprops_members(output, &mut props, 0);
                Ok(())
            });
            continue;
        }
        attempt(output, || {
            let mut props = SLProps::with_parameters(surface.clone(), u, v, n, tol)?;
            slprops_members(output, &mut props, mode);
            Ok(())
        });
    }
    end(output);
}

/// gb_lp_points.
fn points(output: &Output, points: &CurAndInf) {
    p::flag(output, points.is_empty());
    p::int(output, points.nb_points());
    for i in 1..=points.nb_points() {
        p::real(output, points.parameter(i).unwrap());
        p::int(output, points.type_of(i).unwrap() as i32);
    }
}

pub fn curinf2d_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(curve) = built(output, build::curve2d(&mut r)) else {
        return;
    };
    let mode = r.integer();
    let mut analyzer = CurAndInf2d::new();
    p::flag(output, analyzer.is_done());
    attempt(output, || {
        match mode {
            0 => analyzer.perform(&curve)?,
            1 => analyzer.perform_cur_ext(&curve)?,
            2 => analyzer.perform_inf(&curve)?,
            _ => {
                analyzer.perform_cur_ext(&curve)?;
                p::flag(output, analyzer.is_done());
                points(output, &analyzer);
                analyzer.perform_inf(&curve)?;
            }
        }
        Ok(())
    });
    p::flag(output, analyzer.is_done());
    points(output, &analyzer);
    end(output);
}

pub fn curandinf_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let mut store = CurAndInf::new();
    for _ in 0..r.integer() {
        let (op, u) = (r.integer(), r.real());
        match op {
            0 => store.add_inflection(u),
            3 => store.clear(),
            _ => store.add_ext_cur(u, op == 1),
        }
    }
    points(output, &store);
    for _ in 0..r.integer() {
        let q = r.integer();
        p::show(output, store.parameter(q), |value| p::real(output, value));
        p::show(output, store.type_of(q), |kind| p::int(output, kind as i32));
    }
    end(output);
}

pub fn continuity_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let curves = build::curve3d(&mut r).and_then(|c1| Ok((c1, build::curve3d(&mut r)?)));
    let Some((c1, c2)) = built(output, curves) else {
        return;
    };
    let (u1, u2, r1, r2, with_tol) = (r.real(), r.real(), r.flag(), r.flag(), r.flag());
    let (tl, ta) = (r.real(), r.real());
    let shape = if with_tol {
        geom_lprop::continuity_with_tolerances(&c1, &c2, u1, u2, r1, r2, tl, ta)
    } else {
        geom_lprop::continuity(&c1, &c2, u1, u2, r1, r2)
    };
    p::show(output, shape, |shape| p::int(output, shape as i32));
    end(output);
}
