//! Affine diagonal expectations are independent of the renderer's corner lookup.
use super::*;
const DIAGONAL_FIRST: [f32; 4] = [0.125, 0.25, 0.75, 1.];
const DIAGONAL_SECOND: [f32; 4] = [0.75, 0.5, 0.125, 1.];
const DIAGONAL_MIDDLE: [f32; 4] = [0.4375, 0.375, 0.4375, 1.];

#[test]
fn diagonal_authored_corners_match_independent_linear_color_midpoints() {
    for method in [
        AspectGradientFillMethod::Diagonal1,
        AspectGradientFillMethod::Diagonal2,
    ] {
        let layer = diagonal_test_layer(method, [97, 43]);
        let draw = &layer.draws[0];
        for vertex in &draw.vertices {
            let [x, y, _] = vertex.position;
            let weight = diagonal_second_weight(method, x / 97., y / 43.);
            assert_eq!(vertex.color, diagonal_expected_color(weight));
        }
        assert_eq!(draw.indices, [0, 1, 3, 1, 2, 3]);
        assert!(!layer.depth_test && !layer.depth_write && !layer.clear_depth);
    }
}

fn diagonal_test_layer(method: AspectGradientFillMethod, extent: [u32; 2]) -> RenderLayer {
    let colors = [DIAGONAL_FIRST, DIAGONAL_SECOND];
    let layer = gradient_layer(method, colors, extent).unwrap();
    layer.unwrap_or_else(|| panic!("gradient {method:?}, expected an authored diagonal draw"))
}

fn diagonal_second_weight(method: AspectGradientFillMethod, x: f32, y: f32) -> f32 {
    match method {
        AspectGradientFillMethod::Diagonal1 => (x + 1. - y) / 2.,
        AspectGradientFillMethod::Diagonal2 => (1. - x + 1. - y) / 2.,
        _ => panic!("gradient {method:?}, expected Diagonal1 or Diagonal2"),
    }
}

fn diagonal_expected_color(weight: f32) -> [f32; 4] {
    std::array::from_fn(|channel| {
        (1. - weight) * DIAGONAL_FIRST[channel] + weight * DIAGONAL_SECOND[channel]
    })
}

#[test]
fn diagonal_center_and_non_square_quarter_pixels_follow_affine_fields() {
    for method in [
        AspectGradientFillMethod::Diagonal1,
        AspectGradientFillMethod::Diagonal2,
    ] {
        assert_eq!(
            diagonal_expected_color(diagonal_second_weight(method, 0.5, 0.5)),
            DIAGONAL_MIDDLE
        );
        let layer = diagonal_test_layer(method, [97, 43]);
        for pixel in [[0.5, 0.5], [24.5, 10.5], [96.5, 42.5]] {
            assert_diagonal_pixel_field(&layer.draws[0], method, pixel);
        }
    }
}

fn assert_diagonal_pixel_field(
    draw: &PrimitiveDraw,
    method: AspectGradientFillMethod,
    pixel: [f32; 2],
) {
    let [x, y] = [pixel[0] / 97., pixel[1] / 43.];
    let expected = diagonal_expected_color(diagonal_second_weight(method, x, y));
    let actual = authored_affine_field(draw, x, y);
    for channel in 0..4 {
        assert!(
            (actual[channel] - expected[channel]).abs() <= 2. * f32::EPSILON,
            "pixel {pixel:?} mode {method:?} channel {channel} actual {actual:?}, expected affine {expected:?}"
        );
    }
}

fn authored_affine_field(draw: &PrimitiveDraw, x: f32, y: f32) -> [f32; 4] {
    let bottom_left = draw.vertices[3].color;
    let bottom_right = draw.vertices[0].color;
    let top_left = draw.vertices[2].color;
    std::array::from_fn(|channel| {
        bottom_left[channel]
            + x * (bottom_right[channel] - bottom_left[channel])
            + y * (top_left[channel] - bottom_left[channel])
    })
}
