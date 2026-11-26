use super::*;
#[test]
fn polygon_point_coverage_is_explicitly_unknown_instead_of_filled_triangle() {
    assert_eq!(raster_coverage_tag(true, 0), 3);
    for tag in [0, 1, 2] {
        assert_eq!(raster_coverage_tag(false, tag), tag);
    }
}
