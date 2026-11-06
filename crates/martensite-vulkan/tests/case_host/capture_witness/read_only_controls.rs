//! Count calls to the actual render backend while reading the real accepted-frame API.
use super::*;
use martensite_vulkan::{Graphic3dRenderDriver, RenderBackend, RenderError, RenderFrame};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

struct NamedCountingBackend {
    renders: Arc<AtomicUsize>,
}
impl RenderBackend for NamedCountingBackend {
    fn render(&mut self, frame: &RenderFrame) -> Result<FrameCapture, RenderError> {
        self.renders.fetch_add(1, Ordering::SeqCst);
        let [width, height] = frame.extent();
        let count = width as usize * height as usize;
        Ok(FrameCapture {
            width,
            height,
            rgba: [31, 63, 127, 255].repeat(count),
            depth: vec![0.25; count],
        })
    }
}
fn counting_session() -> (
    crate::case_view::viewer_session::ViewerSession,
    Arc<AtomicUsize>,
) {
    let renders = Arc::new(AtomicUsize::new(0));
    let backend = NamedCountingBackend {
        renders: renders.clone(),
    };
    let driver = Graphic3dRenderDriver::with_backend(backend, 64).unwrap();
    let session =
        crate::case_view::viewer_session::ViewerSession::with_view_size(&driver, 2, 2).unwrap();
    (session, renders)
}
#[test]
fn accepted_capture_reads_actual_published_buffers_without_another_render() {
    let (session, renders) = counting_session();
    let view = super::super::super::render_view(&session).unwrap();
    view.try_redraw().unwrap();
    let before = renders.load(Ordering::SeqCst);
    let mut capture = view.capture().unwrap();
    assert_eq!(renders.load(Ordering::SeqCst), before);
    assert_eq!(capture.rgba, [31, 63, 127, 255].repeat(4));
    assert_eq!(capture.depth, vec![0.25; 4]);
    capture.rgba[0] = 0;
    assert_eq!(view.capture().unwrap().rgba[0], 31);
    assert_eq!(renders.load(Ordering::SeqCst), before);
}
