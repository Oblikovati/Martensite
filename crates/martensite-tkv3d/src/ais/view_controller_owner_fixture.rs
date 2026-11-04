//! Three callback slots share actual CPU views and one retained observation owner.
use super::view_controller_slot_fixture::controller_slot_scene;
use super::*;
use crate::v3d::V3dViewSource;
use martensite_tkernel::standard::{Failure, Handle};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

#[derive(Clone, Copy, PartialEq)]
pub(super) enum SelectionOwnerSlot {
    Selection,
    Drag,
    Subview,
}
#[derive(Clone, Copy, PartialEq)]
pub(super) enum SelectionOwnerMode {
    Direct,
    Delegate,
    Reenter,
    Typed,
    Unrelated,
}
#[derive(Clone, Copy)]
pub(super) struct SelectionOwnerCase {
    pub(super) name: &'static str,
    pub(super) slot: SelectionOwnerSlot,
    pub(super) mode: SelectionOwnerMode,
    pub(super) action: AisDragAction,
}
pub(super) struct SelectionOwnerCall {
    pub(super) slot: SelectionOwnerSlot,
    pub(super) context: Handle<AisInteractiveContext>,
    pub(super) view: Handle<dyn V3dViewSource>,
    pub(super) new_view: Handle<dyn V3dViewSource>,
    pub(super) action: AisDragAction,
}
pub(super) struct SelectionOwnerRecorder {
    pub(super) case: SelectionOwnerCase,
    pub(super) expected: SelectionOwnerCall,
    pub(super) armed: AtomicBool,
    pub(super) depth: AtomicUsize,
    pub(super) records: Mutex<Vec<String>>,
    pub(super) panic_token: Arc<AtomicUsize>,
}
struct SelectionLegacyDefaults;
impl AisViewControllerHooks for SelectionLegacyDefaults {}
pub(super) struct SelectionOwnerPanic {
    pub(super) token: Arc<AtomicUsize>,
    pub(super) message: &'static str,
}
impl SelectionOwnerRecorder {
    /// Keep recorder state independently reachable; `SelectionOwnerRecorder::new(case, expected)`.
    pub(super) fn new(case: SelectionOwnerCase, expected: SelectionOwnerCall) -> Arc<Self> {
        Arc::new(Self {
            case,
            expected,
            armed: AtomicBool::new(true),
            depth: AtomicUsize::new(0),
            records: Mutex::new(Vec::new()),
            panic_token: Arc::new(AtomicUsize::new(17)),
        })
    }
    /// Emit actual arguments, never expected replacements; `recorder.emit("enter", call)`.
    pub(super) fn emit(&self, phase: &str, call: &SelectionOwnerCall) {
        let valid = self.valid_arguments(call);
        let action = if call.slot == SelectionOwnerSlot::Drag {
            call.action as i32
        } else {
            -1
        };
        let row = format!(
            "{{\"case\":\"{}\",\"phase\":\"{phase}\",\"depth\":{},\"actual_arguments_match\":{},\"action\":{action}}}",
            self.case.name,
            self.depth.load(Ordering::SeqCst),
            i32::from(valid)
        );
        self.records.lock().unwrap().push(row);
    }
    fn valid_arguments(&self, call: &SelectionOwnerCall) -> bool {
        call.slot == self.case.slot
            && (call.slot != SelectionOwnerSlot::Drag || call.action == self.case.action)
            && call.context.object_address() == self.expected.context.object_address()
            && call.view.object_address() == self.expected.view.object_address()
            && (call.slot != SelectionOwnerSlot::Subview
                || call.new_view.object_address() == self.expected.new_view.object_address())
    }
    /// Keep failures in the trace with their real domain; `recorder.failure(kind, message)`.
    pub(super) fn failure(&self, kind: &str, message: &str) {
        let row = format!(
            "{{\"case\":\"{}\",\"phase\":\"failure\",\"kind\":\"{kind}\",\"message\":\"{message}\"}}",
            self.case.name
        );
        self.records.lock().unwrap().push(row);
    }
    /// Record first, then perform real effects without holding the recorder lock; `recorder.invoke(c, call)`.
    pub(super) fn invoke(
        &self,
        c: &mut AisViewController,
        call: SelectionOwnerCall,
    ) -> Result<(), Failure> {
        self.emit("enter", &call);
        self.effect(c, &call)?;
        if self.case.mode == SelectionOwnerMode::Delegate {
            selection_owner_base(c, &call)?;
        }
        self.emit("leave", &call);
        Ok(())
    }
    fn effect(&self, c: &mut AisViewController, call: &SelectionOwnerCall) -> Result<(), Failure> {
        if !self.armed.swap(false, Ordering::SeqCst) {
            return Ok(());
        }
        match self.case.mode {
            SelectionOwnerMode::Typed => Err(Failure::with_type(
                "Standard_ProgramError",
                "selection callback kind=typed; expected success",
                "",
            )),
            SelectionOwnerMode::Unrelated => self.panic_unrelated(),
            SelectionOwnerMode::Reenter => self.reenter(c, call),
            _ => Ok(()),
        }
    }
    fn panic_unrelated(&self) -> ! {
        let payload = SelectionOwnerPanic {
            token: self.panic_token.clone(),
            message: "selection callback kind=unrelated; expected success",
        };
        std::panic::panic_any(payload)
    }
    fn reenter(&self, c: &mut AisViewController, call: &SelectionOwnerCall) -> Result<(), Failure> {
        self.depth.fetch_add(1, Ordering::SeqCst);
        let result = selection_owner_dispatch(c, call);
        self.depth.fetch_sub(1, Ordering::SeqCst);
        result
    }
}
/// Invoke an actual public callback; `selection_owner_dispatch(controller, call)`.
pub(super) fn selection_owner_dispatch(
    c: &mut AisViewController,
    call: &SelectionOwnerCall,
) -> Result<(), Failure> {
    match call.slot {
        SelectionOwnerSlot::Selection => c.on_selection_changed(&call.context, &call.view),
        SelectionOwnerSlot::Drag => c.on_object_dragged(&call.context, &call.view, call.action),
        SelectionOwnerSlot::Subview => {
            c.on_subview_changed(&call.context, &call.view, &call.new_view)
        }
    }
}
/// Delegate only real base behavior; `selection_owner_base(controller, call)`.
pub(super) fn selection_owner_base(
    c: &mut AisViewController,
    call: &SelectionOwnerCall,
) -> Result<(), Failure> {
    match call.slot {
        SelectionOwnerSlot::Selection => {
            SelectionLegacyDefaults.on_selection_changed(c, &call.context, &call.view)
        }
        SelectionOwnerSlot::Subview => {
            SelectionLegacyDefaults.on_subview_changed(c, &call.context, &call.view, &call.new_view)
        }
        SelectionOwnerSlot::Drag => c.object_dragged_base(&call.context, &call.view, call.action),
    }
}
/// Reuse the existing real CPU scene, without displayed or detected objects; `selection_owner_scene(case)`.
pub(super) fn selection_owner_scene(
    case: SelectionOwnerCase,
) -> (
    super::view_controller_slot_fixture::ControllerSlotScene,
    Arc<SelectionOwnerRecorder>,
) {
    let scene = controller_slot_scene();
    let new_view = scene.view.viewer().create_view().unwrap();
    let expected = SelectionOwnerCall {
        slot: case.slot,
        context: scene.context.clone(),
        view: scene.view.clone(),
        new_view,
        action: case.action,
    };
    let recorder = SelectionOwnerRecorder::new(case, expected);
    (scene, recorder)
}
