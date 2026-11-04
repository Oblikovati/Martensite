//! Hand-computed blits of a subview frame into its parent (GlView::blitSubviews).
use super::*;

/// A `width`x`height` frame whose pixel (x, y) is [x, y, `tag`, 255], with depth `tag`.
fn frame(width: u32, height: u32, tag: u8) -> FrameCapture {
    let pixels = (0..height).flat_map(|y| (0..width).map(move |x| [x as u8, y as u8, tag, 255]));
    FrameCapture {
        width,
        height,
        rgba: pixels.flatten().collect(),
        depth: vec![f32::from(tag); (width * height) as usize],
    }
}
fn pixel(frame: &FrameCapture, x: u32, y: u32) -> [u8; 4] {
    frame.pixel(x, y).unwrap()
}

#[test]
fn the_subview_lands_at_its_top_left_corner_and_keeps_the_parent_depth() {
    let mut parent = frame(8, 6, 1);
    copy_frame(&mut parent, &frame(3, 2, 9), Vec2::new(4, 3));
    assert_eq!(pixel(&parent, 4, 3), [0, 0, 9, 255]);
    assert_eq!(pixel(&parent, 6, 4), [2, 1, 9, 255]);
    assert_eq!(pixel(&parent, 7, 4), [7, 4, 1, 255]);
    assert_eq!(pixel(&parent, 4, 5), [4, 5, 1, 255]);
    assert!(parent.depth.iter().all(|&depth| depth == 1.));
}
#[test]
fn blits_are_clipped_to_the_parent_frame() {
    let mut parent = frame(4, 4, 1);
    copy_frame(&mut parent, &frame(3, 3, 9), Vec2::new(2, -1));
    assert_eq!(pixel(&parent, 2, 0), [0, 1, 9, 255]);
    assert_eq!(pixel(&parent, 3, 1), [1, 2, 9, 255]);
    assert_eq!(pixel(&parent, 2, 2), [2, 2, 1, 255]);
    let untouched = frame(4, 4, 1);
    let mut outside = frame(4, 4, 1);
    copy_frame(&mut outside, &frame(3, 3, 9), Vec2::new(4, 0));
    assert_eq!(outside.rgba, untouched.rgba);
}
#[test]
fn spans_clip_on_both_sides() {
    assert_eq!(clipped_span(-2, 5, 10), Some((2, 0, 3)));
    assert_eq!(clipped_span(8, 5, 10), Some((0, 8, 2)));
    assert_eq!(clipped_span(10, 5, 10), None);
    assert_eq!(clipped_span(-5, 5, 10), None);
}
