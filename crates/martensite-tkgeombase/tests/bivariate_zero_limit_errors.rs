//! Bounded early exits; zero-capacity continuation remains a separate obligation.
use martensite_tkgeombase::adv_app2var::approx_f2var::mma2fnc_mut_;
use martensite_tkgeombase::adv_app2var::{AdvApp2VarEvaluation, AdvApp2VarEvaluatorFunc2Var};
use std::sync::atomic::{AtomicUsize, Ordering};
#[path = "support/bivariate_zero_limits/mod.rs"]
mod support;
use martensite_tkgeombase::adv_app2var::approx_f2var::IsoApproxRequest;
use support::{LimitInputs, LimitOutputs, bits};
fn request_signature(request: &IsoApproxRequest<'_>) -> (Vec<i32>, Vec<u64>) {
    let integers = vec![
        request.ndimen,
        request.nbsesp,
        request.isofav,
        request.nbroot,
        request.iordre,
        request.ideriv,
        request.ndgjac,
        request.nbcrmx,
        request.ncflim,
    ];
    let mut reals = bits(&request.uvfonc);
    reals.push(request.tconst.to_bits());
    (integers, reals)
}
struct LimitEvaluator {
    calls: AtomicUsize,
    error: i32,
}
impl AdvApp2VarEvaluatorFunc2Var for LimitEvaluator {
    fn evaluate(&self, request: &AdvApp2VarEvaluation<'_>, result: &mut [f64], error: &mut i32) {
        assert_callback_request(request);
        let before = bits(request.parameters);
        let suffix = bits(&result[8..]);
        result[..8].fill(0.);
        *error = self.error;
        assert_eq!(bits(request.parameters), before);
        assert_eq!(bits(&result[8..]), suffix);
        self.calls.fetch_add(1, Ordering::Relaxed);
    }
}
fn assert_callback_request(request: &AdvApp2VarEvaluation<'_>) {
    assert_eq!(
        (request.dimension, request.nb_params, request.favor_iso),
        (1, 8, 1)
    );
    assert_eq!((request.u_order, request.v_order), (0, 0));
    assert_eq!(bits(&request.u_start_end), bits(&[0., 1.]));
    assert_eq!(bits(&request.v_start_end), bits(&[0., 1.]));
    assert_eq!(request.const_param.to_bits(), 0.5_f64.to_bits());
}
fn verify_limit_exit(fail_evaluator: bool) {
    let inputs = LimitInputs::new();
    let original = inputs.clone();
    let mut request = inputs.request(fail_evaluator);
    let unchanged = request_signature(&request);
    let mut storage = LimitOutputs::new(fail_evaluator);
    let evaluator = LimitEvaluator {
        calls: AtomicUsize::new(0),
        error: 5,
    };
    let (made, status) = perform_limit_exit(&mut request, &evaluator, &mut storage);
    assert_eq!(
        (made, status),
        if fail_evaluator { (0, 105) } else { (-7, 1) }
    );
    assert_eq!(
        evaluator.calls.load(Ordering::Relaxed),
        usize::from(fail_evaluator)
    );
    assert_eq!(request_signature(&request), unchanged);
    inputs.assert_unchanged(&original, &request);
    storage.assert_expected(fail_evaluator);
}
fn perform_limit_exit(
    request: &mut martensite_tkgeombase::adv_app2var::approx_f2var::IsoApproxRequest<'_>,
    evaluator: &LimitEvaluator,
    storage: &mut LimitOutputs,
) -> (i32, i32) {
    let mut output = storage.borrow();
    let mut status = -9;
    assert_eq!(
        mma2fnc_mut_(request, evaluator, &mut output, &mut status),
        0
    );
    (output.nbcrbe, status)
}
#[test]
fn zero_coefficient_capacity_preserves_outputs_after_evaluator_failure() {
    verify_limit_exit(true);
}
#[test]
fn zero_piece_capacity_preserves_outputs_before_degenerate_domain_return() {
    verify_limit_exit(false);
}

#[test]
fn zero_coefficient_capacity_finishes_the_qualified_zero_function_warning() {
    let inputs = LimitInputs::new();
    let original = inputs.clone();
    let mut request = inputs.request(true);
    let unchanged = request_signature(&request);
    let mut storage = LimitOutputs::new(true);
    let evaluator = LimitEvaluator {
        calls: AtomicUsize::new(0),
        error: 0,
    };
    assert_eq!(
        perform_limit_exit(&mut request, &evaluator, &mut storage),
        (1, -1)
    );
    assert_eq!(evaluator.calls.load(Ordering::Relaxed), 1);
    assert_eq!(request_signature(&request), unchanged);
    inputs.assert_unchanged(&original, &request);
    storage.assert_warning();
}
