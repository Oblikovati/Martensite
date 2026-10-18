//! Adaptor2d and Geom2dAdaptor adaptors: construction, summary and MakeCurve; the
//! counterpart of the frozen behavior fixtures (a2_adaptor, a2_summary...).
use super::common::{Reader, attempt, ax, ax22, curve, flag, int, pnt, real, show, summary, vec};
use super::output::Output;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkg2d::adaptor2d::{
    Adaptor2dCurve2d, Adaptor2dCurve2dBase, Adaptor2dLine2d, Adaptor2dOffsetCurve,
};
use martensite_tkg2d::geom2d_adaptor::{Geom2dAdaptorCurve, make_curve};
use martensite_tkmath::geom_abs::Shape;
use martensite_tkmath::gp::Line2;

fn line_adaptor(kind: i32, r: &mut Reader) -> Handle<dyn Adaptor2dCurve2d> {
    let (p, d) = (r.pnt(), r.dir());
    if kind == 0 {
        let u1 = r.real();
        return Handle::new(Adaptor2dLine2d::new(p, d, u1, r.real())).cast();
    }
    let line = Adaptor2dLine2d::default();
    if kind == 1 {
        line.load(Line2::new(p, d));
    } else {
        let u1 = r.real();
        line.load_bounded(Line2::new(p, d), u1, r.real());
    }
    Handle::new(line).cast()
}

/// One adaptor specification, as a2_adaptor reads it.
pub fn adaptor(r: &mut Reader) -> Result<Handle<dyn Adaptor2dCurve2d>, Failure> {
    let kind = r.integer();
    Ok(match kind {
        0..=2 => line_adaptor(kind, r),
        3 => Handle::new(Geom2dAdaptorCurve::with_curve(&curve(r)?)?).cast(),
        4 => {
            let (u1, u2) = (r.real(), r.real());
            Handle::new(Geom2dAdaptorCurve::with_range(&curve(r)?, u1, u2)?).cast()
        }
        5 => Handle::new(Adaptor2dOffsetCurve::with_curve(&adaptor(r)?)).cast(),
        6 => {
            let d = r.real();
            Handle::new(Adaptor2dOffsetCurve::with_offset(&adaptor(r)?, d)?).cast()
        }
        7 => {
            let (d, w1, w2) = (r.real(), r.real(), r.real());
            Handle::new(Adaptor2dOffsetCurve::with_bounds(&adaptor(r)?, d, w1, w2)).cast()
        }
        8 => Handle::new(Adaptor2dCurve2dBase::default()).cast(),
        _ => panic!("unknown adaptor kind {kind}"),
    })
}

/// Reads an adaptor, reporting a construction failure and ending the line (a2_built).
pub fn built(output: &Output, r: &mut Reader) -> Option<Handle<dyn Adaptor2dCurve2d>> {
    match adaptor(r) {
        Ok(adaptor) => Some(adaptor),
        Err(error) => {
            replay_println!(output, "{}", error.exception_type());
            None
        }
    }
}

const SHAPES: [Shape; 7] = [
    Shape::C0,
    Shape::G1,
    Shape::C1,
    Shape::G2,
    Shape::C2,
    Shape::C3,
    Shape::CN,
];

/// NbIntervals and Intervals for every continuity (a2_intervals).
pub fn intervals(output: &Output, a: &dyn Adaptor2dCurve2d) {
    for shape in SHAPES {
        let n = match a.nb_intervals(shape) {
            Ok(n) => n,
            Err(error) => {
                replay_print!(output, "{} ", error.exception_type());
                continue;
            }
        };
        int(output, n);
        let mut t = Array1::<f64>::with_bounds(1, n + 1).unwrap();
        attempt(output, || {
            a.intervals(&mut t, shape)?;
            int(output, t.lower());
            int(output, t.upper());
            t.iter().for_each(|v| real(output, *v));
            Ok(())
        });
    }
}

fn null_or_poles(output: &Output, result: Result<Option<i32>, Failure>) {
    show(output, result, |poles| {
        flag(output, poles.is_none());
        if let Some(count) = poles {
            int(output, count);
        }
    });
}

