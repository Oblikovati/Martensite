// SPDX-License-Identifier: GPL-2.0-only
//! Closed forms and exact dyadic probes are independent of native frame construction.
use super::*;
fn identity() -> FrameTransportInput {
    FrameTransportInput {
        axis: [0., 0., 1.],
        radial: [1., 0., 0.],
        stored: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
    }
}
#[test]
fn identity_has_exact_zero_error_and_unit_stretch_and_determinant() {
    let proof = certify_frame_transport(identity()).unwrap();
    assert_eq!(proof.matrix_error_bound(), 0.);
    assert_eq!(proof.stretch_bounds(), [1., 1.]);
    assert_eq!(proof.determinant_bounds(), [1., 1.]);
    assert_eq!(proof.gram_residual_upper(), 0.);
}
#[test]
fn exact_signed_permutation_respects_column_orientation() {
    let input = FrameTransportInput {
        axis: [1., 0., 0.],
        radial: [0., 0., -1.],
        stored: [[0., 0., 1.], [0., 1., 0.], [-1., 0., 0.]],
    };
    let proof = certify_frame_transport(input).unwrap();
    assert_eq!(proof.matrix_error_bound(), 0.);
    assert_eq!(proof.determinant_bounds(), [1., 1.]);
}
#[test]
fn oblique_closed_form_agrees_with_the_independent_integer_gram_oracle() {
    let (x, y, z) = (42_f64.sqrt(), 3_f64.sqrt(), 14_f64.sqrt());
    let stored = [
        [5. / x, 1. / y, 1. / z],
        [-4. / x, 1. / y, 2. / z],
        [1. / x, -1. / y, 3. / z],
    ];
    let input = FrameTransportInput {
        axis: [1., 2., 3.],
        radial: [3., -1., 2.],
        stored,
    };
    let proof = certify_frame_transport(input).unwrap();
    assert!(proof.matrix_error_bound() < 1e-14);
    assert!(proof.stretch_bounds()[0] > 1. - 1e-14);
    assert!(proof.stretch_bounds()[1] < 1. + 1e-14);
    assert!(proof.determinant_bounds()[0] > 1. - 1e-14);
    assert!(proof.determinant_bounds()[1] < 1. + 1e-14);
}
fn dyadic_stretch() -> CertifiedFrameTransport {
    let mut input = identity();
    input.stored = [[9. / 8., 0., 0.], [0., 1., 0.], [0., 0., 7. / 8.]];
    certify_frame_transport(input).unwrap()
}
#[test]
fn dyadic_diagonal_has_exact_determinant_and_gram_row_sum() {
    let proof = dyadic_stretch();
    assert_eq!(proof.determinant_bounds(), [63. / 64.; 2]);
    assert_eq!(proof.gram_residual_upper(), 17. / 64.);
    assert_eq!(proof.stretch_bounds()[1], 9. / 8.);
    assert!(proof.stretch_bounds()[0] <= 47_f64.sqrt() / 8.);
    assert!(proof.matrix_error_bound() >= 2_f64.sqrt() / 8.);
    assert!(proof.matrix_error_bound() < 0.177);
}
#[test]
fn extreme_reference_scales_do_not_overflow_or_underflow_normalization() {
    for scale in [f64::MAX, f64::from_bits(1)] {
        let mut input = identity();
        input.axis[2] = scale;
        input.radial[0] = scale;
        assert_eq!(
            certify_frame_transport(input).unwrap().matrix_error_bound(),
            0.
        );
    }
}
#[test]
fn zero_and_parallel_references_refuse_with_actual_vectors() {
    let mut input = identity();
    input.axis = [0.; 3];
    assert!(matches!(
        certify_frame_transport(input),
        Err(FrameTransportError::ZeroAxis { axis: [0., 0., 0.] })
    ));
    input.axis = [0., 0., 2.];
    input.radial = [0., 0., -3.];
    assert!(matches!(
        certify_frame_transport(input),
        Err(FrameTransportError::ParallelRadial {
            axis: [0., 0., 2.],
            radial: [0., 0., -3.]
        })
    ));
}
#[test]
fn reflected_stored_map_retains_its_exact_negative_determinant() {
    let mut input = identity();
    input.stored[0][0] = -1.;
    assert_eq!(
        certify_frame_transport(input).unwrap_err(),
        FrameTransportError::NonPositiveDeterminant { bounds: [-1.; 2] }
    );
}
#[test]
fn singular_stored_map_retains_its_exact_zero_determinant() {
    let mut input = identity();
    input.stored[0][0] = 0.;
    assert_eq!(
        certify_frame_transport(input).unwrap_err(),
        FrameTransportError::NonPositiveDeterminant { bounds: [0.; 2] }
    );
}
#[test]
fn shear_without_a_positive_stretch_bound_refuses_as_inconclusive() {
    let mut input = identity();
    input.stored = [[1., 2., 0.], [0., 1., 0.], [0., 0., 1.]];
    assert_eq!(
        certify_frame_transport(input).unwrap_err(),
        FrameTransportError::UncertifiedStretch {
            gram_residual_upper: 6.
        }
    );
}
#[test]
fn unrepresentable_determinant_cannot_become_an_infinite_certificate() {
    let mut input = identity();
    input.stored = [[f64::MAX, 0., 0.], [0., f64::MAX, 0.], [0., 0., f64::MAX]];
    assert_eq!(
        certify_frame_transport(input).unwrap_err(),
        FrameTransportError::Unrepresentable(FrameTransportQuantity::Determinant)
    );
}
#[test]
fn nonfinite_components_name_the_actual_field_and_index() {
    let mut input = identity();
    input.axis[1] = f64::INFINITY;
    assert!(
        matches!(certify_frame_transport(input), Err(FrameTransportError::NonFinite { field: FrameTransportField::Axis, index: [0,1], value }) if value == f64::INFINITY)
    );
    input = identity();
    input.radial[2] = f64::NAN;
    assert!(
        matches!(certify_frame_transport(input), Err(FrameTransportError::NonFinite { field: FrameTransportField::Radial, index: [0,2], value }) if value.is_nan())
    );
    input = identity();
    input.stored[2][1] = f64::NEG_INFINITY;
    assert!(
        matches!(certify_frame_transport(input), Err(FrameTransportError::NonFinite { field: FrameTransportField::StoredMatrix, index: [2,1], value }) if value == f64::NEG_INFINITY)
    );
}
#[test]
fn certificate_owns_its_binding_and_is_send_sync() {
    let mut input = identity();
    let proof = certify_frame_transport(input).unwrap();
    input.stored[0][0] = 2.;
    assert_eq!(proof.input(), identity());
    assert_ne!(proof.input(), input);
    fn send_sync<T: Send + Sync>() {}
    send_sync::<CertifiedFrameTransport>();
    send_sync::<FrameTransportError>();
    send_sync::<FrameTransportInput>();
    send_sync::<FrameTransportField>();
    send_sync::<FrameTransportQuantity>();
}
#[test]
fn display_keeps_the_refused_values_and_expected_shape() {
    let error = FrameTransportError::ParallelRadial {
        axis: [0., 0., 2.],
        radial: [0., 0., -3.],
    };
    assert!(error.to_string().contains("-3.0"));
    assert!(error.to_string().contains("nonparallel"));
}

