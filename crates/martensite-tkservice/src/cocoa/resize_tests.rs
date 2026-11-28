use super::super::resize::{appkit_frame, bounds_corners, resize_kind, screen_corners};
use super::*;

const FRAME: (i32, i32, i32, i32) = (0, 0, 640, 480);

#[test]
fn edges_within_two_pixels_count_as_unmoved() {
    assert_eq!(
        resize_kind(FRAME, (2, -2, 642, 478)),
        AspectTypeOfResize::NoBorder
    );
    assert_eq!(
        resize_kind(FRAME, (3, 0, 640, 480)),
        AspectTypeOfResize::LeftBorder
    );
}

#[test]
fn each_moved_border_combination_maps_as_occt() {
    let cases = [
        ((0, 0, 800, 480), AspectTypeOfResize::RightBorder),
        ((0, 10, 640, 480), AspectTypeOfResize::TopBorder),
        ((0, 0, 640, 600), AspectTypeOfResize::BottomBorder),
        ((10, 10, 640, 480), AspectTypeOfResize::LeftAndTopBorder),
        ((0, 10, 800, 480), AspectTypeOfResize::TopAndRightBorder),
        ((10, 0, 640, 600), AspectTypeOfResize::BottomAndLeftBorder),
        ((0, 0, 800, 600), AspectTypeOfResize::RightAndBottomBorder),
        ((10, 10, 800, 600), AspectTypeOfResize::Unknown),
        ((10, 0, 800, 480), AspectTypeOfResize::Unknown),
    ];
    for (corners, expected) in cases {
        assert_eq!(resize_kind(FRAME, corners), expected, "{corners:?}");
    }
}

#[test]
fn appkit_and_screen_coordinates_round_trip() {
    let frame = appkit_frame(10, 20, 640, 480, 1080.0);
    assert_eq!(frame, CocoaRect::new(10.0, 580.0, 640.0, 480.0));
    assert_eq!(screen_corners(frame, 1080.0), (10, 20, 650, 500));
    assert_eq!(
        bounds_corners(CocoaRect::new(0.5, 1.5, 640.0, 480.0)),
        (0, 1, 640, 481)
    );
}
