//! The Extrema requests on elementary surfaces (ExtPElS, ExtElCS, ExtElSS); the
//! counterpart of the surface part of the implementation
//! (same encoding and token order).
#![allow(clippy::unit_arg, clippy::type_complexity)]
use super::common::{Reader, attempt, end, flag, int, real};
use super::extrema_el::{Conic3, poncurv, ponsurf, read_circ, read_lin};
use super::print::Output;
use martensite_tkernel::standard::Failure;
use martensite_tkgeombase::extrema::{ExtElCS, ExtElSS, ExtPElS};
use martensite_tkmath::gp::{Axis3, Cone3, Cylinder3, Hyperbola3, Plane3, Sphere3, Torus3};

enum Surface {
    Plane(Plane3),
    Cylinder(Cylinder3),
    Cone(Cone3),
    Sphere(Sphere3),
    Torus(Torus3),
}

fn surface(kind: i32, a: Axis3, r: &mut Reader) -> Result<Surface, Failure> {
    Ok(match kind {
        1 => Surface::Cylinder(Cylinder3::new(a, r.real())?),
        2 => {
            let ang = r.real();
            Surface::Cone(Cone3::new(a, ang, r.real())?)
        }
        3 => Surface::Sphere(Sphere3::new(a, r.real())?),
        4 => {
            let big = r.real();
            Surface::Torus(Torus3::new(a, big, r.real())?)
        }
        _ => Surface::Plane(Plane3::from_position(a)),
    })
}

fn pels_show(output: &Output, e: &ExtPElS) {
    flag(output, e.is_done());
    attempt(output, || {
        let n = e.nb_ext()?;
        int(output, n);
        for i in 1..=n {
            real(output, e.square_distance(i)?);
            ponsurf(output, e.point(i)?);
        }
        Ok(())
    });
    attempt(output, || Ok(real(output, e.square_distance(0)?)));
    attempt(output, || Ok(ponsurf(output, e.point(5)?)));
}

fn pels_step(
    output: &Output,
    s: &Surface,
    reused: &mut ExtPElS,
    pt: martensite_tkmath::gp::Point3,
    tol: f64,
) -> Result<(), Failure> {
    match s {
        Surface::Plane(x) => {
            pels_show(output, &ExtPElS::with_plane(pt, *x, tol));
            reused.perform_plane(pt, x, tol);
        }
        Surface::Cylinder(x) => {
            pels_show(output, &ExtPElS::with_cylinder(pt, *x, tol)?);
            reused.perform_cylinder(pt, x, tol)?;
        }
        Surface::Cone(x) => {
            pels_show(output, &ExtPElS::with_cone(pt, *x, tol)?);
            reused.perform_cone(pt, x, tol)?;
        }
        Surface::Sphere(x) => {
            pels_show(output, &ExtPElS::with_sphere(pt, *x, tol)?);
            reused.perform_sphere(pt, x, tol)?;
        }
        Surface::Torus(x) => {
            pels_show(output, &ExtPElS::with_torus(pt, *x, tol)?);
            reused.perform_torus(pt, x, tol)?;
        }
    }
    Ok(())
}

pub fn pels_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || {
        let kind = r.integer();
        let a = r.ax3();
        let s = surface(kind, a, &mut r)?;
        let n = r.integer();
        let mut reused = ExtPElS::new();
        pels_show(output, &reused);
        for _ in 0..n {
            let pt = r.pnt();
            let tol = r.real();
            attempt(output, || pels_step(output, &s, &mut reused, pt, tol));
            pels_show(output, &reused);
        }
        Ok(())
    });
    end(output);
}

fn elcs_perform(e: &mut ExtElCS, curve: &Conic3, s: &Surface) -> Result<(), Failure> {
    match (curve, s) {
        (Conic3::Line(l), Surface::Plane(p)) => e.perform_line_plane(l, p),
        (Conic3::Line(l), Surface::Cylinder(p)) => e.perform_line_cylinder(l, p)?,
        (Conic3::Line(l), Surface::Cone(p)) => e.perform_line_cone(l, p)?,
        (Conic3::Line(l), Surface::Sphere(p)) => e.perform_line_sphere(l, p)?,
        (Conic3::Line(l), Surface::Torus(p)) => e.perform_line_torus(l, p)?,
        (Conic3::Circle(c), Surface::Plane(p)) => e.perform_circle_plane(c, p)?,
        (Conic3::Circle(c), Surface::Cylinder(p)) => e.perform_circle_cylinder(c, p)?,
        (Conic3::Circle(c), Surface::Cone(p)) => e.perform_circle_cone(c, p)?,
        (Conic3::Circle(c), Surface::Sphere(p)) => e.perform_circle_sphere(c, p)?,
        (Conic3::Circle(c), Surface::Torus(p)) => e.perform_circle_torus(c, p)?,
        (Conic3::Hyperbola(h), Surface::Plane(p)) => e.perform_hyperbola_plane(h, p),
        _ => {}
    }
    Ok(())
}

