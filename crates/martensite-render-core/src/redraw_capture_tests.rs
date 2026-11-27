//! A published redraw applies its host steps once and keeps the pixels it returned.
use super::RedrawCapture;
use crate::device_backend::CapturePlanes;
use crate::driver_backend::{DriverBackend, SharedRenderDevice};
use crate::view_capture::HostCaptureSteps;
use crate::{FrameCapture, RenderBackend, RenderError, RenderFrame, RenderedFrame};
use martensite_tkernel::standard::SharedState;
use std::sync::Arc;

/// A backend that a capture with host pixels must never call.
struct NeverRenderBackend;

impl RenderBackend for NeverRenderBackend {
    fn render(&mut self, _frame: &RenderFrame) -> Result<FrameCapture, RenderError> {
        panic!("a redraw capture with host pixels never renders again");
    }
}

fn injected_device() -> SharedRenderDevice {
    let backend = DriverBackend::Injected(Box::new(NeverRenderBackend));
    Arc::new(SharedState::new(backend))
}

fn two_rows() -> FrameCapture {
    FrameCapture {
        width: 1,
        height: 2,
        rgba: vec![1, 1, 1, 255, 2, 2, 2, 255],
        depth: vec![0.25, 0.75],
    }
}

fn flipped_steps() -> HostCaptureSteps {
    HostCaptureSteps {
        size: [1, 2],
        scaled: None,
        subview_blits: Vec::new(),
        flip_output: true,
    }
}

#[test]
fn host_pixels_take_the_host_steps_once() {
    let device = injected_device();
    let capture = RedrawCapture::new(RenderedFrame::Ready(two_rows()), flipped_steps());
    assert_eq!(capture.size(), [1, 2]);
    let first = capture.pixels(&device).unwrap();
    assert_eq!(first.pixel(0, 0), Some([2, 2, 2, 255]));
    assert_eq!(first.depth, vec![0.75, 0.25]);
    let again = capture.pixels(&device).unwrap();
    assert!(std::ptr::eq(first, again));
    assert_eq!(again.pixel(0, 0), Some([2, 2, 2, 255]));
}

#[test]
fn a_colour_request_returns_host_pixels_with_their_depth() {
    let capture = RedrawCapture::new(RenderedFrame::Ready(two_rows()), flipped_steps());
    let device = injected_device();
    let color = capture.planes(CapturePlanes::Color, &device).unwrap();
    assert_eq!(color.rgba, vec![2, 2, 2, 255, 1, 1, 1, 255]);
    assert_eq!(color.depth, vec![0.75, 0.25]);
}

#[test]
fn colour_steps_flip_a_capture_without_depth() {
    let mut capture = FrameCapture {
        depth: Vec::new(),
        ..two_rows()
    };
    flipped_steps().apply_color(&mut capture);
    assert_eq!(capture.rgba, vec![2, 2, 2, 255, 1, 1, 1, 255]);
    assert!(capture.depth.is_empty());
}
