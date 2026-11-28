//! `Cocoa_Window::DoResize` and `Position` arithmetic, free of AppKit so it is tested on
//! every platform.
use super::appkit::CocoaRect;
use crate::aspect::AspectTypeOfResize;

/// Edges closer than this many pixels count as unmoved (OCCT's `Abs(...) > 2`).
const EDGE_TOLERANCE: i32 = 2;

/// Window corners `(left, top, right, bottom)` in pixels.
pub(super) type CocoaCorners = (i32, i32, i32, i32);

/// The corners of view bounds as OCCT stores them: `NSMinX`, `NSMinY`, `NSMaxX`, `NSMaxY`
/// truncated to integers (AppKit's y grows upwards, so "top" is the bounds' minimum y).
/// Example: `bounds_corners(CocoaRect::new(0.0, 0.0, 640.0, 480.0))` is `(0, 0, 640, 480)`.
pub(super) fn bounds_corners(bounds: CocoaRect) -> CocoaCorners {
    (
        bounds.x as i32,
        bounds.y as i32,
        (bounds.x + bounds.width) as i32,
        (bounds.y + bounds.height) as i32,
    )
}

/// Which borders moved from `old` to `new`, as `DoResize` reports it.
/// Example: `resize_kind((0, 0, 640, 480), (0, 0, 800, 480))` is `RightBorder`.
pub(super) fn resize_kind(old: CocoaCorners, new: CocoaCorners) -> AspectTypeOfResize {
    let moved = |a: i32, b: i32| a.wrapping_sub(b).wrapping_abs() > EDGE_TOLERANCE;
    let mask = u8::from(moved(new.0, old.0))
        | u8::from(moved(new.2, old.2)) << 1
        | u8::from(moved(new.1, old.1)) << 2
        | u8::from(moved(new.3, old.3)) << 3;
    border_mode(mask)
}

/// The resize mode of a moved-border mask (left 1, right 2, top 4, bottom 8).
fn border_mode(mask: u8) -> AspectTypeOfResize {
    match mask {
        0 => AspectTypeOfResize::NoBorder,
        1 => AspectTypeOfResize::LeftBorder,
        2 => AspectTypeOfResize::RightBorder,
        4 => AspectTypeOfResize::TopBorder,
        5 => AspectTypeOfResize::LeftAndTopBorder,
        6 => AspectTypeOfResize::TopAndRightBorder,
        8 => AspectTypeOfResize::BottomBorder,
        9 => AspectTypeOfResize::BottomAndLeftBorder,
        10 => AspectTypeOfResize::RightAndBottomBorder,
        _ => AspectTypeOfResize::Unknown,
    }
}

/// The AppKit content rectangle of a window placed at `left`/`top` (top-left origin
/// screen coordinates) on a screen `screen_height` points tall.
/// Example: `appkit_frame(10, 20, 640, 480, 1080.0).y` is `580.0`.
pub(super) fn appkit_frame(
    left: i32,
    top: i32,
    width: i32,
    height: i32,
    screen_height: f64,
) -> CocoaRect {
    let y = screen_height - f64::from(top) - f64::from(height);
    CocoaRect::new(f64::from(left), y, f64::from(width), f64::from(height))
}

/// The top-left origin corners of an AppKit window `frame` on a screen `screen_height`
/// points tall (`Cocoa_Window::Position`).
/// Example: `screen_corners(CocoaRect::new(10.0, 580.0, 640.0, 480.0), 1080.0)` is `(10, 20, 650, 500)`.
pub(super) fn screen_corners(frame: CocoaRect, screen_height: f64) -> CocoaCorners {
    let left = frame.x as i32;
    let top = (screen_height - frame.y - frame.height) as i32;
    (
        left,
        top,
        left.wrapping_add(frame.width as i32),
        top.wrapping_add(frame.height as i32),
    )
}
