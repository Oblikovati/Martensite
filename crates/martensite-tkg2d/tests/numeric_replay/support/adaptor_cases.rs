//! The adaptor probe operations a2_adaptor, a2_load and a2_offset_load; the
//! counterpart of the *_case functions of the frozen behavior fixtures
use super::adaptor::{adaptor, adaptor_summary, built, intervals, make_curve_summary};
use super::bspl::desc;
use super::common::{self, Reader, attempt, ax, ax22, curve, end, flag, int, real, show};
use super::output::Output;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::adaptor2d::{Adaptor2dCurve2d, Adaptor2dOffsetCurve};
use martensite_tkg2d::geom2d::{
    Geom2dBSplineCurve, Geom2dBezierCurve, Geom2dCurve, Geom2dOffsetCurve,
};
use martensite_tkg2d::geom2d_adaptor::Geom2dAdaptorCurve;
use martensite_tkg2d::geom2d_eval::RepCurveDesc;

/// Summary, MakeCurve, Trim and ShallowCopy of one adaptor (a2_adaptor_case).
pub fn adaptor_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(a) = built(output, &mut r) else {
        return;
    };
    let u = r.reals();
    let (t1, t2) = (r.real(), r.real());
    adaptor_summary(output, &a, &u);
    make_curve_summary(output, &*a, &u);
    attempt(output, || {
        let t = a.trim(t1, t2, 1e-7)?;
        adaptor_summary(output, &t, &u);
        make_curve_summary(output, &*t, &u);
        Ok(())
    });
    attempt(output, || {
        adaptor_summary(output, &a.shallow_copy()?, &u);
        Ok(())
    });
    end(output);
}

// The members safe on an empty Geom2dAdaptorCurve (a2_empty_geom).
fn empty_geom(output: &Output, a: &Handle<Geom2dAdaptorCurve>) {
    flag(output, a.is_initialized());
    flag(output, a.curve().is_null());
    real(output, a.first_parameter().unwrap());
    real(output, a.last_parameter().unwrap());
    int(output, a.get_type().unwrap() as i32);
    show(output, a.continuity(), |c| int(output, c as i32));
    intervals(output, &**a);
    for r in [1e-3, 4.0] {
        show(output, a.resolution(r), |value| real(output, value));
    }
    show(output, a.line(), |l| ax(output, l.position()));
    show(output, a.circle(), |c| ax22(output, c.position()));
    show(output, a.degree(), |value| int(output, value));
    show(output, a.is_rational(), |value| flag(output, value));
    show(output, a.nb_poles(), |value| int(output, value));
    show(output, a.nb_knots(), |value| int(output, value));
    flag(output, a.bezier().unwrap().is_null());
    flag(output, a.bspline().unwrap().is_null());
    attempt(output, || a.trim(0.0, 1.0, 1e-7).map(|_| ()));
    show(output, a.shallow_copy(), |c| {
        real(output, c.last_parameter().unwrap())
    });
    make_curve_summary(output, &**a, &[]);
}

fn curve_name(output: &Output, a: &Geom2dAdaptorCurve) {
    replay_print!(output, "{} ", a.curve().dynamic_type().unwrap().name());
}

/// Load sequences of Geom2dAdaptorCurve (a2_load_case).
pub fn load_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c1) = common::built(output, &mut r) else {
        return;
    };
    let c2 = curve(&mut r).expect("probe curve");
    let u = r.reals();
    let (u1, u2) = (r.real(), r.real());
    let a = Handle::new(Geom2dAdaptorCurve::default());
    let dynamic: Handle<dyn Adaptor2dCurve2d> = a.cast();
    empty_geom(output, &a);
    let null: Handle<dyn Geom2dCurve> = Handle::default();
    attempt(output, || a.load(&null));
    attempt(output, || a.load_range(&null, 0.0, 1.0));
    attempt(output, || {
        a.load(&c1)?;
        curve_name(output, &a);
        adaptor_summary(output, &dynamic, &u);
        Ok(())
    });
    attempt(output, || {
        a.load_range(&c1, u1, u2)?;
        adaptor_summary(output, &dynamic, &u);
        Ok(())
    });
    attempt(output, || a.load_range(&c1, u2 + 1.0, u2));
    real(output, a.first_parameter().unwrap());
    real(output, a.last_parameter().unwrap());
    attempt(output, || {
        a.load_range(&c2, u1, u2)?;
        curve_name(output, &a);
        adaptor_summary(output, &dynamic, &u);
        Ok(())
    });
    // `a->Reset()`: `Handle::reset` is `a.reset(new ...)`, so dereference explicitly.
    (*a).reset();
    empty_geom(output, &a);
    attempt(output, || {
        a.load(&c2)?;
        adaptor_summary(output, &dynamic, &u);
        Ok(())
    });
    end(output);
}

