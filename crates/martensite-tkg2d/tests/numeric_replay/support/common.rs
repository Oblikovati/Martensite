//! In-process replay encoders and shared curve and transformation construction.
use super::output::Output;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::dump::DumpStream;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{
    Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dCircle, Geom2dCurve, Geom2dEllipse,
    Geom2dGeometry, Geom2dHyperbola, Geom2dLine, Geom2dOffsetCurve, Geom2dParabola,
    Geom2dTrimmedCurve,
};
use martensite_tkmath::gp::{Axis2d, Axis22d, Direction2, Point2, Transform2, Vector2, Xy};

pub use super::print::{Reader, attempt, end, flag, int, real, show};

pub fn xy(output: &Output, value: Xy) {
    real(output, value.x());
    real(output, value.y());
}
pub fn pnt(output: &Output, value: Point2) {
    xy(output, value.xy());
}
pub fn vec(output: &Output, value: Vector2) {
    xy(output, value.xy());
}
pub fn dir(output: &Output, value: Direction2) {
    xy(output, value.xy());
}
pub fn ax(output: &Output, value: Axis2d) {
    pnt(output, value.location());
    dir(output, value.direction());
}
pub fn ax22(output: &Output, value: Axis22d) {
    pnt(output, value.location());
    dir(output, value.x_direction());
    dir(output, value.y_direction());
}
pub fn dump(output: &Output, geometry: &dyn Geom2dGeometry, depth: i32) {
    let mut stream = DumpStream::default();
    geometry.dump_json(&mut stream, depth).unwrap();
    replay_print!(output, "{{{}}} ", stream.text());
}

impl Reader<'_> {
    pub fn pnt(&mut self) -> Point2 {
        let x = self.real();
        Point2::new(x, self.real())
    }
    pub fn vec(&mut self) -> Vector2 {
        let x = self.real();
        Vector2::new(x, self.real())
    }
    pub fn dir(&mut self) -> Direction2 {
        let x = self.real();
        Direction2::new(x, self.real()).expect("probe direction")
    }
    pub fn ax(&mut self) -> Axis2d {
        let p = self.pnt();
        Axis2d::new(p, self.dir())
    }
}

/// One transformation, as g2_trsf reads it.
pub fn trsf(r: &mut Reader) -> Transform2 {
    let kind = r.integer();
    let p: Vec<f64> = (0..5).map(|_| r.real()).collect();
    let mut t = Transform2::default();
    let a = Point2::new(p[0], p[1]);
    match kind {
        0 => t.set_mirror_point(a),
        1 => t.set_mirror_axis(Axis2d::new(a, Direction2::new(p[2], p[3]).unwrap())),
        2 => t.set_rotation(a, p[2]),
        3 => t.set_scale(a, p[2]),
        4 => t.set_translation(Vector2::new(p[0], p[1])),
        5 => t.set_translation_between(a, Point2::new(p[2], p[3])),
        6 => {
            let mut s = Transform2::default();
            s.set_scale(Point2::new(p[2], p[3]), p[4]);
            t.set_rotation(a, p[2]);
            t = t.multiplied(s);
        }
        _ => {}
    }
    t
}

fn bezier(r: &mut Reader) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let rational = r.flag();
    let n = r.integer();
    let mut p = Array1::<Point2>::with_bounds(1, n).unwrap();
    let mut w = Array1::<f64>::with_bounds(1, n).unwrap();
    for i in 1..=n {
        p[i] = r.pnt();
        w[i] = r.real();
    }
    let curve = if rational {
        Geom2dBezierCurve::with_weights(&p, &w)?
    } else {
        Geom2dBezierCurve::new(&p)?
    };
    Ok(Handle::new(curve).cast())
}

fn bspline(r: &mut Reader) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let (rational, periodic) = (r.flag(), r.flag());
    let (degree, nk) = (r.integer(), r.integer());
    let mut k = Array1::<f64>::with_bounds(1, nk).unwrap();
    let mut m = Array1::<i32>::with_bounds(1, nk).unwrap();
    for i in 1..=nk {
        k[i] = r.real();
        m[i] = r.integer();
    }
    let np = r.integer();
    let mut p = Array1::<Point2>::with_bounds(1, np).unwrap();
    let mut w = Array1::<f64>::with_bounds(1, np).unwrap();
    for i in 1..=np {
        p[i] = r.pnt();
        w[i] = r.real();
    }
    let curve = if rational {
        Geom2dBSplineCurve::with_weights(&p, &w, &k, &m, degree, periodic)?
    } else {
        Geom2dBSplineCurve::new(&p, &k, &m, degree, periodic)?
    };
    Ok(Handle::new(curve).cast())
}

