//! Explicit native-window witness reuses the existing owned Xlib fixture and production command encoder.
use super::{NativeWindow, native_gui_frame, native_validation_renderer};
use crate::backend::{
    VulkanRenderer, WindowPresenter,
    owned_window_submission::OwnedWindowFrame,
    resource_lifecycle_test_support::{FaultingCloseCalls, inject_completion_failure},
    retained_submission::RetainedOwnerSnapshot,
    window_gui_contract::WindowGuiContract,
    window_submission::WindowSubmissionSignals,
};
use crate::{RenderDeviceCloseOutcome, RenderGuiSurfacePolicy};
use ash::vk::Handle;
struct PendingWindowWitness {
    renderer: VulkanRenderer,
    monitor: std::sync::Arc<crate::backend::validation::ValidationMessages>,
    surface: std::sync::Weak<crate::backend::window_surface_lease::WindowSurfaceLease>,
    expected: RetainedOwnerSnapshot,
}
fn inject_window_frame(
    renderer: &mut VulkanRenderer,
    presenter: &mut WindowPresenter,
) -> RetainedOwnerSnapshot {
    let frame = native_gui_frame([64, 64]);
    let owned = OwnedWindowFrame::new(renderer, &frame).unwrap();
    acquire_and_encode_owned_window(renderer, presenter, &frame, &owned);
    let chain = presenter.chain.as_ref().unwrap();
    let expected = window_owner_identity(&owned, chain);
    let signals = WindowSubmissionSignals::new(chain);
    let primary =
        inject_completion_failure(&owned.commands, &signals.waits, &signals.signals).unwrap_err();
    let error = owned.reject(renderer, presenter, primary);
    assert!(error.device_failure().is_some());
    assert!(presenter.chain.is_none());
    expected
}
impl PendingWindowWitness {
    fn new() -> Self {
        let native = NativeWindow::new(64, 64);
        let mut renderer = native_validation_renderer();
        let monitor = renderer.runtime.validation.clone();
        let mut presenter = renderer.window_presenter(&native.window.cast()).unwrap();
        let surface = presenter.surface_observer();
        let expected = inject_window_frame(&mut renderer, &mut presenter);
        drop(presenter);
        drop(native);
        Self {
            renderer,
            monitor,
            surface,
            expected,
        }
    }
    fn assert_incomplete(&mut self) {
        assert!(self.surface.upgrade().is_some());
        assert_eq!(
            self.renderer.retirements.snapshot().as_slice(),
            std::slice::from_ref(&self.expected)
        );
        let mut calls = FaultingCloseCalls { calls: 0 };
        assert!(matches!(
            self.renderer.close_dispatched(&mut calls),
            RenderDeviceCloseOutcome::Incomplete { .. }
        ));
        assert_eq!(calls.calls, 1);
        assert!(self.surface.upgrade().is_some());
        assert_eq!(
            self.renderer.retirements.snapshot().as_slice(),
            std::slice::from_ref(&self.expected)
        );
    }
    fn close_actual(mut self) {
        assert_eq!(
            self.renderer.close(),
            RenderDeviceCloseOutcome::Closed { failure: None }
        );
        assert!(self.renderer.retirements.snapshot().is_empty());
        assert!(self.surface.upgrade().is_none());
        drop(self.renderer);
        assert!(self.monitor.is_observable());
        assert!(
            self.monitor.errors().unwrap().is_empty(),
            "{:?}",
            self.monitor.errors().unwrap()
        );
        assert_eq!(std::sync::Arc::strong_count(&self.monitor), 1);
    }
}
#[test]
#[ignore = "requires explicit exclusive native/GPU lease and pinned validation layer"]
fn failed_window_submission_keeps_chain_surface_and_native_parent_until_real_close() {
    let mut witness = PendingWindowWitness::new();
    witness.assert_incomplete();
    witness.close_actual();
}

fn window_owner_identity(
    owned: &OwnedWindowFrame,
    chain: &crate::backend::window_chain::WindowChain,
) -> RetainedOwnerSnapshot {
    RetainedOwnerSnapshot::Window {
        command: Some(owned.commands.command.as_raw()),
        swapchain: chain.handle.as_raw(),
    }
}

fn acquire_and_encode_owned_window(
    renderer: &VulkanRenderer,
    presenter: &mut WindowPresenter,
    frame: &crate::RenderFrame,
    owned: &OwnedWindowFrame,
) {
    let contract = WindowGuiContract::new(frame, RenderGuiSurfacePolicy::CheckedV2);
    let index = presenter.acquire([64, 64], false, contract).unwrap();
    renderer
        .encode_gui_window(frame, &owned.commands, &owned.resources, presenter, index)
        .unwrap();
}