// The members safe on an Adaptor2dOffsetCurve without a basis.
fn empty_offset(output: &Output, d: f64, w1: f64, w2: f64) {
    let e = Adaptor2dOffsetCurve::default();
    real(output, e.offset());
    real(output, e.first_parameter().unwrap());
    real(output, e.last_parameter().unwrap());
    flag(output, e.curve().is_null());
    real(output, e.resolution(d).unwrap());
    real(output, e.shallow_copy().unwrap().last_parameter().unwrap());
    show(output, e.trim(w1, w2, 1e-7), |t| {
        real(output, t.first_parameter().unwrap())
    });
    e.load_offset_bounded(d, w1, w2);
    real(output, e.offset());
    real(output, e.last_parameter().unwrap());
}

fn offset_bounds(output: &Output, o: &Adaptor2dOffsetCurve) {
    real(output, o.offset());
    real(output, o.first_parameter().unwrap());
    real(output, o.last_parameter().unwrap());
}

/// Load members of Adaptor2dOffsetCurve (a2_offset_load_case).
pub fn offset_load_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(b1) = built(output, &mut r) else {
        return;
    };
    let b2 = adaptor(&mut r).expect("probe adaptor");
    let u = r.reals();
    let (d, w1, w2) = (r.real(), r.real(), r.real());
    empty_offset(output, d, w1, w2);
    let o = match Adaptor2dOffsetCurve::with_offset(&b1, d) {
        Ok(o) => Handle::new(o),
        Err(error) => {
            replay_println!(output, "{}", error.exception_type());
            return;
        }
    };
    let dynamic: Handle<dyn Adaptor2dCurve2d> = o.cast();
    adaptor_summary(output, &dynamic, &u);
    o.load_curve(&b2);
    offset_bounds(output, &o);
    flag(output, o.curve() == b2);
    adaptor_summary(output, &dynamic, &u);
    attempt(output, || o.load_offset(d));
    offset_bounds(output, &o);
    adaptor_summary(output, &dynamic, &u);
    o.load_offset_bounded(-d, w1, w2);
    adaptor_summary(output, &dynamic, &u);
    end(output);
}

// Sets, or for a null descriptor clears, the evaluation representation (a2_set_rep).
pub fn set_rep(c: &Handle<dyn Geom2dCurve>, d: &Handle<dyn RepCurveDesc>) -> Result<(), Failure> {
    if let Some(b) = c.cast::<Geom2dBezierCurve>().get() {
        return update_rep(
            d,
            |x| b.set_eval_representation(x),
            || b.clear_eval_representation(),
        );
    }
    if let Some(b) = c.cast::<Geom2dBSplineCurve>().get() {
        return update_rep(
            d,
            |x| b.set_eval_representation(x),
            || b.clear_eval_representation(),
        );
    }
    if let Some(b) = c.cast::<Geom2dOffsetCurve>().get() {
        return update_rep(
            d,
            |x| b.set_eval_representation(x),
            || b.clear_eval_representation(),
        );
    }
    Ok(())
}

fn update_rep(
    d: &Handle<dyn RepCurveDesc>,
    set: impl FnOnce(Handle<dyn RepCurveDesc>) -> Result<(), Failure>,
    clear: impl FnOnce(),
) -> Result<(), Failure> {
    if d.is_null() {
        clear();
        return Ok(());
    }
    set(d.clone())
}

/// An adaptor on a curve with an evaluation representation (a2_evalrep_case).
pub fn evalrep_case(output: &Output, values: &[f64]) {
    let mut r = Reader::new(values);
    let Some(c) = common::built(output, &mut r) else {
        return;
    };
    let rep = curve(&mut r).expect("probe representation curve");
    let d = desc(&mut r, rep);
    let u = r.reals();
    let (u1, u2) = (r.real(), r.real());
    attempt(output, || set_rep(&c, &d));
    let a = Handle::new(Geom2dAdaptorCurve::with_range(&c, u1, u2).expect("probe range"));
    let dynamic: Handle<dyn Adaptor2dCurve2d> = a.cast();
    adaptor_summary(output, &dynamic, &u);
    set_rep(&c, &Handle::default()).unwrap();
    adaptor_summary(output, &dynamic, &u);
    a.load_range(&c, u1, u2).unwrap();
    adaptor_summary(output, &dynamic, &u);
    let copy: Handle<dyn Geom2dCurve> = c.copy().unwrap().cast();
    a.load_range(&copy, u1, u2).unwrap();
    adaptor_summary(output, &dynamic, &u);
    end(output);
}
