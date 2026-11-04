//! Sphere-map rim grading of environment-texture dumps (user decision 2026-10-28).
use super::grading::UnexplainedPixel;

fn rgb_pixels(bytes: &[u8]) -> Vec<[u8; 3]> {
    bytes.as_chunks::<3>().0.to_vec()
}

/// Sphere-map rim grading (user decision 2026-10-28): on lavapipe, the unexplained pixels of
/// sphere-mapped texture lookups that exceed one level (the rim) pass when the bottom-up control
/// equals native at every one of them, each is lit in both dumps, and the coverage masks (pixels
/// other than the background) of the Rust and native dumps are identical. One-level pixels keep
/// their own residue rules. Without a control (hardware) nothing is graded. Returns the pixels left
/// unexplained and the count graded;
/// `grade_sphere_map_rim(pixels, &rust, native, control, width, |x, y| mapped)`.
pub fn grade_sphere_map_rim(
    pixels: Vec<UnexplainedPixel>,
    rust: &[u8],
    native: &[u8],
    control: Option<&[u8]>,
    width: usize,
    sphere_mapped: impl Fn(usize, usize) -> bool,
) -> (Vec<UnexplainedPixel>, usize) {
    let Some(control) = control else {
        return (pixels, 0);
    };
    let (rust_px, native_px) = (rgb_pixels(rust), rgb_pixels(native));
    let control_px = rgb_pixels(control);
    let background = native_px[0];
    let same_coverage = rust_px
        .iter()
        .zip(&native_px)
        .all(|(r, n)| (*r == background) == (*n == background));
    let (rim, left): (Vec<_>, Vec<_>) = pixels.into_iter().partition(|p| {
        let beyond_one_level = p.2.iter().zip(&p.3).any(|(a, b)| a.abs_diff(*b) > 1);
        beyond_one_level && sphere_mapped(p.0, p.1)
    });
    let proven = same_coverage
        && rim.iter().all(|p| {
            let i = p.1 * width + p.0;
            control_px[i] == native_px[i] && native_px[i] != background && rust_px[i] != background
        });
    if proven {
        (left, rim.len())
    } else {
        ([rim, left].concat(), 0)
    }
}
