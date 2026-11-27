//! A real concurrent setter cannot publish a frame that straddled the selection change.
use super::*;
use crate::{
    Graphic3dRenderDriver, RenderBackend, RenderFrame, RenderGuiSurfacePolicy,
    RenderLightingPipeline,
};
use std::sync::{
    Barrier,
    atomic::{AtomicBool, Ordering},
};
struct PausedLightingBackend {
    rendezvous: Arc<Barrier>,
    armed: AtomicBool,
}
impl RenderBackend for PausedLightingBackend {
    fn render(&mut self, _: &RenderFrame) -> Result<FrameCapture, RenderError> {
        if self.armed.swap(false, Ordering::SeqCst) {
            self.rendezvous.wait();
            self.rendezvous.wait();
        }
        Ok(FrameCapture {
            width: 1,
            height: 1,
            rgba: vec![0; 4],
            depth: vec![1.],
        })
    }
}
#[test]
fn concurrent_lighting_change_refuses_frame_and_allows_fresh_recovery() {
    for policy in [
        RenderGuiSurfacePolicy::LegacyV1,
        RenderGuiSurfacePolicy::CheckedV2,
    ] {
        check_concurrent_transition(policy);
    }
}
fn check_concurrent_transition(policy: RenderGuiSurfacePolicy) {
    let rendezvous = Arc::new(Barrier::new(2));
    let (driver, view) = paused_fixture(policy, rendezvous.clone());
    let setter = std::thread::spawn(move || {
        rendezvous.wait();
        driver
            .set_lighting_pipeline(RenderLightingPipeline::FixedFunction)
            .unwrap();
        rendezvous.wait();
    });
    let failure = view.try_redraw().unwrap_err();
    setter.join().unwrap();
    assert!(failure.lighting_selection_changed().is_some());
    assert!(view.capture().is_err());
    view.try_redraw().unwrap();
    assert!(view.capture().is_ok());
}
fn paused_fixture(
    policy: RenderGuiSurfacePolicy,
    rendezvous: Arc<Barrier>,
) -> (Handle<Graphic3dRenderDriver>, Handle<Graphic3dRenderView>) {
    let backend = PausedLightingBackend {
        rendezvous,
        armed: AtomicBool::new(true),
    };
    let driver =
        Graphic3dRenderDriver::with_backend_and_gui_surface_policy(backend, 64, policy).unwrap();
    let view = lighting_backend_view(&driver);
    (driver, view)
}
pub(super) fn lighting_backend_view(
    driver: &Handle<Graphic3dRenderDriver>,
) -> Handle<Graphic3dRenderView> {
    let manager = Handle::new(Graphic3dStructureManager::new(&driver.cast()).unwrap());
    let view = driver
        .create_view(&manager)
        .unwrap()
        .cast::<Graphic3dRenderView>();
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_virtual(true);
    window.set_native_handle(91);
    window.set_size(1, 1);
    view.set_window(&Handle::default(), &window.cast(), 0);
    super::view_binding_tests::attach_binding_gui(&view);
    view
}
