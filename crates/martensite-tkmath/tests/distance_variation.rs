// SPDX-License-Identifier: GPL-2.0-only
use martensite_tkmath::predicates::distance_second_variation_sign as variation;
use std::cmp::Ordering;

#[test]
fn second_variation_exact_sign_handles_cancellation_and_underflow() {
    assert_eq!(
        variation([0.; 3], [1., 0., 0.], [0., 1., 0.], [-1., 0., 0.]),
        Ok(Ordering::Equal)
    );
    assert_eq!(
        variation(
            [0.; 3],
            [1., 0., 0.],
            [0., 1., f64::MIN_POSITIVE],
            [-1., 0., 0.]
        ),
        Ok(Ordering::Greater)
    );
    assert_eq!(
        variation(
            [0.; 3],
            [1., 0., f64::MIN_POSITIVE],
            [0., 1., 0.],
            [-1., 0., -f64::MIN_POSITIVE]
        ),
        Ok(Ordering::Less)
    );
}

#[test]
fn second_variation_rejects_nonfinite_coordinate_with_its_index() {
    let failure = variation([0.; 3], [0.; 3], [0.; 3], [0., f64::NAN, 0.]).unwrap_err();
    assert_eq!((failure.point, failure.axis), (3, 1));
    assert!(failure.value.is_nan());
}
