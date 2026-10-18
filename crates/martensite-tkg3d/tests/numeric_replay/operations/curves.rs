//! Curve construction, copying and transformation requests.
use super::common::{
    Reader, attempt, ax1, built_curve, curve_summary, dump, end, name, pnt, real, show, trsf,
};
use super::common::{dir, flag, int, vec};
use super::output::Output;
use super::simple::moves;
use martensite_tkernel::standard::Handle;
use martensite_tkg3d::geom::offset_curve_utils as utils;
use martensite_tkg3d::geom::{
    GeomBoundedCurve, GeomCurve, GeomGeometry, GeomLine, GeomOffsetCurve, GeomTrimmedCurve,
};
use martensite_tkmath::gp::Line3;

pub fn curve_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built_curve(output, &mut r) else {
        return;
    };
    let u = r.reals();
    name(output, &c);
    curve_summary(output, &*c, &u);
    for depth in [-1, 1, 0] {
        dump(output, &*c, depth);
    }
    end(output);
}

/// Prints the summary of a moved curve at the parameters of the request (g3_show_curve).
pub fn show_curve(output: &Output, g: &Handle<dyn GeomGeometry>) {
    let c: Handle<dyn GeomCurve> = g.cast();
    curve_summary(output, &*c, &output.parameters());
}

pub fn transform_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built_curve(output, &mut r) else {
        return;
    };
    let params = r.reals();
    output.set_parameters(params.clone());
    let t = trsf(&mut r);
    for &u in &params {
        real(output, c.transformed_parameter(u, t));
    }
    real(output, c.parametric_transformation(t));
    show(output, c.reversed(), |copy| {
        show_curve(output, &copy.cast())
    });
    show(output, c.reverse(), |()| show_curve(output, &c.cast()));
    moves(output, &c.cast(), &mut r, show_curve);
    end(output);
}

pub fn line_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (a, d, p, b) = (r.ax1(), r.dir(), r.pnt(), r.ax1());
    let u = r.reals();
    let l = Handle::new(GeomLine::new(a));
    let m = GeomLine::from_lin(Line3::from_axis(b));
    let n = GeomLine::from_point(p, d);
    ax1(output, l.position());
    ax1(output, l.lin().position());
    ax1(output, m.position());
    ax1(output, n.position());
    l.set_direction(d);
    ax1(output, l.position());
    l.set_location(p);
    ax1(output, l.position());
    l.set_position(b);
    ax1(output, l.position());
    l.set_lin(Line3::from_axis(a));
    curve_summary(output, &*l, &u);
    dump(output, &*l, -1);
    end(output);
}

fn trimmed_state(output: &Output, t: &GeomTrimmedCurve, u: &[f64]) {
    real(output, t.first_parameter());
    real(output, t.last_parameter());
    show(output, t.start_point(), |value| pnt(output, value));
    show(output, t.end_point(), |value| pnt(output, value));
    name(output, &t.basis_curve());
    curve_summary(output, t, u);
    dump(output, t, -1);
}

fn copied_state(output: &Output, t: &GeomTrimmedCurve, u: &[f64]) {
    show(output, t.copy(), |c| {
        trimmed_state(output, &c.cast::<GeomTrimmedCurve>(), u)
    });
}

pub fn trimmed_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built_curve(output, &mut r) else {
        return;
    };
    let u = r.reals();
    let t: Handle<GeomTrimmedCurve> = c.cast();
    let (u1, u2) = (r.real(), r.real());
    let (sense, adjust) = (r.flag(), r.flag());
    let m = trsf(&mut r);
    trimmed_state(output, &t, &u);
    copied_state(output, &t, &u);
    attempt(output, || t.set_trim(u1, u2, sense, adjust));
    trimmed_state(output, &t, &u);
    copied_state(output, &t, &u);
    attempt(output, || t.reverse());
    trimmed_state(output, &t, &u);
    attempt(output, || t.transform(m));
    trimmed_state(output, &t, &u);
    real(output, t.transformed_parameter(0.5, m));
    real(output, t.parametric_transformation(m));
    end(output);
}

/// GeomOffsetCurveUtils::CalculateD0..D3 directly (g3_offset_utils).
pub fn offset_utils_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let p = r.pnt();
    let (d1, d2, d3, d4) = (r.vec(), r.vec(), r.vec(), r.vec());
    let dir = *r.vec().xyz();
    let (offset, change, tol) = (r.real(), r.flag(), r.real());
    let mut v = p;
    flag(output, utils::calculate_d0(&mut v, d1, dir, offset, tol));
    pnt(output, v);
    let (mut v, mut a1) = (p, d1);
    flag(
        output,
        utils::calculate_d1(&mut v, &mut a1, d2, dir, offset, tol),
    );
    pnt(output, v);
    vec(output, a1);
    let (mut v, mut a1, mut a2) = (p, d1, d2);
    flag(
        output,
        utils::calculate_d2(&mut v, [&mut a1, &mut a2], d3, dir, offset, change, tol),
    );
    pnt(output, v);
    [a1, a2].into_iter().for_each(|value| vec(output, value));
    let (mut v, mut a1, mut a2, mut a3) = (p, d1, d2, d3);
    let derivatives = [&mut a1, &mut a2, &mut a3];
    flag(
        output,
        utils::calculate_d3(&mut v, derivatives, d4, dir, offset, change, tol),
    );
    pnt(output, v);
    [a1, a2, a3]
        .into_iter()
        .for_each(|value| vec(output, value));
    end(output);
}

fn offset_state(output: &Output, o: &GeomOffsetCurve, u: &[f64]) {
    real(output, o.offset());
    dir(output, o.direction());
    int(output, o.basis_curve_continuity() as i32);
    name(output, &o.basis_curve());
    flag(output, o.has_eval_representation());
    curve_summary(output, o, u);
    dump(output, o, -1);
}

/// Offset curve state, copy, setters, reversal and transformation (g3_offset).
pub fn offset_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built_curve(output, &mut r) else {
        return;
    };
    let u = r.reals();
    let o: Handle<GeomOffsetCurve> = c.cast();
    let (d, v, no_check) = (r.real(), r.dir(), r.flag());
    let m = trsf(&mut r);
    let basis = super::shapes::curve(output, &mut r).unwrap();
    offset_state(output, &o, &u);
    offset_state(output, &o.copy().unwrap().cast::<GeomOffsetCurve>(), &u);
    o.set_offset_value(d);
    offset_state(output, &o, &u);
    o.set_direction(v);
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
