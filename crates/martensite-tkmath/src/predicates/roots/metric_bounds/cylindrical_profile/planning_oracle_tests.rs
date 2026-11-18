// SPDX-License-Identifier: GPL-2.0-only
//! Independent exact-rational Hermite remainder controls for #525.
use super::*;
use crate::predicates::roots::certificate_arithmetic::ExactRatio;

struct RemainderOracle {
    level: u8,
    numerator: &'static str,
    denominator: &'static str,
    upper_bits: u64,
}

#[test]
fn linear_fit_matches_independent_remainder_and_level() {
    let authored = &[[-0.25, 0.], [0.25, 0.]];
    let mut query = input(authored);
    query.interval = CylindricalProfileInterval::new(0, 0).unwrap();
    let oracle = RemainderOracle {
        level: 2,
        numerator: "270565505",
        denominator: "1623901933092864",
        upper_bits: 4505390543175097136,
    };
    assert_remainder(query, oracle);
}

#[test]
fn quadratic_fit_matches_independent_remainder_and_level() {
    let authored = &[[0., 0.], [0., 0.], [0.25, 0.]];
    let mut query = input(authored);
    query.interval = CylindricalProfileInterval::new(0, 0).unwrap();
    let oracle = RemainderOracle {
        level: 3,
        numerator: "121516823228125",
        denominator: "438415257672207040512",
        upper_bits: 4508835275200493242,
    };
    assert_remainder(query, oracle);
}

#[test]
fn cubic_fit_matches_independent_remainder_and_level() {
    let authored = &[[0., 0.], [0., 0.], [0., 0.], [0.25, 0.]];
    let mut query = input(authored);
    query.interval = CylindricalProfileInterval::new(0, 0).unwrap();
    let oracle = RemainderOracle {
        level: 4,
        numerator: "1233263609585085",
        denominator: "9352858830340416864256",
        upper_bits: 4504077540104291309,
    };
    assert_remainder(query, oracle);
}

#[test]
fn restricted_cubic_fit_matches_independent_remainder_and_level() {
    let authored = &[[0., 0.], [0., 0.], [0., 0.], [0.25, 0.]];
    let mut query = input(authored);
    query.interval = CylindricalProfileInterval::new(1, 2).unwrap();
    query.spatial_limit = 1. / 16_777_216.;
    let oracle = RemainderOracle {
        level: 2,
        numerator: "1361755051652924596900313465939925",
        denominator: "348408438021893809489071900334108606201856",
        upper_bits: 4481303118502743504,
    };
    assert_remainder(query, oracle);
}

fn input(authored: &[[f64; 2]]) -> CylindricalFitQuery<'_> {
    CylindricalFitQuery {
        authored,
        interval: CylindricalProfileInterval::new(0, 0).unwrap(),
        support_radius: 1.,
        angular_origin: 0.,
        spatial_limit: 1. / 262_144.,
        max_level: 8,
        max_operations: 100_000,
    }
}

fn assert_remainder(query: CylindricalFitQuery<'_>, oracle: RemainderOracle) {
    let plan = plan_cylindrical_fit(query, || false).unwrap();
    let expected = ExactRatio::new(
        oracle.numerator.parse().unwrap(),
        oracle.denominator.parse().unwrap(),
    );
    let actual = plan.ideal_developed_error_bound();
    assert_eq!(plan.subdivision_level(), oracle.level);
    assert!(
        ExactRatio::from_finite(actual) >= expected,
        "underestimated remainder {actual}"
    );
    assert!(
        actual <= f64::from_bits(oracle.upper_bits),
        "unexpected remainder {actual}"
    );
}
