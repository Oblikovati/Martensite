//! The choice between the two pixel rules X servers use for zero-width segments.
use super::centered_line::draw_centered_segment;
use super::zero_line::draw_zero_width_segment;

/// Which X server rasterization a memory window reproduces for zero-width segments.
///
/// Both rules ink both endpoints; they differ only where the exact line passes halfway
/// between two pixels.
///
/// ```
/// use martensite_raster_core::{MemoryWindowSystem, ZeroWidthLineRule};
/// assert_eq!(ZeroWidthLineRule::default(), ZeroWidthLineRule::FrameBuffer);
/// let system = MemoryWindowSystem::with_line_rule(ZeroWidthLineRule::Accelerated);
/// assert_eq!(system.line_rule(), ZeroWidthLineRule::Accelerated);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ZeroWidthLineRule {
    /// The software frame-buffer rules of the X sample server, as an unaccelerated server
    /// (for example a virtual frame-buffer server) draws: Bresenham steps with the default
    /// octant tie bias and clipping that keeps every visible pixel in place.
    #[default]
    FrameBuffer,
    /// The rules of a GL-accelerated server: nearest pixel to the line between pixel
    /// centres, exact ties toward the smaller coordinate. Off-window coordinates near the
    /// 16-bit limits may still differ from a GPU by a pixel, since the GPU clips in floating point.
    Accelerated,
}

impl ZeroWidthLineRule {
    /// Visit each visible pixel of `start..=end` once; `rule.draw([8, 8], [0, 0], [4, 2], &mut plot)`.
    pub(super) fn draw(
        self,
        size: [i32; 2],
        start: [i32; 2],
        end: [i32; 2],
        plot: &mut impl FnMut([i32; 2]),
    ) {
        match self {
            Self::FrameBuffer => draw_zero_width_segment(size, start, end, plot),
            Self::Accelerated => draw_centered_segment(size, start, end, plot),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pixels(rule: ZeroWidthLineRule) -> Vec<[i32; 2]> {
        let mut visited = Vec::new();
        rule.draw([8, 8], [0, 0], [4, 2], &mut |point| visited.push(point));
        visited.sort_unstable();
        visited
    }

    #[test]
    fn the_rules_differ_only_on_exact_ties() {
        let frame_buffer = pixels(ZeroWidthLineRule::FrameBuffer);
        let accelerated = pixels(ZeroWidthLineRule::Accelerated);
        assert_eq!(frame_buffer, [[0, 0], [1, 1], [2, 1], [3, 2], [4, 2]]);
        assert_eq!(accelerated, [[0, 0], [1, 0], [2, 1], [3, 1], [4, 2]]);
    }
}
