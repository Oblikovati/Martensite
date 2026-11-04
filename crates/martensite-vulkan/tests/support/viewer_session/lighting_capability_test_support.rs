//! Named CPU-only frame I/O keeps real session/driver state while counting actual redraws.
use super::*;
use martensite_vulkan::{
    FrameCapture, RenderBackend, RenderError, RenderFrame, RenderUnlitFaceColorPolicy,
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
struct NamedLightingBackend {
    calls: Arc<AtomicUsize>,
}
impl RenderBackend for NamedLightingBackend {
    fn render(&mut self, frame: &RenderFrame) -> Result<FrameCapture, RenderError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        let [width, height] = frame.extent();
        let pixels = (width * height) as usize;
        Ok(FrameCapture {
            width,
            height,
            rgba: vec![0; pixels * 4],
            depth: vec![1.; pixels],
        })
    }
}
/// Build a real session with named CPU frame I/O; `let (s,d,count) = lighting_session(Default::default());`.
pub(crate) fn lighting_session(
    policy: RenderUnlitFaceColorPolicy,
) -> (
    ViewerSession,
    Handle<Graphic3dRenderDriver>,
    Arc<AtomicUsize>,
) {
    let calls = Arc::new(AtomicUsize::new(0));
    let backend = NamedLightingBackend {
        calls: calls.clone(),
    };
    let driver =
        Graphic3dRenderDriver::with_backend_and_unlit_face_color_policy(backend, 64, policy)
            .unwrap();
    let session = ViewerSession::with_view_size(&driver, 8, 8).unwrap();
    (session, driver, calls)
}
