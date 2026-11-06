//! Zero-width segments as GPU-accelerated X servers draw them through GL lines.
//!
//! An accelerated server draws each segment as a GL line between pixel centres, which
//! inks the start but not the end, and then a one-pixel line that inks the end. Along the
//! major axis every pixel takes the minor coordinate nearest the exact line; an exact tie
//! takes the smaller coordinate whatever the direction.

/// Visit the pixels of `start..=end` inside a `size` raster as an accelerated server inks them.
///
/// Example: `draw_centered_segment([8, 8], [0, 0], [4, 2], &mut |p| pixels.push(p))`
/// visits `[0, 0], [1, 0], [2, 1], [3, 1]` and then the end `[4, 2]`.
pub(super) fn draw_centered_segment(
    size: [i32; 2],
    start: [i32; 2],
    end: [i32; 2],
    plot: &mut impl FnMut([i32; 2]),
) {
    let delta = [end[0] - start[0], end[1] - start[1]].map(i64::from);
    let major = usize::from(delta[0].abs() < delta[1].abs());
    let length = delta[major].abs();
    let step = delta[major].signum();
    for index in visible_steps(size[major], start[major], step, length) {
        let mut point = [0; 2];
        point[major] = (i64::from(start[major]) + index * step) as i32;
        point[1 - major] = nearest_minor(start[1 - major], delta[1 - major], index, length);
        plot(point);
    }
    if inside(size, end) {
        plot(end);
    }
}

/// Step indices in `0..length` whose major coordinate lies in `0..extent`.
fn visible_steps(extent: i32, from: i32, step: i64, length: i64) -> std::ops::Range<i64> {
    let from = i64::from(from);
    let last = i64::from(extent) - 1;
    let (first, final_index) = if step > 0 {
        (-from, last - from)
    } else {
        (from - last, from)
    };
    first.max(0)..(final_index + 1).min(length)
}

/// The minor coordinate nearest `from + index * delta / length`, rounding ties down.
fn nearest_minor(from: i32, delta: i64, index: i64, length: i64) -> i32 {
    let scaled = i64::from(from) * length + index * delta;
    (2 * scaled + length - 1).div_euclid(2 * length) as i32
}

fn inside(size: [i32; 2], point: [i32; 2]) -> bool {
    (0..size[0]).contains(&point[0]) && (0..size[1]).contains(&point[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pixels(size: [i32; 2], start: [i32; 2], end: [i32; 2]) -> Vec<[i32; 2]> {
        let mut visited = Vec::new();
        draw_centered_segment(size, start, end, &mut |point| visited.push(point));
        visited
    }

    #[test]
    fn ties_take_the_smaller_minor_coordinate_in_both_directions() {
        let forward = [[0, 0], [1, 0], [2, 1], [3, 1], [4, 2]];
        assert_eq!(pixels([8, 8], [0, 0], [4, 2]), forward);
        let mut backward = pixels([8, 8], [4, 2], [0, 0]);
        backward.sort_unstable();
        assert_eq!(backward, forward);
        assert_eq!(
            pixels([8, 8], [0, 0], [2, 4]),
            [[0, 0], [0, 1], [1, 2], [1, 3], [2, 4]]
        );
    }

    #[test]
    fn a_point_segment_inks_one_pixel() {
        assert_eq!(pixels([8, 8], [3, 3], [3, 3]), [[3, 3]]);
    }

    #[test]
    fn pixels_outside_the_raster_are_skipped_without_walking_them() {
        assert_eq!(pixels([4, 4], [-30000, 1], [30000, 1]).len(), 4);
        assert!(pixels([4, 4], [-9, -9], [-2, -2]).is_empty());
        assert_eq!(
            pixels([4, 4], [6, 2], [-2, 2]),
            [[3, 2], [2, 2], [1, 2], [0, 2]]
        );
    }

    #[test]
    fn nearest_minor_rounds_ties_down_for_negative_slopes() {
        assert_eq!(nearest_minor(0, -2, 1, 4), -1);
        assert_eq!(nearest_minor(0, 2, 1, 4), 0);
        assert_eq!(visible_steps(4, -2, 1, 10), 2..6);
    }
}
