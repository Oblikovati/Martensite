//! A redraw leaves its frame on the GPU, and the flight waits only when it is full (#594).
use super::FRAMES_IN_FLIGHT;
use crate::{RenderBackend, RenderFrame};
use std::time::Duration;

const CLEARS: [[f32; 4]; 3] = [[1., 0., 0., 1.], [0., 1., 0., 1.], [0., 0., 1., 1.]];
const PIXELS: [[u8; 4]; 3] = [[255, 0, 0, 255], [0, 255, 0, 255], [0, 0, 255, 255]];

fn frame(clear: [f32; 4]) -> RenderFrame {
    RenderFrame::new(12, 10, clear, Vec::new()).unwrap()
}

#[test]
fn frames_stay_in_flight_until_the_flight_is_full() {
    let mut renderer = super::super::resource_lifecycle_test_support::create_resource_renderer();
    let mut readbacks = Vec::new();
    for (count, clear) in CLEARS.into_iter().enumerate() {
        readbacks.push(renderer.render_deferred(&frame(clear)).unwrap());
        assert_eq!(renderer.flight.len(), (count + 1).min(FRAMES_IN_FLIGHT));
    }
    for (readback, pixel) in readbacks.iter().zip(PIXELS) {
        assert_eq!(
            renderer.read_frame(readback).unwrap().pixel(5, 4),
            Some(pixel)
        );
    }
    renderer.finish_frames().unwrap();
    assert_eq!(renderer.flight.len(), 0);
    assert!(renderer.validation_errors().unwrap().is_empty());
}

#[test]
fn a_redraw_with_room_in_the_flight_waits_for_nothing() {
    let mut renderer = super::super::resource_lifecycle_test_support::create_resource_renderer();
    renderer.render(&frame(CLEARS[0])).unwrap();
    renderer.finish_frames().unwrap();
    let before = renderer.work_ledger().totals();
    renderer.render_deferred(&frame(CLEARS[0])).unwrap();
    let submitted = renderer.work_ledger().totals().since(&before);
    assert_eq!(submitted.times.waiting, Duration::ZERO, "{submitted:?}");
    assert_eq!(submitted.submissions, 1, "{submitted:?}");
    renderer.finish_frames().unwrap();
    let finished = renderer.work_ledger().totals().since(&before);
    assert!(finished.times.waiting > Duration::ZERO, "{finished:?}");
    assert_eq!(renderer.flight.len(), 0);
}
