//! External fixture catches observe real panic payloads without changing production hooks.
use super::view_controller_owner_callbacks::{SelectionDefaultArc, SelectionLegacyCallbacks};
use super::view_controller_owner_fixture::*;
use super::*;
use std::{
    panic::{AssertUnwindSafe, catch_unwind},
    sync::Arc,
};

#[derive(Clone, Copy)]
pub(super) enum SelectionOwnerInterface {
    Arc,
    Legacy,
    LegacyWithDefaultArc,
}
/// Replay one complete callback/retry case; `selection_owner_replay(case, SelectionOwnerInterface::Arc)`.
pub(super) fn selection_owner_replay(
    case: SelectionOwnerCase,
    interface: SelectionOwnerInterface,
) -> Vec<String> {
    let (mut scene, recorder) = selection_owner_scene(case);
    install_selection_owner(&mut scene.controller, &recorder, interface);
    owner_attempt(&recorder, &mut scene.controller);
    if matches!(
        case.mode,
        SelectionOwnerMode::Typed | SelectionOwnerMode::Unrelated
    ) {
        owner_attempt(&recorder, &mut scene.controller);
    }
    assert!(scene.controller.drag_object().is_null());
    assert!(scene.controller.drag_owner().is_null());
    recorder.records.lock().unwrap().clone()
}
fn install_selection_owner(
    c: &mut AisViewController,
    recorder: &Arc<SelectionOwnerRecorder>,
    interface: SelectionOwnerInterface,
) {
    if matches!(interface, SelectionOwnerInterface::Arc) {
        c.set_camera_hooks(recorder.clone());
        return;
    }
    c.set_hooks(Box::new(SelectionLegacyCallbacks(recorder.clone())));
    if matches!(interface, SelectionOwnerInterface::LegacyWithDefaultArc) {
        c.set_camera_hooks(Arc::new(SelectionDefaultArc));
    }
}
fn owner_attempt(recorder: &SelectionOwnerRecorder, c: &mut AisViewController) {
    let result = catch_unwind(AssertUnwindSafe(|| {
        selection_owner_dispatch(c, &recorder.expected)
    }));
    match result {
        Ok(Ok(())) => {}
        Ok(Err(failure)) => recorder.failure(failure.exception_type(), failure.message()),
        Err(payload) => record_owner_panic(recorder, payload),
    }
}
fn record_owner_panic(recorder: &SelectionOwnerRecorder, payload: Box<dyn std::any::Any + Send>) {
    let marker = payload
        .as_ref()
        .downcast_ref::<SelectionOwnerPanic>()
        .unwrap_or_else(|| {
            panic!(
                "callback payload type={:?}; expected SelectionOwnerPanic",
                payload.as_ref().type_id()
            )
        });
    assert!(
        Arc::ptr_eq(&marker.token, &recorder.panic_token),
        "panic payload identity: expected original callback token"
    );
    recorder.failure("rust-panic", marker.message);
}
