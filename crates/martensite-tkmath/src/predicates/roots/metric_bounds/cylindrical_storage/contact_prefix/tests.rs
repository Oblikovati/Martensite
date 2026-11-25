// SPDX-License-Identifier: GPL-2.0-only
use super::super::super::{CylindricalTrimQuery, MetricBezier, certify_cylindrical_trim};
use super::super::{CylindricalRadialSweepQuery, certify_cylindrical_radial_sweep};
use super::*;
use CylindricalRadialSense::{Inward, Outward};

fn rails(first: f64, far: f64) -> [[[f64; 3]; 2]; 2] {
    [first, far].map(|x| [[x, 0., 3.], [x, 0., 4.]])
}

fn query(
    poles: &[[[f64; 3]; 2]; 2],
    radius: f64,
    sense: CylindricalRadialSense,
    contact_allowance: f64,
) -> CylindricalContactPrefixQuery<'_> {
    let [curve, far_rail] = poles.each_ref().map(|points| MetricBezier {
        poles: points,
        weights: None,
    });
    CylindricalContactPrefixQuery {
        seam: CylindricalTrimQuery {
            curve,
            pcurve: &[[0., 3.], [0., 4.]],
            radius,
            spatial_limit: (poles[0][0][0] - radius).abs(),
            options: Default::default(),
        },
        far_rail,
        sense,
        contact_allowance,
        max_normalized_depth: 1.,
        options: Default::default(),
    }
}

