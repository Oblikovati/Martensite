//! Last old target owners must retire after every publication/binding lease.
use super::super::view_binding_tests::{attach_binding_gui, binding_fixture_for_policy};
use super::*;
use martensite_tkernel::standard::{Transient, WeakHandle};
use std::sync::atomic::{AtomicBool, AtomicUsize};

#[derive(Default)]
struct RetirementWitness {
    calls: AtomicUsize,
    blocked: AtomicBool,
    performed: AtomicBool,
    cursor_completed: AtomicBool,
    panic_after: AtomicBool,
}
impl RetirementWitness {
    fn release(&self, view: &WeakHandle<Graphic3dRenderView>) {
        self.calls.fetch_add(1, Ordering::Relaxed);
        let owner = view.upgrade();
        let Some(view) = owner.get() else {
            return;
        };
        if !self.probe_all_leases(view) {
            return;
        }
        view.clear_gui_layer().unwrap();
        self.cursor_completed
            .store(view.redraw_cursor().is_ok(), Ordering::Relaxed);
        self.performed.store(true, Ordering::Relaxed);
        if self.panic_after.load(Ordering::Relaxed) {
            panic!("NamedFinalOwnerDrop after real public mutation");
        }
    }
    fn probe_all_leases(&self, view: &Graphic3dRenderView) -> bool {
        let gui = view.gui_layers.try_write();
        let outcome = view.outcome.try_write();
        let target = view.redraw_target.try_write();
        let blocked = gui.is_none()
            || outcome.is_none()
            || target.is_none()
            || !view.retained.fbo_revision_unlocked();
        drop(target);
        drop(outcome);
        drop(gui);
        self.blocked.store(blocked, Ordering::Relaxed);
        !blocked
    }
    fn assert_completed(&self) {
        assert_eq!(
            self.calls.load(Ordering::Relaxed),
            1,
            "final owner callback must execute exactly once"
        );
        assert!(
            !self.blocked.load(Ordering::Relaxed),
            "final owner inherited a renderer/binding lease"
        );
        self.assert_mutation();
    }
    fn assert_mutation(&self) {
        assert!(
            self.performed.load(Ordering::Relaxed),
            "actual public clear must execute"
        );
        assert!(
            self.cursor_completed.load(Ordering::Relaxed),
            "actual public cursor must execute"
        );
    }
}
struct FinalOwnerWindow {
    transient: StandardTransient,
    neutral: AspectNeutralWindow,
    view: WeakHandle<Graphic3dRenderView>,
    witness: Arc<RetirementWitness>,
}
martensite_tkernel::impl_transient!(FinalOwnerWindow, "Martensite_FinalOwnerWindow", dyn AspectWindow, transient, interfaces: [dyn AspectWindow]);
impl FinalOwnerWindow {
    fn new(view: &Handle<Graphic3dRenderView>, witness: Arc<RetirementWitness>) -> Self {
        let neutral = AspectNeutralWindow::default();
        neutral.set_virtual(true);
        neutral.set_size(1, 1);
        Self {
            transient: Default::default(),
            neutral,
            view: view.downgrade(),
            witness,
        }
    }
}
impl Drop for FinalOwnerWindow {
    fn drop(&mut self) {
        self.witness.release(&self.view);
    }
}
impl AspectWindow for FinalOwnerWindow {
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
        None
    }
}
type PendingWindowRetirement = (
    Handle<Graphic3dRenderView>,
    Arc<RetirementWitness>,
    WeakHandle<FinalOwnerWindow>,
);
fn pending_window_retirement(policy: crate::RenderGuiSurfacePolicy) -> PendingWindowRetirement {
    let (view, replacement) = binding_fixture_for_policy(policy);
    let witness = Arc::new(RetirementWitness::default());
    let first = Handle::new(FinalOwnerWindow::new(&view, witness.clone()));
    view.set_window(&Handle::default(), &first.cast(), 0);
    attach_binding_gui(&view);
    view.try_redraw().unwrap();
    view.set_window(&Handle::default(), &replacement.cast(), 0);
    attach_binding_gui(&view);
    let weak = first.downgrade();
    drop(first);
    assert_eq!(witness.calls.load(Ordering::Relaxed), 0);
    assert_eq!(weak.upgrade().transient_base().ref_count(), 3);
    (view, witness, weak)
}
fn assert_window_retirement(policy: crate::RenderGuiSurfacePolicy) {
    let (view, witness, weak) = pending_window_retirement(policy);
    view.try_redraw().unwrap();
    witness.assert_completed();
    assert!(
        weak.upgrade().is_null(),
        "old window cannot be retained to avoid its callback"
    );
}
#[test]
fn normal_publication_retires_the_final_old_window_after_all_leases() {
    assert_window_retirement(crate::RenderGuiSurfacePolicy::LegacyV1);
}
#[test]
fn checked_publication_retires_the_final_old_window_after_all_leases() {
    assert_window_retirement(crate::RenderGuiSurfacePolicy::CheckedV2);
}
struct FinalOwnerFramebuffer {
    transient: StandardTransient,
    view: WeakHandle<Graphic3dRenderView>,
    witness: Arc<RetirementWitness>,
}
martensite_tkernel::impl_transient!(
    FinalOwnerFramebuffer,
    "Martensite_FinalOwnerFramebuffer",
    StandardTransient,
    transient
);
impl Drop for FinalOwnerFramebuffer {
    fn drop(&mut self) {
        self.witness.release(&self.view);
    }
}
type PendingFramebufferRetirement = (
    Handle<Graphic3dRenderView>,
    Arc<RetirementWitness>,
    WeakHandle<FinalOwnerFramebuffer>,
);
fn pending_framebuffer_retirement() -> PendingFramebufferRetirement {
    let (view, _) = binding_fixture_for_policy(crate::RenderGuiSurfacePolicy::LegacyV1);
    let witness = Arc::new(RetirementWitness::default());
    let first = Handle::new(FinalOwnerFramebuffer {
        transient: Default::default(),
        view: view.downgrade(),
        witness: witness.clone(),
    });
    view.cast::<dyn Graphic3dCView>().set_fbo(&first.cast());
    let weak = first.downgrade();
    drop(first);
    assert_eq!(witness.calls.load(Ordering::Relaxed), 0);
    assert_eq!(weak.upgrade().transient_base().ref_count(), 2);
    (view, witness, weak)
}
#[test]
fn public_fbo_replacement_retires_foreign_final_owner_outside_binding_revision_lease() {
    let (view, witness, weak) = pending_framebuffer_retirement();
    view.cast::<dyn Graphic3dCView>()
        .set_fbo(&Handle::default());
    witness.assert_completed();
    assert!(weak.upgrade().is_null());
}

