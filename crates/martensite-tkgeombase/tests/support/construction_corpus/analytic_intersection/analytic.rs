//! gb_ia_qq and gb_ia_curve: the analytic intersection and its curves.
use super::super::common::{self as p, Reader, attempt, end, flag, int, real};
use super::super::print::Output;
use super::{quadric, surface};
use martensite_tkernel::ncollection::List;
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::int_ana::{Curve, IntQuadQuad};
use martensite_tkmath::gp::{Axis3, Cone3, Cylinder3, Point3, Vector3};

/// FindParameter of `at` on the curve: the count, then the parameters.
fn find_parameter(output: &Output, c: &Curve, at: Point3) -> Result<(), Failure> {
    let mut params: List<f64> = List::default();
    c.find_parameter(at, &mut params)?;
    int(output, params.len() as i32);
    params.iter().for_each(|&value| real(output, value));
    Ok(())
}

/// gb_ia_curve: the flags, the domain, five points, the tangent at 37 %, a point
/// past the domain and the parameters of two points.
fn curve(output: &Output, c: &Curve) -> Result<(), Failure> {
    flag(output, c.is_open());
    flag(output, c.is_constant());
    flag(output, c.is_first_open());
    flag(output, c.is_last_open());
    let (f, l) = c.domain()?;
    real(output, f);
    real(output, l);
    for k in 0..=4 {
        let t = f + (l - f) * f64::from(k) / 4.0;
        p::show(output, c.value(t), |value| p::pnt(output, value));
    }
    let t = f + (l - f) * 0.37;
    let (mut point, mut vector) = (Point3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
    attempt(output, || {
        flag(output, c.d1u(t, &mut point, &mut vector)?);
        p::pnt(output, point);
        p::vec(output, vector);
        Ok(())
    });
    p::show(output, c.value(l + 1.0), |value| p::pnt(output, value));
    attempt(output, || find_parameter(output, c, c.value(t)?));
    attempt(output, || find_parameter(output, c, c.value(f)?));
    Ok(())
}

fn links(output: &Output, q: &IntQuadQuad, i: i32) -> Result<(), Failure> {
    flag(output, q.has_next_curve(i)?);
    flag(output, q.has_previous_curve(i)?);
    p::show(output, q.next_curve(i), |(j, o)| {
        int(output, j);
        flag(output, o);
    });
    p::show(output, q.previous_curve(i), |(j, o)| {
        int(output, j);
        flag(output, o);
    });
    Ok(())
}

fn perform(r: &mut Reader, k: i32) -> Result<IntQuadQuad, Failure> {
    let a: Axis3 = r.ax3();
    let mut q = IntQuadQuad::new();
    if k == 0 {
        let cylinder = Cylinder3::new(a, r.real())?;
        let quad = quadric(&surface(r)?)?;
        q.perform_cylinder(cylinder, &quad, r.real())?;
    } else {
        let angle = r.real();
        let cone = Cone3::new(a, angle, r.real())?;
        let quad = quadric(&surface(r)?)?;
        q.perform_cone(cone, &quad, r.real())?;
    }
    Ok(q)
}

fn qq(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let k = r.integer();
    if k == 2 {
        let q = IntQuadQuad::new();
        flag(output, q.is_done());
        p::show(output, q.nb_curve(), |value| int(output, value));
        p::show(output, q.has_next_curve(1), |value| flag(output, value));
        return Ok(());
    }
    let q = perform(r, k)?;
    flag(output, q.is_done());
    if !q.is_done() {
        p::show(output, q.nb_curve(), |value| int(output, value));
        return Ok(());
    }
    flag(output, q.identical_elements()?);
    let n = q.nb_curve()?;
    int(output, n);
    int(output, q.nb_pnt()?);
    for i in 1..=n {
        curve(output, q.curve(i)?)?;
        links(output, &q, i)?;
    }
    attempt(output, || curve(output, q.curve(n + 1)?));
    p::show(output, q.point(1), |value| p::pnt(output, value));
    p::show(output, q.has_previous_curve(0), |value| flag(output, value));
    Ok(())
}

pub fn ia_qq_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || qq(output, &mut r));
    end(output);
}

fn curve_direct(output: &Output, r: &mut Reader) -> Result<(), Failure> {
    let k = r.integer();
    let a = r.ax3();
    let (cylinder, cone) = if k == 0 {
        (Some(Cylinder3::new(a, r.real())?), None)
    } else {
        let angle = r.real();
        (None, Some(Cone3::new(a, angle, r.real())?))
    };
    let mut q = [0.0; 10];
    q.iter_mut().for_each(|v| *v = r.real());
    let (tol, inf, sup) = (r.real(), r.real(), r.real());
    let (two, pos) = (r.flag(), r.flag());
    let (first, last) = (r.real(), r.real());
    let (first_open, last_open) = (r.flag(), r.flag());
    let mut c = Curve::new();
    p::show(output, c.domain(), |_| {});
    flag(output, c.is_open());
    match (cylinder, cone) {
        (Some(x), _) => c.set_cylinder_quad_values(x, q, tol, (inf, sup), two, pos),
        (_, Some(x)) => c.set_cone_quad_values(x, q, tol, (inf, sup), two, pos),
        _ => {}
    }
    curve(output, &c)?;
    attempt(output, || {
        c.set_domain(first, last)?;
        curve(output, &c)
    });
    c.set_is_first_open(first_open);
    c.set_is_last_open(last_open);
    curve(output, &c)
}

pub fn ia_curve_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || curve_direct(output, &mut r));
    end(output);
}
