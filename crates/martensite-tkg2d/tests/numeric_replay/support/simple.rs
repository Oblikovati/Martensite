//! Points, directions, vectors, axis placements, transformations and the geometry moves;
//! the counterpart of the frozen behavior fixtures
use super::common::{Reader, attempt, ax, dir, dump, end, flag, int, pnt, real, show, trsf, vec};
use super::output::Output;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{
    Geom2dAxisPlacement, Geom2dCartesianPoint, Geom2dDirection, Geom2dGeometry, Geom2dPoint,
    Geom2dTransformation, Geom2dVector, Geom2dVectorWithMagnitude,
};
use martensite_tkmath::gp::{Direction2, Transform2};

/// The show callback of g2_moves.
pub type Show = fn(&Output, &Handle<dyn Geom2dGeometry>);

fn shown(
    output: &Output,
    show: Show,
    result: Result<Handle<dyn Geom2dGeometry>, Failure>,
) -> Result<(), Failure> {
    show(output, &result?);
    Ok(())
}

/// The Mirror/Rotate/Scale/Translate members and their copying twins (g2_moves).
pub fn moves(output: &Output, g: &Handle<dyn Geom2dGeometry>, r: &mut Reader, show: Show) {
    let (p, a) = (r.pnt(), r.ax());
    let (angle, factor) = (r.real(), r.real());
    let (v, q) = (r.vec(), r.pnt());
    let t: Transform2 = trsf(r);
    attempt(output, || shown(output, show, g.mirrored_point(p)));
    attempt(output, || shown(output, show, g.mirrored_axis(a)));
    attempt(output, || shown(output, show, g.rotated(p, angle)));
    attempt(output, || shown(output, show, g.scaled(p, factor)));
    attempt(output, || shown(output, show, g.transformed(t)));
    attempt(output, || shown(output, show, g.translated(v)));
    attempt(output, || shown(output, show, g.translated_between(p, q)));
    let steps: [&dyn Fn() -> Result<(), Failure>; 7] = [
        &|| g.mirror_point(p),
        &|| g.mirror_axis(a),
        &|| g.rotate(p, angle),
        &|| g.scale(p, factor),
        &|| g.translate(v),
        &|| g.translate_between(p, q),
        &|| g.transform(t),
    ];
    for step in steps {
        attempt(output, || {
            step()?;
            show(output, g);
            Ok(())
        });
    }
}

fn show_point(output: &Output, g: &Handle<dyn Geom2dGeometry>) {
    let p: Handle<Geom2dCartesianPoint> = g.cast();
    let (x, y) = p.coord();
    [x, y].into_iter().for_each(|value| real(output, value));
    pnt(output, p.pnt2d());
    real(output, p.x());
    real(output, p.y());
    dump(output, &*p, -1);
}

pub fn point_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (a, b) = (r.pnt(), r.pnt());
    let (x, y) = (r.real(), r.real());
    let p = Handle::new(Geom2dCartesianPoint::new(a.x(), a.y()));
    let o = Handle::new(Geom2dCartesianPoint::from_pnt2d(b));
    real(output, p.distance(&*o));
    real(output, p.square_distance(&*o));
    show_point(output, &p.copy().unwrap());
    p.set_coord(x, y);
    show_point(output, &p.cast());
    p.set_x(y);
    p.set_y(x);
    show_point(output, &p.cast());
    p.set_pnt2d(b);
    show_point(output, &p.cast());
    dump(output, &*p, 0);
    dump(output, &*p, 1);
    moves(output, &p.cast(), &mut r, show_point);
    end(output);
}

fn show_vector(output: &Output, g: &Handle<dyn Geom2dGeometry>) {
    let v: Handle<dyn Geom2dVector> = g.cast();
    let (x, y) = v.coord();
    [x, y].into_iter().for_each(|value| real(output, value));
    vec(output, v.vec2d());
    [v.x(), v.y(), v.magnitude(), v.square_magnitude()]
        .into_iter()
        .for_each(|value| real(output, value));
    dump(output, &*v, -1);
}

fn vector_pair(output: &Output, v: &dyn Geom2dVector, o: &dyn Geom2dVector) {
    show(output, v.angle(o), |value| real(output, value));
    real(output, v.dot(o));
    real(output, v.crossed(o));
    show(output, v.reversed(), |r| show_vector(output, &r.cast()));
}

pub fn direction_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (x, y, a, b) = (r.real(), r.real(), r.real(), r.real());
    let other = r.vec();
    let d = match Geom2dDirection::new(x, y) {
        Ok(d) => Handle::new(d),
        Err(error) => return replay_println!(output, "{}", error.exception_type()),
    };
    let o = Handle::new(Geom2dVectorWithMagnitude::from_vec2d(other));
    show_vector(output, &d.cast());
    dir(output, d.dir2d().unwrap());
    vector_pair(output, &*d, &*o);
    real(output, d.crossed(&*o));
    attempt(output, || d.set_coord(a, b));
    show_vector(output, &d.cast());
    attempt(output, || d.set_x(a));
    show_vector(output, &d.cast());
    attempt(output, || d.set_y(b));
    show_vector(output, &d.cast());
    d.set_dir2d(Direction2::new(1.0, 1.0).unwrap());
    show_vector(output, &d.cast());
    d.reverse();
    show_vector(output, &d.cast());
    show_vector(output, &d.copy().unwrap());
    moves(output, &d.cast(), &mut r, show_vector);
    end(output);
}

fn show_vwm(output: &Output, v: &Handle<Geom2dVectorWithMagnitude>) {
    show_vector(output, &v.cast());
}

