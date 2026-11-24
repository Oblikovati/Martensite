// SPDX-License-Identifier: GPL-2.0-only
use super::super::super::polynomial::ExactCoefficientPolynomial;
use super::*;
fn equation(rows: &[&[i32]]) -> BivariateEquation {
    BivariateEquation::new(
        rows.iter()
            .map(|row| ExactCoefficientPolynomial::new(row.iter().map(|v| (*v).into()).collect()))
            .collect(),
    )
}
fn ratios(values: &[&[i32]]) -> Matrix {
    values
        .iter()
        .map(|row| {
            row.iter()
                .map(|v| ExactRatio::new((*v).into(), 1.into()))
                .collect()
        })
        .collect()
}
#[test]
fn unit_tensor_coefficients_match_the_independent_polynomial_identity() {
    // 1 + t² + 2u has T-degree two and U-degree one.
    let input = equation(&[&[1, 0, 1], &[2]]);
    let proof = certify(&input, [[0., 1.]; 2], 2, &mut || false)
        .unwrap()
        .unwrap();
    assert_eq!(proof.degrees, [2, 1]);
    assert_eq!(proof.values, ratios(&[&[1, 3], &[1, 3], &[2, 4]]));
    assert_eq!(proof.sign, Ordering::Greater);
}
#[test]
fn affine_restriction_uses_the_actual_unequal_and_extrapolated_ranges() {
    let input = equation(&[&[1, 0, 1], &[2]]);
    let proof = certify(&input, [[2., 3.], [-1., 1.]], 2, &mut || false)
        .unwrap()
        .unwrap();
    assert_eq!(proof.values, ratios(&[&[3, 7], &[5, 9], &[8, 12]]));
}
#[test]
fn strict_negative_coefficients_exclude_without_absolute_value_or_tolerance() {
    let input = equation(&[&[-3, 1], &[-1]]);
    let proof = certify(&input, [[0., 1.]; 2], 1, &mut || false)
        .unwrap()
        .unwrap();
    assert_eq!(proof.values, ratios(&[&[-3, -4], &[-2, -3]]));
    assert_eq!(proof.sign, Ordering::Less);
}
#[test]
fn zero_mixed_and_positive_but_unproved_hulls_are_inconclusive() {
    for input in [
        equation(&[]),
        equation(&[&[0, 0, 1]]),
        equation(&[&[7, -20, 20]]),
    ] {
        assert!(
            certify(&input, [[0., 1.]; 2], 2, &mut || false)
                .unwrap()
                .is_none()
        );
    }
}
#[test]
fn conservative_tensor_degree_admission_never_expands_the_requested_cap() {
    let input = equation(&[&[1, 0, 1]]);
    assert!(
        certify(&input, [[0., 1.]; 2], 1, &mut || false)
            .unwrap()
            .is_none()
    );
}
#[test]
fn cancellation_during_basis_conversion_retains_the_original_cause() {
    let input = equation(&[&[1, 0, 1], &[2]]);
    assert!(matches!(
        certify(&input, [[0., 1.]; 2], 2, &mut || true),
        Err(RootIsolationError::Cancelled)
    ));
}

#[test]
fn fractional_affine_restriction_preserves_exact_coefficient_denominators() {
    let input = equation(&[&[1, 0, 1], &[1]]);
    let proof = certify(&input, [[0., 0.5], [0., 0.25]], 2, &mut || false)
        .unwrap()
        .unwrap();
    assert_eq!(proof.values[0][1], ExactRatio::new(5.into(), 4.into()));
    assert_eq!(proof.values[2][1], ExactRatio::new(3.into(), 2.into()));
}
