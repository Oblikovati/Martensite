use super::*;
fn square() -> [[f64; 3]; 4] {
    [[0., 0., 0.], [1., 0., 0.], [1., 1., 0.], [0., 1., 0.]]
}
fn input(corners: [[f64; 3]; 4]) -> BilinearPatchInput {
    BilinearPatchInput::new(corners, [[0., 1.]; 2], [1.; 4]).unwrap()
}
fn proof(corners: [[f64; 3]; 4]) -> BilinearCertificate {
    match certify_bilinear_patch(input(corners)) {
        BilinearExclusion::Certified(proof) => proof,
        other => panic!("expected strict bilinear projection, got {other:?}"),
    }
}
#[test]
fn convex_planar_and_nonplanar_patches_have_owned_certificates() {
    let mut corners = square();
    let planar = proof(corners);
    corners[2][2] = 3.;
    let nonplanar = proof(corners);
    assert_eq!(planar.projection(), CoordinateProjection::Xy);
    assert_eq!(nonplanar.projection(), CoordinateProjection::Xy);
    assert_ne!(planar.input().corners(), nonplanar.input().corners());
    assert_eq!(nonplanar.orientation(), OrientationSign::Positive);
}
#[test]
fn all_boundary_reindexings_preserve_the_appropriate_sign() {
    let corners = square();
    for offset in 0..4 {
        let forward = std::array::from_fn(|i| corners[(offset + i) % 4]);
        let reverse = std::array::from_fn(|i| corners[(offset + 4 - i) % 4]);
        assert_eq!(proof(forward).orientation(), OrientationSign::Positive);
        assert_eq!(proof(reverse).orientation(), OrientationSign::Negative);
    }
    assert!(matches!(
        certify_bilinear_patch(input([corners[0], corners[2], corners[1], corners[3]])),
        BilinearExclusion::Inconclusive {
            reason: BilinearInconclusive::NoStrictProjection,
            ..
        }
    ));
}
#[test]
fn all_coordinate_planes_are_available() {
    for (axis, expected) in [
        (2, CoordinateProjection::Xy),
        (1, CoordinateProjection::Xz),
        (0, CoordinateProjection::Yz),
    ] {
        let corners = square().map(|p| match axis {
            2 => p,
            1 => [p[0], 0., p[1]],
            _ => [0., p[0], p[1]],
        });
        assert_eq!(proof(corners).projection(), expected);
    }
}
#[test]
fn exact_signs_survive_underflow_and_overflow_in_rounded_determinants() {
    for width in [f64::from_bits(1), f64::MAX] {
        let corners = [
            [0., 0., 0.],
            [width, 0., 0.],
            [width, width, 0.],
            [0., width, 0.],
        ];
        assert_eq!(proof(corners).orientation(), OrientationSign::Positive);
    }
}
#[test]
fn concavity_and_zero_jacobians_are_inconclusive_not_intersection_results() {
    for corner in [[0.25, 0.25, 0.], [1., 0., 0.], [0., 0., 0.]] {
        let mut corners = square();
        corners[2] = corner;
        assert!(matches!(
            certify_bilinear_patch(input(corners)),
            BilinearExclusion::Inconclusive {
                reason: BilinearInconclusive::NoStrictProjection,
                ..
            }
        ));
    }
}
#[test]
fn constant_positive_weights_cancel_but_unequal_weights_are_not_admitted() {
    let uniform = BilinearPatchInput::new(square(), [[2., 5.], [-4., 3.]], [7.; 4]).unwrap();
    assert!(matches!(
        certify_bilinear_patch(uniform),
        BilinearExclusion::Certified(_)
    ));
    let unequal = BilinearPatchInput::new(square(), [[0., 1.]; 2], [1., 2., 1., 1.]).unwrap();
    assert!(matches!(
        certify_bilinear_patch(unequal),
        BilinearExclusion::Inconclusive {
            reason: BilinearInconclusive::NonConstantWeights,
            ..
        }
    ));
}
#[test]
fn unsupported_spans_remain_distinct_from_invalid_domains() {
    for domain in [
        [[-f64::MAX, f64::MAX], [0., 1.]],
        [[0., f64::from_bits(1)], [0., 1.]],
    ] {
        let supplied = BilinearPatchInput::new(square(), domain, [1.; 4]).unwrap();
        assert!(matches!(
            certify_bilinear_patch(supplied),
            BilinearExclusion::Inconclusive {
                reason: BilinearInconclusive::UnrepresentableSpan { direction: 0 },
                ..
            }
        ));
    }
}
#[test]
fn invalid_parameter_domains_are_errors() {
    for interval in [[1., 1.], [2., 1.], [0., f64::INFINITY], [f64::NAN, 1.]] {
        assert!(matches!(
            BilinearPatchInput::new(square(), [interval, [0., 1.]], [1.; 4]),
            Err(BilinearInputError::InvalidDomain { direction: 0, .. })
        ));
    }
}
#[test]
fn invalid_coordinates_and_weights_are_errors() {
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut corners = square();
        corners[2][2] = invalid;
        assert!(matches!(
            BilinearPatchInput::new(corners, [[0., 1.]; 2], [1.; 4]),
            Err(BilinearInputError::NonFiniteCorner(_))
        ));
    }
    for invalid in [0., -1., f64::NAN, f64::INFINITY] {
        assert!(matches!(
            BilinearPatchInput::new(square(), [[0., 1.]; 2], [invalid; 4]),
            Err(BilinearInputError::InvalidWeight { corner: 0, .. })
        ));
    }
}
#[test]
fn representation_binding_is_bitwise_and_does_not_alias_caller_arrays() {
    let mut corners = square();
    let certified = proof(corners);
    corners[0][0] = -0.;
    assert!(!certified.input().same_representation(&input(corners)));
    assert_eq!(certified.input().corners()[0][0].to_bits(), 0f64.to_bits());
    assert_eq!(certified.input().domain(), [[0., 1.]; 2]);
    assert_eq!(certified.input().weights(), [1.; 4]);
    assert!(certified.input().same_representation(&input(square())));
}
#[test]
fn all_published_certificate_types_are_send_and_sync() {
    fn requires_owned_traits<T: Send + Sync>() {}
    requires_owned_traits::<BilinearPatchInput>();
    requires_owned_traits::<BilinearCertificate>();
    requires_owned_traits::<BilinearExclusion>();
    requires_owned_traits::<BilinearInputError>();
    requires_owned_traits::<BilinearInconclusive>();
    requires_owned_traits::<CoordinateProjection>();
}
#[test]
fn invalid_numeric_messages_name_the_value_and_required_shape() {
    let weight = BilinearPatchInput::new(square(), [[0., 1.]; 2], [0.; 4])
        .unwrap_err()
        .to_string();
    assert!(weight.contains("corner 0 weight 0"));
    assert!(weight.contains("positive finite"));
    let domain = BilinearPatchInput::new(square(), [[2., 1.], [0., 1.]], [1.; 4])
        .unwrap_err()
        .to_string();
    assert!(domain.contains("direction 0 interval [2.0, 1.0]"));
    assert!(domain.contains("finite increasing"));
}
