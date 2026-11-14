//! Exact quarter-turn matrices and wrong-frame controls exercise the requested angular reference.
use super::*;
fn identity() -> FrameTransportInput {
    FrameTransportInput {
        axis: [0., 0., 1.],
        radial: [1., 0., 0.],
        stored: [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
    }
}
fn quarter_turn() -> FrameTransportInput {
    FrameTransportInput {
        stored: [[0., -1., 0.], [1., 0., 0.], [0., 0., 1.]],
        ..identity()
    }
}
#[test]
fn zero_rotation_keeps_the_original_certificate_and_debug_representation() {
    let before = certify_frame_transport(identity()).unwrap();
    for angle in [0., -0.] {
        let after = certify_frame_transport_at_angle(identity(), angle).unwrap();
        assert_eq!(after.unrotated_frame(), &before);
        assert_zero_budget(&before, &after);
        assert_eq!(
            format!("{:?}", after.unrotated_frame()),
            format!("{before:?}")
        );
        assert_eq!(after.rotation().operations, 0);
        assert!(!format!("{:?}", after.unrotated_frame()).contains("rotation"));
    }
}
#[test]
fn exact_quarter_turn_matrix_is_close_to_the_exact_binary64_angle_reference() {
    let proof =
        certify_frame_transport_at_angle(quarter_turn(), std::f64::consts::FRAC_PI_2).unwrap();
    assert!(proof.matrix_error_bound() > 0.);
    assert!(proof.matrix_error_bound() < 1e-14);
    let rotation = proof.rotation();
    assert_eq!(rotation.angle, std::f64::consts::FRAC_PI_2);
    assert!(rotation.cosine_bounds[0] > 0.);
    assert!(rotation.cosine_bounds[1] < 1e-15);
    assert!(rotation.sine_bounds[0] <= 1. && rotation.sine_bounds[1] >= 1.);
    assert!(rotation.operations > 0 && rotation.operations <= rotation.operation_limit);
}
#[test]
fn ignoring_the_requested_rotation_cannot_receive_a_small_error_certificate() {
    let proof = certify_frame_transport_at_angle(identity(), 1.).unwrap();
    assert!(proof.matrix_error_bound() > 0.9);
    let input = spatial_domain();
    assert!(matches!(
        proof.spatial_budget(input),
        Err(FrameTransportError::SpatialLimit { .. })
    ));
}
#[test]
fn rotated_certificate_allocates_the_original_spatial_budget() {
    let proof =
        certify_frame_transport_at_angle(quarter_turn(), std::f64::consts::FRAC_PI_2).unwrap();
    let input = spatial_domain();
    let budget = proof.spatial_budget(input).unwrap();
    assert!(budget.canonical_allowance() > 0.);
    assert!(budget.canonical_allowance() < input.spatial_limit);
    assert!(budget.world_error_bound().unwrap() <= input.spatial_limit);
    assert_eq!(budget.frame().rotation(), proof.rotation());
}
#[test]
fn nonfinite_angles_refuse_with_original_settings_and_values() {
    for angle in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let error = certify_frame_transport_at_angle(identity(), angle).unwrap_err();
        assert!(matches!(error.cause, FrameRotationFailure::InvalidAngle));
        assert_eq!(error.angle.to_bits(), angle.to_bits());
        assert_eq!(error.fractional_bits, 128);
        assert_eq!(error.operation_limit, 10_000);
    }
}
#[test]
fn unresolvable_argument_reduction_is_not_a_zero_error_rotation() {
    let error = certify_frame_transport_at_angle(identity(), 1e300).unwrap_err();
    assert!(matches!(
        error.cause,
        FrameRotationFailure::Trigonometric(_)
    ));
    assert_eq!(error.angle, 1e300);
}
#[test]
fn rotated_reference_retains_original_frame_refusals() {
    let input = FrameTransportInput {
        axis: [0.; 3],
        ..identity()
    };
    let error = certify_frame_transport_at_angle(input, 0.25).unwrap_err();
    assert!(matches!(
        error.cause,
        FrameRotationFailure::Frame(FrameTransportError::ZeroAxis { .. })
    ));
}
#[test]
fn angular_evidence_and_errors_are_send_sync() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<FrameRotationReport>();
    send_sync::<CertifiedRotatedFrameTransport>();
    send_sync::<CertifiedRotatedFrameSpatialBudget>();
    send_sync::<FrameRotationError>();
    send_sync::<FrameRotationFailure>();
}
fn oblique(angle: f64) -> FrameTransportInput {
    let frame = oblique_frame();
    let x = *frame.x_direction().xyz().coordinates();
    let y = *frame.y_direction().xyz().coordinates();
    let z = *frame.direction().xyz().coordinates();
    let (sine, cosine) = (martensite_cmath::sin(angle), martensite_cmath::cos(angle));
    let stored = std::array::from_fn(|i| {
        [
            cosine * x[i] + sine * y[i],
            cosine * y[i] - sine * x[i],
            z[i],
        ]
    });
    FrameTransportInput {
        axis: [1., 2., 3.],
        radial: [3., -1., 2.],
        stored,
    }
}
#[test]
fn oblique_reference_rotations_keep_positive_stretch_and_bind_the_requested_angle() {
    for angle in [-6.5, -0.3, 0.25, 1., std::f64::consts::PI, 8., 1e6] {
        let input = oblique(angle);
        let proof = certify_frame_transport_at_angle(input, angle).unwrap();
        assert_eq!(proof.input(), input);
        assert_eq!(proof.rotation().angle, angle);
        assert!(
            proof.matrix_error_bound() < 1e-12,
            "angle={angle} {proof:?}"
        );
        assert!(proof.stretch_bounds()[0] > 0.);
        assert!(proof.determinant_bounds()[0] > 0.);
    }
}

fn oblique_frame() -> crate::gp::Axis3 {
    use crate::gp::{Axis3, Direction3, Point3};
    Axis3::new(
        Point3::default(),
        Direction3::new(1., 2., 3.).unwrap(),
        Direction3::new(3., -1., 2.).unwrap(),
    )
    .unwrap()
}

fn assert_zero_budget(before: &CertifiedFrameTransport, after: &CertifiedRotatedFrameTransport) {
    let input = spatial_domain();
    let before = before.spatial_budget(input).unwrap();
    let after = after.spatial_budget(input).unwrap();
    assert_eq!(before.canonical_allowance(), after.canonical_allowance());
    assert_eq!(
        before.placement_error_bound(),
        after.placement_error_bound()
    );
    assert_eq!(
        before.evaluation_error_bound(),
        after.evaluation_error_bound()
    );
    assert_eq!(
        before.translation_error_bound(),
        after.translation_error_bound()
    );
    assert_eq!(before.world_error_bound(), after.world_error_bound());
}

fn spatial_domain() -> FrameSpatialInput {
    FrameSpatialInput {
        origin: [0.; 3],
        translation: [0.; 3],
        point_norm: 10.,
        spatial_limit: 1e-5,
    }
}
