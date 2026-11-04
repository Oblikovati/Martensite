//! Real controller calls produce complete per-case observations, including unexpected failures.
use super::view_controller_slot_fixture::*;
use super::view_controller_slot_recording::ControllerSlotRecorder;
use super::*;
use crate::v3d::V3dViewSource;
use martensite_tkernel::{
    ncollection::Vec2,
    standard::{Failure, Handle},
};
use std::{
    any::Any,
    panic::{AssertUnwindSafe, catch_unwind},
};

/// Replay the actual public/base entrypoint; `record_controller_slot_case(case)`.
pub(super) fn record_controller_slot_case(case: ControllerSlotCase) -> String {
    let mut scene = controller_slot_scene();
    let recorder = ControllerSlotRecorder::new(case);
    scene.controller.set_camera_hooks(recorder.clone());
    recorder.state("before", &scene.controller, &scene.view);
    attempt_controller_slot(&recorder, &mut scene);
    recorder.state("after", &scene.controller, &scene.view);
    if matches!(
        case.mode,
        ControllerSlotMode::TypedFailure | ControllerSlotMode::UnrelatedFailure
    ) {
        attempt_controller_slot(&recorder, &mut scene);
        recorder.state("retry", &scene.controller, &scene.view);
    }
    recorder.trace_text()
}
fn attempt_controller_slot(recorder: &ControllerSlotRecorder, scene: &mut ControllerSlotScene) {
    let outcome = catch_unwind(AssertUnwindSafe(|| run_controller_slot(recorder, scene)));
    match outcome {
        Ok(Ok(())) => {}
        Ok(Err(failure)) => recorder.failure(
            failure.exception_type(),
            failure.message(),
            &scene.controller,
        ),
        Err(payload) => record_slot_panic(recorder, payload.as_ref(), &scene.controller),
    }
}
fn record_slot_panic(
    recorder: &ControllerSlotRecorder,
    payload: &(dyn Any + Send),
    c: &AisViewController,
) {
    let message = slot_panic_message(payload);
    let kind = if message == "controller callback kind=unrelated; expected success" {
        "rust-panic"
    } else {
        "unexpected-rust-panic"
    };
    recorder.failure(kind, &message, c);
}
fn slot_panic_message(payload: &(dyn Any + Send)) -> String {
    if let Some(message) = payload.downcast_ref::<&str>() {
        return (*message).to_owned();
    }
    if let Some(message) = payload.downcast_ref::<String>() {
        return message.clone();
    }
    format!(
        "payload type={:?}; expected string unrelated callback panic",
        payload.type_id()
    )
}
fn run_controller_slot(
    recorder: &ControllerSlotRecorder,
    scene: &mut ControllerSlotScene,
) -> Result<(), Failure> {
    match recorder.case.operation {
        ControllerSlotOperation::Direct => {
            invoke_selected_slot(recorder, &mut scene.controller, &scene.context, &scene.view)
        }
        ControllerSlotOperation::DirectFlushFalse => {
            scene
                .controller
                .flush_view_events(&scene.context, &scene.view, false)
        }
        operation => run_slot_base_consumer(operation, scene),
    }
}
fn run_slot_base_consumer(
    operation: ControllerSlotOperation,
    scene: &mut ControllerSlotScene,
) -> Result<(), Failure> {
    let (c, ctx, view) = (&mut scene.controller, &scene.context, &scene.view);
    match operation {
        ControllerSlotOperation::OrbitConsumer => run_slot_orbit(c, ctx, view),
        ControllerSlotOperation::FrameConsumer => c.handle_view_events_base(ctx, view),
        ControllerSlotOperation::FlushTrue => c.flush_view_events_base(ctx, view, true),
        ControllerSlotOperation::FlushFalse => c.flush_view_events_base(ctx, view, false),
        _ => Err(Failure::new(&format!(
            "operation={operation:?}; expected qualified controller base consumer"
        ))),
    }
}
fn run_slot_orbit(
    c: &mut AisViewController,
    ctx: &Handle<AisInteractiveContext>,
    view: &Handle<dyn V3dViewSource>,
) -> Result<(), Failure> {
    c.render_input.orbit_rotation.to_start = true;
    c.render_input.orbit_rotation.point_start = Vec2::new(320.0, 240.0);
    c.handle_camera_actions(ctx, view, &AisWalkDelta::default())
}
fn invoke_selected_slot(
    recorder: &ControllerSlotRecorder,
    c: &mut AisViewController,
    ctx: &Handle<AisInteractiveContext>,
    view: &Handle<dyn V3dViewSource>,
) -> Result<(), Failure> {
    match recorder.case.slot {
        ControllerSlot::Gravity => record_direct_gravity(recorder, c, ctx, view),
        ControllerSlot::Navigation => record_direct_navigation(recorder, c, ctx, view),
        ControllerSlot::Frame => c.handle_view_events(ctx, view),
        ControllerSlot::Flush => c.flush_view_events(ctx, view, true),
    }
}
fn record_direct_gravity(
    recorder: &ControllerSlotRecorder,
    c: &mut AisViewController,
    ctx: &Handle<AisInteractiveContext>,
    view: &Handle<dyn V3dViewSource>,
) -> Result<(), Failure> {
    let point = c.gravity_point(ctx, view)?;
    recorder.point("direct-result", &point);
    Ok(())
}
fn record_direct_navigation(
    recorder: &ControllerSlotRecorder,
    c: &mut AisViewController,
    ctx: &Handle<AisInteractiveContext>,
    view: &Handle<dyn V3dViewSource>,
) -> Result<(), Failure> {
    let walk = c.handle_navigation_keys(ctx, view)?;
    recorder.walk("direct-result", &walk);
    Ok(())
}