fn conic_kinds(output: &Output, a: &dyn Adaptor2dCurve2d) {
    show(output, a.line(), |l| ax(output, l.position()));
    show(output, a.circle(), |c| {
        ax22(output, c.position());
        real(output, c.radius());
    });
    show(output, a.ellipse(), |e| {
        ax22(output, e.axis());
        real(output, e.major_radius());
        real(output, e.minor_radius());
    });
    show(output, a.hyperbola(), |h| {
        ax22(output, h.axis());
        real(output, h.major_radius());
        real(output, h.minor_radius());
    });
    show(output, a.parabola(), |p| {
        ax22(output, p.axis());
        real(output, p.focal());
    });
}

/// The kind queries (a2_kinds).
pub fn kinds(output: &Output, a: &dyn Adaptor2dCurve2d) {
    show(output, a.get_type(), |t| int(output, t as i32));
    conic_kinds(output, a);
    show(output, a.degree(), |value| int(output, value));
    show(output, a.is_rational(), |value| flag(output, value));
    show(output, a.nb_poles(), |value| int(output, value));
    show(output, a.nb_knots(), |value| int(output, value));
    show(output, a.nb_samples(), |value| int(output, value));
    null_or_poles(output, a.bezier().map(|b| b.get().map(|b| b.nb_poles())));
    null_or_poles(output, a.bspline().map(|b| b.get().map(|b| b.nb_poles())));
}

fn eval_members(output: &Output, a: &dyn Adaptor2dCurve2d, u: f64) {
    show(output, a.eval_d0(u), |value| pnt(output, value));
    show(output, a.eval_d1(u), |e| {
        pnt(output, e.point);
        vec(output, e.d1);
    });
    show(output, a.eval_d2(u), |e| {
        pnt(output, e.point);
        [e.d1, e.d2]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    show(output, a.eval_d3(u), |e| {
        pnt(output, e.point);
        [e.d1, e.d2, e.d3]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    for n in 0..=5 {
        show(output, a.eval_dn(u, n), |value| vec(output, value));
    }
}

/// The evaluators at one parameter (a2_evaluations).
pub fn evaluations(output: &Output, a: &dyn Adaptor2dCurve2d, u: f64) {
    eval_members(output, a, u);
    show(output, a.value(u), |value| pnt(output, value));
    show(output, a.d0(u), |value| pnt(output, value));
    show(output, a.d1(u), |(p, v1)| {
        pnt(output, p);
        vec(output, v1);
    });
    show(output, a.d2(u), |(p, v1, v2)| {
        pnt(output, p);
        [v1, v2].into_iter().for_each(|value| vec(output, value));
    });
    show(output, a.d3(u), |(p, v1, v2, v3)| {
        pnt(output, p);
        [v1, v2, v3]
            .into_iter()
            .for_each(|value| vec(output, value));
    });
    show(output, a.dn(u, 2), |value| vec(output, value));
}

/// The adaptor summary (a2_summary).
pub fn adaptor_summary(output: &Output, a: &Handle<dyn Adaptor2dCurve2d>, params: &[f64]) {
    replay_print!(output, "{} ", a.dynamic_type().unwrap().name());
    show(output, a.first_parameter(), |value| real(output, value));
    show(output, a.last_parameter(), |value| real(output, value));
    show(output, a.continuity(), |c| int(output, c as i32));
    intervals(output, &**a);
    show(output, a.is_closed(), |value| flag(output, value));
    show(output, a.is_periodic(), |value| flag(output, value));
    show(output, a.period(), |value| real(output, value));
    for r in [1e-7, 1e-3, 0.5, 10.0] {
        show(output, a.resolution(r), |value| real(output, value));
    }
    kinds(output, &**a);
    for &u in params {
        evaluations(output, &**a, u);
    }
}

/// Geom2dAdaptor::MakeCurve and the summary of its result (a2_make_curve).
pub fn make_curve_summary(output: &Output, a: &dyn Adaptor2dCurve2d, params: &[f64]) {
    show(output, make_curve(a), |c| {
        flag(output, c.is_null());
        if let Some(built) = c.get() {
            replay_print!(output, "{} ", c.dynamic_type().unwrap().name());
            summary(output, built, params);
        }
    });
}
