//! Analytic curve descriptors and derivative requests.
use super::common::{
    Reader, ax2, built_curve, curve_summary, dump, end, flag, int, name, pnt, real, show, vec,
};
use super::output::Output;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg3d::geom::{GeomBoundedCurve, GeomCurve};
use martensite_tkg3d::geom_eval::{
    GeomEvalAHTBezierCurve, GeomEvalCircularHelixCurve, GeomEvalSineWaveCurve, GeomEvalTBezierCurve,
};
use martensite_tkmath::gp::{Point3, Transform3, Vector3};

// Kinds 22 and 23: tbezier rational lower np poles nw weights alpha |
// aht rational lower degree alpha beta np poles nw weights.
fn bezier(_output: &Output, kind: i32, r: &mut Reader) -> Result<Handle<dyn GeomCurve>, Failure> {
    let rational = r.flag();
    let lower = r.integer();
    let (mut degree, mut alpha, mut beta) = (0, 0.0, 0.0);
    if kind == 23 {
        (degree, alpha, beta) = (r.integer(), r.real(), r.real());
    }
    let np = r.integer();
    let mut p = Array1::<Point3>::with_bounds(lower, lower + np - 1).unwrap();
    for i in lower..lower + np {
        p[i] = r.pnt();
    }
    let nw = r.integer();
    let mut w = Array1::<f64>::with_bounds(lower, lower + nw - 1).unwrap();
    for i in lower..lower + nw {
        w[i] = r.real();
    }
    if kind == 22 {
        let a = r.real();
        let curve = if rational {
            GeomEvalTBezierCurve::with_weights(&p, &w, a)?
        } else {
            GeomEvalTBezierCurve::new(&p, a)?
        };
        return Ok(Handle::new(curve).cast());
    }
    let curve = if rational {
        GeomEvalAHTBezierCurve::with_weights(&p, &w, degree, alpha, beta)?
    } else {
        GeomEvalAHTBezierCurve::new(&p, degree, alpha, beta)?
    };
    Ok(Handle::new(curve).cast())
}

/// One eval curve specification of kind `kind` (20..=23), as g3_eval_curve reads it.
pub fn spec(output: &Output, kind: i32, r: &mut Reader) -> Result<Handle<dyn GeomCurve>, Failure> {
    if kind == 22 || kind == 23 {
        return bezier(output, kind, r);
    }
    assert!(kind == 20 || kind == 21, "unknown eval curve kind {kind}");
    let (f, x, y) = (r.ax2(), r.real(), r.real());
    if kind == 21 {
        return Ok(Handle::new(GeomEvalCircularHelixCurve::new(f, x, y)?).cast());
    }
    Ok(Handle::new(GeomEvalSineWaveCurve::new(f, x, y, r.real())?).cast())
}

fn poles(output: &Output, p: &Array1<'_, Point3>) {
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

fn tbezier_state(output: &Output, t: &GeomEvalTBezierCurve) {
    int(output, t.nb_poles());
    int(output, t.order());
    real(output, t.alpha());
    flag(output, t.is_rational());
    poles(output, &t.poles());
    weights(output, &t.weights(), true);
    show(output, t.start_point(), |value| pnt(output, value));
    show(output, t.end_point(), |value| pnt(output, value));
}
fn aht_state(output: &Output, t: &GeomEvalAHTBezierCurve) {
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

fn state(output: &Output, c: &Handle<dyn GeomCurve>) {
    if let Some(t) = c.cast::<GeomEvalTBezierCurve>().get() {
        tbezier_state(output, t);
    }
    if let Some(t) = c.cast::<GeomEvalAHTBezierCurve>().get() {
        aht_state(output, t);
    }
    if let Some(s) = c.cast::<GeomEvalSineWaveCurve>().get() {
        ax2(output, s.position());
        [s.amplitude(), s.omega(), s.phase()]
            .into_iter()
            .for_each(|value| real(output, value));
    }
    if let Some(s) = c.cast::<GeomEvalCircularHelixCurve>().get() {
        ax2(output, s.position());
        [s.radius(), s.pitch()]
            .into_iter()
            .for_each(|value| real(output, value));
    }
}

// Reverse, Reversed, Transform and Transformed, which print only their failures.
fn refused_members(output: &Output, c: &Handle<dyn GeomCurve>) {
    let mut t = Transform3::default();
    t.set_translation(Vector3::new(1., 2., 3.));
    show(output, c.reverse(), |_| {});
    show(output, c.reversed(), |_| {});
    show(output, c.transform(t), |_| {});
    show(output, c.transformed(t), |_| {});
}

pub fn eval_curve_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = built_curve(output, &mut r) else {
        return;
    };
    let u = r.reals();
    name(output, &c);
    state(output, &c);
    let copy: Handle<dyn GeomCurve> = c.copy().unwrap().cast();
    name(output, &copy);
    state(output, &copy);
    curve_summary(output, &*copy, &u);
    for &v in &u {
        for n in 6..=9 {
            show(output, c.eval_dn(v, n), |value| vec(output, value));
        }
    }
    refused_members(output, &c);
    for depth in [0, 1, 2, -1] {
        dump(output, &*c, depth);
    }
    end(output);
}
