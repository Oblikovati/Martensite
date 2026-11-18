// SPDX-License-Identifier: GPL-2.0-only
//! Independent dyadic translation and actual binary64 evaluation controls.
use super::*;
fn frame() -> CertifiedFrameTransport {
    certify_frame_transport(FrameTransportInput {
        axis: [0., 0., 1.],
        radial: [1., 0., 0.],
        stored: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
    })
    .unwrap()
}
fn domain() -> FrameSpatialInput {
    FrameSpatialInput {
        origin: [0.; 3],
        translation: [0.; 3],
        point_norm: 12.,
        spatial_limit: 1e-5,
    }
}
#[test]
fn identity_keeps_the_original_allocation_exactly() {
    let proof = frame().spatial_budget(domain()).unwrap();
    assert_eq!(proof.placement_error_bound(), 0.);
    assert_eq!(proof.evaluation_error_bound(), 0.);
    assert_eq!(proof.canonical_allowance(), domain().spatial_limit);
    assert_eq!(proof.world_error_bound().unwrap(), domain().spatial_limit);
}
#[test]
fn signed_permutation_without_translation_is_evaluated_exactly() {
    let proof = certify_frame_transport(FrameTransportInput {
        axis: [1., 0., 0.],
        radial: [0., 0., -1.],
        stored: [[0., 0., 1.], [0., 1., 0.], [-1., 0., 0.]],
    })
    .unwrap()
    .spatial_budget(domain())
    .unwrap();
    assert_eq!(proof.placement_error_bound(), 0.);
    assert_eq!(proof.canonical_allowance(), domain().spatial_limit);
}
#[test]
fn translation_difference_uses_the_independent_three_four_five_norm() {
    let input = FrameSpatialInput {
        translation: [3., 4., 0.],
        spatial_limit: 8.,
        ..domain()
    };
    let proof = frame().spatial_budget(input).unwrap();
    assert_eq!(proof.translation_error_bound(), 5.);
    assert!(proof.placement_error_bound() > 5.);
    assert!(proof.canonical_allowance() < 3.);
    assert!(proof.world_error_bound().unwrap() <= input.spatial_limit);
}
#[test]
fn equal_large_origins_still_consume_evaluation_error() {
    let input = FrameSpatialInput {
        origin: [1e16; 3],
        translation: [1e16; 3],
        ..domain()
    };
    assert!(matches!(frame().spatial_budget(input),
        Err(FrameTransportError::SpatialLimit { requested: 1e-5, placement_upper }) if placement_upper >= 6.));
}
#[test]
fn zero_point_domain_does_not_invent_translation_rounding() {
    let input = FrameSpatialInput {
        origin: [1e16; 3],
        translation: [1e16; 3],
        point_norm: 0.,
        ..domain()
    };
    assert_eq!(
        frame()
            .spatial_budget(input)
            .unwrap()
            .placement_error_bound(),
        0.
    );
}
#[test]
fn smallest_positive_limit_survives_an_exact_identity() {
    let input = FrameSpatialInput {
        spatial_limit: f64::from_bits(1),
        ..domain()
    };
    assert_eq!(
        frame().spatial_budget(input).unwrap().canonical_allowance(),
        input.spatial_limit
    );
}
#[test]
fn nonpositive_or_nonfinite_domain_scalars_keep_the_rejected_values() {
    for point_norm in [-1., f64::INFINITY, f64::NAN] {
        assert!(matches!(
            frame().spatial_budget(FrameSpatialInput {
                point_norm,
                ..domain()
            }),
            Err(FrameTransportError::SpatialInput { .. })
        ));
    }
    for spatial_limit in [0., -1., f64::INFINITY, f64::NAN] {
        assert!(matches!(
            frame().spatial_budget(FrameSpatialInput {
                spatial_limit,
                ..domain()
            }),
            Err(FrameTransportError::SpatialInput { .. })
        ));
    }
}
#[test]
fn origin_admission_identifies_the_coordinate() {
    let input = FrameSpatialInput {
        origin: [0., f64::INFINITY, 0.],
        ..domain()
    };
    assert!(matches!(
        frame().spatial_budget(input),
        Err(FrameTransportError::NonFinite {
            field: FrameTransportField::Origin,
            index: [0, 1],
            ..
        })
    ));
}
#[test]
fn translation_admission_identifies_the_coordinate() {
    let input = FrameSpatialInput {
        translation: [0., 0., f64::NAN],
        ..domain()
    };
    assert!(matches!(
        frame().spatial_budget(input),
        Err(FrameTransportError::NonFinite {
            field: FrameTransportField::Translation,
            index: [0, 2],
            ..
        })
    ));
}
fn oblique() -> CertifiedFrameTransport {
    let (x, y, z) = (42_f64.sqrt(), 3_f64.sqrt(), 14_f64.sqrt());
    certify_frame_transport(FrameTransportInput {
        axis: [1., 2., 3.],
        radial: [3., -1., 2.],
        stored: [
            [5. / x, 1. / y, 1. / z],
            [-4. / x, 1. / y, 2. / z],
            [1. / x, -1. / y, 3. / z],
        ],
    })
    .unwrap()
}
#[test]
fn oblique_allocation_is_positive_and_world_bound_stays_inside_the_requested_limit() {
    let proof = oblique().spatial_budget(domain()).unwrap();
    assert!(proof.placement_error_bound() > 0.);
    assert!(proof.canonical_allowance() > 0.);
    assert!(proof.canonical_allowance() < domain().spatial_limit);
    assert!(proof.world_error_bound().unwrap() <= domain().spatial_limit);
}
#[test]
fn exact_arithmetic_bounds_real_evaluation_errors_over_the_dyadic_cube() {
    let proof = oblique()
        .spatial_budget(FrameSpatialInput {
            origin: [3., -4., 5.],
            translation: [3., -4., 5.],
            point_norm: 2.,
            ..domain()
        })
        .unwrap();
    for ordinal in 0..125 {
        let coordinates = [ordinal / 25, ordinal / 5 % 5, ordinal % 5];
        let point = coordinates.map(|index| (index as f64 - 2.) / 2.);
        assert_evaluation_error(&proof, point);
    }
}
fn assert_evaluation_error(proof: &CertifiedFrameSpatialBudget, point: [f64; 3]) {
    let matrix = proof.frame().input().stored;
    let mut squared = ExactRatio::from_finite(0.);
    for (row, translation) in matrix.into_iter().zip(proof.input().translation) {
        let actual = row[0] * point[0] + row[1] * point[1] + row[2] * point[2] + translation;
        let ideal = arithmetic::dot(&arithmetic::vector(row), &arithmetic::vector(point))
            .add(&ExactRatio::from_finite(translation));
        let difference = ExactRatio::from_finite(actual).subtract(&ideal);
        squared = squared.add(&difference.multiply(&difference));
    }
    let upper = ExactRatio::from_finite(proof.evaluation_error_bound());
    assert!(squared <= upper.multiply(&upper));
}
#[test]
fn checked_native_rigid_transform_matches_the_certified_evaluation_order() {
    let proof = oblique()
        .spatial_budget(FrameSpatialInput {
            origin: [3., -4., 5.],
            translation: [3., -4., 5.],
            ..domain()
        })
        .unwrap();
    let transform = native_transform(&proof);
    assert_eq!(transform.scale_factor(), 1.);
    for ordinal in 0..125 {
        let point =
            [ordinal / 25, ordinal / 5 % 5, ordinal % 5].map(|index| (index as f64 - 2.) / 2.);
        assert_native_evaluation(&proof, transform, point);
        assert_evaluation_error(&proof, point);
    }
}
fn assert_native_evaluation(
    proof: &CertifiedFrameSpatialBudget,
    transform: crate::gp::Transform3,
    point: [f64; 3],
) {
    let [x, y, z] = point;
    let actual = transform.apply_coordinates(crate::gp::Xyz::new(x, y, z));
    let matrix = proof.frame().input().stored;
    let expected: [f64; 3] = std::array::from_fn(|i| {
        matrix[i][0] * point[0]
            + matrix[i][1] * point[1]
            + matrix[i][2] * point[2]
            + proof.input().translation[i]
    });
    assert_eq!(
        actual.coordinates().map(f64::to_bits),
        expected.map(f64::to_bits)
    );
}
fn native_transform(proof: &CertifiedFrameSpatialBudget) -> crate::gp::Transform3 {
    let rows = std::array::from_fn(|i| {
        let [a, b, c] = proof.frame().input().stored[i];
        [a, b, c, proof.input().translation[i]]
    });
    crate::gp::Transform3::from_stored_rigid_coefficients(rows).unwrap()
}
#[test]
fn spatial_diagnostics_state_the_rejected_values_and_expected_scalar_shape() {
    let error = frame()
        .spatial_budget(FrameSpatialInput {
            point_norm: -2.,
            ..domain()
        })
        .unwrap_err();
    let message = error.to_string();
    assert!(message.contains("-2.0"));
    assert!(message.contains("finite point_norm >= 0"));
    assert!(message.contains("finite spatial_limit > 0"));
}
#[test]
fn tiny_products_are_not_silently_rounded_to_zero_error() {
    let input = FrameSpatialInput {
        point_norm: f64::from_bits(1),
        spatial_limit: 1.,
        ..domain()
    };
    assert!(
        oblique()
            .spatial_budget(input)
            .unwrap()
            .evaluation_error_bound()
            > 0.
    );
}
#[test]
fn evaluation_overflow_refuses_instead_of_publishing_infinite_error() {
    let input = FrameSpatialInput {
        origin: [f64::MAX; 3],
        translation: [f64::MAX; 3],
        point_norm: f64::MAX,
        spatial_limit: 1.,
    };
    assert!(matches!(
        frame().spatial_budget(input),
        Err(FrameTransportError::Unrepresentable(_))
    ));
}
#[test]
fn allocation_certificate_owns_its_frame_and_domain_and_is_send_sync() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<FrameSpatialInput>();
    send_sync::<CertifiedFrameSpatialBudget>();
    let mut input = domain();
    let expected = oblique().spatial_budget(input).unwrap();
    input.translation[0] = 10.;
    assert_eq!(expected.input(), domain());
    assert_ne!(expected.input(), input);
    let workers: Vec<_> = (0..4)
        .map(|_| std::thread::spawn(|| oblique().spatial_budget(domain()).unwrap()))
        .collect();
    for worker in workers {
        assert_eq!(worker.join().unwrap(), expected);
    }
}
