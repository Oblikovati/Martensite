//! Uncovered-surface ties and undefined loci accept only what their same-driver controls show.
use super::*;

const SIZE: usize = 64;
const LINE: [u8; 4] = [0, 255, 255, 255];
const OTHER: [u8; 4] = [9, 9, 9, 255];

/// A face whose texel differs per pixel, so a shifted face sample never matches.
fn face() -> Vec<u8> {
    (0..SIZE * SIZE)
        .flat_map(|i| [(i % 251) as u8, (i / 7 % 251) as u8, 40, 255])
        .collect()
}

/// The face with a 3 px horizontal line over `rows`; `with_line(30..=32)`.
fn with_line(rows: std::ops::RangeInclusive<usize>) -> Vec<u8> {
    let mut rgba = face();
    for row in rows {
        for column in 8..40 {
            rgba[(row * SIZE + column) * 4..][..4].copy_from_slice(&LINE);
        }
    }
    rgba
}

fn depth() -> Vec<f32> {
    vec![0.5; SIZE * SIZE]
}

fn tie_rows() -> [TieLocus; 2] {
    [TieLocus::fill_row(30.5), TieLocus::fill_row(33.5)]
}

/// Grade Vulkan rows 30-32 against native rows 31-33 with `controls`.
fn grade_band(rust: &[u8], native: &[u8], controls: &TieControls) -> Result<TieGrade, String> {
    let d = depth();
    let frame = |rgba| TieFrame { rgba, depth: &d };
    controlled_tie_violation(
        SIZE,
        SIZE,
        frame(rust),
        frame(native),
        &tie_rows(),
        controls,
    )
}

#[test]
fn tie_pixels_that_uncover_the_face_pass_with_both_uncovered_controls() {
    let (rust, native, uncovered, d) = (with_line(30..=32), with_line(31..=33), face(), depth());
    let control = TieFrame {
        rgba: &uncovered,
        depth: &d,
    };
    let own = DriverControls {
        uncovered: Some(control),
        ..Default::default()
    };
    let controls = TieControls {
        rust: own,
        native: own,
        undefined: &[],
    };
    assert_eq!(grade_band(&rust, &native, &controls).unwrap().differing, 64);
    let none = TieControls::default();
    assert!(grade_band(&rust, &native, &none).is_err());
}

#[test]
fn an_uncovered_control_that_differs_at_the_pixel_is_rejected() {
    let (rust, native, d) = (with_line(30..=32), with_line(31..=33), depth());
    let wrong = with_line(30..=33);
    let control = TieFrame {
        rgba: &wrong,
        depth: &d,
    };
    let own = DriverControls {
        uncovered: Some(control),
        ..Default::default()
    };
    let controls = TieControls {
        rust: own,
        native: own,
        undefined: &[],
    };
    assert!(grade_band(&rust, &native, &controls).is_err());
}

#[test]
fn an_uncovered_surface_off_every_proven_tie_is_rejected() {
    let (rust, native, uncovered, d) = (with_line(29..=31), with_line(31..=33), face(), depth());
    let control = TieFrame {
        rgba: &uncovered,
        depth: &d,
    };
    let own = DriverControls {
        uncovered: Some(control),
        ..Default::default()
    };
    let controls = TieControls {
        rust: own,
        native: own,
        undefined: &[],
    };
    let error = grade_band(&rust, &native, &controls).unwrap_err();
    assert!(error.contains("off every proven tie"), "{error}");
}

/// Face-only and edge-only controls of one driver: the face, and the full line.
fn undefined_controls<'a>(face: &'a [u8], edge: &'a [u8], d: &'a [f32]) -> TieControls<'a> {
    let own = DriverControls {
        face_only: Some(TieFrame {
            rgba: face,
            depth: d,
        }),
        edge_only: Some(TieFrame {
            rgba: edge,
            depth: d,
        }),
        uncovered: None,
    };
    TieControls {
        rust: own,
        native: own,
        undefined: &[],
    }
}

const BAND: [UndefinedLocus; 1] = [UndefinedLocus {
    a: 0.,
    b: 1.,
    c: -32.,
    half_width: 1.5,
}];

#[test]
fn an_undefined_locus_accepts_either_own_control_and_nothing_else() {
    let (face, edge, d) = (face(), with_line(30..=33), depth());
    let controls = TieControls {
        undefined: &BAND,
        ..undefined_controls(&face, &edge, &d)
    };
    // Vulkan shows the face, native the edge, across the whole band.
    assert!(grade_band(&face, &edge, &controls).is_ok());
    let mut stray = edge.clone();
    stray[(31 * SIZE + 9) * 4..][..4].copy_from_slice(&OTHER);
    let error = grade_band(&face, &stray, &controls).unwrap_err();
    assert!(error.contains("undefined locus"), "{error}");
}

#[test]
fn a_difference_outside_the_undefined_band_stays_exact() {
    let (face, edge, d) = (face(), with_line(30..=33), depth());
    let controls = TieControls {
        undefined: &BAND,
        ..undefined_controls(&face, &edge, &d)
    };
    let mut outside = face.clone();
    outside[(40 * SIZE + 9) * 4..][..4].copy_from_slice(&LINE);
    let error = grade_band(&outside, &face, &controls).unwrap_err();
    assert!(error.contains("off every proven tie"), "{error}");
    assert!(BAND[0].contains(5, 30) && BAND[0].contains(5, 33) && !BAND[0].contains(5, 34));
}

#[test]
fn a_thin_line_tie_that_uncovers_the_face_needs_both_uncovered_controls() {
    let (rust, native, uncovered, d) = (with_line(15..=15), with_line(16..=16), face(), depth());
    let frame = |rgba| TieFrame { rgba, depth: &d };
    let own = DriverControls {
        uncovered: Some(frame(&uncovered)),
        ..Default::default()
    };
    let controls = TieControls {
        rust: own,
        native: own,
        undefined: &[],
    };
    let loci = [TieLocus::thin_row(16.)];
    let grade = |c: &TieControls| {
        controlled_tie_violation(SIZE, SIZE, frame(&rust), frame(&native), &loci, c)
    };
    assert_eq!(grade(&controls).unwrap().differing, 64);
    assert!(grade(&TieControls::default()).is_err());
    let off = [TieLocus::thin_row(16.25)];
    let shifted =
        controlled_tie_violation(SIZE, SIZE, frame(&rust), frame(&native), &off, &controls);
    assert!(shifted.unwrap_err().contains("off every proven tie"));
}
