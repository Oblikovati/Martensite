//! Analytic separable functions write only their requested sample result and status.
use super::*;
use martensite_tkgeombase::adv_app2var::{AdvApp2VarEvaluation, AdvApp2VarEvaluatorFunc2Var};
use std::sync::atomic::{AtomicUsize, Ordering};
pub struct OrdinaryEvaluator<'a> {
    pub spec: &'a CaseSpec,
    pub calls: AtomicUsize,
}
fn monomial(power: i32, order: i32, value: f64) -> f64 {
    if order > power {
        return 0.;
    }
    let (mut scale, mut term) = (1., 1.);
    for i in 0..order {
        scale *= f64::from(power - i);
    }
    for _ in 0..power - order {
        term *= value;
    }
    scale * term
}
fn harmonic(cosine: bool, order: i32, value: f64) -> f64 {
    let mut scale = 1.;
    for _ in 0..order {
        scale *= 6.;
    }
    let angle = 6. * value;
    let wave = match (order + if cosine { 0 } else { 3 }) % 4 {
        0 => martensite_cmath::cos(angle),
        1 => -martensite_cmath::sin(angle),
        2 => -martensite_cmath::cos(angle),
        _ => martensite_cmath::sin(angle),
    };
    scale * wave
}
pub fn component(kind: i32, axis: usize, du: i32, dv: i32, u: f64, v: f64) -> f64 {
    if kind == 2 || (du > 0 && dv > 0) {
        return 0.;
    }
    if kind == 0 {
        let (pu, pv) = if axis == 0 { (1, 6) } else { (6, 1) };
        return (if dv == 0 { monomial(pu, du, u) } else { 0. })
            + (if du == 0 { monomial(pv, dv, v) } else { 0. });
    }
    harmonic_component(axis, du, dv, u, v)
}
fn harmonic_component(axis: usize, du: i32, dv: i32, u: f64, v: f64) -> f64 {
    let left = if dv == 0 {
        harmonic(axis == 0, du, u)
    } else {
        0.
    };
    let right = if du == 0 {
        harmonic(axis != 0, dv, v)
    } else {
        0.
    };
    left + right
}
fn fill_samples(kind: i32, call: &AdvApp2VarEvaluation<'_>, result: &mut [f64]) {
    for sample in 0..call.nb_params as usize {
        let (u, v) = if call.favor_iso == 1 {
            (call.const_param, call.parameters[sample])
        } else {
            (call.parameters[sample], call.const_param)
        };
        for axis in 0..2 {
            result[axis + sample * 2] = component(kind, axis, call.u_order, call.v_order, u, v);
        }
    }
}
fn callback_snapshot(call: &AdvApp2VarEvaluation<'_>) -> (Vec<i32>, Vec<u64>) {
    let integers = vec![
        call.dimension,
        call.favor_iso,
        call.nb_params,
        call.u_order,
        call.v_order,
    ];
    let reals = call
        .u_start_end
        .iter()
        .chain(call.v_start_end.iter())
        .chain(std::iter::once(&call.const_param))
        .chain(call.parameters.iter())
        .map(|value| value.to_bits())
        .collect();
    (integers, reals)
}
impl AdvApp2VarEvaluatorFunc2Var for OrdinaryEvaluator<'_> {
    fn evaluate(&self, call: &AdvApp2VarEvaluation<'_>, result: &mut [f64], error: &mut i32) {
        let before = callback_snapshot(call);
        assert_eq!(call.dimension, 2);
        assert!(call.nb_params > 0);
        fill_samples(self.spec.kind, call, result);
        self.calls.fetch_add(1, Ordering::SeqCst);
        *error = 0;
        assert_eq!(callback_snapshot(call), before);
    }
}
