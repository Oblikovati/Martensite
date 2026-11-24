//! A completed frame reads its attachments only on demand and keeps them while held (#594).
use crate::{RenderBackend, RenderFrame};

const RED: [u8; 4] = [255, 0, 0, 255];
const GREEN: [u8; 4] = [0, 255, 0, 255];

fn frame(clear: [f32; 4]) -> RenderFrame {
    RenderFrame::new(12, 10, clear, Vec::new()).unwrap()
}

#[test]
fn a_frame_reads_its_attachments_only_when_asked() {
    let mut renderer = super::super::resource_lifecycle_test_support::create_resource_renderer();
    let red = frame([1., 0., 0., 1.]);
    renderer.render(&red).unwrap();
    let before = renderer.work_ledger().totals();
    let readback = renderer.render_deferred(&red).unwrap();
    let rendered = renderer.work_ledger().totals().since(&before);
    assert_eq!(rendered.read_back_bytes, 0);
    assert_eq!(rendered.buffers_created, 0);
    let capture = renderer.read_frame(&readback).unwrap();
    assert_eq!(capture.pixel(5, 4), Some(RED));
    let read = renderer.work_ledger().totals().since(&before);
    assert_eq!(read.read_back_bytes, 2 * readback.attachment_bytes() as u64);
    // The first render's readback left its colour buffer for reuse, so only depth is new.
    assert_eq!(read.buffers_created, 1);
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn a_colour_read_copies_only_the_colour_attachment() {
    let mut renderer = super::super::resource_lifecycle_test_support::create_resource_renderer();
    let readback = renderer.render_deferred(&frame([1., 0., 0., 1.])).unwrap();
    let before = renderer.work_ledger().totals();
    let color = renderer
        .read_frame_planes(&readback, super::CapturePlanes::Color)
        .unwrap();
    let read = renderer.work_ledger().totals().since(&before);
    assert_eq!(read.read_back_bytes, readback.attachment_bytes() as u64);
    assert_eq!(read.buffers_created, 1);
    assert!(color.depth.is_empty());
    let full = renderer.read_frame(&readback).unwrap();
    assert_eq!((color.width, color.height), (full.width, full.height));
    assert_eq!(color.rgba, full.rgba);
    assert_eq!(full.depth.len(), 12 * 10);
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn a_held_frame_keeps_its_pixels_while_later_frames_render() {
    let mut renderer = super::super::resource_lifecycle_test_support::create_resource_renderer();
    let held = renderer.render_deferred(&frame([1., 0., 0., 1.])).unwrap();
    let later = renderer.render_deferred(&frame([0., 1., 0., 1.])).unwrap();
    assert_ne!(held.color.handle, later.color.handle);
    assert_eq!(
        renderer.read_frame(&later).unwrap().pixel(5, 4),
        Some(GREEN)
    );
    assert_eq!(renderer.read_frame(&held).unwrap().pixel(5, 4), Some(RED));
    assert_eq!(renderer.read_frame(&held).unwrap().pixel(5, 4), Some(RED));
    assert!(renderer.validation_errors().unwrap().is_empty());
}
