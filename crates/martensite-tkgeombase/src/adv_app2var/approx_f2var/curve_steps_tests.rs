//! Curve steps tests behavior and supporting operations.
use super::*;
use crate::adv_app2var::{AdvApp2VarEvaluation, AdvApp2VarEvaluationMut};
use std::cell::RefCell;

struct SkipSecondVOrder(RefCell<Vec<i32>>);

impl AdvApp2VarEvaluatorFunc2Var for SkipSecondVOrder {
    fn evaluate(&self, _: &AdvApp2VarEvaluation<'_>, _: &mut [f64], _: &mut i32) {
        unreachable!("test callback uses the native mutable protocol")
    }

    fn evaluate_native(
        &self,
        call: &mut AdvApp2VarEvaluationMut<'_>,
        _: &mut [f64],
        error: &mut i32,
    ) {
        self.0.borrow_mut().push(*call.v_order);
        if *call.v_order == 1 {
            *call.v_order = 2;
        }
        *error = 0;
    }
}

#[test]
fn mutable_v_order_is_incremented_after_the_callback_returns() {
    let func = SkipSecondVOrder(RefCell::new(Vec::new()));
    let mut state = sample_state();
    let (mut parameters, mut result, mut error) = ([0.5], [0.; 8], 0);
    let mut other = [];
    let mut output = EndpointPieceTables::new(&mut result, &mut other);
    evaluate_cross_orders(
        &mut state,
        &func,
        (&mut parameters, 1, EndpointParameter::Lower),
        (&mut output, ConstraintEnd::Lower, 1),
        2,
        1,
        true,
        &mut error,
    );
    assert_eq!(func.0.borrow().as_slice(), &[1]);
    assert_eq!((state.v_order, error), (3, 0));
}

fn sample_state() -> SamplingCall {
    SamplingCall {
        subspace_count: 1,
        caller_orders: [0, 0],
        caller_limits: [1, 3],
        caller_sampling: [3, 2],
        dimension: 1,
        u_start_end: [0., 1.],
        v_start_end: [0., 1.],
        favor_iso: 1,
        const_param: 0.,
        nb_params: 1,
        u_order: 0,
        v_order: 1,
    }
}

#[test]
fn endpoint_parameter_identity_keeps_lower_fixed_and_indexes_upper_by_root_count() {
    for roots in [1, 2, 3, 8] {
        assert_eq!(EndpointParameter::Lower.index(roots), 0);
        assert_eq!(EndpointParameter::Upper.index(roots), (roots + 1) as usize);
    }
}

#[test]
fn retained_endpoint_callback_receives_the_required_live_parameter_beyond_initial_zone() {
    let mut state = sample_state();
    state.caller_sampling[0] = 10;
    let evaluator = |call: &AdvApp2VarEvaluation<'_>, _: &mut [f64], error: &mut i32| {
        assert_eq!(call.parameters, &[0.75]);
        *error = 0;
    };
    let (mut parameters, mut result, mut error) = ([0.; 14], [0.; 8], -1);
    parameters[11] = 0.75;
    let mut other = [];
    let mut output = EndpointPieceTables::new(&mut result, &mut other);
    evaluate_cross_orders(
        &mut state,
        &evaluator,
        (&mut parameters, 10, EndpointParameter::Upper),
        (&mut output, ConstraintEnd::Lower, 1),
        1,
        1,
        true,
        &mut error,
    );
    assert_eq!(error, 0);
}

#[test]
fn endpoint_span_selects_the_varying_axis_without_mutating_bounds() {
    let domain = [0., 4., 0., 2.];
    assert_eq!(endpoint_span(1, &domain), 2.);
    assert_eq!(endpoint_span(2, &domain), 4.);
    assert_eq!(domain, [0., 4., 0., 2.]);
}