fn sweep_query(query: CylindricalContactPrefixQuery<'_>) -> CylindricalRadialSweepQuery<'_> {
    CylindricalRadialSweepQuery {
        rails: query.rails(),
        axis_origin: [0.; 2],
        sense: query.sense,
        minimum: 0.,
        options: Default::default(),
    }
}

fn evidence(
    query: CylindricalContactPrefixQuery<'_>,
) -> (CertifiedCylindricalRadialSweep, CertifiedCylindricalTrim) {
    (
        certify_cylindrical_radial_sweep(sweep_query(query), || false).unwrap(),
        certify_cylindrical_trim(query.seam, || false).unwrap(),
    )
}

fn prefix(query: CylindricalContactPrefixQuery<'_>) -> CertifiedCylindricalContactPrefix {
    let (sweep, trim) = evidence(query);
    certify_cylindrical_contact_prefix(query, &sweep, &trim, || false).unwrap()
}

fn refusal(
    query: CylindricalContactPrefixQuery<'_>,
    sweep: &CertifiedCylindricalRadialSweep,
    trim: &CertifiedCylindricalTrim,
) -> Failure {
    certify_cylindrical_contact_prefix(query, sweep, trim, || false)
        .unwrap_err()
        .cause
}

#[test]
fn exact_zero_seam_uses_square_contact_gap_in_both_senses() {
    let outward = rails(4., 6.);
    let inward = rails(4., 2.);
    for (poles, sense, expected, derivative) in [
        (&outward, Outward, 9. / 16., 16.),
        (&inward, Inward, 7. / 8., 8.),
    ] {
        let source = query(poles, 4., sense, 1.);
        let proof = prefix(source);
        assert_eq!(proof.normalized_depth_upper(), expected);
        assert_eq!(proof.signed_squared_radius_derivative_lower(), derivative);
        assert_eq!(proof.seam_xy_error(), 0.);
        assert!(proof.matches_source(source));
        assert!(proof.operations() > 0);
    }
}

#[test]
fn unequal_contact_and_seam_errors_use_distinct_strict_margins() {
    let outward = rails(3.5, 7.5);
    let inward = rails(4.5, 2.5);
    for (poles, sense, expected) in [
        (&outward, Outward, (4.25_f64 * 4.25 - 3.5 * 3.5) / 28.),
        (&inward, Inward, (4.5_f64 * 4.5 - 3.75 * 3.75) / 10.),
    ] {
        let proof = prefix(query(poles, 4., sense, 0.25));
        assert_eq!(proof.seam_xy_error(), 0.5);
        assert!((expected..=expected.next_up()).contains(&proof.normalized_depth_upper()));
    }
}

#[test]
fn uses_achieved_xy_evidence_instead_of_spatial_budget_or_height_error() {
    let mut poles = rails(4., 6.);
    poles[0][0][2] = 3.25;
    poles[0][1][2] = 4.25;
    let mut source = query(&poles, 4., Outward, 1.);
    source.seam.spatial_limit = 0.5;
    let proof = prefix(source);
    assert_eq!(proof.seam_xy_error(), 0.);
    assert_eq!(proof.normalized_depth_upper(), 9. / 16.);
}

#[test]
fn factored_gap_retains_contact_smaller_than_a_radius_ulp() {
    let radius = martensite_cmath::powi(2., 40);
    let contact = martensite_cmath::powi(2., -30);
    let poles = rails(radius, radius + 1.);
    assert_eq!(radius + contact, radius);
    let proof = prefix(query(&poles, radius, Outward, contact));
    // The exact depth is c + c²/(2R), lying between c and the next binary64.
    assert_eq!(proof.normalized_depth_upper(), contact.next_up());
}

#[test]
fn directed_radius_checks_reject_axis_contact_and_subgrid_margin() {
    let regular = rails(0.125, 0.5);
    let source = query(&regular, 0.125, Outward, 0.125);
    let (sweep, trim) = evidence(source);
    assert_eq!(
        refusal(source, &sweep, &trim),
        Failure::RadiusAllowance { seam_error: 0. }
    );
    let mut source = query(&regular, 0.125, Outward, 0.125f64.next_down());
    source.options.fractional_bits = 8;
    assert_eq!(
        refusal(source, &sweep, &trim),
        Failure::RadiusAllowance { seam_error: 0. }
    );
    let wide = rails(0.25, 0.5);
    let source = query(&wide, 0.125, Outward, 0.);
    let (sweep, trim) = evidence(source);
    assert_eq!(
        refusal(source, &sweep, &trim),
        Failure::RadiusAllowance { seam_error: 0.125 }
    );
}

#[test]
fn zero_prefix_completes_and_equal_limit_refuses() {
    let poles = rails(4., 6.);
    let mut source = query(&poles, 4., Outward, 0.);
    assert_eq!(prefix(source).normalized_depth_upper(), 0.);
    source.contact_allowance = 1.;
    source.max_normalized_depth = 9. / 16.;
    let (sweep, trim) = evidence(source);
    assert_eq!(
        refusal(source, &sweep, &trim),
        Failure::Uncertified { upper: 9. / 16. }
    );
    source.max_normalized_depth = source.max_normalized_depth.next_up();
    assert_eq!(prefix(source).normalized_depth_upper(), 9. / 16.);
}

#[test]
fn bound_at_full_depth_is_incomplete() {
    let poles = rails(4., 5.125);
    let source = query(&poles, 4., Outward, 1.);
    let (sweep, trim) = evidence(source);
    assert_eq!(
        refusal(source, &sweep, &trim),
        Failure::Uncertified { upper: 1. }
    );
}

#[test]
fn rational_zero_depth_rail_uses_its_actual_weights_and_trim() {
    let poles = rails(4., 6.);
    let mut source = query(&poles, 4., Outward, 1.);
    source.seam.curve.weights = Some(&[1., 2.]);
    source.far_rail.weights = Some(&[1., 2.]);
    source.seam.spatial_limit = 1.;
    let proof = prefix(source);
    assert_eq!(proof.normalized_depth_upper(), 9. / 16.);
    assert!(proof.matches_source(source));
    source.seam.curve.weights = Some(&[2., 4.]);
    source.far_rail.weights = Some(&[2., 4.]);
    assert!(!proof.matches_source(source));
}

#[test]
fn scale_requires_new_proofs_preserving_normalized_depth() {
    let base = rails(4., 6.);
    let scaled = rails(8., 12.);
    let original = query(&base, 4., Outward, 1.);
    let second = query(&scaled, 8., Outward, 2.);
    let (sweep, trim) = evidence(original);
    let (scaled_sweep, _) = evidence(second);
    assert_eq!(refusal(second, &sweep, &trim), Failure::SweepSource);
    assert_eq!(refusal(second, &scaled_sweep, &trim), Failure::TrimSource);
    let proof = prefix(original);
    assert!(!proof.matches_source(second));
    assert_eq!(
        prefix(second).normalized_depth_upper(),
        proof.normalized_depth_upper()
    );
}

#[test]
fn altered_axis_sense_and_raw_weight_representation_are_refused() {
    let poles = rails(4., 6.);
    let source = query(&poles, 4., Outward, 1.);
    let (sweep, trim) = evidence(source);
    let mut changed = source;
    changed.sense = Inward;
    assert_eq!(refusal(changed, &sweep, &trim), Failure::SweepSource);
    changed = source;
    changed.far_rail.weights = Some(&[1., 1.]);
    assert_eq!(refusal(changed, &sweep, &trim), Failure::SweepSource);
    let mut shifted = sweep_query(source);
    shifted.axis_origin = [0.25, 0.];
    let shifted = certify_cylindrical_radial_sweep(shifted, || false).unwrap();
    assert_eq!(refusal(source, &shifted, &trim), Failure::SweepSource);
}

#[test]
fn altered_pcurve_radius_seam_policy_and_contact_receipt_are_refused() {
    let poles = rails(4., 6.);
    let source = query(&poles, 4., Outward, 1.);
    let (sweep, trim) = evidence(source);
    let proof = prefix(source);
    let mut changed = source;
    changed.seam.pcurve = &[[0., 3.], [0., 4.25]];
    assert_eq!(refusal(changed, &sweep, &trim), Failure::TrimSource);
    assert!(!proof.matches_source(changed));
    changed = source;
    changed.seam.radius = 4.25;
    assert_eq!(refusal(changed, &sweep, &trim), Failure::TrimSource);
    changed = source;
    changed.seam.spatial_limit = 0.25;
    assert_eq!(refusal(changed, &sweep, &trim), Failure::TrimSource);
    changed = source;
    changed.seam.options.max_operations += 1;
    assert_eq!(refusal(changed, &sweep, &trim), Failure::TrimSource);
    changed = source;
    changed.contact_allowance = 1.25;
    assert!(!proof.matches_source(changed));
}

#[test]
fn invalid_depth_work_and_cancellation_never_publish() {
    use crate::predicates::roots::certificate_arithmetic::bounded_failure::BoundedArithmeticFailure;
    let poles = rails(4., 6.);
    let source = query(&poles, 4., Outward, 1.);
    let (sweep, trim) = evidence(source);
    let mut changed = source;
    changed.max_normalized_depth = 2.;
    assert_eq!(refusal(changed, &sweep, &trim), Failure::MaximumDepth(2.));
    changed = source;
    changed.seam.radius = 0.;
    assert_eq!(refusal(changed, &sweep, &trim), Failure::Radius(0.));
    changed = source;
    changed.contact_allowance = f64::INFINITY;
    assert_eq!(
        refusal(changed, &sweep, &trim),
        Failure::ContactAllowance(f64::INFINITY)
    );
    changed = source;
    changed.options.max_operations = 1;
    assert_eq!(
        refusal(changed, &sweep, &trim),
        Failure::Arithmetic(BoundedArithmeticFailure::WorkLimit {
            completed: 1,
            limit: 1
        })
    );
    let error = certify_cylindrical_contact_prefix(source, &sweep, &trim, || true).unwrap_err();
    assert_eq!(
        error.cause,
        Failure::Arithmetic(BoundedArithmeticFailure::Cancelled)
    );
}
