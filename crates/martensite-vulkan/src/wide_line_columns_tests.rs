use super::*;
use crate::LineStyle;

/// Clip transform mapping window pixels of a 64x64 framebuffer (GL y-up) to clip space, w = 2.
const PIXELS: [f32; 16] = [
    1. / 16.,
    0.,
    0.,
    0.,
    0.,
    1. / 16.,
    0.,
    0.,
    0.,
    0.,
    2.,
    0.,
    -2.,
    -2.,
    0.,
    2.,
];

fn segment(from: [f32; 2], to: [f32; 2]) -> PrimitiveDraw {
    let vertices = [from, to]
        .iter()
        .map(|&[x, y]| RenderVertex::new([x, y, 0.], [1.; 4]))
        .collect();
    let style = LineStyle::new(12., 0xffff, 1).unwrap();
    PrimitiveDraw::segments(vertices, vec![0, 1], PIXELS, style).unwrap()
}

fn assert_near(actual: [f32; 2], expected: [f32; 2]) {
    let close = actual
        .iter()
        .zip(expected)
        .all(|(a, e)| (a - e).abs() < 1e-4);
    assert!(close, "{actual:?} != {expected:?}");
}

fn moved_x(draw: &PrimitiveDraw, x_major: bool) -> [f32; 2] {
    let axis = usize::from(!x_major);
    match segment_columns(draw, [0, 1], [64, 64], (x_major, &[-0.5, 0.5])) {
        SegmentColumns::Moved(ends) => ends.map(|vertex| vertex.position[axis]),
        other => panic!("expected moved endpoints, got {other:?}"),
    }
}

#[test]
fn diagonal_start_off_its_diamond_skips_its_column() {
    // Like the recorded wide_lines capture "d" (window (11.7, 21.8) to (52.3, 42.1), where
    // llvmpipe and radeonsi cover columns 12..=51), with the start clearly off its diamond: the
    // replica runs from the centre of column 12 to that of 52.
    let draw = segment([11.7, 21.9], [52.3, 42.1]);
    assert_near(moved_x(&draw, true), [12.5, 52.5]);
}

#[test]
fn reversed_segments_cover_the_same_columns_from_the_other_side() {
    let draw = segment([52.3, 42.1], [11.7, 21.9]);
    assert_near(moved_x(&draw, true), [51.5, 11.5]);
}

#[test]
fn y_major_segments_move_along_rows() {
    // The diagonal above with its axes swapped: rows 12..=51.
    let draw = segment([21.9, 11.7], [42.1, 52.3]);
    assert_near(moved_x(&draw, false), [12.5, 52.5]);
}

#[test]
fn moved_endpoints_interpolate_colour_in_clip_space() {
    let mut draw = segment([11.7, 21.9], [52.3, 42.1]);
    draw.vertices[1].color = [0., 0., 0., 1.];
    let SegmentColumns::Moved([start, end]) =
        segment_columns(&draw, [0, 1], [64, 64], (true, &[-0.5, 0.5]))
    else {
        panic!("expected moved endpoints");
    };
    // The clip weight is constant, so colours follow the window-space fraction along x.
    let fade = |x: f32| 1. - (x - 11.7) / (52.3 - 11.7);
    assert!(
        (start.color[0] - fade(12.5)).abs() < 1e-5,
        "{:?}",
        start.color
    );
    assert!((end.color[0] - fade(52.5)).abs() < 1e-5, "{:?}", end.color);
    assert_eq!(end.color[3], 1.);
}

#[test]
fn segments_whose_replicas_already_cover_the_native_columns_keep_their_endpoints() {
    let draw = segment([0.5, 0.25], [10.5, 0.25]);
    assert_eq!(
        segment_columns(&draw, [0, 1], [64, 64], (true, &[-0.5, 0.5])),
        SegmentColumns::Unchanged
    );
}

#[test]
fn segments_inside_one_skipped_column_cover_nothing_and_behind_eye_stay() {
    // Start right of its centre going right: neither test draws column 3, the end is excluded.
    assert_eq!(
        segment_columns(
            &segment([3.8, 3.1], [3.9, 3.15]),
            [0, 1],
            [64, 64],
            (true, &[-0.5, 0.5])
        ),
        SegmentColumns::Empty
    );
    let mut behind = segment([0., 0.], [10., 0.]);
    behind.vertices[0].weight = -1.;
    assert_eq!(
        segment_columns(&behind, [0, 1], [64, 64], (true, &[-0.5, 0.5])),
        SegmentColumns::Unchanged
    );
}

#[test]
fn perspective_parameter_follows_the_clip_weights() {
    assert_eq!(perspective_parameter(0.5, 1., 1.), 0.5);
    assert_eq!(perspective_parameter(0.5, 1., 3.), 0.25);
    assert_eq!(perspective_parameter(1., 2., 5.), 1.);
}

#[test]
fn replica_endpoints_on_their_diamond_edge_are_moved_off_it() {
    // A half-pixel replica of (57.6, 6.4) lands on (57.6, 6.9), on the diamond edge, where the
    // device's rounding would decide; the endpoints move to column centres instead.
    let draw = segment([6.4, 6.4], [57.6, 6.4]);
    assert_near(moved_x(&draw, true), [6.5, 57.5]);
}
