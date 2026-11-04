//! The exact-tie grader accepts only proven ties shifted at most 1 px and rejects the rest.
use super::*;

const SIZE: usize = 64;
const BACKGROUND: [u8; 4] = [0, 0, 255, 255];
const LINE: [u8; 4] = [255, 255, 255, 255];
const FACE: [u8; 4] = [255, 0, 0, 255];

/// Uniform depth for colour-only scenarios; `flat_depth()`.
fn flat_depth() -> Vec<f32> {
    vec![0.5; SIZE * SIZE]
}

/// Grade two RGBA images with equal flat depth; `grade(&rust, &native, &loci)`.
fn grade(rust: &[u8], native: &[u8], loci: &[TieLocus]) -> Result<TieGrade, String> {
    let depth = flat_depth();
    exact_tie_violation(
        SIZE,
        SIZE,
        TieFrame {
            rgba: rust,
            depth: &depth,
        },
        TieFrame {
            rgba: native,
            depth: &depth,
        },
        loci,
    )
}

/// A large face with a one-pixel line on `row`; `frame(Some(15))`.
fn frame(line_row: Option<usize>) -> Vec<u8> {
    let mut rgba = BACKGROUND.repeat(SIZE * SIZE);
    for row in 2..62 {
        for column in 2..62 {
            let colour = if Some(row) == line_row && column < 6 {
                LINE
            } else {
                FACE
            };
            rgba[(row * SIZE + column) * 4..][..4].copy_from_slice(&colour);
        }
    }
    rgba
}

#[test]
fn thin_line_on_a_pixel_boundary_ties_both_adjacent_rows_only() {
    let locus = TieLocus::thin_row(16.);
    assert!(locus.ties(3, 15) && locus.ties(3, 16));
    assert!(!locus.ties(3, 14) && !locus.ties(3, 17));
    assert!(!TieLocus::thin_row(16.25).ties(3, 15));
}

#[test]
fn fill_edge_through_pixel_centres_ties_that_row_only() {
    let locus = TieLocus::fill_row(30.5);
    assert!(locus.ties(9, 30));
    assert!(!locus.ties(9, 29) && !locus.ties(9, 31));
}

#[test]
fn a_proven_tie_shift_within_the_bound_is_accepted() {
    let grade = grade(
        &frame(Some(15)),
        &frame(Some(16)),
        &[TieLocus::thin_row(16.)],
    )
    .unwrap();
    assert_eq!(grade.differing, 8);
}

#[test]
fn a_one_pixel_shift_off_any_tie_is_rejected() {
    // Rows 20/21 straddle no tie of a line at y = 16: a genuine shift, not a tie.
    let error = grade(
        &frame(Some(20)),
        &frame(Some(21)),
        &[TieLocus::thin_row(16.)],
    )
    .unwrap_err();
    assert!(error.contains("off every proven tie"), "{error}");
}

#[test]
fn a_proven_tie_beyond_half_a_percent_of_lit_pixels_has_no_cap() {
    let mut rust = frame(None);
    let native = frame(None);
    // Shift a 3,600-pixel face's whole top row: 60 tie pixels exceed 0.5% (18) and still pass.
    for column in 2..62 {
        rust[(2 * SIZE + column) * 4..][..4].copy_from_slice(&BACKGROUND);
        rust[(SIZE + column) * 4..][..4].copy_from_slice(&FACE);
    }
    let loci = [TieLocus::fill_row(1.5), TieLocus::fill_row(2.5)];
    assert_eq!(grade(&rust, &native, &loci).unwrap().differing, 120);
}

#[test]
fn a_two_pixel_shift_on_proven_ties_is_rejected() {
    // Rows 14 and 16 each lie on a tie, but the line moved two rows: no 1 px match exists.
    let loci = [TieLocus::thin_row(15.), TieLocus::thin_row(17.)];
    let error = grade(&frame(Some(14)), &frame(Some(16)), &loci).unwrap_err();
    assert!(error.contains("no match within 1 px"), "{error}");
}

#[test]
fn a_tie_pixel_with_a_different_depth_is_rejected() {
    // The line shifts one row on its tie but lands at another depth: not the same sample.
    let line_depth = |row: usize, value: f32| {
        let mut depth = flat_depth();
        (2..6).for_each(|column| depth[row * SIZE + column] = value);
        depth
    };
    let (rust, native) = (frame(Some(15)), frame(Some(16)));
    let error = exact_tie_violation(
        SIZE,
        SIZE,
        TieFrame {
            rgba: &rust,
            depth: &line_depth(15, 0.25),
        },
        TieFrame {
            rgba: &native,
            depth: &line_depth(16, 0.75),
        },
        &[TieLocus::thin_row(16.)],
    )
    .unwrap_err();
    assert!(error.contains("no match within 1 px"), "{error}");
}

#[test]
fn a_depth_only_difference_off_any_tie_is_rejected() {
    let image = frame(None);
    let mut depth = flat_depth();
    depth[40 * SIZE + 40] = 0.5 + 2e-6;
    let error = exact_tie_violation(
        SIZE,
        SIZE,
        TieFrame {
            rgba: &image,
            depth: &depth,
        },
        TieFrame {
            rgba: &image,
            depth: &flat_depth(),
        },
        &[TieLocus::thin_row(16.)],
    )
    .unwrap_err();
    assert!(error.contains("pixel (40,40)"), "{error}");
}

#[test]
fn identical_images_pass_without_loci() {
    assert_eq!(grade(&frame(None), &frame(None), &[]).unwrap().differing, 0);
}
