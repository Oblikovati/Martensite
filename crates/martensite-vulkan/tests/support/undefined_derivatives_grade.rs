//! Undefined shader derivatives: where a shader takes
//! screen-space derivatives after an early return or discard (undefined in GLSL, and radeonsi
//! and llvmpipe disagree), as in faceted PBR under depth-peeling OIT, the AMD output is graded
//! against the recorded llvmpipe dump: each pixel equal or within one colour level, with
//! identical coverage. Lavapipe keeps its byte-exact same-driver grading.
use super::grading::{DumpReport, UnexplainedPixel};

/// True when `colour` differs from the background `background` (the pixel is covered).
fn covered(colour: &[u8], background: &[u8]) -> bool {
    colour != background
}

/// One pixel passes: same coverage as the llvmpipe dump and at most one level per channel.
fn admitted(rust: &[u8], llvmpipe: &[u8], background: &[u8]) -> bool {
    let near = rust.iter().zip(llvmpipe).all(|(a, b)| a.abs_diff(*b) <= 1);
    near && covered(rust, background) == covered(llvmpipe, background)
}

/// The AMD report of `name` against the native llvmpipe dump, or `None` on lavapipe where the
/// rule does not apply; the background is the llvmpipe dump's first pixel;
/// `undefined_derivatives_report(&device, "pbrfacet-oit-peeling", &rust, llvmpipe, width)`.
pub fn undefined_derivatives_report(
    device: &str,
    name: &'static str,
    rust: &[u8],
    llvmpipe: &[u8],
    width: usize,
) -> Option<DumpReport> {
    if device.contains("llvmpipe") {
        return None;
    }
    assert_eq!(rust.len(), llvmpipe.len(), "expected equal RGB sizes");
    let background = &llvmpipe[..3];
    let pixel = |bytes: &[u8], i: usize| bytes[3 * i..3 * i + 3].to_vec();
    let differing: Vec<usize> = (0..rust.len() / 3)
        .filter(|&i| pixel(rust, i) != pixel(llvmpipe, i))
        .collect();
    let unexplained: Vec<UnexplainedPixel> = differing
        .iter()
        .filter(|&&i| !admitted(&pixel(rust, i), &pixel(llvmpipe, i), background))
        .map(|&i| {
            (
                i % width,
                i / width,
                pixel(rust, i),
                pixel(llvmpipe, i),
                None,
            )
        })
        .collect();
    Some((name, differing.len(), unexplained))
}
