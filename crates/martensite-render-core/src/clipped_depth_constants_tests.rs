use super::*;
fn identity() -> [f32; 16] {
    let mut matrix = [0.; 16];
    for index in [0, 5, 10, 15] {
        matrix[index] = 1.;
    }
    matrix
}
#[test]
fn identity_and_camera_translation_match_independent_homogeneous_equations() {
    let mut view = identity();
    view[12] = 4.;
    view[13] = -2.;
    view[14] = 8.;
    assert_eq!(
        clipped_depth_reference_equation([1., 2., 3., 5.], identity(), view, true),
        Ok([1., 2., 3., -19.])
    );
    assert_eq!(
        clipped_depth_reference_equation([1., 2., 3., 5.], identity(), identity(), true),
        Ok([1., 2., 3., 5.])
    );
}
#[test]
fn exact_quarter_rotation_and_orthographic_scale_preserve_the_same_halfspace() {
    let mut view = identity();
    view[0] = 0.;
    view[1] = 1.;
    view[4] = -1.;
    view[5] = 0.;
    let mut projection = identity();
    projection[0] = 2.;
    projection[5] = 4.;
    projection[10] = 0.5;
    let clip = clipped_depth_reference_equation([1., 0., 0., -1.], projection, view, true).unwrap();
    assert_eq!(clip, [0., 0.25, 0., -1.]);
    assert_eq!(row_product(clip, projection).unwrap(), [0., 1., 0., -1.]);
}
#[test]
fn missing_camera_provenance_and_unsupported_projection_never_infer_a_route() {
    assert_eq!(
        clipped_depth_reference_equation([1.; 4], identity(), identity(), false),
        Err(ClipConstantFailure::CameraSource)
    );
    let mut projection = identity();
    projection[11] = -1.;
    projection[15] = 0.;
    assert_eq!(
        clipped_depth_reference_equation([1.; 4], projection, identity(), true),
        Err(ClipConstantFailure::ProjectionDomain)
    );
}
#[test]
fn singular_nonfinite_and_arithmetic_overflow_refuse() {
    let mut projection = identity();
    projection[0] = 0.;
    assert_eq!(
        orthographic_inverse(projection),
        Err(ClipConstantFailure::ProjectionDomain)
    );
    assert_eq!(
        clipped_depth_reference_equation([f32::NAN; 4], identity(), identity(), true),
        Err(ClipConstantFailure::Nonfinite)
    );
    assert_eq!(
        row_product([f32::MAX; 4], [f32::MAX; 16]),
        Err(ClipConstantFailure::Nonfinite)
    );
}

#[test]
fn retained_camera_fixture_derives_the_reviewed_clip_equation_without_a_captured_constant_path() {
    let projection = [
        1008104717, 0, 0, 0, 0, 1008104717, 0, 0, 0, 0, 3154577461, 0, 2147483648, 2147483648,
        3221225427, 1065353216,
    ]
    .map(f32::from_bits);
    let view = [
        1064603886, 3182615351, 1049660521, 0, 838860798, 3212087534, 3197587053, 0, 1050103404,
        1049660521, 3211371673, 0, 1111371140, 3270683636, 3279280596, 1065353216,
    ]
    .map(f32::from_bits);
    let equation =
        clipped_depth_reference_equation([0., 0., 1., 0.], projection, view, true).unwrap();
    assert_eq!(
        equation.map(f32::to_bits),
        [1107343597, 1106637382, 1121810878, 1099589272]
    );
}
#[test]
fn shared_incidence_camera_validation_never_claims_a_rounded_matrix_is_its_own_inverse() {
    let mut rounded = identity();
    rounded[0] = 0.70710677;
    rounded[1] = 0.70710677;
    rounded[4] = -0.70710677;
    rounded[5] = 0.70710677;
    assert!(require_incidence_camera(identity(), rounded, true).is_ok());
    assert!(require_incidence_camera(identity(), rounded, false).is_err());
    let mut perspective = identity();
    perspective[11] = -1.;
    assert!(require_incidence_camera(perspective, rounded, true).is_err());
    rounded[0] = f32::NAN;
    assert!(require_incidence_camera(identity(), rounded, true).is_err());
}
