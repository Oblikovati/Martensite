// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::{
    CylindricalStorageLimits, CylindricalStorageQuery, MetricBezier, certify_cylindrical_storage,
};

fn storage(radii: [f64; 2]) -> CertifiedCylindricalStorage {
    storage_with_y(radii, 0.)
}

fn storage_with_y(radii: [f64; 2], y: f64) -> CertifiedCylindricalStorage {
    let curves = radii.map(|r| [[r, y, 0.], [r, y, 1.]]);
    let query = CylindricalStorageQuery {
        curves: curves.each_ref().map(|poles| MetricBezier {
            poles,
            weights: None,
        }),
        radii,
        limits: CylindricalStorageLimits::uniform(0.125),
        options: Default::default(),
    };
    certify_cylindrical_storage(query, || false).unwrap()
}

#[test]
fn exact_addition_and_linear_depth_composition_have_zero_rounding_error() {
    let retained = storage([9., 12.]);
    let proof = certify_cylindrical_depths(10., [-1., 2.], &retained, 0.).unwrap();
    assert_eq!(proof.support_radius(), 10.);
    assert_eq!(proof.requested_depths(), [-1., 2.]);
    assert_eq!(proof.stored_radii(), [9., 12.]);
    assert_eq!(proof.radius_rounding_bounds(), [0., 0.]);
    assert_eq!(proof.normal_sweep_error_bound(), 0.);
    assert_eq!(proof.requested_limit(), 0.);
}

#[test]
fn swallowed_binary64_offset_still_consumes_the_combined_storage_budget() {
    let retained = storage([1., 1.]);
    let offset = f64::EPSILON / 4.;
    assert_eq!(1. + offset, 1.);
    let proof = certify_cylindrical_depths(1., [0., offset], &retained, offset).unwrap();
    assert_eq!(proof.radius_rounding_bounds(), [0., offset]);
    assert_eq!(proof.normal_sweep_error_bound(), offset);
    let error =
        certify_cylindrical_depths(1., [0., offset], &retained, offset.next_down()).unwrap_err();
    assert!(matches!(error, CylindricalDepthFailure::Uncertified { bound, .. } if bound == offset));
}

#[test]
fn nonpositive_exact_radii_and_reversed_depths_refuse() {
    let retained = storage([1., 2.]);
    assert!(matches!(
        certify_cylindrical_depths(1., [-1., 1.], &retained, 1.).unwrap_err(),
        CylindricalDepthFailure::NonpositiveRadius { index: 0, .. }
    ));
    assert!(matches!(
        certify_cylindrical_depths(1., [1., 0.], &retained, 1.).unwrap_err(),
        CylindricalDepthFailure::DepthOrder { .. }
    ));
}

#[test]
fn nonfinite_inputs_negative_limits_and_unrepresentable_bounds_refuse() {
    let retained = storage([1., 1.]);
    for (radius, depths, limit) in [
        (f64::NAN, [0., 1.], 1.),
        (1., [0., f64::INFINITY], 1.),
        (1., [0., 1.], -1.),
    ] {
        assert!(matches!(
            certify_cylindrical_depths(radius, depths, &retained, limit).unwrap_err(),
            CylindricalDepthFailure::Value { .. }
        ));
    }
    assert!(matches!(
        certify_cylindrical_depths(f64::MAX, [0., f64::MAX], &retained, f64::MAX).unwrap_err(),
        CylindricalDepthFailure::UnrepresentableBound { .. }
    ));
}

#[test]
fn depth_reports_and_errors_are_send_and_sync() {
    fn require<T: Send + Sync>() {}
    require::<CertifiedCylindricalDepths>();
    require::<CylindricalDepthFailure>();
}

#[test]
fn radius_rounding_and_curve_storage_share_one_combined_allowance() {
    let retained = storage_with_y([1., 1.], 0.125);
    let offset = f64::EPSILON / 4.;
    let expected = retained.normal_sweep_error_bound() + offset;
    assert_eq!(retained.normal_sweep_error_bound(), 1. / 64.);
    let proof = certify_cylindrical_depths(1., [0., offset], &retained, expected).unwrap();
    assert_eq!(proof.normal_sweep_error_bound(), expected);
    assert!(matches!(
        certify_cylindrical_depths(
            1.,
            [0., offset],
            &retained,
            retained.normal_sweep_error_bound()
        )
        .unwrap_err(),
        CylindricalDepthFailure::Uncertified { .. }
    ));
}
