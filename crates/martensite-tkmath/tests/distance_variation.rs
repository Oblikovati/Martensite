// SPDX-License-Identifier: GPL-2.0-only
use martensite_tkmath::predicates::{
    DistanceJetCertificate, certify_distance_stationary_jet as certify,
};
use std::cmp::Ordering;

#[test]
fn zero_second_variation_remains_unresolved() {
    assert_eq!(
        certify([0.; 3], [1., 0., 0.], [0., 1., 0.], [-1., 0., 0.], 0.),
        Ok(DistanceJetCertificate::WithinResolution)
    );
}

#[test]
fn positive_subnormal_second_variation_has_an_exact_sign() {
    assert_eq!(
        certify(
            [0.; 3],
            [1., 0., 0.],
            [0., 1., f64::MIN_POSITIVE],
            [-1., 0., 0.],
            0.
        ),
        Ok(DistanceJetCertificate::Resolved(Ordering::Greater))
    );
}

#[test]
fn negative_subnormal_second_variation_has_an_exact_sign() {
    assert_eq!(
        certify(
            [0.; 3],
            [1., 0., f64::MIN_POSITIVE],
            [0., 1., 0.],
            [-1., 0., -f64::MIN_POSITIVE],
            0.
        ),
        Ok(DistanceJetCertificate::Resolved(Ordering::Less))
    );
}

#[test]
fn exact_jet_refuses_a_false_rounded_conditioning_certificate() {
    // #406: B=5*2^50+1; b²-a*(a-16)=64 exactly, while rounded subtraction gives 2^58.
    let base = 5. * 2_f64.powi(53);
    let (a, b) = (base + 16., base + 8.);
    let jet = ([16., 0., 0.], [a, 0., 0.], [0., b, 0.], [-a, 0., 0.]);
    assert_eq!(
        certify(jet.0, jet.1, jet.2, jet.3, 1e-7),
        Ok(DistanceJetCertificate::WithinResolution)
    );
}

#[test]
fn exact_jet_detects_a_normal_residual_lost_by_rounded_subtraction() {
    let coordinate = 2_f64.powi(54);
    assert_eq!(
        certify(
            [1., 0., 0.],
            [coordinate, coordinate, 0.],
            [1., -1., 0.],
            [0.; 3],
            1e-7
        ),
        Ok(DistanceJetCertificate::Nonstationary)
    );
}

#[test]
fn zero_tangent_is_explicitly_singular() {
    assert_eq!(
        certify([0.; 3], [0.; 3], [0.; 3], [0.; 3], 1e-7),
        Ok(DistanceJetCertificate::SingularTangent)
    );
}

#[test]
fn exact_jet_rejects_nonfinite_coordinate_with_its_index() {
    let failure = certify([0.; 3], [0.; 3], [0., 1., 0.], [0., f64::NAN, 0.], 1e-7).unwrap_err();
    assert_eq!((failure.point, failure.axis), (3, 1));
    assert!(failure.value.is_nan());
    let failure = certify([0.; 3], [0.; 3], [0., 1., 0.], [0.; 3], f64::INFINITY).unwrap_err();
    assert_eq!((failure.point, failure.axis), (4, 0));
}

#[test]
fn signed_query_displacement_scales_define_the_same_norm_bound() {
    let positive = certify(
        [2., 0., 0.],
        [1., 0., 0.],
        [0., 1., 0.],
        [-1., 0., 0.],
        1e-7,
    );
    let negative = certify(
        [2., 0., 0.],
        [1., 0., 0.],
        [0., 1., 0.],
        [-1., 0., 0.],
        -1e-7,
    );
    assert_eq!(
        positive,
        Ok(DistanceJetCertificate::Resolved(Ordering::Greater))
    );
    assert_eq!(positive, negative);
}

#[test]
fn second_variation_band_is_closed_at_its_exact_boundary() {
    let below = f64::from_bits(1_f64.to_bits() - 1);
    assert_eq!(
        certify([1., 0., 0.], [1., 0., 0.], [0., 1., 0.], [-1., 0., 0.], 1.),
        Ok(DistanceJetCertificate::WithinResolution)
    );
    assert_eq!(
        certify(
            [1., 0., 0.],
            [1., 0., 0.],
            [0., 1., 0.],
            [-1., 0., 0.],
            below
        ),
        Ok(DistanceJetCertificate::Resolved(Ordering::Greater))
    );
}

#[test]
fn normal_residual_band_is_closed_at_its_exact_boundary() {
    let below = f64::from_bits(1_f64.to_bits() - 1);
    assert_eq!(
        certify([0.; 3], [1., 0., 0.], [1., 0., 0.], [0.; 3], 1.),
        Ok(DistanceJetCertificate::Resolved(Ordering::Greater))
    );
    assert_eq!(
        certify([0.; 3], [1., 0., 0.], [1., 0., 0.], [0.; 3], below),
        Ok(DistanceJetCertificate::Nonstationary)
    );
}
