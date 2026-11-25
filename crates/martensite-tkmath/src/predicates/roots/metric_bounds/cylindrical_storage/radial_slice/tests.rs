// SPDX-License-Identifier: GPL-2.0-only
use super::*;

const INNER: [[f64; 3]; 2] = [[2., 0., 0.], [2., 2., 3.]];
const OUTER: [[f64; 3]; 2] = [[4., 0., 0.], [4., 4., 3.]];

fn geometry<'a>(inner: &'a [[f64; 3]], outer: &'a [[f64; 3]]) -> CylindricalSliceQuery<'a> {
    CylindricalSliceQuery {
        rails: [inner, outer].map(|poles| MetricBezier {
            poles,
            weights: None,
        }),
        sense: CylindricalRadialSense::Outward,
        chart_radius: 10.,
        direction: [1., 0.],
        depth: [-0.25, 1.25],
        options: Default::default(),
    }
}

#[test]
fn homothetic_xy_certifies_identical_fixed_radius_slope_in_both_senses() {
    let original = geometry(&INNER, &OUTER);
    let proof = certify_cylindrical_slice_direction(original, || false).unwrap();
    assert!(proof.matches_source(original));
    // X/Y angle is atan(t), independent of depth. Its minimum scaled derivative is 5.
    assert!(proof.derivative_lower() > 4.999999 && proof.derivative_lower() <= 5.);
    let inward = CylindricalSliceQuery {
        rails: [original.rails[1], original.rails[0]],
        sense: CylindricalRadialSense::Inward,
        ..original
    };
    assert!(!proof.matches_source(inward));
    let reverse = certify_cylindrical_slice_direction(inward, || false).unwrap();
    assert!(reverse.derivative_lower() > 4.999999 && reverse.derivative_lower() <= 5.);
}

#[test]
fn exact_height_and_angular_projection_combine_under_explicit_collar() {
    let projected = CylindricalSliceQuery {
        direction: [1., 0.5],
        ..geometry(&INNER, &OUTER)
    };
    let proof = certify_cylindrical_slice_direction(projected, || false).unwrap();
    let extended = CylindricalSliceQuery {
        depth: [-1., 1.],
        ..projected
    };
    assert!(!proof.matches_source(extended));
    assert!(proof.derivative_lower() > 6.499999 && proof.derivative_lower() <= 6.5);
    assert!(matches!(
        certify_cylindrical_slice_direction(extended, || false),
        Err(CylindricalSliceRefusal::Arithmetic(
            BoundedArithmeticFailure::DenominatorUnresolved { .. }
        ))
    ));
}

#[test]
fn height_transport_and_motion_sense_must_match_actual_rails() {
    let mut raised = OUTER;
    raised[0][2] = 0.125;
    assert!(matches!(
        certify_cylindrical_slice_direction(geometry(&INNER, &raised), || false),
        Err(CylindricalSliceRefusal::RailControl { index: 0, .. })
    ));
    let wrong_order = geometry(&OUTER, &INNER);
    assert!(matches!(
        certify_cylindrical_slice_direction(wrong_order, || false),
        Err(CylindricalSliceRefusal::Arithmetic(
            BoundedArithmeticFailure::DenominatorUnresolved { .. }
        ))
    ));
}

#[test]
fn nonhomothetic_rational_rail_chart_uses_the_fixed_radius_derivative() {
    let inner = [[2., 0., 0.], [2., 1., 0.]];
    let outer = [[4., 0.5, 0.], [4., 2.5, 0.]];
    let weights = [1., 2.];
    let query = CylindricalSliceQuery {
        rails: [&inner[..], &outer[..]].map(|poles| MetricBezier {
            poles,
            weights: Some(&weights),
        }),
        depth: [0., 1.],
        ..geometry(&inner, &outer)
    };
    let proof = certify_cylindrical_slice_direction(query, || false).unwrap();
    // In linear parameter t, the directional minimum is 160/47 at t=depth=1.
    // The rational parameter t=2u/(1+u) halves that minimum at u=1.
    let expected = 80. / 47.;
    assert!(proof.derivative_lower() > expected - 1e-8);
    assert!(proof.derivative_lower() <= expected);
    assert!(proof.matches_source(query));
}

#[test]
fn zero_or_exhausted_direction_cannot_issue_a_receipt() {
    let query = geometry(&INNER, &OUTER);
    let reversed = CylindricalSliceQuery {
        direction: [-1., 0.],
        ..query
    };
    assert!(matches!(
        certify_cylindrical_slice_direction(reversed, || false),
        Err(CylindricalSliceRefusal::Nonpositive { .. })
    ));
    assert!(matches!(
        certify_cylindrical_slice_direction(query, || true),
        Err(CylindricalSliceRefusal::Arithmetic(
            BoundedArithmeticFailure::Cancelled
        ))
    ));
}

#[test]
fn extended_depth_proves_common_radius_existence_in_both_senses() {
    let inner = [[2., 0., 0.], [2., 0., 3.]];
    let outer = [[4., 0., 0.], [4., 0., 3.]];
    let base = geometry(&inner, &outer);
    let stored = certify_cylindrical_storage(
        CylindricalStorageQuery {
            curves: base.rails,
            radii: [2., 4.],
            limits: CylindricalStorageLimits::uniform(1e-9),
            options: Default::default(),
        },
        || false,
    )
    .unwrap();
    for sense in [
        CylindricalRadialSense::Outward,
        CylindricalRadialSense::Inward,
    ] {
        let rails = match sense {
            CylindricalRadialSense::Outward => base.rails,
            CylindricalRadialSense::Inward => [base.rails[1], base.rails[0]],
        };
        let query = CylindricalSliceQuery {
            rails,
            sense,
            direction: [0., 1.],
            ..base
        };
        let reference = certify_cylindrical_slice_direction(query, || false).unwrap();
        reference
            .require_common_radial_interval(query, &stored, [0.05; 2], || false)
            .unwrap();
        assert!(matches!(
            reference.require_common_radial_interval(query, &stored, [0.1; 2], || false),
            Err(CylindricalSliceRefusal::RadialCoverMargin {
                radius_index: 1,
                ..
            })
        ));
        let altered = CylindricalSliceQuery {
            depth: [-0.25, 1.5],
            ..query
        };
        assert!(matches!(
            reference.require_common_radial_interval(altered, &stored, [0.05; 2], || false),
            Err(CylindricalSliceRefusal::RadialCoverSource { .. })
        ));
    }
}
