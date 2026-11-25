// SPDX-License-Identifier: GPL-2.0-only
use super::*;

fn checked(regions: &[[f64; 2]]) -> Result<(), ConicalReferenceFailure> {
    let mut stop = || false;
    let policy = ArithmeticPolicy {
        fractional_bits: 128,
        max_operations: 10_000,
    };
    let mut math = Arithmetic::new(policy, &mut stop);
    let lifts: Vec<_> = regions
        .iter()
        .map(|&[lo, hi]| Interval {
            lower: math.scalar(lo).unwrap().lower,
            upper: math.scalar(hi).unwrap().upper,
        })
        .collect();
    separation(&lifts, &mut math)
}
#[test]
fn separates_regions_with_a_joint_lift_wider_than_a_period() {
    assert!(checked(&[[-4.1, -3.9], [4.1, 4.3]]).is_ok());
    assert!(checked(&[[4.1, 4.3], [-4.1, -3.9]]).is_ok());
}
#[test]
fn compares_periodic_images_including_the_final_wrap_gap() {
    assert!(checked(&[[-4.1, -3.9], [2.25, 2.39]]).is_err());
    assert!(checked(&[[0.05, 0.1], [6.24, 6.41]]).is_err());
    assert!(checked(&[[0.2, 0.3], [6.24, 6.41]]).is_ok());
    assert!(checked(&[[1.0, 2.0], [2.0, 3.0]]).is_err());
}
