//! Source-reviewed capacity candidate; the retained profile remains separately qualified.
use martensite_tkgeombase::adv_app2var::approx_f2var::{IsoApproxRequest, mma2fnc_, mma2fnc_mut_};
#[path = "support/bivariate_zero_owners/mod.rs"]
mod support;
use support::{ZeroCase, ZeroEvaluator, ZeroInputs, ZeroOutputs};

fn verify_initialized_capacity(case: ZeroCase) {
    let inputs = ZeroInputs::new(case);
    let original = inputs.clone();
    let mut request = inputs.request(case);
    let evaluator = ZeroEvaluator::new(case);
    let mut storage = ZeroOutputs::new(case);
    let mut output = storage.borrow();
    let mut status = -9;
    assert_eq!(
        mma2fnc_mut_(&mut request, &evaluator, &mut output, &mut status),
        0
    );
    assert_eq!((status, output.nbcrbe), (0, 1));
    evaluator.assert_one_call();
    inputs.assert_borrowed_inputs(&request);
    inputs.assert_unchanged(&original);
    storage.assert_expected(case);
}

#[test]
fn limit_two_with_degree_zero_keeps_one_coefficient_and_zero_spare() {
    verify_initialized_capacity(ZeroCase {
        dimension: 1,
        order: -1,
        degree: 0,
        limit: 2,
        partitions: &[1],
        retained: 1,
    });
}

#[test]
fn leading_zero_subspace_uses_initialized_scratch_before_positive_component() {
    verify_initialized_capacity(ZeroCase {
        dimension: 1,
        order: -1,
        degree: 2,
        limit: 3,
        partitions: &[0, 1],
        retained: 1,
    });
}

#[test]
fn zero_dimension_even_rule_uses_initialized_central_scratch() {
    verify_initialized_capacity(ZeroCase {
        dimension: 0,
        order: -1,
        degree: 2,
        limit: 3,
        partitions: &[0],
        retained: 1,
    });
}

fn zero_budget_physical_case() -> ZeroCase {
    ZeroCase {
        dimension: 1,
        order: -1,
        degree: 0,
        limit: 1,
        partitions: &[1],
        retained: 1,
    }
}

fn perform_zero_budget_request(
    request: &IsoApproxRequest<'_>,
    evaluator: &ZeroEvaluator,
    storage: &mut ZeroOutputs,
) -> (i32, i32, i32) {
    let mut output = storage.borrow();
    let mut status = -9;
    assert_eq!(mma2fnc_(request, evaluator, &mut output, &mut status), 0);
    (status, output.nbcrbe, output.ncoeff[0])
}

#[test]
fn zero_piece_budget_accepts_exact_constant_with_explicit_first_piece_storage() {
    let case = zero_budget_physical_case();
    let inputs = ZeroInputs::new(case);
    let original = inputs.clone();
    let mut request = inputs.request(case);
    request.nbcrmx = 0;
    let evaluator = ZeroEvaluator::new(case);
    // Physical caller payloads contain one piece independently of the logical zero budget.
    let mut storage = ZeroOutputs::new(case);
    let actual = perform_zero_budget_request(&request, &evaluator, &mut storage);
    assert_eq!(actual, (0, 1, 1));
    assert_eq!(request.nbcrmx, 0);
    evaluator.assert_one_call();
    inputs.assert_borrowed_inputs(&request);
    inputs.assert_unchanged(&original);
    storage.assert_expected(case);
    emit_zero_budget_trace(actual);
}

fn emit_zero_budget_trace(actual: (i32, i32, i32)) {
    println!(
        "{{\"case\":\"M0-Z\",\"status\":{},\"pieces\":{},\"retained\":{},\"callbacks_verified\":1}}",
        actual.0, actual.1, actual.2
    );
}