struct RejectRetirementFrame;
impl crate::RenderViewFrameHooks for RejectRetirementFrame {
    fn redraw(
        &self,
        _: &Graphic3dRenderView,
        _: &mut crate::RenderFrameStep<'_>,
    ) -> Result<(), RenderError> {
        Err(RenderError::rejected(
            "NamedRetirementFrameFailure",
            "successful frame",
        ))
    }
}
struct PanicRetirementFrame;
impl crate::RenderViewFrameHooks for PanicRetirementFrame {
    fn redraw(
        &self,
        _: &Graphic3dRenderView,
        _: &mut crate::RenderFrameStep<'_>,
    ) -> Result<(), RenderError> {
        panic!("NamedRetirementFrameUnwind before publication");
    }
}
fn retirement_policies() -> [crate::RenderGuiSurfacePolicy; 2] {
    use crate::RenderGuiSurfacePolicy::{CheckedV2, LegacyV1};
    [LegacyV1, CheckedV2]
}
#[test]
fn failed_publication_retires_old_owner_and_keeps_actual_failure_on_both_policies() {
    for policy in retirement_policies() {
        let (view, witness, weak) = pending_window_retirement(policy);
        view.set_frame_hooks(Some(Arc::new(RejectRetirementFrame)));
        let error = view.try_redraw().unwrap_err();
        view.set_frame_hooks(None);
        assert!(error.to_string().contains("NamedRetirementFrameFailure"));
        witness.assert_completed();
        assert!(weak.upgrade().is_null());
    }
}
#[test]
fn old_owner_callback_unwind_has_no_publication_lease_on_either_policy() {
    for policy in retirement_policies() {
        assert_window_callback_unwind(policy);
    }
}
fn assert_window_callback_unwind(policy: crate::RenderGuiSurfacePolicy) {
    let (view, witness, weak) = pending_window_retirement(policy);
    witness.panic_after.store(true, Ordering::Relaxed);
    let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| view.try_redraw()));
    assert!(failed.is_err());
    witness.assert_completed();
    assert!(weak.upgrade().is_null());
    assert!(witness.probe_all_leases(&view));
    assert!(view.redraw_cursor().is_ok());
}
#[test]
fn attempt_unwind_keeps_prior_owner_until_the_next_real_publication() {
    for policy in retirement_policies() {
        assert_attempt_unwind_retirement(policy);
    }
}
fn assert_attempt_unwind_retirement(policy: crate::RenderGuiSurfacePolicy) {
    let (view, witness, weak) = pending_window_retirement(policy);
    view.set_frame_hooks(Some(Arc::new(PanicRetirementFrame)));
    let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| view.try_redraw()));
    assert!(failed.is_err());
    view.set_frame_hooks(None);
    assert_eq!(witness.calls.load(Ordering::Relaxed), 0);
    assert!(!weak.upgrade().is_null());
    assert!(witness.probe_all_leases(&view));
    view.try_redraw().unwrap();
    witness.assert_completed();
    assert!(weak.upgrade().is_null());
}
#[test]
fn public_fbo_final_owner_unwind_releases_sdk_and_revision_leases() {
    let (view, witness, weak) = pending_framebuffer_retirement();
    witness.panic_after.store(true, Ordering::Relaxed);
    let api = view.cast::<dyn Graphic3dCView>();
    let failed = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        api.set_fbo(&Handle::default())
    }));
    assert!(failed.is_err());
    witness.assert_completed();
    assert!(weak.upgrade().is_null());
    assert!(witness.probe_all_leases(&view));
    assert!(api.fbo().is_null());
    assert!(view.redraw_cursor().is_ok());
}
