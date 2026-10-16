//! The Geom2dEval curves and the Geom2dHash hashers; the counterpart of
//! the frozen behavior fixtures
use super::common::{Reader, ax, built, dump, end, flag, int, pnt, real, show, summary, vec};
use super::output::Output;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::geom2d::{Geom2dBoundedCurve, Geom2dCurve};
use martensite_tkg2d::geom2d_eval::{
    Geom2dEvalAHTBezierCurve, Geom2dEvalArchimedeanSpiralCurve, Geom2dEvalCircleInvoluteCurve,
    Geom2dEvalLogarithmicSpiralCurve, Geom2dEvalSineWaveCurve, Geom2dEvalTBezierCurve,
};
use martensite_tkg2d::geom2d_hash::{
    Geom2dHashAxisPlacement, Geom2dHashCurveHasher, Geom2dHashDirectionHasher,
    Geom2dHashPointHasher,
};
use martensite_tkmath::gp::{Axis22d, Point2};

fn bezier(kind: i32, r: &mut Reader) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    let rational = r.flag();
    let lower = r.integer();
    let (mut degree, mut alpha, mut beta) = (0, 0.0, 0.0);
    if kind == 16 {
        (degree, alpha, beta) = (r.integer(), r.real(), r.real());
    }
    let np = r.integer();
    let mut p = Array1::<Point2>::with_bounds(lower, lower + np - 1).unwrap();
    for i in lower..lower + np {
        p[i] = r.pnt();
    }
    let nw = r.integer();
    let mut w = Array1::<f64>::with_bounds(lower, lower + nw - 1).unwrap();
    for i in lower..lower + nw {
        w[i] = r.real();
    }
    if kind == 15 {
        let a = r.real();
        let curve = if rational {
            Geom2dEvalTBezierCurve::with_weights(&p, &w, a)?
        } else {
            Geom2dEvalTBezierCurve::new(&p, a)?
        };
        return Ok(Handle::new(curve).cast());
    }
    let curve = if rational {
        Geom2dEvalAHTBezierCurve::with_weights(&p, &w, degree, alpha, beta)?
    } else {
        Geom2dEvalAHTBezierCurve::new(&p, degree, alpha, beta)?
    };
    Ok(Handle::new(curve).cast())
}

/// Curve kinds 11..16, as g2_eval_curve reads them.
pub fn curve(kind: i32, r: &mut Reader) -> Result<Handle<dyn Geom2dCurve>, Failure> {
    if kind >= 15 {
        return bezier(kind, r);
    }
    let a = r.ax();
    let x = r.real();
    if kind == 12 {
        return Ok(Handle::new(Geom2dEvalCircleInvoluteCurve::new(a, x)?).cast());
    }
    let y = r.real();
    match kind {
        11 => Ok(Handle::new(Geom2dEvalArchimedeanSpiralCurve::new(a, x, y)?).cast()),
        13 => Ok(Handle::new(Geom2dEvalLogarithmicSpiralCurve::new(a, x, y)?).cast()),
        _ => Ok(Handle::new(Geom2dEvalSineWaveCurve::new(a, x, y, r.real())?).cast()),
    }
}

fn poles(output: &Output, p: &Array1<'_, Point2>) {
    int(output, p.lower());
    int(output, p.len() as i32);
    p.iter().copied().for_each(|value| pnt(output, value));
}
fn weights(output: &Output, w: &Array1<'_, f64>, values: bool) {
    int(output, w.lower());
    int(output, w.len() as i32);
    if values {
        w.iter().copied().for_each(|value| real(output, value));
    }
}

fn tbezier_state(output: &Output, t: &Geom2dEvalTBezierCurve) {
    int(output, t.nb_poles());
    int(output, t.order());
    real(output, t.alpha());
    flag(output, t.is_rational());
    poles(output, &t.poles());
    weights(output, &t.weights(), true);
    show(output, t.start_point(), |value| pnt(output, value));
    show(output, t.end_point(), |value| pnt(output, value));
}
fn aht_state(output: &Output, t: &Geom2dEvalAHTBezierCurve) {
    int(output, t.nb_poles());
    int(output, t.alg_degree());
    real(output, t.alpha());
    real(output, t.beta());
    flag(output, t.is_rational());
    poles(output, &t.poles());
    weights(output, &t.weights(), t.is_rational());
    show(output, t.start_point(), |value| pnt(output, value));
    show(output, t.end_point(), |value| pnt(output, value));
}

