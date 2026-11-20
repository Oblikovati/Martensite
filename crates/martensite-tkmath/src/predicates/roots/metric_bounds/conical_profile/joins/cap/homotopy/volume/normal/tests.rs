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
        cap.slant_motion = DyadicBound::from_f64(1. / 32.).unwrap();
        let meridian = Meridian::capture(&cap, &mut arithmetic).unwrap();
        cap.squared_world_motion = rectangle_motion(&meridian, outward, &mut arithmetic);
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
    cap.slant_motion = DyadicBound::from_f64(1.).unwrap();
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
fn positive_outward_depth_does_not_hide_a_developed_apex_crossing() {
    let mut never = || false;
    let mut arithmetic = volume_arithmetic(Default::default(), &mut never).unwrap();
    let moments = rectangle_moments(&mut arithmetic);
    let mut cap = rectangle_support(4., true);
    cap.reference = 0.5_f64.to_bits();
    cap.support[2] = 2_f64.to_bits();
    cap.slant_motion = DyadicBound::from_f64(1.).unwrap();
    let meridian = Meridian::capture(&cap, &mut arithmetic).unwrap();
    assert!(matches!(
        spatial(&cap, &moments, &meridian, 2., &mut arithmetic),
        Err(ConicalReferenceFailure::Bound {
            field: "positive developed slant over the filled cap interpolation",
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
    cap.slant_motion = DyadicBound::from_f64(1. / 32.).unwrap();
    cap.squared_world_motion = DyadicBound::from_f64(1. / 1024.).unwrap();
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

fn rectangle_motion(
    meridian: &Meridian,
    outward: bool,
    arithmetic: &mut Arithmetic<'_>,
) -> DyadicBound {
    use crate::predicates::roots::metric_bounds::conical_profile::joins::cap::metric::MotionMetric;
    let upper_depth = if outward { 2. } else { 0. };
    let metric = MotionMetric::new(
        10.,
        upper_depth,
        [&meridian.sine, &meridian.cosine],
        arithmetic,
    )
    .unwrap();
    let slant_hull = arithmetic.scalar(3.).unwrap();
    let angle = arithmetic.scalar(1. / 128.).unwrap();
    let slant = arithmetic.scalar(1. / 32.).unwrap();
    let squared = metric
        .squared_motion(&slant_hull, &angle, &slant, arithmetic)
        .unwrap();
    DyadicBound::new(squared.upper, -i32::from(arithmetic.fractional_bits()))
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
    assert!((upper - martensite_cmath::hypot(1. / 32., expected_high / 128.)).abs() < 1e-12);
}
