//! Analytic reference for one-pixel lines: the GL/Vulkan diamond-exit rule.
//!
//! GL 2.1 §3.4.1 and Vulkan "Bresenham line segment rasterization" produce fragment f when the
//! segment crosses the open diamond R_f = {|x - xf| + |y - yf| < 1/2} around the fragment centre
//! and does not end inside it. Tie cases on diamond edges are implementation-defined, and two
//! implementations may snap endpoints to different subpixel grids.
//!
//! Bound (stated per D2): every drawn pixel lies within one pixel (Chebyshev) of a rule pixel and
//! every rule pixel within one pixel of a drawn pixel; the drawn pixel count differs from the
//! rule count by at most one per segment, since each segment can gain or lose at most its
//! end fragment on a tie.
use std::collections::BTreeSet;

pub type Pixel = (i64, i64);

/// The rule's fragments of one window-space segment (pixel centres at +0.5); `diamond_exit(a, b)`.
pub fn diamond_exit(a: [f64; 2], b: [f64; 2]) -> BTreeSet<Pixel> {
    let (low, high) = (
        [a[0].min(b[0]), a[1].min(b[1])],
        [a[0].max(b[0]), a[1].max(b[1])],
    );
    let mut pixels = BTreeSet::new();
    for x in (low[0].floor() as i64 - 1)..=(high[0].ceil() as i64 + 1) {
        for y in (low[1].floor() as i64 - 1)..=(high[1].ceil() as i64 + 1) {
            let centre = [x as f64 + 0.5, y as f64 + 0.5];
            if crosses_diamond(a, b, centre) && !inside_diamond(b, centre) {
                pixels.insert((x, y));
            }
        }
    }
    pixels
}

fn inside_diamond(point: [f64; 2], centre: [f64; 2]) -> bool {
    (point[0] - centre[0]).abs() + (point[1] - centre[1]).abs() < 0.5
}

/// The segment meets the open diamond: its closest L1 approach is below 1/2.
fn crosses_diamond(a: [f64; 2], b: [f64; 2], centre: [f64; 2]) -> bool {
    // |dx| + |dy| is convex along the segment; minimise it over the breakpoints and endpoints.
    let along = |t: f64| [a[0] + t * (b[0] - a[0]), a[1] + t * (b[1] - a[1])];
    let mut candidates = vec![0., 1.];
    for k in 0..2 {
        let delta = b[k] - a[k];
        if delta != 0. {
            candidates.push(((centre[k] - a[k]) / delta).clamp(0., 1.));
        }
    }
    candidates
        .into_iter()
        .any(|t| inside_diamond(along(t), centre))
}

/// The first violation of the stated bound, if any; `rule_violation(&drawn, &rule, segments)`.
pub fn rule_violation(
    drawn: &BTreeSet<Pixel>,
    rule: &BTreeSet<Pixel>,
    segments: usize,
) -> Option<String> {
    let near = |set: &BTreeSet<Pixel>, (x, y): Pixel| {
        (-1..=1).any(|dx| (-1..=1).any(|dy| set.contains(&(x + dx, y + dy))))
    };
    if let Some(pixel) = drawn.iter().find(|pixel| !near(rule, **pixel)) {
        return Some(format!(
            "drawn pixel {pixel:?} is more than one pixel from the rule"
        ));
    }
    if let Some(pixel) = rule.iter().find(|pixel| !near(drawn, **pixel)) {
        return Some(format!(
            "rule pixel {pixel:?} has no drawn pixel within one pixel"
        ));
    }
    let excess = drawn.len().abs_diff(rule.len());
    (excess > segments).then(|| {
        format!(
            "drawn {} vs rule {} pixels; expected within {segments}",
            drawn.len(),
            rule.len()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x_major_segments_produce_one_fragment_per_column_without_the_end_fragment() {
        let pixels = diamond_exit([0.5, 0.5], [4.5, 1.7]);
        assert_eq!(pixels, BTreeSet::from([(0, 0), (1, 0), (2, 1), (3, 1)]));
        let vertical = diamond_exit([2.5, 1.5], [2.5, 4.5]);
        assert_eq!(vertical, BTreeSet::from([(2, 1), (2, 2), (2, 3)]));
    }

    #[test]
    fn the_bound_accepts_a_tie_shift_and_rejects_displaced_or_missing_lines() {
        let rule = diamond_exit([0.5, 0.5], [8.5, 0.5]);
        let shifted: BTreeSet<Pixel> = rule.iter().map(|&(x, y)| (x + 1, y)).collect();
        assert_eq!(rule_violation(&shifted, &rule, 1), None);
        let displaced: BTreeSet<Pixel> = rule.iter().map(|&(x, y)| (x, y + 2)).collect();
        assert!(rule_violation(&displaced, &rule, 1).is_some());
        let mut short = rule.clone();
        short.pop_last();
        short.pop_last();
        assert!(rule_violation(&short, &rule, 1).is_some());
        assert!(rule_violation(&BTreeSet::new(), &rule, 1).is_some());
    }
}
