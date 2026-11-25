// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::certificate_arithmetic::bounded_failure::BoundedArithmeticFailure;
#[test]
fn parent_two_keeps_exact_term_cells_and_independent_sector_value() {
    let spans = sector();
    let input = query(&spans);
    let v1 = certify_conical_reference_volume(input, || false).unwrap();
    let mut input = input;
    input.options.behavior_version = 2;
    let v2 = certify_conical_reference_volume(input, || false).unwrap();
    contains(
        &v2,
        expected(std::f64::consts::FRAC_PI_4, 5., 3., input.depths, true),
    );
    assert_eq!(v2.integration_subintervals(), v1.integration_subintervals());
    assert!(v2.arithmetic_operations() > v1.arithmetic_operations());
    assert_eq!(v2.requested_options(), input.options);
    assert_eq!(v2.effective_options(), input.options);
    changed_reference_policy(&v2, input);
}
#[test]
fn parent_two_reports_actual_remaining_import_work_before_refusal() {
    let spans = sector();
    let input = query(&spans);
    let coefficients = coefficients::compute(input, &mut || false).unwrap().1;
    let mut input = input;
    input.options.behavior_version = 2;
    input.arithmetic.max_operations = coefficients + 6;
    let failure = certify_conical_reference_volume(input, || false).unwrap_err();
    assert_eq!(failure.operations, coefficients + 6);
    assert_eq!(failure.span, Some(0));
    assert!(
        matches!(*failure.cause,ConicalReferenceVolumeFailure::Arithmetic(cause)
        if matches!(*cause,BoundedArithmeticFailure::WorkLimit{completed:6,limit:6}))
    );
}

fn changed_reference_policy(
    v2: &CertifiedConicalReferenceVolume,
    input: ConicalReferenceVolumeQuery<'_>,
) {
    assert!(v2.matches_source(input));
    let mut changed = input;
    changed.options.behavior_version = 1;
    assert!(!v2.matches_source(changed));
    changed = input;
    changed.arithmetic.fractional_bits -= 1;
    assert!(!v2.matches_source(changed));
}
#[test]
fn parent_two_reference_binding_preserves_signed_zero_policy_bits() {
    let spans = [span(&[0.], &[1.], &[1.])];
    let mut input = query(&spans);
    input.options.behavior_version = 2;
    input.options.absolute_error_limit = -0.;
    let proof = certify_conical_reference_volume(input, || false).unwrap();
    assert_eq!(proof.enclosure(), [0., 0.]);
    assert!(proof.matches_source(input));
    input.options.absolute_error_limit = 0.;
    assert!(!proof.matches_source(input));
}
