use super::*;
#[test]
fn independent_dyadic_closedforms_preserve_slope_origin_and_sample() {
    for exponent in -8..9 {
        let scale = martensite_cmath::powi(f64::from(2f32), exponent) as f32;
        let window = [[0., 0., 0.25], [scale, 0., 0.5], [0., scale, 0.75]];
        let plane = framebuffer_depth_plane(window).unwrap();
        assert_eq!(plane.origin, 0.25);
        assert_eq!(plane.gradient, [0.25 / scale, 0.5 / scale]);
        assert_eq!(plane.slope, 0.5 / scale);
        assert_eq!(
            framebuffer_depth_sample(plane, [scale / 4., scale / 4.]),
            Ok(0.4375)
        );
    }
}
#[test]
fn orientation_reversal_preserves_the_exact_closedform_plane() {
    let original = [[0., 0., 0.], [4., 0., 0.25], [0., 4., 0.5]];
    let reversed = [original[0], original[2], original[1]];
    assert_eq!(
        framebuffer_depth_plane(original),
        framebuffer_depth_plane(reversed)
    );
}
#[test]
fn actual_constant_depth_is_preserved_exactly() {
    let plane =
        framebuffer_depth_plane([[523., 450., 0.5], [527., 451., 0.5], [520., 455., 0.5]]).unwrap();
    assert_eq!(plane.gradient, [0., 0.]);
    assert_eq!(plane.slope, 0.);
    assert_eq!(
        framebuffer_depth_sample(plane, [523.5625, 450.3125]),
        Ok(0.5)
    );
}
#[test]
fn degeneracy_nonfinite_and_operation_overflow_refuse() {
    assert_eq!(
        framebuffer_depth_plane([[0.; 3]; 3]),
        Err(FramebufferPlaneFailure::Degenerate)
    );
    assert_eq!(
        framebuffer_depth_plane([[f32::NAN; 3]; 3]),
        Err(FramebufferPlaneFailure::NonfiniteInput)
    );
    let huge = [[f32::MAX, 0., 0.], [-f32::MAX, 1., 0.], [0., 0., 1.]];
    assert_eq!(
        framebuffer_depth_plane(huge),
        Err(FramebufferPlaneFailure::Arithmetic)
    );
}
#[test]
fn reciprocal_overflow_and_invalid_sample_never_publish_a_plane_or_depth() {
    let tiny = f32::from_bits(1);
    assert_eq!(
        framebuffer_depth_plane([[0., 0., 0.], [tiny, 0., 1.], [0., 1., 0.]]),
        Err(FramebufferPlaneFailure::Arithmetic)
    );
    let plane = FramebufferDepthPlane {
        origin: 0.,
        gradient: [1., 1.],
        slope: 1.,
    };
    assert_eq!(
        framebuffer_depth_sample(plane, [f32::NAN, 0.]),
        Err(FramebufferPlaneFailure::Arithmetic)
    );
}
#[test]
fn a_nan_slope_component_cannot_be_hidden_by_the_other_finite_component() {
    let window = [[0., 0., 0.], [0., -1e20, -1e20], [1., 1e20, 1e20]];
    assert_eq!(
        framebuffer_depth_plane(window),
        Err(FramebufferPlaneFailure::Arithmetic)
    );
}
