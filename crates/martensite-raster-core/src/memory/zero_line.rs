//! Zero-width segments drawn pixel for pixel as the X sample server's frame buffer draws them.
//!
//! Both endpoints are inked (butt caps). Exact error ties step axially in octants 2-5 and
//! diagonally elsewhere, so a segment covers the same pixels in either direction. Clipped
//! segments resume the unclipped error term, so clipping never moves a visible pixel.
use super::zero_clip::{ClipRect, LineSlope, X_DECREASING, Y_DECREASING, Y_MAJOR, zero_clip_line};

/// Octants preferring an axial step on an exact error tie: 2, 3, 4 and 5 of the X server's
/// default zero-line bias (`1 << (Y_DECREASING | Y_MAJOR)`, `1 << 7`, `1 << 6`, `1 << 4`).
const DEFAULT_ZERO_LINE_BIAS: u32 = 0xd8;

/// Bresenham parameters of one segment, before clipping.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BresenhamLine {
    step: [i32; 2],
    major: usize,
    slope: LineSlope,
    error: i32,
    minor_increment: i32,
    major_decrement: i32,
}

/// Visit every pixel of the segment `start..=end` inside a `size` raster, each exactly once.
///
/// Example: `draw_zero_width_segment([8, 8], [0, 0], [4, 2], &mut |p| pixels.push(p))`
/// visits `[0, 0], [1, 1], [2, 1], [3, 2], [4, 2]`.
pub(super) fn draw_zero_width_segment(
    size: [i32; 2],
    start: [i32; 2],
    end: [i32; 2],
    plot: &mut impl FnMut([i32; 2]),
) {
    let line = BresenhamLine::new(start, end);
    let rect = ClipRect {
        min: [0, 0],
        max: [size[0] - 1, size[1] - 1],
    };
    let outcodes = [rect.outcode(start), rect.outcode(end)];
    if outcodes[0] | outcodes[1] == 0 {
        line.walk(start, line.error, line.major_length(start, end) + 1, plot);
        return;
    }
    let Some(clipped) = zero_clip_line(rect, line.slope, [start, end], outcodes) else {
        return;
    };
    let error = if clipped.start_clipped {
        line.unwind_error(start, clipped.start)
    } else {
        line.error
    };
    let count = line.major_length(clipped.start, clipped.end) + 1;
    line.walk(clipped.start, error, count, plot);
}

impl BresenhamLine {
    /// Derive octant, tie bias and error terms; a diagonal counts as y-major.
    fn new(start: [i32; 2], end: [i32; 2]) -> Self {
        let delta = [end[0] - start[0], end[1] - start[1]];
        let absolute = delta.map(i32::unsigned_abs);
        let mut octant = 0;
        if delta[0] < 0 {
            octant |= X_DECREASING;
        }
        if delta[1] < 0 {
            octant |= Y_DECREASING;
        }
        let major = usize::from(absolute[0] <= absolute[1]);
        octant |= if major == 1 { Y_MAJOR } else { 0 };
        let bias = (DEFAULT_ZERO_LINE_BIAS >> octant) & 1;
        let [major_length, minor_length] = [absolute[major], absolute[1 - major]].map(|v| v as i32);
        Self {
            step: delta.map(|value| if value < 0 { -1 } else { 1 }),
            major,
            slope: LineSlope {
                delta: absolute,
                octant,
                bias,
            },
            error: -major_length - bias as i32,
            minor_increment: minor_length << 1,
            major_decrement: -(major_length << 1),
        }
    }

    /// Pixels along the major axis between two points of this line.
    fn major_length(&self, from: [i32; 2], to: [i32; 2]) -> i32 {
        (to[self.major] - from[self.major]).abs()
    }

    /// The error term after walking from the original start to a clipped start.
    fn unwind_error(&self, start: [i32; 2], clipped: [i32; 2]) -> i32 {
        let major_steps = (clipped[self.major] - start[self.major]).abs();
        let minor_steps = (clipped[1 - self.major] - start[1 - self.major]).abs();
        self.error
            .wrapping_add(self.major_decrement.wrapping_mul(minor_steps))
            .wrapping_add(self.minor_increment.wrapping_mul(major_steps))
    }

    /// Plot `count` pixels from `from`, stepping the minor axis whenever the error turns non-negative.
    fn walk(&self, from: [i32; 2], error: i32, count: i32, plot: &mut impl FnMut([i32; 2])) {
        let minor = 1 - self.major;
        let mut point = from;
        let mut error = error;
        for _ in 0..count {
            plot(point);
            point[self.major] += self.step[self.major];
            error = error.wrapping_add(self.minor_increment);
            if error >= 0 {
                point[minor] += self.step[minor];
                error = error.wrapping_add(self.major_decrement);
            }
        }
    }
}

#[cfg(test)]
#[path = "zero_line_tests.rs"]
mod tests;
