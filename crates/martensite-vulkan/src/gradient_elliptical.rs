//! A polygon fan retains the authored elliptical background's linear RGB colors.
use super::{BackgroundChart, RenderError, RenderLayer, RenderVertex, unlit_gradient_layer};
const ELLIPSE_SEGMENTS: u32 = 64;

/// Build the fan over the chart's view size; `layer([center, rim], &chart)?`.
pub(super) fn layer(
    colors: [[f32; 4]; 2],
    chart: &BackgroundChart,
) -> Result<RenderLayer, RenderError> {
    let transform = chart.world_view();
    let vertices = elliptical_vertices(colors, chart.pixel_size());
    let indices = elliptical_indices();
    unlit_gradient_layer(vertices, indices, transform)
}

fn elliptical_vertices(colors: [[f32; 4]; 2], extent: [u32; 2]) -> Vec<RenderVertex> {
    let center = extent.map(|dimension| dimension as f32 / 2.);
    let mut vertices = Vec::with_capacity(66);
    vertices.push(RenderVertex::new([center[0], center[1], 0.], colors[0]));
    let step = (std::f64::consts::PI * 2.) / f64::from(ELLIPSE_SEGMENTS);
    let mut angle: f64 = 0.;
    for _ in 0..=ELLIPSE_SEGMENTS {
        let point = elliptical_position(angle, extent, center);
        vertices.push(RenderVertex::new(point, colors[1]));
        angle += step;
    }
    vertices
}

fn elliptical_position(angle: f64, extent: [u32; 2], center: [f32; 2]) -> [f32; 3] {
    // Native repeats double-angle additions and rounds the center to f32; do not snap the seam.
    let x = martensite_cmath::cos(angle) * std::f64::consts::SQRT_2 * f64::from(extent[0]) / 2.;
    let y = martensite_cmath::sin(angle) * std::f64::consts::SQRT_2 * f64::from(extent[1]) / 2.;
    [
        (x + f64::from(center[0])) as f32,
        (y + f64::from(center[1])) as f32,
        0.,
    ]
}

fn elliptical_indices() -> Vec<u32> {
    (0..ELLIPSE_SEGMENTS)
        .flat_map(|segment| [0, segment + 1, segment + 2])
        .collect()
}

#[cfg(test)]
#[path = "gradient_elliptical_tests.rs"]
mod tests;
