//! Cross-driver grading per the 2026-10-27 user decision D2 (the design notes):
//! same-driver bytes stay exact; a difference that exists only between the native OpenGL driver
//! and the Vulkan driver on the same GPU is graded by bounds derived from rounding and snapping
//! arguments, never fitted to the measured differences.
//!
//! Colour is the shared D2 `shading` reference (the recording tool): identical
//! foreground mask and alpha, every RGB channel within one 8-bit level, because two faithful
//! roundings of the same exact value are at most one level apart.
//!
//! Depth: both drivers snap vertices to the GPU's 2^-8 pixel subpixel grid (RADV and radeonsi on
//! the same AMD GPU), so float transform differences move a vertex by at most one grid step and a
//! planar depth sample by at most (|dz/dx| + |dz/dy|) * 2^-8, slopes taken from the native depth
//! image, plus the canonical 1e-6 native depth gate.
use martensite_vulkan::FrameCapture;

/// Subpixel grid step of both drivers on the proven AMD profile.
const SUBPIXEL_STEP: f32 = 1. / 256.;
/// The canonical native depth gate of `native_golden::assert_golden_depth`.
const DEPTH_GATE: f32 = 1e-6;
/// The cleared depth value; background samples carry no surface slope.
const CLEAR_DEPTH: f32 = 1.;

/// The first violation of the D2 shading colour reference, if any; `shading_violation(&capture, rgba)`.
pub fn shading_violation(capture: &FrameCapture, rgba: &[u8]) -> Option<String> {
    assert_eq!(capture.rgba.len(), rgba.len(), "received wrong RGBA shape");
    let native_pixels = rgba.as_chunks::<4>().0;
    let background = native_pixels[0];
    let pixels = capture.rgba.as_chunks::<4>().0.iter().zip(native_pixels);
    for (index, (actual, native)) in pixels.enumerate() {
        let masked = (*actual == background) != (*native == background);
        let delta = (0..3).map(|i| actual[i].abs_diff(native[i])).max().unwrap();
        if masked || actual[3] != native[3] || delta > 1 {
            return Some(format!("pixel {index}: {actual:?} vs native {native:?}"));
        }
    }
    None
}

/// The first depth sample beyond the snapping bound, if any; `depth_violation(&capture, depth)`.
pub fn depth_violation(capture: &FrameCapture, depth: &[u8]) -> Option<String> {
    assert_eq!(
        capture.depth.len() * 4,
        depth.len(),
        "received wrong depth shape"
    );
    let native: Vec<f32> = depth
        .as_chunks::<4>()
        .0
        .iter()
        .map(|bytes| f32::from_le_bytes(*bytes))
        .collect();
    let width = capture.width as usize;
    for (index, actual) in capture.depth.iter().enumerate() {
        let bound = DEPTH_GATE + native_slope(&native, width, index) * SUBPIXEL_STEP;
        let delta = (actual - native[index]).abs();
        if delta > bound {
            return Some(format!(
                "depth {index}: {actual} vs native {}; bound {bound}",
                native[index]
            ));
        }
    }
    None
}

/// |dz/dx| + |dz/dy| from same-surface native neighbours; `native_slope(&depth, 64, index)`.
fn native_slope(native: &[f32], width: usize, index: usize) -> f32 {
    let (x, y, height) = (index % width, index / width, native.len() / width);
    let centre = native[index];
    let step = |neighbours: [Option<usize>; 2]| {
        neighbours
            .into_iter()
            .flatten()
            .filter(|&n| native[n] != CLEAR_DEPTH && centre != CLEAR_DEPTH)
            .map(|n| (native[n] - centre).abs())
            .fold(0., f32::max)
    };
    let horizontal =
        [x.checked_sub(1), (x + 1 < width).then_some(x + 1)].map(|c| c.map(|c| y * width + c));
    let vertical =
        [y.checked_sub(1), (y + 1 < height).then_some(y + 1)].map(|r| r.map(|r| r * width + x));
    step(horizontal) + step(vertical)
}

/// Grade one fixture by both D2 references; `assert_cross_driver(&capture, rgba, depth, "lit-colored")`.
pub fn assert_cross_driver(capture: &FrameCapture, rgba: &[u8], depth: &[u8], case: &str) {
    let violation = shading_violation(capture, rgba).or_else(|| depth_violation(capture, depth));
    println!(
        "{{\"cross_driver_case\":\"{case}\",\"passed\":{}}}",
        violation.is_none()
    );
    assert!(
        violation.is_none(),
        "received {violation:?} for {case}; expected the D2 shading and subpixel depth bounds"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    fn capture(rgba: Vec<u8>, depth: Vec<f32>) -> FrameCapture {
        FrameCapture {
            width: 2,
            height: 1,
            rgba,
            depth,
        }
    }
    fn bytes(depth: [f32; 2]) -> Vec<u8> {
        depth.iter().flat_map(|v| v.to_le_bytes()).collect()
    }
    const NATIVE: [u8; 8] = [0, 0, 0, 255, 100, 50, 25, 255];
    #[test]
    fn one_level_colour_and_depth_inside_the_snapping_bound_pass() {
        // Slope 0.256 per pixel: the bound is 1e-6 + 0.256 / 256 = 1.001e-3.
        let actual = capture(
            vec![0, 0, 0, 255, 101, 49, 25, 255],
            vec![0.5, 0.756 + 1e-3],
        );
        assert_cross_driver(&actual, &NATIVE, &bytes([0.5, 0.756]), "unit");
    }
    #[test]
    fn mask_alpha_two_levels_or_depth_beyond_the_bound_fail() {
        for (rgba, depth) in [
            (vec![1, 0, 0, 255, 100, 50, 25, 255], [0.5, 0.756]),
            (vec![0, 0, 0, 255, 100, 50, 25, 254], [0.5, 0.756]),
            (vec![0, 0, 0, 255, 102, 50, 25, 255], [0.5, 0.756]),
            (vec![0, 0, 0, 255, 100, 50, 25, 255], [0.5, 0.756 + 2e-3]),
        ] {
            let actual = capture(rgba, depth.to_vec());
            let failed = shading_violation(&actual, &NATIVE)
                .or_else(|| depth_violation(&actual, &bytes([0.5, 0.756])));
            assert!(failed.is_some(), "{actual:?}");
        }
    }
    #[test]
    fn background_samples_have_no_slope_allowance() {
        let actual = capture(NATIVE.to_vec(), vec![1. - 1e-3, 0.5]);
        assert!(depth_violation(&actual, &bytes([1., 0.5])).is_some());
    }
}
