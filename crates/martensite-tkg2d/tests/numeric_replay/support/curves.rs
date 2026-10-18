//! Curve evaluation, curve moves, conics, trimmed and offset curves, and the offset
//! utilities; the counterpart of the frozen behavior fixtures
use super::common::{
    Reader, attempt, ax, ax22, built, curve, dump, end, flag, int, pnt, real, show, summary, trsf,
    vec,
};
use super::output::Output;
use super::simple::moves;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::offset_curve_utils::{
    calculate_d0, calculate_d1, calculate_d2, calculate_d3,
};
use martensite_tkg2d::geom2d::{
    Geom2dBoundedCurve, Geom2dCircle, Geom2dConic, Geom2dCurve, Geom2dEllipse, Geom2dGeometry,
    Geom2dHyperbola, Geom2dOffsetCurve, Geom2dParabola, Geom2dTrimmedCurve,
};
use martensite_tkmath::gp::Axis22d;

fn type_name(output: &Output, c: &Handle<dyn Geom2dCurve>) {
    replay_print!(output, "{} ", c.dynamic_type().unwrap().name());
}

pub fn curve_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built(output, &mut r) else {
        return;
    };
    let u = r.reals();
    type_name(output, &c);
    summary(output, &*c, &u);
    dump(output, &*c, -1);
    dump(output, &*c, 1);
    end(output);
}

fn show_curve(output: &Output, g: &Handle<dyn Geom2dGeometry>) {
    let c: Handle<dyn Geom2dCurve> = g.cast();
    summary(output, &*c, &output.parameters());
}

pub fn transform_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built(output, &mut r) else {
        return;
    };
    let u = r.reals();
    output.set_parameters(u.clone());
    let t = trsf(&mut r);
    for &v in &u {
        real(output, c.transformed_parameter(v, t));
    }
    real(output, c.parametric_transformation(t));
    show(output, c.reversed(), |r| show_curve(output, &r.cast()));
    attempt(output, || {
        c.reverse()?;
        show_curve(output, &c.cast());
        Ok(())
    });
    moves(output, &c.cast(), &mut r, show_curve);
    end(output);
}

fn conic_common(output: &Output, c: &dyn Geom2dConic) {
    ax(output, c.x_axis());
    ax(output, c.y_axis());
    pnt(output, c.location());
    ax22(output, c.position());
    show(output, c.eccentricity(), |value| real(output, value));
}

fn circle_specific(output: &Output, e: &Geom2dCircle, value: f64) {
    real(output, e.radius());
    attempt(output, || e.set_radius(value));
    real(output, e.radius());
    let g = e.circ2d().unwrap();
    ax22(output, g.axis());
    real(output, g.radius());
    e.set_circ2d(g);
}

fn ellipse_specific(output: &Output, e: &Geom2dEllipse, value: f64) {
    real(output, e.major_radius());
    real(output, e.minor_radius());
    show(output, e.directrix1(), |value| ax(output, value));
    show(output, e.directrix2(), |value| ax(output, value));
    real(output, e.focal());
    pnt(output, e.focus1());
    pnt(output, e.focus2());
    real(output, e.parameter());
    attempt(output, || e.set_minor_radius(value));
    attempt(output, || e.set_major_radius(value));
    real(output, e.major_radius());
    real(output, e.minor_radius());
    attempt(output, || {
        let g = e.elips2d()?;
        ax22(output, g.axis());
        e.set_elips2d(g);
        Ok(())
    });
}

fn hyperbola_specific(output: &Output, e: &Geom2dHyperbola, value: f64) {
    real(output, e.major_radius());
    real(output, e.minor_radius());
    show(output, e.asymptote1(), |value| ax(output, value));
    show(output, e.asymptote2(), |value| ax(output, value));
    show(output, e.conjugate_branch1(), |h| ax22(output, h.axis()));
    show(output, e.conjugate_branch2(), |h| ax22(output, h.axis()));
    show(output, e.directrix1(), |value| ax(output, value));
    show(output, e.directrix2(), |value| ax(output, value));
    real(output, e.focal());
    pnt(output, e.focus1());
    pnt(output, e.focus2());
    show(output, e.other_branch(), |h| ax22(output, h.axis()));
    show(output, e.parameter(), |value| real(output, value));
    attempt(output, || e.set_major_radius(value));
    attempt(output, || e.set_minor_radius(-value));
    real(output, e.major_radius());
    real(output, e.minor_radius());
    attempt(output, || {
        let g = e.hypr2d()?;
        ax22(output, g.axis());
        e.set_hypr2d(g);
        Ok(())
    });
}

fn parabola_specific(output: &Output, e: &Geom2dParabola, value: f64) {
    real(output, e.focal());
    pnt(output, e.focus());
    real(output, e.parameter());
    show(output, e.directrix(), |value| ax(output, value));
    attempt(output, || e.set_focal(value));
    real(output, e.focal());
    attempt(output, || {
        let g = e.parab2d()?;
        ax22(output, g.axis());
        e.set_parab2d(g);
        Ok(())
    });
}

