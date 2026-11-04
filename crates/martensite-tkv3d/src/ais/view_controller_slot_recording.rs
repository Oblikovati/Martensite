//! Observations borrow the actual controller without keeping a lock across callbacks.
use super::view_controller_slot_fixture::*;
use super::*;
use crate::v3d::V3dView;
use martensite_tkmath::gp::Point3;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

pub(super) struct ControllerSlotRecorder {
    pub(super) case: ControllerSlotCase,
    pub(super) armed: AtomicBool,
    pub(super) depth: AtomicUsize,
    trace: Mutex<Vec<String>>,
}
impl ControllerSlotRecorder {
    /// Retain one callback owner through failures; `ControllerSlotRecorder::new(case)`.
    pub(super) fn new(case: ControllerSlotCase) -> Arc<Self> {
        Arc::new(Self {
            case,
            armed: AtomicBool::new(true),
            depth: AtomicUsize::new(0),
            trace: Mutex::new(Vec::new()),
        })
    }
    fn begin(&self, phase: &str) -> String {
        format!(
            "{{\"case\":\"{}\",\"phase\":\"{phase}\",\"slot\":\"{}\",\"depth\":{}",
            self.case.name,
            slot_name(self.case.slot),
            self.depth.load(Ordering::SeqCst)
        )
    }
    fn append(&self, row: String) {
        // Release the recorder before any application callback may reenter it.
        self.trace.lock().unwrap().push(row + "}");
    }
    /// Observe controller-owned buffers at the call boundary; `recorder.enter("enter", c, None)`.
    pub(super) fn enter(&self, phase: &str, c: &AisViewController, to_handle: Option<bool>) {
        let mut row =
            self.begin(phase) + &format!(",\"invoked\":\"{}\"", slot_name(self.case.slot));
        if let Some(value) = to_handle {
            row += &format!(",\"to_handle\":{}", i32::from(value));
        }
        self.append(row + &slot_queues(c));
    }
    /// Retain the actual point returned by an entrypoint; `recorder.point("return", &point)`.
    pub(super) fn point(&self, phase: &str, point: &Point3) {
        let coordinates = format!("[{},{},{}]", point.x(), point.y(), point.z());
        self.append(self.begin(phase) + ",\"point\":" + &coordinates);
    }
    /// Retain returned navigation values; `recorder.walk("return", &walk)`.
    pub(super) fn walk(&self, phase: &str, walk: &AisWalkDelta) {
        let part = walk[AisWalkTranslation::Forward];
        self.append(
            self.begin(phase)
                + &format!(
                    ",\"defined\":{},\"forward\":[{},{},{}]",
                    i32::from(walk.is_defined()),
                    part.value,
                    part.pressure,
                    part.duration
                ),
        );
    }
    /// Observe retained orbit state after the real consumer; `recorder.state("after", c, view)`.
    pub(super) fn state(&self, phase: &str, c: &AisViewController, view: &V3dView) {
        let pivot = c.rotate_pnt_3d();
        let row = self.begin(phase)
            + &format!(
                ",\"pivot\":[{},{},{}],\"scale\":{},\"ask_next\":{}",
                pivot.x(),
                pivot.y(),
                pivot.z(),
                view.camera().scale(),
                i32::from(c.to_ask_next_frame())
            );
        self.append(row + &slot_queues(c));
    }
    /// Preserve the observed language-specific failure kind; `recorder.failure(kind, message, c)`.
    pub(super) fn failure(&self, kind: &str, message: &str, c: &AisViewController) {
        let row = self.begin("failure") + &format!(",\"kind\":{kind:?},\"message\":{message:?}");
        self.append(row + &slot_queues(c));
    }
    /// Keep complete callback order for exact comparison; `recorder.trace_text()`.
    pub(super) fn trace_text(&self) -> String {
        let rows = self.trace.lock().unwrap();
        rows.join("\n") + "\n"
    }
}
fn slot_name(slot: ControllerSlot) -> &'static str {
    match slot {
        ControllerSlot::Gravity => "gravity",
        ControllerSlot::Navigation => "navigation",
        ControllerSlot::Frame => "frame",
        ControllerSlot::Flush => "flush",
    }
}
fn slot_queues(c: &AisViewController) -> String {
    let ui = queue_deltas(&c.ui_input);
    let gl = queue_deltas(&c.render_input);
    format!(",\"ui\":{ui},\"gl\":{gl}")
}
fn queue_deltas(input: &AisViewInputBuffer) -> String {
    let values: Vec<String> = input
        .zoom_actions
        .iter()
        .map(|scroll| scroll.delta.to_string())
        .collect();
    format!("[{}]", values.join(","))
}
