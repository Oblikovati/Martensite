//! CPU coverage predicates for the versioned single-sample thin-line path.
mod diamond;
mod expand;
mod fused_projection;
mod user_clipping;
mod validation;

pub(super) fn selected_draw(
    behavior: crate::RenderThinLineCoverage,
    axes: crate::RenderThinAxisCoverage,
    draw: &crate::PrimitiveDraw,
    extent: ash::vk::Extent2D,
    single_sample: bool,
) -> Result<Option<crate::PrimitiveDraw>, crate::RenderError> {
    expand::selected_draw(behavior, axes, draw, extent, single_sample)
}

pub(super) fn stipple_on(pattern: u16, factor: u16, bottom_up_pixel: u32) -> bool {
    // Match Graphic3dShaderManager's `floor(gl_FragCoord / factor + 0.5)` arithmetic.
    let fragment_coordinate = bottom_up_pixel as f32 + 0.5;
    let bit = ((fragment_coordinate / f32::from(factor) + 0.5).floor() as u32) % 16;
    pattern & (1 << bit) != 0
}

pub(super) fn major_axis(start: [i64; 2], end: [i64; 2]) -> usize {
    let delta = [end[0] - start[0], end[1] - start[1]];
    if delta[1].abs() > delta[0].abs() {
        1
    } else {
        0
    }
}

#[cfg(test)]
#[path = "expand_tests.rs"]
mod expand_tests;
#[cfg(test)]
mod strip_tests;
#[cfg(test)]
mod tests;
