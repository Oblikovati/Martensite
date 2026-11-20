// SPDX-License-Identifier: GPL-2.0-only
use super::super::tests::{rectangle_moments, rectangle_support};
use super::*;

#[test]
fn metric_bound_uses_swept_radius_for_both_normal_senses() {
    let mut never = || false;
    let mut arithmetic = volume_arithmetic(Default::default(), &mut never).unwrap();
    let moments = rectangle_moments(&mut arithmetic);
    for outward in [false, true] {
        let mut cap = rectangle_support(2., outward);
        cap.component_motion = motion(1. / 128., 1. / 32.);
        let meridian = Meridian::capture(&cap, &mut arithmetic).unwrap();
        let [error_bound, radii] =
            spatial(&cap, &moments, &meridian, 0.2, &mut arithmetic).unwrap();
        check_closed_form(error_bound, radii, outward, &arithmetic);
    }
}

#[test]
fn a_positive_decoded_cap_can_still_have_an_unresolved_swept_radius() {
    let mut never = || false;
    let mut arithmetic = volume_arithmetic(Default::default(), &mut never).unwrap();
    let moments = rectangle_moments(&mut arithmetic);
    let mut cap = rectangle_support(5., false);
    cap.component_motion = motion(0., 1.);
    assert!(finish(&cap, rectangle_moments(&mut arithmetic), &mut arithmetic).is_ok());
    let meridian = Meridian::capture(&cap, &mut arithmetic).unwrap();
    assert!(matches!(
        spatial(&cap, &moments, &meridian, 2., &mut arithmetic),
        Err(ConicalReferenceFailure::Bound {
            field: "positive radius over the filled cap interpolation",
            ..
        })
    ));
}

#[test]
fn exceeding_the_spatial_allowance_preserves_the_requested_limit() {
    let mut never = || false;
    let mut arithmetic = volume_arithmetic(Default::default(), &mut never).unwrap();
    let moments = rectangle_moments(&mut arithmetic);
    let mut cap = rectangle_support(2., true);
    cap.component_motion = motion(0., 1. / 32.);
    let meridian = Meridian::capture(&cap, &mut arithmetic).unwrap();
    assert!(matches!(
        spatial(&cap, &moments, &meridian, 1. / 64., &mut arithmetic),
        Err(ConicalReferenceFailure::Bound {
            field: "filled cap normal discrepancy",
            actual,
            limit,
        }) if actual == 1. / 32. && limit == 1. / 64.
    ));
}

fn motion(angle: f64, slant: f64) -> [DyadicBound; 2] {
    [angle, slant].map(|value| DyadicBound::from_f64(value).unwrap())
}

fn check_closed_form(bound: Interval, radii: Interval, outward: bool, arithmetic: &Arithmetic<'_>) {
    let sine = martensite_cmath::sin(0.5);
    let offset = 2. * martensite_cmath::cos(0.5);
    let expected_low = (10. - 1. / 32.) * sine - if outward { 0. } else { offset };
    let expected_high = (13. + 1. / 32.) * sine + if outward { offset } else { 0. };
    let bounds = exact_enclosure(&radii, arithmetic.fractional_bits());
    assert!((bounds[0].binary64_bounds().unwrap()[0] - expected_low).abs() < 1e-12);
    assert!((bounds[1].binary64_bounds().unwrap()[1] - expected_high).abs() < 1e-12);
    let upper = arithmetic.upper_f64(bound.upper).unwrap();
    assert!((upper - (1. / 32. + expected_high / 128.)).abs() < 1e-12);
}
