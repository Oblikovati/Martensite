use super::*;

const PIXEL: (usize, usize, &[u8], &[u8]) = (40, 67, &[120, 80, 40], &[121, 80, 40]);

#[test]
fn a_proven_one_level_lit_pixel_passes_on_llvmpipe() {
    assert!(compile_path_explained(
        "llvmpipe (LLVM 20.1.2, 128 bits)",
        "bug21753",
        PIXEL,
        PROVEN_PIXELS
    ));
}

#[test]
fn unlisted_pixels_dumps_and_devices_are_refused() {
    let (column, row, rust, native) = PIXEL;
    let lavapipe = "llvmpipe";
    assert!(!compile_path_explained(
        lavapipe,
        "bug21753",
        (column + 1, row, rust, native),
        PROVEN_PIXELS
    ));
    assert!(!compile_path_explained(
        lavapipe,
        "fuse-ph1",
        PIXEL,
        PROVEN_PIXELS
    ));
    assert!(!compile_path_explained(
        "AMD Radeon RX 7900 XTX (RADV NAVI31)",
        "bug21753",
        PIXEL,
        PROVEN_PIXELS
    ));
}

#[test]
fn two_levels_or_a_coverage_change_are_refused() {
    let (column, row, rust, _) = PIXEL;
    let device = "llvmpipe";
    let two_levels: &[u8] = &[122, 80, 40];
    assert!(!compile_path_explained(
        device,
        "bug21753",
        (column, row, rust, two_levels),
        PROVEN_PIXELS
    ));
    let uncovered: &[u8] = &[0, 0, 0];
    let near_black: &[u8] = &[1, 0, 0];
    assert!(!compile_path_explained(
        device,
        "bug21753",
        (column, row, near_black, uncovered),
        PROVEN_PIXELS
    ));
    assert!(!compile_path_explained(
        device,
        "bug21753",
        (column, row, uncovered, near_black),
        PROVEN_PIXELS
    ));
}
