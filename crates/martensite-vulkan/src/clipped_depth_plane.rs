//! Explicit framebuffer-origin f32 arithmetic; vertex-origin/Q12 replay is separate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum FramebufferPlaneFailure {
    NonfiniteInput,
    Degenerate,
    Arithmetic,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct FramebufferDepthPlane {
    pub origin: f32,
    pub gradient: [f32; 2],
    pub slope: f32,
}
fn finite<const N: usize>(values: [f32; N]) -> bool {
    values.into_iter().all(f32::is_finite)
}
fn edges(window: [[f32; 3]; 3]) -> Result<([f32; 3], [f32; 3], f32), FramebufferPlaneFailure> {
    if !window.into_iter().all(finite) {
        return Err(FramebufferPlaneFailure::NonfiniteInput);
    }
    let first = std::array::from_fn(|i| window[0][i] - window[1][i]);
    let second = std::array::from_fn(|i| window[2][i] - window[0][i]);
    let determinant = first[0] * second[1] - first[1] * second[0];
    if determinant == 0. {
        return Err(FramebufferPlaneFailure::Degenerate);
    }
    if !determinant.is_finite() {
        return Err(FramebufferPlaneFailure::Arithmetic);
    }
    let inverse = 1. / determinant;
    if !finite(first) || !finite(second) || !inverse.is_finite() {
        return Err(FramebufferPlaneFailure::Arithmetic);
    }
    Ok((first, second, inverse))
}
fn gradient(first: [f32; 3], second: [f32; 3], inverse: f32) -> [f32; 2] {
    let divided_y = [second[1] * inverse, first[1] * inverse];
    let divided_x = [second[0] * inverse, first[0] * inverse];
    [
        first[2] * divided_y[0] - second[2] * divided_y[1],
        second[2] * divided_x[1] - first[2] * divided_x[0],
    ]
}
fn maximum_slope(
    first: [f32; 3],
    second: [f32; 3],
    inverse: f32,
) -> Result<f32, FramebufferPlaneFailure> {
    let numerator = [
        second[2] * first[1] - second[1] * first[2],
        second[0] * first[2] - second[2] * first[0],
    ];
    let slope = numerator.map(|value| (value * inverse).abs());
    if !finite(numerator) || !finite(slope) {
        return Err(FramebufferPlaneFailure::Arithmetic);
    }
    Ok(slope[0].max(slope[1]))
}
pub(crate) fn framebuffer_depth_plane(
    window: [[f32; 3]; 3],
) -> Result<FramebufferDepthPlane, FramebufferPlaneFailure> {
    let (first, second, inverse) = edges(window)?;
    let gradient = gradient(first, second, inverse);
    let projected = gradient[0] * window[0][0] + gradient[1] * window[0][1];
    let origin = window[0][2] - projected;
    let slope = maximum_slope(first, second, inverse)?;
    if !finite([gradient[0], gradient[1], origin, slope]) {
        return Err(FramebufferPlaneFailure::Arithmetic);
    }
    Ok(FramebufferDepthPlane {
        origin,
        gradient,
        slope,
    })
}
pub(crate) fn framebuffer_depth_sample(
    plane: FramebufferDepthPlane,
    sample: [f32; 2],
) -> Result<f32, FramebufferPlaneFailure> {
    let first = plane.gradient[0] * sample[0] + plane.origin;
    let depth = plane.gradient[1] * sample[1] + first;
    if !finite(sample) || !depth.is_finite() {
        return Err(FramebufferPlaneFailure::Arithmetic);
    }
    Ok(depth)
}
#[cfg(test)]
#[path = "clipped_depth_plane_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "clipped_depth_plane_capture_tests.rs"]
mod capture_tests;
