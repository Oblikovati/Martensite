//! Points, axes, vectors and shared geometry requests.
use super::common::{
    Reader, attempt, ax1, ax2, dir, dump, end, flag, int, pnt, real, show, trsf, vec,
};
use super::output::Output;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{
    GeomAxis1Placement, GeomAxis2Placement, GeomAxisPlacement, GeomCartesianPoint, GeomDirection,
    GeomGeometry, GeomPoint, GeomTransformation, GeomVector, GeomVectorWithMagnitude,
};
use martensite_tkmath::gp::{Axis1, Axis2, Direction3, Point3};

/// How a probe prints one geometry after each move.
pub type Show = fn(&Output, &Handle<dyn GeomGeometry>);

/// The Mirror/Rotate/Scale/Translate members and their copying twins (g3_moves).
pub fn moves(output: &Output, g: &Handle<dyn GeomGeometry>, r: &mut Reader, show_geometry: Show) {
    let (p, a, f) = (r.pnt(), r.ax1(), r.ax2());
    let (angle, factor) = (r.real(), r.real());
    let (v, q) = (r.vec(), r.pnt());
    let t = trsf(r);
    let copies: [&dyn Fn() -> Result<Handle<dyn GeomGeometry>, Failure>; 8] = [
        &|| g.mirrored_point(p),
        &|| g.mirrored_axis(a),
        &|| g.mirrored_plane(f),
        &|| g.rotated(a, angle),
        &|| g.scaled(p, factor),
        &|| g.transformed(t),
        &|| g.translated(v),
        &|| g.translated_between(p, q),
    ];
    for copy in copies {
        show(output, copy(), |c| show_geometry(output, &c));
    }
    let steps: [&dyn Fn() -> Result<(), Failure>; 8] = [
        &|| g.mirror_point(p),
        &|| g.mirror_axis(a),
        &|| g.mirror_plane(f),
        &|| g.rotate(a, angle),
        &|| g.scale(p, factor),
        &|| g.translate(v),
        &|| g.translate_between(p, q),
        &|| g.transform(t),
    ];
    for step in steps {
        show(output, step(), |()| show_geometry(output, g));
    }
}

fn show_point(output: &Output, g: &Handle<dyn GeomGeometry>) {
    let p: Handle<GeomCartesianPoint> = g.cast();
    let (x, y, z) = p.coord();
    [x, y, z].into_iter().for_each(|value| real(output, value));
    pnt(output, p.pnt());
    [p.x(), p.y(), p.z()]
        .into_iter()
        .for_each(|value| real(output, value));
    dump(output, &*p, -1);
}

pub fn point_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (a, b) = (r.pnt(), r.pnt());
    let (x, y, z) = (r.real(), r.real(), r.real());
    let p = Handle::new(GeomCartesianPoint::new(a.x(), a.y(), a.z()));
    let o = Handle::new(GeomCartesianPoint::from_pnt(b));
    real(output, p.distance(&*o));
    real(output, p.square_distance(&*o));
    show_point(output, &p.copy().unwrap());
    p.set_coord(x, y, z);
    show_point(output, &p.cast());
    p.set_x(y);
    p.set_y(z);
    p.set_z(x);
    show_point(output, &p.cast());
    p.set_pnt(b);
    show_point(output, &p.cast());
    dump(output, &*p, 0);
    dump(output, &*p, 1);
    moves(output, &p.cast(), &mut r, show_point);
    end(output);
}

fn show_vector(output: &Output, g: &Handle<dyn GeomGeometry>) {
    let v: Handle<dyn GeomVector> = g.cast();
    let (x, y, z) = v.coord();
    [x, y, z].into_iter().for_each(|value| real(output, value));
    vec(output, v.vec());
    [v.x(), v.y(), v.z(), v.magnitude(), v.square_magnitude()]
        .into_iter()
        .for_each(|value| real(output, value));
    dump(output, &*v, -1);
}

fn vector_pair(
    output: &Output,
    v: &dyn GeomVector,
    o: &dyn GeomVector,
    reference: &dyn GeomVector,
) {
    show(output, v.angle(o), |value| real(output, value));
    show(output, v.angle_with_ref(o, reference), |value| {
        real(output, value)
    });
    real(output, v.dot(o));
    real(output, v.dot_cross(o, reference));
    show(output, v.crossed(o), |c| show_vector(output, &c.cast()));
    show(output, v.cross_crossed(o, reference), |c| {
        show_vector(output, &c.cast())
    });
    show(output, v.reversed(), |c| show_vector(output, &c.cast()));
    show(output, v.copy(), |c| show_vector(output, &c));
}

