//! Rejecting tests of support/glsl_control_grade.rs.
use super::glsl_control_grade::admits_glsl_control;

/// A 3x3 native image with a 50 50 50 pixel at its corner, a Rust image differing at the
/// centre and the control.
fn images(rust_centre: u8, control_centre: u8) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut native = vec![10; 27];
    native[..3].fill(50);
    let (mut rust, mut control) = (native.clone(), native.clone());
    rust[12..15].fill(rust_centre);
    control[12..15].fill(control_centre);
    (rust, native, control)
}
const LAVAPIPE: &str = "llvmpipe (LLVM 20.1.2, 128 bits)";

#[test]
fn a_controlled_pixel_with_a_neighbouring_native_colour_passes() {
    let (rust, native, control) = images(50, 10);
    assert!(admits_glsl_control(
        LAVAPIPE,
        (1, 1),
        (&rust, &native, &control),
        (3, 3)
    ));
}

#[test]
fn a_pixel_whose_control_differs_from_native_fails() {
    let (rust, native, control) = images(50, 50);
    assert!(!admits_glsl_control(
        LAVAPIPE,
        (1, 1),
        (&rust, &native, &control),
        (3, 3)
    ));
}

#[test]
fn a_colour_absent_from_the_native_neighbourhood_fails() {
    let (rust, native, control) = images(51, 10);
    assert!(!admits_glsl_control(
        LAVAPIPE,
        (1, 1),
        (&rust, &native, &control),
        (3, 3)
    ));
}

#[test]
fn the_bound_does_not_apply_on_amd() {
    let (rust, native, control) = images(50, 10);
    let amd = "AMD Radeon 8060S Graphics (RADV STRIX_HALO)";
    assert!(!admits_glsl_control(
        amd,
        (1, 1),
        (&rust, &native, &control),
        (3, 3)
    ));
}
