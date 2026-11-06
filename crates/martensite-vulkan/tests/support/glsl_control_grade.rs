//! Ray-traced compile-path residue (user decision 2026-10-28, the design notes):
//! for ray/path-traced output on lavapipe only, a differing pixel passes when the recorded GLSL
//! control (our shader source and the frame's exact data through Mesa's GLSL compiler on
//! llvmpipe, the recorded controls) equals native at that
//! pixel and the Rust pixel's colour is present in the native 3x3 neighbourhood. There is no
//! colour-distance bound: refraction amplifies the rounding into another colour of the scene.

fn rgb(image: &[u8], width: usize, (x, y): (usize, usize)) -> &[u8] {
    &image[3 * (y * width + x)..][..3]
}

/// Whether the native 3x3 neighbourhood of `pixel` (clipped to the image) holds `colour`.
fn in_native_neighbourhood(
    native: &[u8],
    size: (usize, usize),
    pixel: (usize, usize),
    colour: &[u8],
) -> bool {
    let near = |v: usize, limit: usize| v.saturating_sub(1)..=(v + 1).min(limit - 1);
    near(pixel.1, size.1)
        .any(|y| near(pixel.0, size.0).any(|x| rgb(native, size.0, (x, y)) == colour))
}

/// Whether `pixel` is a proven ray-traced compile-path residue on `device`.
/// Example: `admits_glsl_control(&device, (x, y), (&rust, &native, &control), (256, 256))`.
pub fn admits_glsl_control(
    device: &str,
    pixel: (usize, usize),
    (rust, native, control): (&[u8], &[u8], &[u8]),
    size: (usize, usize),
) -> bool {
    device.contains("llvmpipe")
        && rgb(control, size.0, pixel) == rgb(native, size.0, pixel)
        && in_native_neighbourhood(native, size, pixel, rgb(rust, size.0, pixel))
}
