// SPDX-License-Identifier: GPL-2.0-only
use super::*;

#[test]
fn radial_authored_ray_has_zero_angular_rate_and_positive_slant_rate() {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(
        sector_api::policy(ConeClearanceOptions::default()),
        &mut never,
    );
    let source = capture_planar(&[[3., 1.5], [6., 3.]], &mut arithmetic).unwrap();
    let sine = arithmetic.scalar(0.125).unwrap();
    let [angle, slant] = authored_rate(&source, &sine, &mut arithmetic).unwrap();
    assert_eq!(angle, arithmetic.zero());
    assert!(slant.lower > 0.into());
    let known = 3. * 1.25_f64.sqrt();
    let [lower, upper] = exact_enclosure(&slant, arithmetic.fractional_bits())
        .map(|bound| bound.binary64_bounds().unwrap());
    assert!(lower[0] <= known && known <= upper[1]);
}

#[test]
fn opposing_derivatives_have_no_common_positive_projection() {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(
        sector_api::policy(ConeClearanceOptions::default()),
        &mut never,
    );
    let left = [[arithmetic.zero(), arithmetic.scalar(1.).unwrap()]];
    let right = [[arithmetic.zero(), arithmetic.scalar(-1.).unwrap()]];
    assert!(matches!(
        common_direction(&left, &right, &mut arithmetic),
        Err(CombinedFailure::DirectionUnresolved)
    ));
}

#[test]
fn cap_gap_refuses_even_with_the_same_authored_join_and_positive_rates() {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(
        sector_api::policy(ConeClearanceOptions::default()),
        &mut never,
    );
    let zero = arithmetic.zero();
    let positive = arithmetic.scalar(1.).unwrap();
    let make = |start: f64, end: f64| Projection {
        authored: [[zero.clone(), zero.clone()], [zero.clone(), zero.clone()]],
        cap: [[0, start.to_bits()], [0, end.to_bits()]],
        rates: vec![[positive.clone(), zero.clone()]],
    };
    let first = make(1., 2.);
    assert!(first.join(&make(2., 3.), &mut arithmetic).is_ok());
    assert!(matches!(
        first.join(&make(2_f64.next_up(), 3.), &mut arithmetic),
        Err(CombinedFailure::JoinMismatch)
    ));
}
