//! Circle, ellipse, hyperbola and parabola request cases.
use super::common::{
    Reader, attempt, ax1, ax2, built_curve, curve_summary, dump, end, pnt, real, show,
};
use super::output::Output;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomCircle, GeomConic, GeomEllipse, GeomHyperbola, GeomParabola};
use martensite_tkmath::gp::{Axis1, Axis2, Circle3, Ellipse3, Hyperbola3, Parabola3, Point3};

fn conic_common(output: &Output, c: &dyn GeomConic) {
    ax1(output, c.x_axis());
    ax1(output, c.y_axis());
    ax1(output, c.axis());
    pnt(output, c.location());
    ax2(output, c.position());
    show(output, c.eccentricity(), |value| real(output, value));
}

fn circle_specific(output: &Output, e: &GeomCircle, value: f64) {
    real(output, e.radius());
    attempt(output, || e.set_radius(value));
    real(output, e.radius());
    show(output, e.circ(), |g| {
        ax2(output, g.position());
        real(output, g.radius());
        e.set_circ(g);
    });
}

fn ellipse_specific(output: &Output, e: &GeomEllipse, value: f64) {
    real(output, e.major_radius());
    real(output, e.minor_radius());
    show(output, e.directrix1(), |value| ax1(output, value));
    show(output, e.directrix2(), |value| ax1(output, value));
    real(output, e.focal());
    pnt(output, e.focus1());
    pnt(output, e.focus2());
    real(output, e.parameter());
    attempt(output, || e.set_minor_radius(value));
    attempt(output, || e.set_major_radius(value));
    real(output, e.major_radius());
    real(output, e.minor_radius());
    show(output, e.elips(), |g| {
        ax2(output, g.position());
        e.set_elips(g);
    });
}

fn hyperbola_specific(output: &Output, e: &GeomHyperbola, value: f64) {
    real(output, e.major_radius());
    real(output, e.minor_radius());
    show(output, e.asymptote1(), |value| ax1(output, value));
    show(output, e.asymptote2(), |value| ax1(output, value));
    show(output, e.conjugate_branch1(), |g| ax2(output, g.position()));
    show(output, e.conjugate_branch2(), |g| ax2(output, g.position()));
    show(output, e.directrix1(), |value| ax1(output, value));
    show(output, e.directrix2(), |value| ax1(output, value));
    real(output, e.focal());
    pnt(output, e.focus1());
    pnt(output, e.focus2());
    show(output, e.other_branch(), |g| ax2(output, g.position()));
    show(output, e.parameter(), |value| real(output, value));
    attempt(output, || e.set_major_radius(value));
    attempt(output, || e.set_minor_radius(-value));
    real(output, e.major_radius());
    real(output, e.minor_radius());
    show(output, e.hypr(), |g| {
        ax2(output, g.position());
        e.set_hypr(g);
    });
}

fn parabola_specific(output: &Output, e: &GeomParabola, value: f64) {
    real(output, e.focal());
    pnt(output, e.focus());
    real(output, e.parameter());
    show(output, e.directrix(), |value| ax1(output, value));
    attempt(output, || e.set_focal(value));
    real(output, e.focal());
    show(output, e.parab(), |g| {
        ax2(output, g.position());
        e.set_parab(g);
    });
}

fn conic_specific(output: &Output, c: &Handle<dyn GeomConic>, value: f64) {
    if let Some(e) = c.cast::<GeomCircle>().get() {
        circle_specific(output, e, value);
    }
    if let Some(e) = c.cast::<GeomEllipse>().get() {
        ellipse_specific(output, e, value);
    }
    if let Some(e) = c.cast::<GeomHyperbola>().get() {
        hyperbola_specific(output, e, value);
    }
    if let Some(e) = c.cast::<GeomParabola>().get() {
        parabola_specific(output, e, value);
    }
}

pub fn conic_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(curve) = built_curve(output, &mut r) else {
        return;
    };
    let u = r.reals();
    let c: Handle<dyn GeomConic> = curve.cast();
    let (value, a, p, f) = (r.real(), r.ax1(), r.pnt(), r.ax2());
    conic_common(output, &*c);
    conic_specific(output, &c, value);
    curve_summary(output, &*c, &u);
    c.set_location(p);
    conic_common(output, &*c);
    attempt(output, || c.set_axis(a));
    conic_common(output, &*c);
    c.set_position(f);
    conic_common(output, &*c);
    c.reverse().unwrap();
    conic_common(output, &*c);
    curve_summary(output, &*c, &u);
    dump(output, &*c, -1);
    end(output);
}

/// The conic built from gp objects, one kind per request (g3_conic_build).
fn build_conic(
    kind: i32,
    f: Axis2,
    radii: (f64, f64),
    directrix: (Axis1, Point3),
) -> Result<Handle<dyn GeomConic>, Failure> {
    let (a, b) = radii;
    Ok(match kind {
        1 => Handle::new(GeomCircle::from_circ(Circle3::new(f, a)?)).cast(),
        2 => Handle::new(GeomEllipse::from_elips(Ellipse3::new(f, a, b)?)).cast(),
        3 => Handle::new(GeomHyperbola::from_hypr(Hyperbola3::new(f, a, b)?)).cast(),
        4 => Handle::new(GeomParabola::from_parab(Parabola3::new(f, a)?)).cast(),
        _ => Handle::new(GeomParabola::from_directrix(directrix.0, directrix.1)?).cast(),
    })
}

pub fn conic_build_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let kind = r.integer();
    let (f, a, b) = (r.ax2(), r.real(), r.real());
    let (d, p) = (r.ax1(), r.pnt());
    match build_conic(kind, f, (a, b), (d, p)) {
        Ok(c) => {
            conic_common(output, &*c);
            dump(output, &*c, -1);
        }
        Err(error) => replay_print!(output, "{} ", error.exception_type()),
    }
    end(output);
}