fn conic_specific(output: &Output, c: &Handle<dyn Geom2dCurve>, value: f64) {
    if let Some(e) = c.cast::<Geom2dCircle>().get() {
        circle_specific(output, e, value);
    }
    if let Some(e) = c.cast::<Geom2dEllipse>().get() {
        ellipse_specific(output, e, value);
    }
    if let Some(e) = c.cast::<Geom2dHyperbola>().get() {
        hyperbola_specific(output, e, value);
    }
    if let Some(e) = c.cast::<Geom2dParabola>().get() {
        parabola_specific(output, e, value);
    }
}

pub fn conic_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(curve) = built(output, &mut r) else {
        return;
    };
    let u = r.reals();
    let c: Handle<dyn Geom2dConic> = curve.cast();
    let value = r.real();
    let (a, p) = (r.ax(), r.pnt());
    conic_common(output, &*c);
    conic_specific(output, &curve, value);
    summary(output, &*c, &u);
    c.set_location(p);
    conic_common(output, &*c);
    attempt(output, || c.set_x_axis(a));
    conic_common(output, &*c);
    attempt(output, || c.set_y_axis(a));
    conic_common(output, &*c);
    c.set_axis(Axis22d::new(p, a.direction(), true).unwrap());
    conic_common(output, &*c);
    c.reverse().unwrap();
    conic_common(output, &*c);
    summary(output, &*c, &u);
    dump(output, &*c, -1);
    end(output);
}

fn trimmed_state(output: &Output, t: &Geom2dTrimmedCurve, u: &[f64]) {
    real(output, t.first_parameter());
    real(output, t.last_parameter());
    show(output, t.start_point(), |value| pnt(output, value));
    show(output, t.end_point(), |value| pnt(output, value));
    type_name(output, &t.basis_curve());
    summary(output, t, u);
    dump(output, t, -1);
}

fn trimmed_copy(output: &Output, t: &Geom2dTrimmedCurve, u: &[f64]) -> Result<(), Failure> {
    let copy: Handle<Geom2dTrimmedCurve> = t.copy()?.cast();
    trimmed_state(output, &copy, u);
    Ok(())
}

pub fn trimmed_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built(output, &mut r) else {
        return;
    };
    let u = r.reals();
    let t: Handle<Geom2dTrimmedCurve> = c.cast();
    let (u1, u2) = (r.real(), r.real());
    let (sense, adjust) = (r.flag(), r.flag());
    let m = trsf(&mut r);
    trimmed_state(output, &t, &u);
    attempt(output, || trimmed_copy(output, &t, &u));
    attempt(output, || t.set_trim(u1, u2, sense, adjust));
    trimmed_state(output, &t, &u);
    attempt(output, || trimmed_copy(output, &t, &u));
    attempt(output, || t.reverse());
    trimmed_state(output, &t, &u);
    attempt(output, || t.transform(m));
    trimmed_state(output, &t, &u);
    real(output, t.transformed_parameter(0.5, m));
    real(output, t.parametric_transformation(m));
    end(output);
}

fn offset_state(output: &Output, o: &Geom2dOffsetCurve, u: &[f64]) {
    real(output, o.offset());
    int(output, o.basis_curve_continuity() as i32);
    type_name(output, &o.basis_curve());
    flag(output, o.has_eval_representation());
    summary(output, o, u);
    dump(output, o, -1);
}

pub fn offset_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built(output, &mut r) else {
        return;
    };
    let u = r.reals();
    let o: Handle<Geom2dOffsetCurve> = c.cast();
    let d = r.real();
    let no_check = r.flag();
    let m = trsf(&mut r);
    let basis = curve(&mut r).unwrap();
    offset_state(output, &o, &u);
    offset_state(output, &o.copy().unwrap().cast::<Geom2dOffsetCurve>(), &u);
    o.set_offset_value(d);
    offset_state(output, &o, &u);
    attempt(output, || o.reverse());
    offset_state(output, &o, &u);
    attempt(output, || o.transform(m));
    offset_state(output, &o, &u);
    attempt(output, || o.set_basis_curve(&basis, no_check));
    offset_state(output, &o, &u);
    attempt(output, || o.set_basis_curve(&o.cast(), no_check));
    offset_state(output, &o, &u);
    real(output, o.transformed_parameter(0.5, m));
    real(output, o.parametric_transformation(m));
    end(output);
}

pub fn offset_utils_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let p = r.pnt();
    let (d1, d2, d3, d4) = (r.vec(), r.vec(), r.vec(), r.vec());
    let offset = r.real();
    let change = r.flag();
    let mut v = p;
    attempt(output, || {
        flag(output, calculate_d0(&mut v, d1, offset)?);
        pnt(output, v);
        Ok(())
    });
    let (mut a1, mut v) = (d1, p);
    flag(output, calculate_d1(&mut v, &mut a1, d2, offset));
    pnt(output, v);
    vec(output, a1);
    let (mut a1, mut a2, mut v) = (d1, d2, p);
    flag(
        output,
        calculate_d2(&mut v, &mut a1, &mut a2, d3, change, offset),
    );
    pnt(output, v);
    vec(output, a1);
    vec(output, a2);
    let (mut a1, mut a2, mut a3, mut v) = (d1, d2, d3, p);
    flag(
        output,
        calculate_d3(&mut v, [&mut a1, &mut a2, &mut a3], d4, change, offset),
    );
    pnt(output, v);
    [a1, a2, a3]
        .into_iter()
        .for_each(|value| vec(output, value));
    end(output);
}
