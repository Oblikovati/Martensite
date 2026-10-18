//! The Rust twin of the ProjectOnPlane operation of
//! the implementation.
use super::common::{self as p, Reader, attempt, end};
use super::geom_convert::{bezier, bspline};
use super::geometry_adaptor_fixtures::adapted_curve;
use super::geometry_record_fixtures::ax3;
use super::print::Output;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkgeombase::proj_lib::ProjectOnPlane;
use martensite_tkmath::geom_abs::Shape;

/// gb_pl_on_plane_eval.
pub fn eval(output: &Output, c: &dyn Adaptor3dCurve, u: f64) {
    p::show(output, c.value(u), |value| p::pnt(output, value));
    attempt(output, || {
        let (q, v1) = c.d1(u)?;
        p::pnt(output, q);
        p::vec(output, v1);
        Ok(())
    });
    attempt(output, || {
        let (q, v1, v2) = c.d2(u)?;
        p::pnt(output, q);
        p::vec(output, v1);
        p::vec(output, v2);
        Ok(())
    });
    attempt(output, || {
        let (q, v1, v2, v3) = c.d3(u)?;
        p::pnt(output, q);
        p::vec(output, v1);
        p::vec(output, v2);
        p::vec(output, v3);
        Ok(())
    });
    p::show(output, c.dn(u, 2), |value| p::vec(output, value));
}

fn typed(output: &Output, x: &ProjectOnPlane) {
    p::show(output, x.line(), |value| p::lin(output, value));
    p::show(output, x.circle(), |value| p::circ(output, value));
    p::show(output, x.ellipse(), |value| p::elips(output, value));
    p::show(output, x.hyperbola(), |value| p::hypr(output, value));
    p::show(output, x.parabola(), |value| p::parab(output, value));
    p::show(output, x.degree(), |value| p::int(output, value));
    p::show(output, x.is_rational(), |value| p::flag(output, value));
    p::show(output, x.nb_poles(), |value| p::int(output, value));
    p::show(output, x.nb_knots(), |value| p::int(output, value));
    p::show(output, x.bezier(), |b| bezier(output, &b));
    p::show(output, x.bspline(), |b| bspline(output, &b));
}

fn intervals(output: &Output, x: &ProjectOnPlane) -> Result<(), Failure> {
    let n = x.nb_intervals(Shape::C2)?;
    p::int(output, n);
    let mut t: Array1<'static, f64> = Array1::with_bounds(1, n + 1).expect("probe bounds");
    x.intervals(&mut t, Shape::C2)?;
    t.as_slice().iter().for_each(|v| p::real(output, *v));
    Ok(())
}

fn samples(output: &Output, x: &ProjectOnPlane) -> Result<(), Failure> {
    let (f, l) = (x.first_parameter()?, x.last_parameter()?);
    p::real(output, f);
    p::real(output, l);
    for i in 0..=4 {
        eval(output, x, f + (l - f) * f64::from(i) / 4.);
    }
    attempt(output, || {
        let t = x.trim(f + (l - f) / 8., l - (l - f) / 8., 1.0e-7)?;
        p::real(output, t.first_parameter()?);
        p::real(output, t.last_parameter()?);
        p::pnt(output, t.value(0.5 * (f + l))?);
        Ok(())
    });
    attempt(output, || {
        let c = x.shallow_copy()?;
        p::int(output, c.get_type()? as i32);
        eval(output, c.get().expect("probe copy"), 0.5 * (f + l));
        Ok(())
    });
    Ok(())
}

fn results(output: &Output, x: &ProjectOnPlane) -> Result<(), Failure> {
    p::int(output, x.get_type()? as i32);
    p::ax3(output, x.get_plane());
    p::dir(output, x.get_direction());
    p::flag(output, x.get_result().is_null());
    p::flag(output, x.get_curve().is_null());
    p::show(output, x.is_periodic(), |value| p::flag(output, value));
    p::show(output, x.period(), |value| p::real(output, value));
    p::show(output, x.is_closed(), |value| p::flag(output, value));
    p::show(output, x.continuity(), |s| p::int(output, s as i32));
    p::show(output, x.resolution(1.0e-3), |value| p::real(output, value));
    attempt(output, || intervals(output, x));
    typed(output, x);
    attempt(output, || samples(output, x));
    Ok(())
}

pub fn on_plane_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (variant, keep, tol) = (r.integer(), r.flag(), r.real());
    let (pl, d) = (ax3(&mut r), r.dir());
    attempt(output, || {
        let c: Handle<dyn Adaptor3dCurve> = adapted_curve(&mut r)?;
        let x = match variant {
            0 => ProjectOnPlane::new(),
            1 => ProjectOnPlane::with_plane(&pl),
            _ => ProjectOnPlane::with_direction(&pl, d),
        };
        p::int(output, x.get_type()? as i32);
        x.load(&c, tol, keep)?;
        if variant == 3 {
            x.load(&c, tol, !keep)?;
        }
        results(output, &x)
    });
    end(output);
}
