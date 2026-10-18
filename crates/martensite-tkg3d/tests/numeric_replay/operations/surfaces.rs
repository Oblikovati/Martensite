//! Analytic surface construction and evaluation requests.
use super::common::{
    Reader, attempt, ax1, ax3, built_surface, dump, end, gtrsf2d, isos, name, pnt, real, show,
    surface_summary, trsf,
};
use super::output::Output;
use super::simple::moves;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{
    GeomConicalSurface, GeomCylindricalSurface, GeomElementarySurface, GeomGeometry, GeomPlane,
    GeomSphericalSurface, GeomSurface, GeomToroidalSurface,
};
use martensite_tkmath::gp::{Cone3, Cylinder3, Plane3, Sphere3, Torus3};

pub fn surface_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(s) = built_surface(output, &mut r) else {
        return;
    };
    let (uv, iso_params, params) = (r.reals(), r.reals(), r.reals());
    name(output, &s);
    surface_summary(output, &*s, &uv);
    isos(output, &*s, &iso_params, &params);
    for depth in [-1, 1, 0] {
        dump(output, &*s, depth);
    }
    end(output);
}

/// Prints the summary of a moved surface at the (u, v) pairs of the request (g3_show_surface).
pub fn show_surface(output: &Output, g: &Handle<dyn GeomGeometry>) {
    let s: Handle<dyn GeomSurface> = g.cast();
    surface_summary(output, &*s, &output.parameters());
}

pub fn surface_moves_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(s) = built_surface(output, &mut r) else {
        return;
    };
    let uv = r.reals();
    output.set_parameters(uv.clone());
    let t = trsf(&mut r);
    for pair in uv.as_chunks::<2>().0 {
        let (u, v) = s.transform_parameters(pair[0], pair[1], t);
        real(output, u);
        real(output, v);
    }
    show(output, s.parametric_transformation(t), |value| {
        gtrsf2d(output, value)
    });
    show(output, s.u_reversed(), |c| show_surface(output, &c.cast()));
    show(output, s.v_reversed(), |c| show_surface(output, &c.cast()));
    show(output, s.copy(), |c| show_surface(output, &c));
    show(output, s.u_reverse(), |()| show_surface(output, &s.cast()));
    show(output, s.v_reverse(), |()| show_surface(output, &s.cast()));
    moves(output, &s.cast(), &mut r, show_surface);
    dump(output, &*s, -1);
    end(output);
}

fn elementary_common(output: &Output, e: &dyn GeomElementarySurface) {
    ax1(output, e.axis());
    pnt(output, e.location());
    ax3(output, e.position());
}

fn plane_specific(output: &Output, p: &GeomPlane) {
    show(output, p.coefficients(), |c| {
        c.into_iter().for_each(|value| real(output, value))
    });
    ax3(output, p.pln().position());
    p.set_pln(Plane3::from_position(p.position()));
}

fn cylinder_specific(output: &Output, p: &GeomCylindricalSurface, a: f64) {
    real(output, p.radius());
    p.coefficients()
        .into_iter()
        .for_each(|value| real(output, value));
    show(output, p.cylinder(), |g: Cylinder3| {
        ax3(output, g.position());
        real(output, g.radius());
        p.set_cylinder(g);
    });
    attempt(output, || p.set_radius(a));
    real(output, p.radius());
}

fn cone_specific(output: &Output, p: &GeomConicalSurface, a: f64, b: f64) {
    real(output, p.ref_radius());
    real(output, p.semi_angle());
    pnt(output, p.apex());
    p.coefficients()
        .into_iter()
        .for_each(|value| real(output, value));
    show(output, p.cone(), |g: Cone3| {
        ax3(output, g.position());
        real(output, g.semi_angle());
        p.set_cone(g);
    });
    attempt(output, || p.set_radius(a));
    attempt(output, || p.set_semi_angle(b));
    real(output, p.ref_radius());
    real(output, p.semi_angle());
    pnt(output, p.apex());
}

fn sphere_specific(output: &Output, p: &GeomSphericalSurface, a: f64) {
    [p.radius(), p.area(), p.volume()]
        .into_iter()
        .for_each(|value| real(output, value));
    p.coefficients()
        .into_iter()
        .for_each(|value| real(output, value));
    show(output, p.sphere(), |g: Sphere3| {
        ax3(output, g.position());
        p.set_sphere(g);
    });
    attempt(output, || p.set_radius(a));
    real(output, p.radius());
}

