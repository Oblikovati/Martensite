//! One-parameter approximation fixtures and result encoding.
use super::common::{Reader, end, flag, int, pnt, real, show};
use super::output::Output;
use martensite_tkernel::ncollection::Array1;
use martensite_tkernel::standard::Failure;
use martensite_tkg3d::adv_approx::{
    AdvApproxApproxAFunction, AdvApproxApproximationInput, AdvApproxApproximationOutput,
    AdvApproxCutting, AdvApproxDichoCutting, AdvApproxEvaluation, AdvApproxEvaluatorFunction,
    AdvApproxPrefAndRec, AdvApproxSimpleApprox, approximation,
};
use martensite_tkmath::geom_abs::Shape;
use martensite_tkmath::gp::{Point2, Point3};
use martensite_tkmath::plib::{self, JacobiPolynomial};

fn polynomial(s: f64, c: f64, t: f64, d: i32) -> f64 {
    match d {
        0 => ((s * t - 2.0) * t + 0.5) * t + c,
        1 => (3.0 * s * t - 4.0) * t + 0.5,
        _ => 6.0 * s * t - 4.0,
    }
}

fn trigonometric(s: f64, c: f64, t: f64, d: i32) -> f64 {
    match d {
        0 => martensite_cmath::sin(s * t) + c * martensite_cmath::cos(0.5 * t),
        1 => s * martensite_cmath::cos(s * t) - 0.5 * c * martensite_cmath::sin(0.5 * t),
        _ => -s * s * martensite_cmath::sin(s * t) - 0.25 * c * martensite_cmath::cos(0.5 * t),
    }
}

fn rational(s: f64, t: f64, d: i32) -> f64 {
    let q: f64 = 1.0 + s * t * t;
    match d {
        0 => 1.0 / q,
        1 => -2.0 * s * t / (q * q),
        _ => (6.0 * s * s * t * t - 2.0 * s) / (q * q * q),
    }
}

fn helix(c: i32, t: f64, d: i32) -> f64 {
    let r: f64 = 2.0 + f64::from(c / 3);
    match (c % 3, d) {
        (0, 0) => r * martensite_cmath::cos(t),
        (0, 1) => -r * martensite_cmath::sin(t),
        (0, _) => -r * martensite_cmath::cos(t),
        (1, 0) => r * martensite_cmath::sin(t),
        (1, 1) => r * martensite_cmath::cos(t),
        (1, _) => -r * martensite_cmath::sin(t),
        (_, 0) => 0.3 * t,
        (_, 1) => 0.3,
        _ => 0.0,
    }
}

fn kink(s: f64, t: f64, d: i32) -> f64 {
    let x: f64 = t - 0.3;
    let a: f64 = x.abs();
    match d {
        0 => s * martensite_cmath::pow(a, 2.5),
        1 => s * 2.5 * martensite_cmath::pow(a, 1.5) * if x < 0.0 { -1.0 } else { 1.0 },
        _ => s * 3.75 * martensite_cmath::pow(a, 0.5),
    }
}

fn exponential(s: f64, t: f64, d: i32) -> f64 {
    match d {
        0 => martensite_cmath::exp(0.5 * s * t),
        1 => 0.5 * s * martensite_cmath::exp(0.5 * s * t),
        _ => 0.25 * s * s * martensite_cmath::exp(0.5 * s * t),
    }
}

/// Coordinate `c` of function `kind`, derivative `d` (g3_aa_value).
fn value(kind: i32, c: i32, t: f64, d: i32) -> f64 {
    let s: f64 = f64::from(c) + 1.0;
    match kind {
        0 => polynomial(s, f64::from(c), t, d),
        1 => trigonometric(s, f64::from(c), t, d),
        2 => rational(s, t, d),
        3 => helix(c, t, d),
        4 => kink(s, t, d),
        _ => exponential(s, t, d),
    }
}

/// The probe evaluator (G3AdvEvaluator).
struct Evaluator {
    kind: i32,
    mode: i32,
    fail_at: i32,
    calls: i32,
}

impl AdvApproxEvaluatorFunction for Evaluator {
    fn evaluate(&mut self, dimension: i32, _: [f64; 2], t: f64, d: i32, result: &mut [f64]) -> i32 {
        self.evaluate_values(dimension, t, d, result)
    }

    fn evaluate_native(&mut self, call: &mut AdvApproxEvaluation<'_>) {
        if self.mode == 3 && self.calls == 0 {
            call.start_end[1] = 2.0;
            *call.parameter = 0.25;
        }
        if self.mode == 3 && self.calls == 4 {
            *call.derivative_request = 1;
        }
        let dimension = *call.dimension;
        *call.error_code = self.evaluate_values(
            dimension,
            *call.parameter,
            *call.derivative_request,
            call.result,
        );
        if self.mode == 3 && self.calls == 5 {
            *call.dimension = 1;
        }
    }
}

impl Evaluator {
    fn evaluate_values(&mut self, dimension: i32, t: f64, d: i32, result: &mut [f64]) -> i32 {
        self.calls += 1;
        if (self.mode == 1 && d == 2) || (self.mode == 2 && self.calls == self.fail_at) {
            return 1;
        }
        for c in 0..dimension {
            result[c as usize] = value(self.kind, c, t, d);
        }
        0
    }
}