fn vector_cross(
    output: &Output,
    v: &dyn GeomVector,
    o: &dyn GeomVector,
    reference: &dyn GeomVector,
) {
    let c: Handle<dyn GeomVector> = v.copy().unwrap().cast();
    attempt(output, || c.cross(o));
    show_vector(output, &c.cast());
    let c: Handle<dyn GeomVector> = v.copy().unwrap().cast();
    attempt(output, || c.cross_cross(o, reference));
    show_vector(output, &c.cast());
    let c: Handle<dyn GeomVector> = v.copy().unwrap().cast();
    attempt(output, || c.cross(&*c));
    show_vector(output, &c.cast());
}

pub fn direction_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (x, y, z) = (r.real(), r.real(), r.real());
    let (a, b, c) = (r.real(), r.real(), r.real());
    let (other, reference) = (r.vec(), r.vec());
    let d = match GeomDirection::new(x, y, z) {
        Ok(d) => Handle::new(d),
        Err(error) => {
            replay_println!(output, "{}", error.exception_type());
            return;
        }
    };
    let o = GeomVectorWithMagnitude::from_vec(other);
    let f = GeomVectorWithMagnitude::from_vec(reference);
    show_vector(output, &d.cast());
    dir(output, d.dir().unwrap());
    vector_pair(output, &*d, &o, &f);
    vector_cross(output, &*d, &o, &f);
    attempt(output, || d.set_coord(a, b, c));
    show_vector(output, &d.cast());
    attempt(output, || d.set_x(a));
    show_vector(output, &d.cast());
    attempt(output, || d.set_y(b));
    show_vector(output, &d.cast());
    attempt(output, || d.set_z(c));
    show_vector(output, &d.cast());
    d.set_dir(Direction3::new(1.0, 1.0, 1.0).unwrap());
    show_vector(output, &d.cast());
    d.reverse();
    show_vector(output, &d.cast());
    moves(output, &d.cast(), &mut r, show_vector);
    end(output);
}

pub fn vector_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (a, b, reference) = (r.vec(), r.vec(), r.vec());
    let s = r.real();
    let (p, q) = (r.pnt(), r.pnt());
    let v = Handle::new(GeomVectorWithMagnitude::new(a.x(), a.y(), a.z()));
    let o = GeomVectorWithMagnitude::from_vec(b);
    let f = GeomVectorWithMagnitude::from_vec(reference);
    show_vector(
        output,
        &Handle::new(GeomVectorWithMagnitude::between(p, q)).cast(),
    );
    vector_pair(output, &*v, &o, &f);
    vector_cross(output, &*v, &o, &f);
    show_vector(output, &v.added(&o).cast());
    show_vector(output, &v.subtracted(&o).cast());
    show_vector(output, &v.multiplied(s).cast());
    show_vector(output, &v.divided(s).cast());
    show(output, v.normalized(), |n| show_vector(output, &n.cast()));
    vector_in_place(output, &v, &o, s);
    v.set_coord(s, -s, 2.0 * s);
    show_vector(output, &v.cast());
    v.set_x(a.x());
    v.set_y(a.y());
    v.set_z(a.z());
    show_vector(output, &v.cast());
    v.set_vec(b);
    show_vector(output, &v.cast());
    moves(output, &v.cast(), &mut r, show_vector);
    end(output);
}

fn vector_in_place(
    output: &Output,
    v: &Handle<GeomVectorWithMagnitude>,
    o: &GeomVectorWithMagnitude,
    s: f64,
) {
    v.add(o);
    show_vector(output, &v.cast());
    v.subtract(o);
    show_vector(output, &v.cast());
    v.multiply(s);
    show_vector(output, &v.cast());
    v.divide(s);
    show_vector(output, &v.cast());
    attempt(output, || v.normalize());
    show_vector(output, &v.cast());
}

fn show_axis1(output: &Output, g: &Handle<dyn GeomGeometry>) {
    let a: Handle<GeomAxis1Placement> = g.cast();
    ax1(output, a.ax1());
    ax1(output, a.axis());
    dir(output, a.direction());
    pnt(output, a.location());
    dump(output, &*a, -1);
}

pub fn axis1_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (a, b, d, p) = (r.ax1(), r.ax1(), r.dir(), r.pnt());
    let x = Handle::new(GeomAxis1Placement::new(a));
    let y = GeomAxis1Placement::from_location(b.location(), b.direction());
    show_axis1(output, &x.cast());
    real(output, x.angle(&y));
    show_axis1(output, &x.reversed().cast());
    show_axis1(output, &x.copy().unwrap());
    x.reverse();
    show_axis1(output, &x.cast());
    x.set_direction(d).unwrap();
    show_axis1(output, &x.cast());
    x.set_location(p);
    show_axis1(output, &x.cast());
    x.set_axis(b);
    show_axis1(output, &x.cast());
    moves(output, &x.cast(), &mut r, show_axis1);
    end(output);
}