pub fn vector_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (a, b) = (r.vec(), r.vec());
    let s = r.real();
    let (p, q) = (r.pnt(), r.pnt());
    let v = Handle::new(Geom2dVectorWithMagnitude::new(a.x(), a.y()));
    let o = Handle::new(Geom2dVectorWithMagnitude::from_vec2d(b));
    show_vwm(
        output,
        &Handle::new(Geom2dVectorWithMagnitude::between(p, q)),
    );
    vector_pair(output, &*v, &*o);
    show_vwm(output, &v.added(&*o));
    show_vwm(output, &v.subtracted(&*o));
    show_vwm(output, &v.multiplied(s));
    show_vwm(output, &v.divided(s));
    show(output, v.normalized(), |n| show_vwm(output, &n));
    show_vwm(output, &v.added(&*o));
    show_vwm(output, &v.subtracted(&*o));
    show_vwm(output, &v.divided(s));
    real(output, v.crossed(&*o));
    vector_updates(output, &v, &o, s);
    v.set_coord(s, -s);
    show_vwm(output, &v);
    v.set_x(a.x());
    v.set_y(a.y());
    show_vwm(output, &v);
    v.set_vec2d(b);
    show_vwm(output, &v);
    moves(output, &v.cast(), &mut r, show_vector);
    end(output);
}

// The in-place arithmetic block of g2_vector_case, operators included.
fn vector_updates(
    output: &Output,
    v: &Handle<Geom2dVectorWithMagnitude>,
    o: &Handle<Geom2dVectorWithMagnitude>,
    s: f64,
) {
    v.add(&**o);
    show_vwm(output, v);
    v.subtract(&**o);
    show_vwm(output, v);
    v.multiply(s);
    show_vwm(output, v);
    v.divide(s);
    show_vwm(output, v);
    attempt(output, || v.normalize());
    show_vwm(output, v);
    v.add(&**o);
    v.subtract(&**o);
    v.multiply(s);
    v.divide(s);
    show_vwm(output, v);
}

fn show_axis(output: &Output, g: &Handle<dyn Geom2dGeometry>) {
    let a: Handle<Geom2dAxisPlacement> = g.cast();
    ax(output, a.ax2d());
    dir(output, a.direction());
    pnt(output, a.location());
    dump(output, &*a, -1);
}

pub fn axis_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let first = r.ax();
    let (p, d) = (r.pnt(), r.dir());
    let other = r.ax();
    let a = Handle::new(Geom2dAxisPlacement::new(first));
    let o = Handle::new(Geom2dAxisPlacement::from_location(p, d));
    real(output, a.angle(&o));
    show_axis(output, &a.reversed().cast());
    a.reverse();
    show_axis(output, &a.cast());
    a.set_location(p);
    show_axis(output, &a.cast());
    a.set_direction(d);
    show_axis(output, &a.cast());
    a.set_axis(other);
    show_axis(output, &a.cast());
    show_axis(output, &a.copy().unwrap());
    moves(output, &a.cast(), &mut r, show_axis);
    end(output);
}

fn trsf_state(output: &Output, t: &Geom2dTransformation) {
    flag(output, t.is_negative());
    int(output, t.form() as i32);
    real(output, t.scale_factor());
    for row in 0..=3 {
        for col in 0..=4 {
            show(output, t.value(row, col), |value| real(output, value));
        }
    }
    let (x, y) = t.transforms(1.25, -3.5);
    real(output, x);
    real(output, y);
}

fn initial_trsf(
    r: &mut Reader,
) -> (
    Handle<Geom2dTransformation>,
    i32,
    martensite_tkmath::gp::Point2,
    f64,
) {
    let kind = r.integer();
    let (p, q) = (r.pnt(), r.pnt());
    let (a, b) = (r.ax(), r.ax());
    let s = r.real();
    let n = r.integer();
    let t = Handle::new(Geom2dTransformation::default());
    match kind {
        0 => t.set_mirror_point(p),
        1 => t.set_mirror_axis(a),
        2 => t.set_rotation(p, s),
        3 => t.set_scale(p, s),
        4 => t.set_transformation_between(a, b),
        5 => t.set_transformation_to(a),
        6 => t.set_translation(martensite_tkmath::gp::Vector2::between(p, q)),
        7 => t.set_translation_between(p, q),
        8 => {
            let mut g = Transform2::default();
            g.set_rotation(q, s);
            t.set_trsf2d(g);
        }
        _ => {}
    }
    (t, n, q, s)
}

pub fn trsf_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (t, n, q, _) = initial_trsf(&mut r);
    trsf_state(output, &t);
    let o = Handle::new(Geom2dTransformation::new(t.trsf2d()));
    o.set_rotation(q, 0.75);
    show(output, t.inverted(), |i| trsf_state(output, &i));
    trsf_state(output, &t.multiplied(&o));
    trsf_state(output, &t.multiplied(&o));
    show(output, t.powered(n), |p| trsf_state(output, &p));
    trsf_state(output, &t.copy());
    let c = t.copy();
    c.multiply(&o);
    trsf_state(output, &c);
    let c = t.copy();
    c.multiply(&o);
    trsf_state(output, &c);
    let c = t.copy();
    c.pre_multiply(&o);
    trsf_state(output, &c);
    let c = t.copy();
    attempt(output, || c.power(n));
    trsf_state(output, &c);
    let c = t.copy();
    attempt(output, || c.invert());
    trsf_state(output, &c);
    end(output);
}