fn elcs_show(output: &Output, e: &ExtElCS, allocated: &mut bool) {
    flag(output, e.is_done());
    let mut par = false;
    attempt(output, || {
        par = e.is_parallel()?;
        flag(output, par);
        Ok(())
    });
    attempt(output, || {
        let m = e.nb_ext()?;
        int(output, m);
        if !par {
            for i in 1..=m {
                real(output, e.square_distance(i)?);
                let (a, b) = e.points(i)?;
                poncurv(output, a);
                ponsurf(output, b);
            }
            if m > 0 {
                *allocated = true;
            }
        } else {
            real(output, e.square_distance(1)?);
            if *allocated {
                let (a, b) = e.points(1)?;
                poncurv(output, a);
                ponsurf(output, b);
            }
        }
        Ok(())
    });
    attempt(output, || Ok(real(output, e.square_distance(0)?)));
}

fn elcs_curve(curve: i32, r: &mut Reader) -> Result<Conic3, Failure> {
    Ok(match curve {
        0 => Conic3::Line(read_lin(r)),
        1 => Conic3::Circle(read_circ(r)?),
        _ => {
            let a = r.ax2();
            let x = r.real();
            Conic3::Hyperbola(Hyperbola3::new(a, x, r.real())?)
        }
    })
}

pub fn elcs_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || {
        let n = r.integer();
        let mut e = ExtElCS::new();
        let mut allocated = false;
        for _ in 0..n {
            let kind = r.integer();
            let c = elcs_curve(kind / 5, &mut r)?;
            let a = r.ax3();
            let s = surface(kind % 5, a, &mut r)?;
            attempt(output, || elcs_perform(&mut e, &c, &s));
            elcs_show(output, &e, &mut allocated);
        }
        Ok(())
    });
    end(output);
}

fn elss_step(output: &Output, e: &mut ExtElSS, kind: i32, r: &mut Reader) -> Result<(), Failure> {
    let a = r.ax3();
    if kind == 0 {
        let b = r.ax3();
        e.perform_planes(&Plane3::from_position(a), &Plane3::from_position(b));
        return Ok(());
    }
    if kind == 1 {
        let b = r.ax3();
        let s = Sphere3::new(b, r.real())?;
        attempt(output, || {
            e.perform_plane_sphere(&Plane3::from_position(a), &s)
        });
        return Ok(());
    }
    let s = Sphere3::new(a, r.real())?;
    let b = r.ax3();
    let x = r.real();
    match kind {
        2 => {
            let t = Sphere3::new(b, x)?;
            attempt(output, || e.perform_spheres(&s, &t));
        }
        3 => {
            let t = Cylinder3::new(b, x)?;
            attempt(output, || e.perform_sphere_cylinder(&s, &t));
        }
        4 => {
            let t = Cone3::new(b, x, r.real())?;
            attempt(output, || e.perform_sphere_cone(&s, &t));
        }
        _ => {
            let t = Torus3::new(b, x, r.real())?;
            attempt(output, || e.perform_sphere_torus(&s, &t));
        }
    }
    Ok(())
}

pub fn elss_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    attempt(output, || {
        let n = r.integer();
        let mut e = ExtElSS::new();
        for _ in 0..n {
            let kind = r.integer();
            elss_step(output, &mut e, kind, &mut r)?;
            flag(output, e.is_done());
            attempt(output, || Ok(flag(output, e.is_parallel()?)));
            attempt(output, || {
                let m = e.nb_ext()?;
                int(output, m);
                for i in 1..=m {
                    real(output, e.square_distance(i)?);
                }
                Ok(())
            });
            attempt(output, || Ok(real(output, e.square_distance(0)?)));
        }
        Ok(())
    });
    end(output);
}