fn conic(kind: i32, r: &mut Reader) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let a = r.ax();
    let sense = r.flag();
    let x = r.real();
    if kind == 1 {
        return Ok(Handle::new(Geom2dCircle::from_axis(a, x, sense)?).cast());
    }
    if kind == 4 {
        return Ok(Handle::new(Geom2dParabola::from_axis(a, x, sense)?).cast());
    }
    let y = r.real();
    if kind == 2 {
        return Ok(Handle::new(Geom2dEllipse::from_axis(a, x, y, sense)?).cast());
    }
    Ok(Handle::new(Geom2dHyperbola::from_axis(a, x, y, sense)?).cast())
}

/// One curve specification, as g2_curve reads it.
pub fn curve(r: &mut Reader) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let kind = r.integer();
    match kind {
        0 => {
            let p = r.pnt();
            Ok(Handle::new(Geom2dLine::from_point(p, r.dir())).cast())
        }
        1..=4 => conic(kind, r),
        5 => bezier(r),
        6 => bspline(r),
        7 => {
            let (u1, u2) = (r.real(), r.real());
            let (sense, adjust) = (r.flag(), r.flag());
            let basis = curve(r)?;
            Ok(Handle::new(Geom2dTrimmedCurve::new(&basis, u1, u2, sense, adjust)?).cast())
        }
        8 => {
            let d = r.real();
            let no_check = r.flag();
            let basis = curve(r)?;
            Ok(Handle::new(Geom2dOffsetCurve::new(&basis, d, no_check)?).cast())
        }
        9 => {
            let (p, x, y) = (r.pnt(), r.dir(), r.dir());
            let frame = Axis22d::with_directions(p, x, y)?;
            Ok(Handle::new(Geom2dCircle::new(frame, r.real())?).cast())
        }
        10 => {
            let d = r.ax();
            Ok(Handle::new(Geom2dParabola::from_directrix(d, r.pnt())?).cast())
        }
        11..=16 => super::eval::curve(kind, r),
        _ => panic!("unknown curve kind {kind}"),
    }
}

/// Reads a curve, reporting a construction failure and ending the line (g2_built).
pub fn built(output: &Output, r: &mut Reader) -> Option<Handle<dyn Geom2dCurve>> {
    match curve(r) {
        Ok(curve) => Some(curve),
        Err(error) => {
            replay_println!(output, "{}", error.exception_type());
            None
        }
    }
}

/// The curve summary of g2_summary.
pub fn summary(output: &Output, c: &dyn Geom2dCurve, params: &[f64]) {
    real(output, c.first_parameter());
    real(output, c.last_parameter());
    show(output, c.is_closed(), |value| flag(output, value));
    flag(output, c.is_periodic());
    show(output, c.period(), |value| real(output, value));
    int(output, c.continuity() as i32);
    for n in -1..=4 {
        show(output, c.is_cn(n), |value| flag(output, value));
    }
    for &u in params {
        evaluations(output, c, u);
    }
}

fn evaluations(output: &Output, c: &dyn Geom2dCurve, u: f64) {
    show(output, c.eval_d0(u), |value| pnt(output, value));
    attempt(output, || {
        let r = c.eval_d1(u)?;
        pnt(output, r.point);
        vec(output, r.d1);
        Ok(())
    });
    attempt(output, || {
        let r = c.eval_d2(u)?;
        pnt(output, r.point);
        vec(output, r.d1);
        vec(output, r.d2);
        Ok(())
    });
    attempt(output, || {
        let r = c.eval_d3(u)?;
        pnt(output, r.point);
        [r.d1, r.d2, r.d3]
            .into_iter()
            .for_each(|value| vec(output, value));
        Ok(())
    });
    for n in 0..=5 {
        show(output, c.eval_dn(u, n), |value| vec(output, value));
    }
    show(output, c.value(u), |value| pnt(output, value));
    show(output, c.reversed_parameter(u), |value| real(output, value));
}
