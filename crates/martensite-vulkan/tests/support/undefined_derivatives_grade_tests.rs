//! Rejecting tests of support/undefined_derivatives_grade.rs.
use super::undefined_derivatives_grade::undefined_derivatives_report;

const AMD: &str = "AMD Radeon 8060S Graphics (RADV STRIX_HALO)";
const LAVAPIPE: &str = "llvmpipe (LLVM 20.1.2, 128 bits)";

/// A 3x3 llvmpipe image on a 0 0 0 background with a lit 100 100 100 centre, and a Rust image
/// whose centre is `rust_centre` and whose corner at (2, 2) is `rust_corner`.
fn images(rust_centre: [u8; 3], rust_corner: [u8; 3]) -> (Vec<u8>, Vec<u8>) {
    let mut llvmpipe = vec![0; 27];
    llvmpipe[12..15].fill(100);
    let mut rust = llvmpipe.clone();
    rust[12..15].copy_from_slice(&rust_centre);
    rust[24..27].copy_from_slice(&rust_corner);
    (rust, llvmpipe)
}

fn unexplained(device: &str, rust: &[u8], llvmpipe: &[u8]) -> Option<Vec<(usize, usize)>> {
    let report = undefined_derivatives_report(device, "peeling", rust, llvmpipe, 3)?;
    Some(report.2.iter().map(|pixel| (pixel.0, pixel.1)).collect())
}

#[test]
fn a_lit_pixel_within_one_level_of_llvmpipe_passes() {
    let (rust, llvmpipe) = images([101, 99, 100], [0, 0, 0]);
    let report = undefined_derivatives_report(AMD, "peeling", &rust, &llvmpipe, 3).unwrap();
    assert_eq!((report.1, report.2.len()), (1, 0));
}

#[test]
fn a_lit_pixel_two_levels_from_llvmpipe_fails() {
    let (rust, llvmpipe) = images([102, 100, 100], [0, 0, 0]);
    assert_eq!(unexplained(AMD, &rust, &llvmpipe), Some(vec![(1, 1)]));
}

#[test]
fn a_covered_background_pixel_within_one_level_fails() {
    let (rust, llvmpipe) = images([100, 100, 100], [1, 0, 0]);
    assert_eq!(unexplained(AMD, &rust, &llvmpipe), Some(vec![(2, 2)]));
}

#[test]
fn an_uncovered_pixel_within_one_level_fails() {
    let (mut rust, mut llvmpipe) = images([100, 100, 100], [0, 0, 0]);
    llvmpipe[12..15].copy_from_slice(&[1, 0, 0]);
    rust[12..15].fill(0);
    assert_eq!(unexplained(AMD, &rust, &llvmpipe), Some(vec![(1, 1)]));
}

#[test]
fn the_rule_does_not_apply_on_lavapipe() {
    let (rust, llvmpipe) = images([101, 100, 100], [0, 0, 0]);
    assert_eq!(unexplained(LAVAPIPE, &rust, &llvmpipe), None);
}
