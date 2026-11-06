use super::*;

const RECT: ClipRect = ClipRect {
    min: [0, 0],
    max: [9, 9],
};

fn slope(delta: [u32; 2], octant: u32, bias: u32) -> LineSlope {
    LineSlope {
        delta,
        octant,
        bias,
    }
}

fn clip(start: [i32; 2], end: [i32; 2], line: LineSlope) -> Option<ClippedLine> {
    zero_clip_line(
        RECT,
        line,
        [start, end],
        [RECT.outcode(start), RECT.outcode(end)],
    )
}

#[test]
fn outcodes_name_every_violated_side() {
    assert_eq!(RECT.outcode([5, 5]), 0);
    assert_eq!(RECT.outcode([-1, 5]), OUT_LEFT);
    assert_eq!(RECT.outcode([10, -1]), OUT_RIGHT | OUT_ABOVE);
    assert_eq!(RECT.outcode([0, 10]), OUT_BELOW);
}

#[test]
fn a_horizontal_line_is_cut_at_both_sides() {
    let clipped = clip([-5, 2], [15, 2], slope([20, 0], 0, 0)).unwrap();
    assert_eq!((clipped.start, clipped.end), ([0, 2], [9, 2]));
    assert!(clipped.start_clipped);
}

#[test]
fn a_clipped_end_keeps_the_start_unmarked() {
    let clipped = clip([5, 5], [5, 40], slope([0, 35], Y_MAJOR, 0)).unwrap();
    assert_eq!((clipped.start, clipped.end), ([5, 5], [5, 9]));
    assert!(!clipped.start_clipped);
}

#[test]
fn a_sloped_line_lands_on_the_bresenham_pixel_of_its_boundary() {
    // From [-4, 0] to [4, 2] the walk reaches column 0 on row 1 (tie broken diagonally).
    let clipped = clip([-4, 0], [4, 2], slope([8, 2], 0, 0)).unwrap();
    assert_eq!(clipped.start, [0, 1]);
    // The reverse walk (octant 4, axial bias) covers the same pixels, so it also ends there.
    let reverse = clip(
        [4, 2],
        [-4, 0],
        slope([8, 2], X_DECREASING | Y_DECREASING, 1),
    )
    .unwrap();
    assert_eq!(reverse.end, [0, 1]);
}

#[test]
fn a_far_start_is_solved_from_the_near_end() {
    let start = [-32768, 3];
    let clipped = clip(start, [5, 3], slope([32773, 0], 0, 0)).unwrap();
    assert_eq!((clipped.start, clipped.end), ([0, 3], [5, 3]));
}

#[test]
fn a_line_missing_the_rectangle_is_rejected() {
    assert_eq!(
        clip([-5, 3], [3, -5], slope([8, 8], Y_DECREASING | Y_MAJOR, 1)),
        None
    );
    assert_eq!(clip([-5, -5], [-1, 20], slope([4, 25], Y_MAJOR, 0)), None);
}

#[test]
fn equation_selection_mirrors_for_swapped_and_far_endpoints() {
    assert_eq!(equation_terms(true, true, false, false), EQN1);
    assert_eq!(equation_terms(true, true, true, false), EQN2);
    assert_eq!(equation_terms(true, false, false, true), EQN4B);
    assert_eq!(equation_terms(false, false, true, true), EQN7);
    assert_eq!(first_violated_edge(OUT_LEFT | OUT_ABOVE, RECT).axis, 0);
    assert_eq!(first_violated_edge(OUT_RIGHT | OUT_ABOVE, RECT).axis, 1);
}
