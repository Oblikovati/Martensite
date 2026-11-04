use super::{diamond::exits_diamond, major_axis, stipple_on};
const PIXEL_SCALE: i64 = 2;

#[test]
fn interior_crossings_cover_steep_and_shallow_slopes() {
    assert!(exits_diamond([-4, 0], [4, 0], [0, 0], PIXEL_SCALE));
    assert!(exits_diamond([-4, -4], [4, 4], [0, 0], PIXEL_SCALE));
    assert!(exits_diamond([-2, -4], [2, 4], [0, 0], PIXEL_SCALE));
}

#[test]
fn reflections_preserve_strict_interior_coverage() {
    for start in [[-5, -2], [-2, -5], [5, 2], [2, 5]] {
        let end = [-start[0], -start[1]];
        assert!(exits_diamond(start, end, [0, 0], PIXEL_SCALE));
    }
}

#[test]
fn symbolic_endpoint_perturbation_resolves_diamond_boundary_ties() {
    assert!(exits_diamond([0, 1], [2, -1], [0, 0], PIXEL_SCALE));
    assert!(!exits_diamond([0, -1], [2, -3], [0, 0], PIXEL_SCALE));
}

#[test]
fn final_endpoint_inside_diamond_is_excluded() {
    assert!(!exits_diamond([-4, 0], [0, 0], [0, 0], PIXEL_SCALE));
    assert!(exits_diamond([-4, 0], [3, 0], [0, 0], PIXEL_SCALE));
}

#[test]
fn stipple_uses_bottom_up_fragment_centers_and_native_axis_phase() {
    let pattern = 0xff18;
    assert!(stipple_on(pattern, 1, 355));
    assert!(stipple_on(pattern, 1, 153));
    assert!(!stipple_on(pattern, 1, 353));
    assert!(!stipple_on(pattern, 2, 355));
    assert!(stipple_on(1 << 1, 3, 1));
    assert!(!stipple_on(1 << 1, 3, 0));
}

#[test]
fn stipple_axis_tracks_segment_slope_and_reflection() {
    assert_eq!(major_axis([0, 0], [8, 3]), 0);
    assert_eq!(major_axis([0, 0], [3, 8]), 1);
    assert_eq!(major_axis([8, 3], [0, 0]), 0);
    assert_eq!(major_axis([3, 8], [0, 0]), 1);
}
