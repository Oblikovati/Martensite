//! The attribute-rounding reference accepts only one-level residues that the bottom-up control removes.
use super::*;

const PIXELS: usize = 4;
const DEPTH: [f32; PIXELS] = [1., 0.5, 0.5, 0.5];
const BACKGROUND: [u8; 4] = [0, 0, 255, 255];
const TEXEL: [u8; 4] = [100, 150, 200, 255];

/// A background pixel followed by three textured pixels; `textured()`.
fn textured() -> Vec<u8> {
    [BACKGROUND, TEXEL, TEXEL, TEXEL].concat()
}

/// `textured()` with pixel `index` set to `colour`; `with_pixel(2, [101, 150, 200, 255])`.
fn with_pixel(index: usize, colour: [u8; 4]) -> Vec<u8> {
    let mut rgba = textured();
    rgba[index * 4..index * 4 + 4].copy_from_slice(&colour);
    rgba
}

/// Explain `rust` against the textured native image and `control`; `explain(&r, &c, &d)`.
fn explain(rust: &[u8], control: &[u8], rust_depth: &[f32]) -> RoundedFrame {
    let (native, depth) = (textured(), DEPTH);
    let frame = |rgba, depth| TieFrame { rgba, depth };
    explain_attribute_rounding(
        frame(rust, rust_depth),
        frame(&native, &depth),
        frame(control, &depth),
    )
    .unwrap()
}

#[test]
fn a_one_level_residue_removed_by_the_control_takes_the_native_value() {
    let rounded = explain(&with_pixel(2, [101, 149, 200, 255]), &textured(), &DEPTH);
    assert_eq!((rounded.rounded, rounded.rgba), (1, textured()));
}

#[test]
fn a_two_level_difference_is_rejected() {
    let rust = with_pixel(2, [102, 150, 200, 255]);
    assert_eq!(explain(&rust, &textured(), &DEPTH).rgba, rust);
}

#[test]
fn a_difference_that_persists_in_the_control_is_rejected() {
    let rust = with_pixel(2, [101, 150, 200, 255]);
    assert_eq!(explain(&rust, &rust, &DEPTH).rounded, 0);
}

#[test]
fn a_coverage_change_is_rejected() {
    // One level from the background colour is still a coverage change.
    let rust = with_pixel(0, [1, 0, 255, 255]);
    assert_eq!(explain(&rust, &textured(), &DEPTH).rounded, 0);
    let moved = [1., 0.5, 0.5 + 2e-6, 0.5];
    // The same one-level colour, but the surface moved: not the same coverage.
    let rust = with_pixel(2, [101, 150, 200, 255]);
    assert_eq!(explain(&rust, &textured(), &moved).rounded, 0);
}

#[test]
fn an_alpha_difference_is_rejected() {
    let rust = with_pixel(2, [100, 150, 200, 254]);
    assert_eq!(explain(&rust, &textured(), &DEPTH).rounded, 0);
}