fn evaluator(r: &mut Reader) -> Evaluator {
    let (kind, mode) = (r.integer(), r.integer());
    let fail_at = r.integer();
    Evaluator {
        kind,
        mode,
        fail_at,
        calls: 0,
    }
}

fn table(_output: &Output, values: &[f64]) -> Array1<'static, f64> {
    let mut array = Array1::with_bounds(1, values.len() as i32).unwrap();
    array.as_mut_slice().copy_from_slice(values);
    array
}

/// A cutting spec (g3_aa_cutting): None for kind 0.
fn cutting(output: &Output, r: &mut Reader) -> Result<Option<Box<dyn AdvApproxCutting>>, Failure> {
    match r.integer() {
        1 => Ok(Some(Box::new(AdvApproxDichoCutting::new()))),
        2 => {
            let (rec, pref) = (r.reals(), r.reals());
            let w = r.real();
            let tool = AdvApproxPrefAndRec::new(&table(output, &rec), &table(output, &pref), w)?;
            Ok(Some(Box::new(tool)))
        }
        _ => Ok(None),
    }
}

fn shape(code: i32) -> Shape {
    match code {
        i32::MIN..=0 => Shape::C0,
        1 => Shape::C1,
        2 => Shape::C2,
        3 => Shape::C3,
        _ => Shape::G1,
    }
}

fn tolerances(output: &Output, r: &mut Reader, n: i32) -> Option<Array1<'static, f64>> {
    let t = r.real();
    (n != 0).then(|| {
        table(
            output,
            &(1..=n).map(|i| t * f64::from(i)).collect::<Vec<f64>>(),
        )
    })
}

fn poles(output: &Output, a: &AdvApproxApproxAFunction) {
    let n = a.nb_poles().unwrap();
    if let Some(p) = a.poles_1d() {
        p.as_slice()
            .iter()
            .copied()
            .for_each(|value| real(output, value));
        for i in 1..=a.num_sub_spaces(1).unwrap() {
            let mut column: Array1<'static, f64> = Array1::with_bounds(1, n).unwrap();
            a.poles_1d_of(i, &mut column).unwrap();
            column
                .as_slice()
                .iter()
                .copied()
                .for_each(|value| real(output, value));
        }
    }
    let pnt2 = |v: &Point2| {
        real(output, v.x());
        real(output, v.y());
    };
    if let Some(p) = a.poles_2d() {
        p.as_slice().iter().for_each(pnt2);
        for i in 1..=a.num_sub_spaces(2).unwrap() {
            let mut column: Array1<'static, Point2> = Array1::with_bounds(1, n).unwrap();
            a.poles_2d_of(i, &mut column).unwrap();
            column.as_slice().iter().for_each(pnt2);
        }
    }
    if let Some(p) = a.poles() {
        p.as_slice()
            .iter()
            .copied()
            .for_each(|value| pnt(output, value));
        for i in 1..=a.num_sub_spaces(3).unwrap() {
            let mut column: Array1<'static, Point3> = Array1::with_bounds(1, n).unwrap();
            a.poles_of(i, &mut column).unwrap();
            column
                .as_slice()
                .iter()
                .copied()
                .for_each(|value| pnt(output, value));
        }
    }
}

fn errors(output: &Output, a: &AdvApproxApproxAFunction) {
    for d in 1..=3 {
        if a.num_sub_spaces(d).unwrap() == 0 {
            flag(output, a.max_errors(d).unwrap().is_none());
            flag(output, a.average_errors(d).unwrap().is_none());
            continue;
        }
        for i in 1..=a.num_sub_spaces(d).unwrap() {
            real(output, a.max_error(d, i).unwrap());
            real(output, a.average_error(d, i).unwrap());
        }
        a.max_errors(d)
            .unwrap()
            .unwrap()
            .as_slice()
            .iter()
            .copied()
            .for_each(|value| real(output, value));
        a.average_errors(d)
            .unwrap()
            .unwrap()
            .as_slice()
            .iter()
            .copied()
            .for_each(|value| real(output, value));
    }
    if let Err(error) = a.max_errors(0) {
        replay_print!(output, "{} ", error.exception_type());
    }
    if let Err(error) = a.average_errors(4) {
        replay_print!(output, "{} ", error.exception_type());
    }
}

fn dump_text(output: &Output, write: impl FnOnce(&mut Vec<u8>) -> std::io::Result<()>) {
    let mut text: Vec<u8> = Vec::new();
    write(&mut text).unwrap();
    replay_print!(
        output,
        "[{}] ",
        String::from_utf8(text).unwrap().replace('\n', "|")
    );
}

fn result(output: &Output, a: &AdvApproxApproxAFunction) {
    flag(output, a.is_done());
    flag(output, a.has_result());
    for d in 0..=4 {
        show(output, a.num_sub_spaces(d), |value| int(output, value));
    }
    int(output, a.nb_poles().unwrap());
    if !a.has_result() {
        replay_print!(output, "noresult ");
        return;
    }
    int(output, a.degree());
    int(output, a.nb_knots().unwrap());
    a.knots()
        .unwrap()
        .as_slice()
        .iter()
        .copied()
        .for_each(|value| real(output, value));
    a.multiplicities()
        .unwrap()
        .as_slice()
        .iter()
        .copied()
        .for_each(|value| int(output, value));
    poles(output, a);
    errors(output, a);
    dump_text(output, |text| a.dump(text));
}

include!("advapprox/operations.rs");