fn analytic_state(output: &Output, c: &Handle<dyn Geom2dCurve>) {
    if let Some(s) = c.cast::<Geom2dEvalArchimedeanSpiralCurve>().get() {
        ax(output, s.position());
        [s.initial_radius(), s.growth_rate()]
            .into_iter()
            .for_each(|value| real(output, value));
    }
    if let Some(s) = c.cast::<Geom2dEvalCircleInvoluteCurve>().get() {
        ax(output, s.position());
        real(output, s.radius());
    }
    if let Some(s) = c.cast::<Geom2dEvalLogarithmicSpiralCurve>().get() {
        ax(output, s.position());
        [s.scale(), s.growth_exponent()]
            .into_iter()
            .for_each(|value| real(output, value));
    }
    if let Some(s) = c.cast::<Geom2dEvalSineWaveCurve>().get() {
        ax(output, s.position());
        [s.amplitude(), s.omega(), s.phase()]
            .into_iter()
            .for_each(|value| real(output, value));
    }
}

fn state(output: &Output, c: &Handle<dyn Geom2dCurve>) {
    if let Some(t) = c.cast::<Geom2dEvalTBezierCurve>().get() {
        tbezier_state(output, t);
    }
    if let Some(t) = c.cast::<Geom2dEvalAHTBezierCurve>().get() {
        aht_state(output, t);
    }
    analytic_state(output, c);
}

fn type_name(output: &Output, c: &Handle<dyn Geom2dCurve>) {
    replay_print!(output, "{} ", c.dynamic_type().unwrap().name());
}

pub fn eval_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built(output, &mut r) else {
        return;
    };
    let u = r.reals();
    type_name(output, &c);
    state(output, &c);
    let copy: Handle<dyn Geom2dCurve> = c.copy().unwrap().cast();
    type_name(output, &copy);
    state(output, &copy);
    summary(output, &*copy, &u);
    for &v in &u {
        for n in 6..=9 {
            show(output, c.eval_dn(v, n), |value| vec(output, value));
        }
    }
    for depth in [0, 1, 2] {
        dump(output, &*c, depth);
    }
    end(output);
}

fn hash_value(output: &Output, value: usize) {
    replay_print!(output, "h{value} ");
}

pub fn hash_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (comp, tol) = (r.real(), r.real());
    let n = r.integer();
    let mut curves: Vec<Handle<dyn Geom2dCurve>> = Vec::new();
    for _ in 0..n {
        let c = match r.integer() {
            0 => {
                let Some(c) = built(output, &mut r) else {
                    return;
                };
                c
            }
            2 => curves[r.integer() as usize].clone(),
            _ => Handle::default(),
        };
        curves.push(c);
    }
    let h = Geom2dHashCurveHasher::new(comp, tol);
    curves.iter().for_each(|c| hash_value(output, h.hash(c)));
    for a in &curves {
        curves.iter().for_each(|b| flag(output, h.equal(a, b)));
    }
    end(output);
}

pub fn hash_frame_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let (comp, tol) = (r.real(), r.real());
    let (p1, p2, d1, d2) = (r.pnt(), r.pnt(), r.dir(), r.dir());
    let points = Geom2dHashPointHasher::new(comp, tol);
    let directions = Geom2dHashDirectionHasher::new(comp, tol);
    hash_value(output, points.hash(p1));
    hash_value(output, points.hash(p2));
    flag(output, points.equal(p1, p2));
    hash_value(output, directions.hash(d1));
    hash_value(output, directions.hash(d2));
    flag(output, directions.equal(d1, d2));
    let mut frames: Vec<Axis22d> = Vec::new();
    for _ in 0..2 {
        let (p, x, y) = (r.pnt(), r.dir(), r.dir());
        match Axis22d::with_directions(p, x, y) {
            Ok(frame) => frames.push(frame),
            Err(error) => {
                replay_println!(output, "{}", error.exception_type());
                return;
            }
        }
    }
    let axes = Geom2dHashAxisPlacement::new(comp, tol);
    hash_value(output, axes.hash(frames[0]));
    hash_value(output, axes.hash(frames[1]));
    flag(output, axes.equal(frames[0], frames[1]));
    end(output);
}
