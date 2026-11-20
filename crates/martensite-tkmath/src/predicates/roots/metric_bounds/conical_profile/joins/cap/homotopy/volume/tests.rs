// SPDX-License-Identifier: GPL-2.0-only
use super::*;

#[test]
fn rectangular_cap_has_exact_signed_polynomial_moments() {
    let mut never = || false;
    let options = ConeClearanceOptions::default();
    let mut arithmetic = Arithmetic::new(sector_api::policy(options), &mut never);
    let moments = rectangle_moments(&mut arithmetic);
    assert_eq!(moments.area, arithmetic.scalar(6.).unwrap());
    assert_eq!(moments.slant, arithmetic.scalar(18.).unwrap());
    assert_eq!(moments.minimum_v, Some(0.into()));
}

#[test]
fn depth_midpoint_uses_outward_and_inward_normal_senses() {
    let mut never = || false;
    let options = ConeClearanceOptions::default();
    let mut arithmetic = Arithmetic::new(sector_api::policy(options), &mut never);
    for outward in [false, true] {
        let moments = rectangle_moments(&mut arithmetic);
        let cap = rectangle_support(2., outward);
        let volume = finish(&cap, moments, &mut arithmetic).unwrap();
        check_rectangle_volume(volume, outward, options);
    }
}

#[test]
fn a_nonpositive_radius_refuses_the_cap_volume() {
    let mut never = || false;
    let mut arithmetic = Arithmetic::new(
        sector_api::policy(ConeClearanceOptions::default()),
        &mut never,
    );
    let moments = rectangle_moments(&mut arithmetic);
    assert!(matches!(
        finish(&rectangle_support(20., false), moments, &mut arithmetic),
        Err(ConicalCapVolumeFailure::Radius { .. })
    ));
}

pub(super) fn rectangle_moments(arithmetic: &mut Arithmetic<'_>) -> Moments {
    let mut moments = Moments::empty(arithmetic);
    let points = [[0_f64, 0.], [0., 3.], [2., 3.], [2., 0.]].map(|point| point.map(f64::to_bits));
    for prefix in 0..points.len() {
        let edge = [points[prefix], points[(prefix + 1) % points.len()]];
        moments.include(&edge, arithmetic).unwrap();
    }
    moments
}

pub(super) fn rectangle_support(depth: f64, outward: bool) -> CapBinding {
    CapBinding {
        reference: 10_f64.to_bits(),
        support: [0.5_f64.to_bits(), 0, 0, depth.to_bits(), u64::from(outward)],
        carriers: [[0; 3]; 2],
        motion: DyadicBound::new(0.into(), 0),
        slant_motion: DyadicBound::new(0.into(), 0),
        squared_world_motion: DyadicBound::new(0.into(), 0),
        trace_position_upper: 0.,
        spans: Vec::new(),
    }
}

fn check_rectangle_volume(volume: Interval, outward: bool, options: ConeClearanceOptions) {
    // Angle width 2, slant range [10,13], signed midpoint depth ±1, and thickness 2.
    let sense = if outward { 1. } else { -1. };
    let expected = 12. * (11.5 * martensite_cmath::sin(0.5) + sense * martensite_cmath::cos(0.5));
    let [lower, upper] = exact_enclosure(&volume, options.fractional_bits)
        .map(|bound| bound.binary64_bounds().unwrap());
    assert!(upper[1] - lower[0] < 1e-12);
    assert!((lower[0] - expected).abs() < 1e-12);
}
