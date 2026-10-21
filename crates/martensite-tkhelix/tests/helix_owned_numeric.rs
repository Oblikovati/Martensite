//! Independent zero-angle identities pin the owned analytic and legacy contracts.
mod support;
use martensite_tkg3d::adaptor3d::Adaptor3dCurve;
use martensite_tkhelix::helix_geom::HelixGeomHelixCurve;
use std::f64::consts::{FRAC_PI_4, TAU};
use support::expect_double_eq;

const ZERO_ANGLE_CORPUS: &[(f64, f64, f64, f64)] = &[
    (0., 3., 0., 0.),
    (TAU, 0., 0., 0.),
    (TAU, 3., 0., 0.),
    (TAU, 3., FRAC_PI_4, 1.),
    (TAU, 3., -FRAC_PI_4, -1.),
    (2. * TAU, 1., FRAC_PI_4, 2.),
];

#[test]
fn zero_angle_corpus_covers_pitch_radius_taper_and_handedness() {
    for &(pitch, radius, taper, slope) in ZERO_ANGLE_CORPUS {
        for clockwise in [true, false] {
            let curve = HelixGeomHelixCurve::new();
            curve
                .load(-2., 4., pitch, radius, taper, clockwise)
                .unwrap();
            check_zero_angle(&curve, pitch / TAU, radius, slope, clockwise);
        }
    }
}

fn check_zero_angle(
    curve: &HelixGeomHelixCurve,
    rise: f64,
    radius: f64,
    slope: f64,
    clockwise: bool,
) {
    let sign = if clockwise { 1. } else { -1. };
    let (point, first, second) = curve.d2(0.).unwrap();
    check_components((point.x(), point.y(), point.z()), (radius, 0., 0.));
    check_components(
        (first.x(), first.y(), first.z()),
        (slope, sign * radius, rise),
    );
    // The final Y component deliberately pins the legacy tapered-D2 sign defect.
    check_components(
        (second.x(), second.y(), second.z()),
        (-radius, -sign * 2. * slope, 0.),
    );
}

fn check_components(actual: (f64, f64, f64), expected: (f64, f64, f64)) {
    expect_double_eq(actual.0, expected.0);
    expect_double_eq(actual.1, expected.1);
    expect_double_eq(actual.2, expected.2);
}

#[test]
fn tapered_second_y_is_the_retained_opposite_of_the_analytic_derivative() {
    let curve = HelixGeomHelixCurve::new();
    curve.load(-2., 4., TAU, 3., FRAC_PI_4, true).unwrap();
    let (_, _, second) = curve.d2(0.).unwrap();
    // Differentiating (3+t)sin(t) twice gives +2 at zero, while the legacy API gives -2.
    expect_double_eq(second.y(), -2.);
    assert!(second.y() < 0.);
}
