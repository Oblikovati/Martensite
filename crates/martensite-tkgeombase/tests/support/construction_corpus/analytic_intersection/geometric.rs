//! gb_ia_qqgeo: the geometric quadric-quadric intersection.
use super::super::common::{self as p, Reader, attempt, end, flag, int};
use super::super::print::Output;
use super::{Surface, surface};
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::int_ana::{QuadQuadGeo, ResultType};

fn solution(output: &Output, q: &QuadQuadGeo, i: i32) -> Result<(), Failure> {
    match q.type_inter()? {
        ResultType::Point => p::show(output, q.point(i), |value| p::pnt(output, value)),
        ResultType::Line => p::show(output, q.line(i), |value| p::lin(output, value)),
        ResultType::Circle => p::show(output, q.circle(i), |value| p::circ(output, value)),
        ResultType::PointAndCircle => {
            p::show(output, q.point(1), |value| p::pnt(output, value));
            p::show(output, q.circle(1), |value| p::circ(output, value));
        }
        ResultType::Ellipse => p::show(output, q.ellipse(i), |value| p::elips(output, value)),
        ResultType::Parabola => p::show(output, q.parabola(i), |value| p::parab(output, value)),
        ResultType::Hyperbola => p::show(output, q.hyperbola(i), |value| p::hypr(output, value)),
        _ => {}
    }
    Ok(())
}

fn qqgeo(output: &Output, q: &QuadQuadGeo) -> Result<(), Failure> {
    flag(output, q.is_done());
    flag(output, q.has_common_gen());
    p::pnt(output, q.p_char());
    if !q.is_done() {
        p::show(output, q.type_inter(), |t| int(output, t as i32));
        return Ok(());
    }
    int(output, q.type_inter()? as i32);
    let n = q.nb_solutions()?;
    int(output, n);
    for i in 1..=n {
        solution(output, q, i)?;
    }
    p::show(output, q.point(1), |value| p::pnt(output, value));
    p::show(output, q.line(n + 1), |value| p::lin(output, value));
    p::show(output, q.circle(n + 1), |value| p::circ(output, value));
    p::show(output, q.ellipse(0), |value| p::elips(output, value));
    p::show(output, q.parabola(2), |value| p::parab(output, value));
    p::show(output, q.hyperbola(n + 1), |value| p::hypr(output, value));
    Ok(())
}

/// The overload of the pair of surfaces, with its tolerances read in order.
fn perform(q: &mut QuadQuadGeo, a: Surface, b: Surface, r: &mut Reader) -> Result<(), Failure> {
    use Surface::*;
    match (a, b) {
        (Plane(x), Plane(y)) => {
            let ta = r.real();
            q.perform_planes(x, y, ta, r.real())
        }
        (Plane(x), Cylinder(y)) => {
            let (ta, t) = (r.real(), r.real());
            q.perform_plane_cylinder(x, y, ta, t, r.real())
        }
        (Plane(x), Sphere(y)) => q.perform_plane_sphere(x, y),
        (Plane(x), Cone(y)) => {
            let ta = r.real();
            q.perform_plane_cone(x, y, ta, r.real())
        }
        (a, b) => perform_with_tol(q, a, b, r.real()),
    }
}

fn perform_with_tol(q: &mut QuadQuadGeo, a: Surface, b: Surface, t: f64) -> Result<(), Failure> {
    use Surface::*;
    match (a, b) {
        (Cylinder(x), Cylinder(y)) => q.perform_cylinders(x, y, t),
        (Cylinder(x), Sphere(y)) => q.perform_cylinder_sphere(x, y, t),
        (Cylinder(x), Cone(y)) => q.perform_cylinder_cone(x, y, t),
        (Sphere(x), Sphere(y)) => q.perform_spheres(x, y, t),
        (Sphere(x), Cone(y)) => q.perform_sphere_cone(x, y, t),
        (Cone(x), Cone(y)) => q.perform_cones(x, y, t),
        (Plane(x), Torus(y)) => q.perform_plane_torus(x, y, t),
        (Cylinder(x), Torus(y)) => q.perform_cylinder_torus(x, y, t),
        (Cone(x), Torus(y)) => q.perform_cone_torus(x, y, t),
        (Sphere(x), Torus(y)) => q.perform_sphere_torus(x, y, t),
        (Torus(x), Torus(y)) => q.perform_tori(x, y, t),
        _ => Ok(()),
    }
}

pub fn ia_qqgeo_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || {
        let a = surface(&mut r)?;
        let b = surface(&mut r)?;
        let mut q = QuadQuadGeo::new();
        perform(&mut q, a, b, &mut r)?;
        qqgeo(output, &q)
    });
    end(output);
}
