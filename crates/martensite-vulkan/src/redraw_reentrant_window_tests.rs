//! Named reentrant windows prove observer callbacks do not inherit internal leases.
use super::super::view_binding_tests::{attach_binding_gui, binding_fixture_for_policy};
use super::*;
use std::sync::atomic::{AtomicBool, AtomicUsize};
#[derive(Clone, Copy, PartialEq)]
enum CallbackPoint {
    Size,
    Native,
}
#[derive(Clone, Copy)]
enum CallbackMutation {
    ClearGui,
    RebindWindow,
    Redraw,
}
struct ReentrantObservationWindow {
    transient: StandardTransient,
    neutral: AspectNeutralWindow,
    view: martensite_tkernel::standard::WeakHandle<Graphic3dRenderView>,
    point: CallbackPoint,
    mutation: CallbackMutation,
    trigger_at: AtomicUsize,
    armed: AtomicBool,
    calls: AtomicUsize,
    blocked: AtomicBool,
    performed: AtomicBool,
}
martensite_tkernel::impl_transient!(ReentrantObservationWindow, "Martensite_ReentrantObservationWindow", dyn AspectWindow, transient, interfaces: [dyn AspectWindow]);
impl ReentrantObservationWindow {
    fn new(
        view: &Handle<Graphic3dRenderView>,
        point: CallbackPoint,
        mutation: CallbackMutation,
    ) -> Self {
        Self {
            transient: Default::default(),
            neutral: reentrant_neutral_window(),
            view: view.downgrade(),
            point,
            mutation,
            trigger_at: AtomicUsize::new(1),
            armed: AtomicBool::new(false),
            calls: AtomicUsize::new(0),
            blocked: AtomicBool::new(false),
            performed: AtomicBool::new(false),
        }
    }
    fn observe_callback(&self, point: CallbackPoint) {
        if point != self.point || !self.armed.load(Ordering::Relaxed) {
            return;
        }
        if self.calls.fetch_add(1, Ordering::Relaxed) != self.trigger_at.load(Ordering::Relaxed) {
            return;
        }
        self.armed.store(false, Ordering::Relaxed);
        let view = self.view.upgrade();
        if !self.probe_callback_writes(&view) {
            return;
        }
        self.perform_callback_mutation(&view);
    }
    fn probe_callback_writes(&self, view: &Graphic3dRenderView) -> bool {
        let gui = view.gui_layers.try_write();
        let outcome = view.outcome.try_write();
        let blocked = gui.is_none() || outcome.is_none();
        drop(outcome);
        drop(gui);
        self.blocked.store(blocked, Ordering::Relaxed);
        !blocked
    }
    fn perform_callback_mutation(&self, view: &Graphic3dRenderView) {
        match self.mutation {
            CallbackMutation::ClearGui => view.clear_gui_layer().unwrap(),
            CallbackMutation::Redraw => view.try_redraw().unwrap(),
            CallbackMutation::RebindWindow => {
                view.set_window(&Handle::default(), &view.window(), 0)
            }
        }
        self.performed.store(true, Ordering::Relaxed);
    }
}
fn reentrant_neutral_window() -> AspectNeutralWindow {
    let neutral = AspectNeutralWindow::default();
    neutral.set_virtual(true);
    neutral.set_size(1, 1);
    neutral
}
impl AspectWindow for ReentrantObservationWindow {
    fn window_base(&self) -> &AspectWindowBase {
        let native = &self.neutral;
        native.window_base()
    }
    fn is_mapped(&self) -> bool {
        let native = &self.neutral;
        native.is_mapped()
    }
    fn map(&self) {
        let native = &self.neutral;
        native.map();
    }
    fn unmap(&self) {
        let native = &self.neutral;
        native.unmap();
    }
    fn do_resize(&self) -> AspectTypeOfResize {
        let native = &self.neutral;
        native.do_resize()
    }
    fn do_mapping(&self) -> bool {
        let native = &self.neutral;
        native.do_mapping()
    }
    fn ratio(&self) -> f64 {
        let native = &self.neutral;
        native.ratio()
    }
    fn position(&self) -> (i32, i32, i32, i32) {
        let native = &self.neutral;
        native.position()
    }
    fn size(&self) -> (i32, i32) {
        self.observe_callback(CallbackPoint::Size);
        let native = &self.neutral;
        native.size()
    }
    fn native_handle(&self) -> AspectDrawable {
        let native = &self.neutral;
        native.native_handle()
    }
    fn native_parent_handle(&self) -> AspectDrawable {
        let native = &self.neutral;
        native.native_parent_handle()
    }
    fn native_fb_config(&self) -> AspectFbConfig {
        let native = &self.neutral;
        native.native_fb_config()
    }
    fn native_window_closure(&self) -> Option<AspectNativeWindowClosure> {
        self.observe_callback(CallbackPoint::Native);
        None
    }
}
fn assert_reentrant_observer(point: CallbackPoint, policy: crate::RenderGuiSurfacePolicy) {
    assert_reentrant_mutation(point, policy, CallbackMutation::ClearGui, false);
}
fn assert_reentrant_mutation(
    point: CallbackPoint,
    policy: crate::RenderGuiSurfacePolicy,
    mutation: CallbackMutation,
    issue: bool,
) {
    let (view, window) = reentrant_fixture(point, policy, mutation);
    let before = view.redraw_cursor().unwrap();
    view.try_redraw().unwrap();
    window
        .trigger_at
        .store(usize::from(!issue), Ordering::Relaxed);
    window.armed.store(true, Ordering::Relaxed);
    let result = if issue {
        view.redraw_cursor()
    } else {
        view.check_redraw_since(&before)
    };
    require_reentrant_refusal(&window, result);
}
fn reentrant_fixture(
    point: CallbackPoint,
    policy: crate::RenderGuiSurfacePolicy,
    mutation: CallbackMutation,
) -> (
    Handle<Graphic3dRenderView>,
    Handle<ReentrantObservationWindow>,
) {
    let (view, _) = binding_fixture_for_policy(policy);
    let window = Handle::new(ReentrantObservationWindow::new(&view, point, mutation));
    view.set_window(&Handle::default(), &window.cast(), 0);
    attach_binding_gui(&view);
    (view, window)
}
fn require_reentrant_refusal(
    window: &ReentrantObservationWindow,
    result: Result<RenderRedrawCursor, RenderRedrawFailure>,
) {
    assert!(
        !window.blocked.load(Ordering::Relaxed),
        "observer callback inherited GUI/outcome lease"
    );
    assert!(
        window.performed.load(Ordering::Relaxed),
        "the armed mutation must actually execute"
    );
    assert!(matches!(
        result,
        Err(RenderRedrawFailure::TargetChanged { .. })
    ));
}