fn show_axis2(output: &Output, g: &Handle<dyn GeomGeometry>) {
    let a: Handle<GeomAxis2Placement> = g.cast();
    show(output, a.ax2(), |value| ax2(output, value));
    ax1(output, a.axis());
    dir(output, a.x_direction());
    dir(output, a.y_direction());
    dump(output, &*a, -1);
}

pub fn axis2_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (f, p) = (r.ax2(), r.pnt());
    let (n, vx, d, e) = (r.dir(), r.dir(), r.dir(), r.dir());
    let a = r.ax1();
    let x = Handle::new(GeomAxis2Placement::new(f));
    let y = match GeomAxis2Placement::from_directions(p, n, vx) {
        Ok(y) => Handle::new(y),
        Err(error) => {
            replay_print!(output, "{} ", error.exception_type());
            x.clone()
        }
    };
    show_axis2(output, &x.cast());
    show_axis2(output, &y.cast());
    real(output, x.angle(&*y));
    show_axis2(output, &y.copy().unwrap());
    attempt(output, || x.set_direction(d));
    show_axis2(output, &x.cast());
    attempt(output, || x.set_x_direction(e));
    show_axis2(output, &x.cast());
    attempt(output, || x.set_y_direction(d));
    show_axis2(output, &x.cast());
    x.set_location(p);
    show_axis2(output, &x.cast());
    x.set_axis(a);
    show_axis2(output, &x.cast());
    x.set_ax2(f);
    show_axis2(output, &x.cast());
    moves(output, &x.cast(), &mut r, show_axis2);
    end(output);
}

fn show_trsf(output: &Output, t: &GeomTransformation) {
    flag(output, t.is_negative());
    int(output, t.form() as i32);
    real(output, t.scale_factor());
    for i in 1..=3 {
        for j in 1..=4 {
            real(output, t.value(i, j).unwrap());
        }
    }
    show(output, t.value(0, 1), |value| real(output, value));
    let (x, y, z) = t.transforms(1.5, -2., 0.25);
    [x, y, z].into_iter().for_each(|value| real(output, value));
    trsf_dump(output, t, -1);
}

fn trsf_dump(output: &Output, t: &GeomTransformation, depth: i32) {
    let mut stream = DumpStream::default();
    t.dump_json(&mut stream, depth).unwrap();
    replay_print!(output, "{{{}}} ", stream.text());
}

pub fn trsf_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (a, b) = (trsf(&mut r), trsf(&mut r));
    let n = r.integer();
    let (f, h, p, q) = (r.ax3(), r.ax3(), r.pnt(), r.pnt());
    let (ax, pl) = (r.ax1(), r.ax2());
    let (angle, factor, v) = (r.real(), r.real(), r.vec());
    let t = GeomTransformation::new(a);
    let o = GeomTransformation::new(b);
    show_trsf(output, &GeomTransformation::default());
    show_trsf(output, &t);
    show_trsf(output, &t.copy());
    show_trsf(output, &t.multiplied(&o));
    show(output, t.inverted(), |i| show_trsf(output, &i));
    show(output, t.powered(n), |i| show_trsf(output, &i));
    let c: Handle<GeomTransformation> = t.copy();
    c.multiply(&o);
    show_trsf(output, &c);
    c.pre_multiply(&o);
    show_trsf(output, &c);
    attempt(output, || c.power(n));
    show_trsf(output, &c);
    attempt(output, || c.invert());
    show_trsf(output, &c);
    trsf_setters(output, &c, (p, q), (ax, pl), (angle, factor));
    c.set_transformation_between(f, h);
    show_trsf(output, &c);
    c.set_transformation_to(f);
    show_trsf(output, &c);
    c.set_translation(v);
    show_trsf(output, &c);
    c.set_translation_between(p, q);
    show_trsf(output, &c);
    c.set_trsf(a);
    show_trsf(output, &c);
    trsf_dump(output, &t, 0);
    end(output);
}

type Axes = (Axis1, Axis2);

fn trsf_setters(
    output: &Output,
    c: &GeomTransformation,
    points: (Point3, Point3),
    axes: Axes,
    values: (f64, f64),
) {
    let (p, _) = points;
    let (ax, pl) = axes;
    c.set_mirror_point(p);
    show_trsf(output, c);
    c.set_mirror_axis(ax);
    show_trsf(output, c);
    c.set_mirror_plane(pl);
    show_trsf(output, c);
    attempt(output, || c.set_rotation(ax, values.0));
    show_trsf(output, c);
    attempt(output, || c.set_scale(p, values.1));
    show_trsf(output, c);
}
