//! Only three source-reviewed zero-owner paths are characterized; no blanket zero-domain contract.
use martensite_tkgeombase::adv_app2var::approx_f2var::{IsoApproxRequest, mma2fnc_mut_};
#[path = "support/bivariate_zero_owners/mod.rs"]
mod support;
use support::{ZeroCase, ZeroEvaluator, ZeroInputs, ZeroOutputs, bits};
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
fn verify_zero_owner_case(case: ZeroCase) {
    let inputs = ZeroInputs::new(case);
    let original = inputs.clone();
    let mut request = inputs.request(case);
    let unchanged = request_signature(&request);
    let mut storage = ZeroOutputs::new(case);
    let evaluator = ZeroEvaluator::new(case);
    let (made, status) = perform_zero_case(&mut request, &evaluator, &mut storage);
    assert_eq!((made, status), (1, 0));
    assert_eq!(request_signature(&request), unchanged);
    evaluator.assert_one_call();
    inputs.assert_borrowed_inputs(&request);
    inputs.assert_unchanged(&original);
    storage.assert_expected(case);
}
fn perform_zero_case(
    request: &mut IsoApproxRequest<'_>,
    evaluator: &ZeroEvaluator,
    storage: &mut ZeroOutputs,
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
fn empty_dimension_and_empty_partition_keep_a_zero_retained_count() {
    verify_zero_owner_case(ZeroCase {
        dimension: 0,
        order: -1,
        degree: 0,
        limit: 1,
        partitions: &[],
        retained: 0,
    });
}
#[test]
fn zero_dimension_with_passage_constraints_initializes_norm_scratch() {
    verify_zero_owner_case(ZeroCase {
        dimension: 0,
        order: 0,
        degree: 2,
        limit: 3,
        partitions: &[0],
        retained: 1,
    });
}
#[test]
fn trailing_zero_subspace_reuses_the_initialized_component_bound() {
    verify_zero_owner_case(ZeroCase {
        dimension: 1,
        order: -1,
        degree: 2,
        limit: 3,
        partitions: &[1, 0],
        retained: 1,
    });
}