fn torus_specific(output: &Output, p: &GeomToroidalSurface, a: f64, b: f64) {
    [p.major_radius(), p.minor_radius(), p.area(), p.volume()]
        .into_iter()
        .for_each(|value| real(output, value));
    let mut coefficients = Array1::<f64>::with_bounds(1, 35).unwrap();
    let mut small = Array1::<f64>::with_bounds(1, 30).unwrap();
    attempt(output, || {
        p.coefficients(&mut coefficients)?;
        coefficients
            .as_slice()
            .iter()
            .copied()
            .for_each(|value| real(output, value));
        Ok(())
    });
    attempt(output, || p.coefficients(&mut small));
    show(output, p.torus(), |g: Torus3| {
        ax3(output, g.position());
        p.set_torus(g);
    });
    attempt(output, || p.set_major_radius(a));
    attempt(output, || p.set_minor_radius(b));
    real(output, p.major_radius());
    real(output, p.minor_radius());
}

// The class getters and setters of g3_elementary_specific.
fn elementary_specific(output: &Output, e: &Handle<dyn GeomElementarySurface>, a: f64, b: f64) {
    if let Some(p) = e.cast::<GeomPlane>().get() {
        plane_specific(output, p);
    }
    if let Some(p) = e.cast::<GeomCylindricalSurface>().get() {
        cylinder_specific(output, p, a);
    }
    if let Some(p) = e.cast::<GeomConicalSurface>().get() {
        cone_specific(output, p, a, b);
    }
    if let Some(p) = e.cast::<GeomSphericalSurface>().get() {
        sphere_specific(output, p, a);
    }
    if let Some(p) = e.cast::<GeomToroidalSurface>().get() {
        torus_specific(output, p, a, b);
    }
}

pub fn elementary_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(s) = built_surface(output, &mut r) else {
        return;
    };
    let uv = r.reals();
    let e: Handle<dyn GeomElementarySurface> = s.cast();
    let (a, b) = (r.real(), r.real());
    let (axis, p, f) = (r.ax1(), r.pnt(), r.ax3());
    elementary_common(output, &*e);
    elementary_specific(output, &e, a, b);
    surface_summary(output, &*e, &uv);
    e.set_location(p);
    elementary_common(output, &*e);
    attempt(output, || e.set_axis(axis));
    elementary_common(output, &*e);
    e.set_position(f);
    elementary_common(output, &*e);
    surface_summary(output, &*e, &uv);
    dump(output, &*e, -1);
    end(output);
}

// The constructor of g3_elementary_build for `kind`, the error printed when it raises.
fn build_elementary(
    output: &Output,
    kind: i32,
    r: &mut Reader,
) -> Option<Handle<dyn GeomElementarySurface>> {
    let f = r.ax3();
    let (a, b, p, n) = (r.real(), r.real(), r.pnt(), r.dir());
    let c: Vec<f64> = (0..4).map(|_| r.real()).collect();
    let built: Result<Handle<dyn GeomElementarySurface>, Failure> = match kind {
        0 => Ok(Handle::new(GeomPlane::from_pln(Plane3::from_position(f))).cast()),
        1 => Cylinder3::new(f, a)
            .map(|g| Handle::new(GeomCylindricalSurface::from_cylinder(g)).cast()),
        2 => Cone3::new(f, b, a).map(|g| Handle::new(GeomConicalSurface::from_cone(g)).cast()),
        3 => Sphere3::new(f, a).map(|g| Handle::new(GeomSphericalSurface::from_sphere(g)).cast()),
        4 => Torus3::new(f, a, b).map(|g| Handle::new(GeomToroidalSurface::from_torus(g)).cast()),
        5 => GeomPlane::from_point(p, n).map(|g| Handle::new(g).cast()),
        _ => GeomPlane::from_coefficients(c[0], c[1], c[2], c[3]).map(|g| Handle::new(g).cast()),
    };
    match built {
        Ok(e) => Some(e),
        Err(error) => {
            replay_print!(output, "{} ", error.exception_type());
            None
        }
    }
}

pub fn elementary_build_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let kind = r.integer();
    if let Some(e) = build_elementary(output, kind, &mut r) {
        elementary_common(output, &*e);
        dump(output, &*e, -1);
    }
    end(output);
}
