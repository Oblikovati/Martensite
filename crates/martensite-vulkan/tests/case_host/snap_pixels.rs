//! Pixels whose coverage depends on vertex snapping alone, written into `<image>.ties.json`
//! (`snap_pixels`) for the cross-driver `snap_coverage` grader (D2, AMD radeonsi vs RADV,
//! accepted 2026-10-28; the recording tool).
//!
//! RADV snaps window coordinates to 1/256 pixel and radeonsi to 1/256 or 1/4096 (docs/
//! vulkan-raster-precision.md), so each driver moves a vertex by at most half its step per axis,
//! and an edge, which interpolates its endpoints, by at most sqrt(2) times that. A pixel centre
//! closer than the sum to a submitted triangle edge may fall on either side on either driver.
use std::collections::BTreeSet;

/// `sqrt(2) * (1/512 + 1/8192)` px, about 0.0029 px.
pub(crate) const SNAP_COVERAGE_BOUND: f64 = std::f64::consts::SQRT_2 * (1. / 512. + 1. / 8192.);

/// Pixels `(column, row)` of a `size` window whose centre lies within `SNAP_COVERAGE_BOUND`
/// of an edge of `triangles` (window coordinates, rows top-down); `snap_pixels(&t, [409, 409])`.
pub(crate) fn snap_pixels(triangles: &[[[f64; 2]; 3]], size: [i32; 2]) -> BTreeSet<(i32, i32)> {
    let mut pixels = BTreeSet::new();
    for triangle in triangles {
        for k in 0..3 {
            edge_snap_pixels(triangle[k], triangle[(k + 1) % 3], size, &mut pixels);
        }
    }
    pixels
}

/// Walk the edge's major axis one pixel centre at a time; only the nearest minor-axis centre
/// can lie within the bound, which is far below half a pixel.
fn edge_snap_pixels(a: [f64; 2], b: [f64; 2], size: [i32; 2], out: &mut BTreeSet<(i32, i32)>) {
    let major = usize::from((b[1] - a[1]).abs() > (b[0] - a[0]).abs());
    let (low, high) = (a[major].min(b[major]), a[major].max(b[major]));
    for index in (low - 0.5).floor() as i64..=(high + 0.5).ceil() as i64 {
        let along = index as f64 + 0.5;
        let span = b[major] - a[major];
        let t = if span == 0. {
            0.
        } else {
            ((along - a[major]) / span).clamp(0., 1.)
        };
        let across = ((a[1 - major] + t * (b[1 - major] - a[1 - major])) - 0.5).round() + 0.5;
        let mut centre = [0.; 2];
        (centre[major], centre[1 - major]) = (along, across);
        if segment_distance(centre, a, b) <= SNAP_COVERAGE_BOUND {
            insert_inside(centre, size, out);
        }
    }
}

fn insert_inside(centre: [f64; 2], size: [i32; 2], out: &mut BTreeSet<(i32, i32)>) {
    let (column, row) = (centre[0].floor() as i64, centre[1].floor() as i64);
    if (0..i64::from(size[0])).contains(&column) && (0..i64::from(size[1])).contains(&row) {
        out.insert((column as i32, row as i32));
    }
}

/// Euclidean distance from `p` to the segment `a`-`b`.
fn segment_distance(p: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
    let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
    let length = dx * dx + dy * dy;
    let t = if length == 0. {
        0.
    } else {
        (((p[0] - a[0]) * dx + (p[1] - a[1]) * dy) / length).clamp(0., 1.)
    };
    martensite_cmath::hypot(p[0] - a[0] - t * dx, p[1] - a[1] - t * dy)
}

/// `"snap_pixels": [[x, y], ...]` for the sidecar.
pub(crate) fn snap_pixels_json(pixels: &BTreeSet<(i32, i32)>) -> String {
    let items: Vec<String> = pixels.iter().map(|(x, y)| format!("[{x}, {y}]")).collect();
    format!("\"snap_pixels\": [{}]", items.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_centre_a_ten_thousandth_from_an_edge_snaps() {
        // The edge passes 0.0002 px left of the centre (10.5, 20.5), as in bug29807_b5a (162,74).
        let triangle = [[10.4998, 0.], [10.4998, 40.], [0., 20.]];
        assert!(snap_pixels(&[triangle], [64, 64]).contains(&(10, 20)));
    }

    #[test]
    fn a_centre_a_hundredth_from_an_edge_does_not_snap() {
        let triangle = [[10.49, 0.], [10.49, 40.], [0., 20.]];
        assert!(!snap_pixels(&[triangle], [64, 64]).contains(&(10, 20)));
        let diagonal = [[0.01, 0.], [40.01, 40.], [0., 40.]];
        assert!(
            snap_pixels(&[diagonal], [64, 64])
                .iter()
                .all(|&(x, y)| x != y)
        );
    }

    #[test]
    fn diagonal_edges_and_segment_ends_are_measured_exactly() {
        let diagonal = [[0., 0.], [40., 40.], [0., 40.]];
        let pixels = snap_pixels(&[diagonal], [64, 64]);
        assert!((0..40).all(|i| pixels.contains(&(i, i))));
        assert!(!pixels.contains(&(41, 41)), "beyond the end of the segment");
        assert_eq!(
            snap_pixels_json(&BTreeSet::from([(1, 2)])),
            "\"snap_pixels\": [[1, 2]]"
        );
    }
}
