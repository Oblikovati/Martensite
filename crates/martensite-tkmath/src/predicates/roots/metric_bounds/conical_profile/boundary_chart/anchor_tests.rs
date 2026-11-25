// SPDX-License-Identifier: GPL-2.0-only
use super::super::angle::{
    AnchorRequest, AngleResult, InitialAngleFailure, anchor_capture_in_chart, anchor_in_chart,
    anchor_with_sine,
};
use super::tests::{domain, options};
use super::*;
fn request(point: [f64; 2], alpha: f64) -> AnchorRequest {
    let angle = martensite_cmath::atan2(point[1], point[0]) / martensite_cmath::sin(alpha);
    AnchorRequest {
        point,
        semi_angle: alpha,
        origin: 0.,
        developed_turn: 0,
        physical_turn: 0,
        fit_center: 0.,
        fit_half_angle_tangent: martensite_cmath::tan(angle / 2.),
    }
}
fn evaluate(
    request: AnchorRequest,
    domain: &ConicalChartDomainEvidence,
    cached: bool,
    stop: usize,
) -> (AngleResult<(Interval, Interval)>, u32, usize) {
    let mut polls = 0;
    let mut callback = || {
        polls += 1;
        polls == stop
    };
    let mut arithmetic = Arithmetic::new(sector_api::policy(options()), &mut callback);
    let result = match cached {
        true => anchor_in_chart(&request, domain, &mut arithmetic),
        false => anchor_with_sine(&request, &mut arithmetic),
    };
    let work = arithmetic.operations();
    drop(arithmetic);
    (result, work, polls)
}
#[test]
fn cached_sine_retains_exact_anchor_intervals_and_uses_less_actual_work() {
    for alpha in [1e-6, 0.125, std::f64::consts::FRAC_PI_2 - 1e-6] {
        for y in [-1e-6, 1e-6] {
            check_anchor(alpha, y);
        }
    }
}
fn check_anchor(alpha: f64, y: f64) {
    let domain = domain(alpha);
    let request = request([80., y], alpha);
    let (old, old_work, _) = evaluate(request, &domain, false, usize::MAX);
    let (new, new_work, _) = evaluate(request, &domain, true, usize::MAX);
    assert_eq!(old.unwrap(), new.unwrap());
    assert!(new_work < old_work);
}
#[test]
fn actual_point_changes_are_not_replaced_by_cached_source_angles() {
    let domain = domain(0.125);
    let input = request([80., 1.], 0.125);
    let original = evaluate(input, &domain, true, usize::MAX).0.unwrap();
    let mut changed = input;
    changed.point = [80., -1.];
    let actual = evaluate(changed, &domain, true, usize::MAX).0.unwrap();
    assert_ne!(actual, original);
    assert_eq!(
        actual,
        evaluate(changed, &domain, false, usize::MAX).0.unwrap()
    );
}
#[test]
fn cached_path_retains_support_and_fit_admission() {
    let domain = domain(0.125);
    let mut input = request([80., 0.], 0.125);
    input.semi_angle = f64::NAN;
    assert!(matches!(
        evaluate(input, &domain, true, usize::MAX).0,
        Err(InitialAngleFailure::InvalidSupport { .. })
    ));
    input.semi_angle = 0.125;
    input.fit_half_angle_tangent = f64::NAN;
    assert!(matches!(
        evaluate(input, &domain, true, usize::MAX).0,
        Err(InitialAngleFailure::InvalidFit { .. })
    ));
}
#[test]
fn cancellation_keeps_every_cached_anchor_result_unpublished() {
    let domain = domain(0.125);
    let input = request([80., 1.], 0.125);
    let (_, _, polls) = evaluate(input, &domain, true, usize::MAX);
    for stop in [1, polls / 2, polls] {
        assert!(matches!(
            evaluate(input, &domain, true, stop).0,
            Err(InitialAngleFailure::Arithmetic(
                BoundedArithmeticFailure::Cancelled
            ))
        ));
    }
}

#[test]
fn captured_fit_phase_is_the_already_executed_unshifted_ideal_phase() {
    let domain = domain(0.125);
    let mut input = request([80., 1.], 0.125);
    input.physical_turn = 1;
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(sector_api::policy(options()), &mut never);
    let captured = anchor_capture_in_chart(&input, &domain, &mut arithmetic).unwrap();
    let expected = super::super::angle::fit_phase(&input, &mut arithmetic).unwrap();
    assert_eq!(captured.fit_phase, expected);
}