#[test]
fn oblique_extreme_reference_scales_retain_the_same_normalized_frame() {
    let (x, y, z) = (6_f64.sqrt(), 2_f64.sqrt(), 3_f64.sqrt());
    let stored = [
        [1. / x, 1. / y, 1. / z],
        [-2. / x, 0., 1. / z],
        [1. / x, -1. / y, 1. / z],
    ];
    for scale in [f64::MAX, f64::from_bits(1)] {
        let input = FrameTransportInput {
            axis: [scale; 3],
            radial: [scale, -scale, scale],
            stored,
        };
        assert!(certify_frame_transport(input).unwrap().matrix_error_bound() < 1e-14);
    }
}

#[test]
fn dyadic_vector_corpus_satisfies_exact_squared_transport_inequalities() {
    let proof = dyadic_stretch();
    let coordinates: [f64; 5] = [-1., -0.5, 0., 0.5, 1.];
    let side = coordinates.len();
    for ordinal in 0..side.pow(3) {
        let indices = [ordinal / side / side, ordinal / side % side, ordinal % side];
        assert_squared_vector_bounds(&proof, indices.map(|index| coordinates[index]));
    }
}
fn assert_squared_vector_bounds(proof: &CertifiedFrameTransport, [x, y, z]: [f64; 3]) {
    let norm = ExactRatio::from_finite(x * x + y * y + z * z);
    let actual = ExactRatio::from_finite((81. * x * x + 64. * y * y + 49. * z * z) / 64.);
    let error = ExactRatio::from_finite((x * x + z * z) / 64.);
    let [lower, upper] = proof.stretch_bounds().map(ExactRatio::from_finite);
    let distance = ExactRatio::from_finite(proof.matrix_error_bound());
    assert!(actual >= lower.multiply(&lower).multiply(&norm));
    assert!(actual <= upper.multiply(&upper).multiply(&norm));
    assert!(error <= distance.multiply(&distance).multiply(&norm));
}
#[test]
fn concurrent_certificates_match_the_owned_sequential_reference() {
    let expected = dyadic_stretch();
    let input = expected.input();
    let workers: Vec<_> = (0..8)
        .map(|_| std::thread::spawn(move || certify_frame_transport(input).unwrap()))
        .collect();
    for worker in workers {
        assert_eq!(worker.join().unwrap(), expected);
    }
}
