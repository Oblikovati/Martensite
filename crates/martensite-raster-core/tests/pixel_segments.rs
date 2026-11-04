//! Pixel range and stable palette contracts.
use martensite_raster_core::{COLOR_NAMES, MAXCOLOR, PixelSegment};
#[test]
fn pixel_coordinates_wrap_and_endpoints_are_checked() {
    let mut segment = PixelSegment::new(-32769, 32768, 70000, -70000);
    assert_eq!(segment.points, [[32767, -32768], [4464, -4464]]);
    assert_eq!(segment.point(2), None);
    assert_eq!(segment.point_mut(2), None);
    *segment.point_mut(1).unwrap() = [5, 6];
    assert_eq!(segment.point(1), Some([5, 6]));
}
#[test]
fn geometry_palette_indices_remain_stable() {
    assert_eq!(MAXCOLOR, 15);
    assert_eq!(
        COLOR_NAMES,
        [
            "White", "Red", "Green", "Blue", "Cyan", "Gold", "Magenta", "Maroon", "Orange", "Pink",
            "Salmon", "Violet", "Yellow", "Khaki", "Coral"
        ]
    );
}

#[cfg(target_os = "linux")]
#[test]
fn default_raster_failure_keeps_value_and_expected_contract() {
    use martensite_raster_core::{RasterFailureContext, WindowFailureContext};
    let failure = RasterFailureContext::failure("existing window=9");
    assert_eq!(
        failure.message(),
        "Raster X11 existing window=9; expected a new virtual window and a resolvable palette color"
    );
}
