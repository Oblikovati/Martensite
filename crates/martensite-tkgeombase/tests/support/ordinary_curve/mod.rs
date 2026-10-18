//! One retained complete-operation path and independent analytic controls.
mod evaluator;
mod independent;
mod specification;
mod storage;
pub use specification::{CaseSpec, RetainedObservation};
use specification::{PREFIX, SUFFIX};
use storage::OriginalOutputs;
pub const TABLE_NAMES: [&str; 8] = [
    "curve",
    "sums",
    "differences",
    "lower",
    "upper",
    "cuts",
    "maximum",
    "mean",
];
use martensite_tkgeombase::adv_app2var::approx_f2var::{
    IsoApproxOutput, IsoApproxRequest, mma2fnc_,
};
use std::sync::atomic::Ordering;
pub fn run_retained(spec: &CaseSpec, expected: &RetainedObservation) {
    let mut tables = OriginalOutputs::new(spec);
    let evaluator = evaluator::OrdinaryEvaluator {
        spec,
        calls: 0.into(),
    };
    let precision = [spec.precision; 2];
    let mut request = request(spec, expected);
    request.epsapr = &precision;
    let before = input_snapshot(&request);
    let mut output = tables.output();
    let mut status = -9;
    assert_eq!(mma2fnc_(&request, &evaluator, &mut output, &mut status), 0);
    check_completion(spec, expected, status, output.nbcrbe, &evaluator);
    assert_eq!(input_snapshot(&request), before);
    tables.check_complete_owner(spec, expected);
    println!(
        "ordinary_curve_contract|case={}|status={status}|made={}|callbacks={}|all_inputs_unchanged=true|all_caller_real_slots_verified=true",
        spec.name, expected.made, expected.calls
    );
}
fn check_completion(
    spec: &CaseSpec,
    expected: &RetainedObservation,
    status: i32,
    made: i32,
    evaluator: &evaluator::OrdinaryEvaluator<'_>,
) {
    assert_eq!(
        (status, made),
        (expected.status, expected.made),
        "{}",
        spec.name
    );
    assert_eq!(
        evaluator.calls.load(Ordering::SeqCst),
        expected.calls,
        "{}",
        spec.name
    );
}
fn request<'a>(spec: &CaseSpec, expected: &'a RetainedObservation) -> IsoApproxRequest<'a> {
    IsoApproxRequest {
        ndimen: 2,
        nbsesp: 2,
        ndimse: &[1, 1],
        uvfonc: spec.domain,
        tconst: spec.constant,
        isofav: spec.iso,
        nbroot: spec.roots as i32,
        rootlg: &expected.roots[PREFIX..PREFIX + spec.roots],
        iordre: spec.order,
        ideriv: spec.cross,
        ndgjac: spec.degree,
        nbcrmx: spec.maximum as i32,
        ncflim: spec.limit as i32,
        epsapr: &[],
    }
}
fn input_snapshot(request: &IsoApproxRequest<'_>) -> (Vec<i32>, Vec<u64>) {
    (integer_snapshot(request), real_snapshot(request))
}
fn integer_snapshot(request: &IsoApproxRequest<'_>) -> Vec<i32> {
    let mut integers = vec![
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
    integers.extend_from_slice(request.ndimse);
    integers
}
fn real_snapshot(request: &IsoApproxRequest<'_>) -> Vec<u64> {
    request
        .uvfonc
        .iter()
        .chain(std::iter::once(&request.tconst))
        .chain(request.rootlg.iter())
        .chain(request.epsapr.iter())
        .map(|v| v.to_bits())
        .collect()
}
pub fn cases() -> Vec<(CaseSpec, RetainedObservation)> {
    specification::cases()
}
