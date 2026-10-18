//! The Rust twin of the CompProjectedCurve operation of
//! the implementation.
use super::common::{self as p, Reader, attempt, end};
use super::geom_convert::{curve as print_curve, curve2d as print_curve2d};
use super::geometry_adaptor_fixtures::{adapted_curve, adapted_surface, shape};
use super::print::Output;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::adaptor2d::Adaptor2dCurve2d;
use martensite_tkgeombase::proj_lib::CompProjectedCurve;
use martensite_tkmath::gp::Point3;

fn parts(output: &Output, x: &CompProjectedCurve) -> Result<(), Failure> {
    p::int(output, x.nb_curves());
    let sequence = x.get_sequence();
    for i in 1..=x.nb_curves() {
        let (f, l) = x.bounds(i)?;
        p::real(output, f);
        p::real(output, l);
        let (single, pt) = x.is_single_pnt(i)?;
        p::flag(output, single);
        p::pnt2d(output, pt);
        let (u_iso, u) = x.is_u_iso(i)?;
        p::flag(output, u_iso);
        p::real(output, u);
        let (v_iso, v) = x.is_v_iso(i)?;
        p::flag(output, v_iso);
        p::real(output, v);
        p::real(output, x.max_distance(i)?);
        let part = sequence
            .get()
            .expect("probe sequence")
            .sequence(|s| s.value(i as usize).cloned())?;
        let points: Vec<Point3> = part
            .get()
            .expect("probe part")
            .sequence(|s| s.iter().copied().collect());
        p::int(output, points.len() as i32);
        points.into_iter().for_each(|value| p::pnt(output, value));
    }
    Ok(())
}

fn eval(output: &Output, x: &CompProjectedCurve) -> Result<(), Failure> {
    if x.nb_curves() < 1 {
        return Ok(());
    }
    let (f, l) = x.bounds(1)?;
    for i in 0..=4 {
        let u = f + (l - f) * f64::from(i) / 4.;
        p::show(output, x.value(u), |value| p::pnt2d(output, value));
        attempt(output, || {
            let (q, v) = x.d1(u)?;
            p::pnt2d(output, q);
            p::vec2d(output, v);
            Ok(())
        });
        attempt(output, || {
            let (q, v1, v2) = x.d2(u)?;
            p::pnt2d(output, q);
            p::vec2d(output, v1);
            p::vec2d(output, v2);
            Ok(())
        });
        p::show(output, x.dn(u, 1), |value| p::vec2d(output, value));
        p::show(output, x.dn(u, 3), |value| p::vec2d(output, value));
        p::show(output, x.dn(u, 0), |value| p::vec2d(output, value));
    }
    Ok(())
}

fn intervals(output: &Output, x: &CompProjectedCurve, s: i32) -> Result<(), Failure> {
    let n = x.nb_intervals(shape(s))?;
    p::int(output, n);
    let mut t: Array1<'static, f64> = Array1::with_bounds(1, n + 1).expect("probe bounds");
    x.intervals(&mut t, shape(s))?;
    t.as_slice().iter().for_each(|v| p::real(output, *v));
    Ok(())
}

fn adaptor(output: &Output, x: &CompProjectedCurve) {
    p::show(output, x.first_parameter(), |value| p::real(output, value));
    p::show(output, x.last_parameter(), |value| p::real(output, value));
    p::show(output, x.continuity(), |s| p::int(output, s as i32));
    p::int(output, x.get_type().expect("probe type") as i32);
    let (tu, tv) = x.get_tolerance();
    p::real(output, tu);
    p::real(output, tv);
    p::flag(output, x.get_proj2d());
    p::flag(output, x.get_proj3d());
    for s in 0..=6 {
        attempt(output, || intervals(output, x, s));
    }
    attempt(output, || {
        let c: Handle<dyn Adaptor2dCurve2d> = x.shallow_copy()?;
        p::int(output, c.get_type()? as i32);
        p::pnt2d(
            output,
            c.value(0.5 * (c.first_parameter()? + c.last_parameter()?))?,
        );
        Ok(())
    });
    attempt(output, || {
        let (f, l) = x.bounds(1)?;
        let t = x.trim(f, 0.5 * (f + l), 1.0e-7)?;
        p::real(output, t.first_parameter()?);
        p::real(output, t.last_parameter()?);
        Ok(())
    });
    attempt(output, || x.line().map(|_| ()));
    attempt(output, || x.resolution(1.0e-3).map(|_| ()));
    p::show(output, x.is_periodic(), |value| p::flag(output, value));
}

fn results(output: &Output, x: &CompProjectedCurve) -> Result<(), Failure> {
    for i in 1..=x.nb_curves() {
        p::flag(output, x.result_is_point(i)?);
        p::real(output, x.get_result_2d_u_approx_error(i)?);
        p::real(output, x.get_result_2d_v_approx_error(i)?);
        p::real(output, x.get_result_3d_approx_error(i)?);
        if x.get_proj2d() {
            print_curve2d(output, &x.get_result_2d_c(i)?);
            p::show(output, x.get_result_2d_p(i), |value| {
                p::pnt2d(output, value)
            });
        }
        if x.get_proj3d() {
            print_curve(output, &x.get_result_3d_c(i)?);
            p::show(output, x.get_result_3d_p(i), |value| p::pnt(output, value));
        }
    }
    Ok(())
}

pub fn comp_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let variant = r.integer();
    let (tol3d, tu, tv, md) = (r.real(), r.real(), r.real(), r.real());
    let (p2, p3, cont) = (r.flag(), r.flag(), shape(r.integer()));
    let (deg, seg) = (r.integer(), r.integer());
    attempt(output, || {
        let c = adapted_curve(&mut r)?;
        let s = adapted_surface(&mut r)?;
        let x = match variant {
            0 => CompProjectedCurve::with_tolerances(&s, &c, tu, tv)?,
            1 => CompProjectedCurve::with_max_dist(&s, &c, tu, tv, md)?,
            _ => CompProjectedCurve::with_tol3d(tol3d, &s, &c, md)?,
        };
        parts(output, &x)?;
        eval(output, &x)?;
        adaptor(output, &x);
        x.set_tol3d(tol3d);
        x.set_continuity(cont);
        x.set_max_degree(deg);
        x.set_max_seg(seg);
        x.set_proj2d(p2);
        x.set_proj3d(p3);
        attempt(output, || {
            x.perform()?;
            results(output, &x)
        });
        Ok(())
    });
    end(output);
}
