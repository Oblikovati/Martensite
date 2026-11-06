use super::*;

/// Pixels of one segment in a 40x40 raster, relative to the origin `[10, 10]`, sorted.
fn relative_pixels(start: [i32; 2], end: [i32; 2]) -> Vec<[i32; 2]> {
    let shift = |p: [i32; 2]| [p[0] + 10, p[1] + 10];
    let mut pixels = Vec::new();
    draw_zero_width_segment([40, 40], shift(start), shift(end), &mut |p| {
        pixels.push([p[0] - 10, p[1] - 10]);
    });
    pixels.sort_unstable();
    pixels
}

fn sorted(mut pixels: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    pixels.sort_unstable();
    pixels
}

/// The unclipped walk restricted to `0..size`: what clipping must reproduce.
fn unclipped_visible(size: [i32; 2], start: [i32; 2], end: [i32; 2]) -> Vec<[i32; 2]> {
    let line = BresenhamLine::new(start, end);
    let mut pixels = Vec::new();
    let count = line.major_length(start, end) + 1;
    line.walk(start, line.error, count, &mut |p| {
        if (0..size[0]).contains(&p[0]) && (0..size[1]).contains(&p[1]) {
            pixels.push(p);
        }
    });
    sorted(pixels)
}

fn clipped(size: [i32; 2], start: [i32; 2], end: [i32; 2]) -> Vec<[i32; 2]> {
    let mut pixels = Vec::new();
    draw_zero_width_segment(size, start, end, &mut |p| pixels.push(p));
    sorted(pixels)
}

#[test]
fn axis_aligned_and_diagonal_segments_include_both_endpoints() {
    let horizontal = (0..=5).map(|x| [x, 0]).collect::<Vec<_>>();
    assert_eq!(relative_pixels([0, 0], [5, 0]), horizontal);
    assert_eq!(
        relative_pixels([0, 0], [0, -3]),
        [[0, -3], [0, -2], [0, -1], [0, 0]]
    );
    assert_eq!(
        relative_pixels([0, 0], [3, 3]),
        [[0, 0], [1, 1], [2, 2], [3, 3]]
    );
    assert_eq!(
        relative_pixels([0, 0], [-3, 3]),
        sorted(vec![[0, 0], [-1, 1], [-2, 2], [-3, 3]])
    );
    assert_eq!(relative_pixels([2, 2], [2, 2]), [[2, 2]]);
}

#[test]
fn every_octant_breaks_exact_ties_like_the_software_server() {
    let cases: [([i32; 2], [[i32; 2]; 5]); 8] = [
        ([4, 2], [[0, 0], [1, 1], [2, 1], [3, 2], [4, 2]]),
        ([2, 4], [[0, 0], [1, 1], [1, 2], [2, 3], [2, 4]]),
        ([-2, 4], [[0, 0], [-1, 1], [-1, 2], [-2, 3], [-2, 4]]),
        ([-4, 2], [[-1, 0], [0, 0], [-3, 1], [-2, 1], [-4, 2]]),
        ([-4, -2], [[-4, -2], [-3, -1], [-2, -1], [-1, 0], [0, 0]]),
        ([-2, -4], [[-2, -4], [-1, -3], [-1, -2], [0, -1], [0, 0]]),
        ([2, -4], [[2, -4], [1, -3], [1, -2], [0, -1], [0, 0]]),
        ([4, -2], [[3, -2], [4, -2], [1, -1], [2, -1], [0, 0]]),
    ];
    for (end, expected) in cases {
        assert_eq!(
            relative_pixels([0, 0], end),
            sorted(expected.to_vec()),
            "end {end:?}"
        );
    }
}

#[test]
fn reversing_a_segment_keeps_its_pixels() {
    for end in [[4, 2], [-4, -2], [7, 3], [3, -7], [-5, 2], [1, 6]] {
        assert_eq!(
            relative_pixels([0, 0], end),
            relative_pixels(end, [0, 0]),
            "end {end:?}"
        );
    }
}

#[test]
fn clipping_keeps_exactly_the_visible_pixels_of_the_unclipped_line() {
    let size = [37, 23];
    let coordinates = [-41, -9, -1, 0, 5, 18, 22, 23, 36, 37, 60];
    for &x1 in &coordinates {
        for &y1 in &coordinates {
            for (x2, y2) in [(13, 7), (-30, 50), (55, -12), (36, 22), (0, 22), (40, 3)] {
                let (start, end) = ([x1, y1], [x2, y2]);
                assert_eq!(
                    clipped(size, start, end),
                    unclipped_visible(size, start, end),
                    "{start:?}->{end:?}"
                );
            }
        }
    }
}

#[test]
fn far_off_window_coordinates_clip_like_the_unclipped_line() {
    let size = [800, 400];
    let ends = [
        [-32768, -32768],
        [32767, 32767],
        [-32768, 32767],
        [32767, -20000],
        [400, -32768],
    ];
    for start in ends {
        for end in [[10, 10], [799, 399], [-5, 200], [300, 32767]] {
            assert_eq!(
                clipped(size, start, end),
                unclipped_visible(size, start, end),
                "{start:?}->{end:?}"
            );
        }
    }
}

#[test]
fn segments_wholly_outside_draw_nothing() {
    assert!(clipped([10, 10], [-5, -5], [-1, 20]).is_empty());
    assert!(clipped([10, 10], [10, 0], [30, 9]).is_empty());
    assert!(clipped([10, 10], [-20, 5], [5, -20]).is_empty());
}

#[test]
fn the_error_bias_prefers_axial_steps_in_octants_two_to_five() {
    assert_eq!(BresenhamLine::new([0, 0], [4, 2]).slope.bias, 0);
    assert_eq!(BresenhamLine::new([0, 0], [-4, 2]).slope.bias, 1);
    assert_eq!(BresenhamLine::new([0, 0], [-4, -2]).slope.bias, 1);
    assert_eq!(BresenhamLine::new([0, 0], [2, -4]).slope.bias, 1);
    assert_eq!(BresenhamLine::new([0, 0], [2, 4]).slope.octant, Y_MAJOR);
}