#[test]
fn checked_extent_callback_can_reenter_gui_without_a_lock_cycle() {
    assert_reentrant_observer(
        CallbackPoint::Size,
        crate::RenderGuiSurfacePolicy::CheckedV2,
    );
}
#[test]
fn late_native_callback_can_reenter_gui_without_a_lock_cycle() {
    assert_reentrant_observer(
        CallbackPoint::Native,
        crate::RenderGuiSurfacePolicy::LegacyV1,
    );
}

#[test]
fn native_callback_rebinds_the_real_window_and_refuses_stale_completion() {
    assert_reentrant_mutation(
        CallbackPoint::Native,
        crate::RenderGuiSurfacePolicy::CheckedV2,
        CallbackMutation::RebindWindow,
        false,
    );
}
#[test]
fn cursor_issue_reconciles_a_real_reentrant_window_rebind() {
    assert_reentrant_mutation(
        CallbackPoint::Native,
        crate::RenderGuiSurfacePolicy::LegacyV1,
        CallbackMutation::RebindWindow,
        true,
    );
}

#[test]
fn observer_refuses_a_publication_made_by_a_reentrant_callback() {
    let (view, window) = reentrant_fixture(
        CallbackPoint::Native,
        crate::RenderGuiSurfacePolicy::LegacyV1,
        CallbackMutation::Redraw,
    );
    let before = view.redraw_cursor().unwrap();
    view.try_redraw().unwrap();
    window.trigger_at.store(0, Ordering::Relaxed);
    window.armed.store(true, Ordering::Relaxed);
    let result = view.check_redraw_since(&before);
    assert!(!window.blocked.load(Ordering::Relaxed));
    assert!(window.performed.load(Ordering::Relaxed));
    assert!(matches!(
        result,
        Err(RenderRedrawFailure::UnexpectedPublications {
            before: 1,
            observed: 2
        })
    ));
}
