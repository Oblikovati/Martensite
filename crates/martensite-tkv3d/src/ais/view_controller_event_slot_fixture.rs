//! Shared-owner overrides of the eight mouse and frame-selection virtuals; every call is
//! recorded and only the case slot applies its mode, as in
//! the recorded controller_event_slots capture.
use super::*;
use crate::v3d::V3dViewSource;
use martensite_tkernel::ncollection::Vec2;
use martensite_tkernel::standard::{Failure, Handle};
use martensite_tkservice::aspect::{AspectVKeyFlags, AspectVKeyMouse};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum EventSlot {
    Click,
    Buttons,
    Position,
    MoveTo,
    Pick,
    Highlight,
    Poly,
    Lazy,
}
const EVENT_SLOT_NAMES: [&str; 8] = [
    "click",
    "buttons",
    "position",
    "moveto",
    "pick",
    "highlight",
    "poly",
    "lazy",
];
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum EventSlotMode {
    Direct,
    Delegate,
    Reenter,
    Typed,
    Unrelated,
}
#[derive(Clone, Copy)]
pub(super) struct EventSlotCase {
    pub(super) name: &'static str,
    pub(super) slot: EventSlot,
    pub(super) mode: EventSlotMode,
}
/// Parse a recorded case name; `event_slot_case("lazy-typed")`.
pub(super) fn event_slot_case(name: &'static str) -> EventSlotCase {
    let (slot, mode) = name
        .split_once('-')
        .unwrap_or_else(|| panic!("case={name}; expected <slot>-<mode>"));
    let index = EVENT_SLOT_NAMES
        .iter()
        .position(|known| *known == slot)
        .unwrap_or_else(|| panic!("case slot={slot}; expected one of {EVENT_SLOT_NAMES:?}"));
    let slots = [
        EventSlot::Click,
        EventSlot::Buttons,
        EventSlot::Position,
        EventSlot::MoveTo,
        EventSlot::Pick,
        EventSlot::Highlight,
        EventSlot::Poly,
        EventSlot::Lazy,
    ];
    EventSlotCase {
        name,
        slot: slots[index],
        mode: event_slot_mode(mode),
    }
}
/// Parse a native override mode; `event_slot_mode("typed") == EventSlotMode::Typed`.
pub(super) fn event_slot_mode(mode: &str) -> EventSlotMode {
    use EventSlotMode as M;
    match mode {
        "direct" => M::Direct,
        "delegate" => M::Delegate,
        "reenter" => M::Reenter,
        "typed" => M::Typed,
        "unrelated" => M::Unrelated,
        _ => panic!("case mode={mode}; expected direct, delegate, reenter, typed or unrelated"),
    }
}
/// Records every virtual call, its result and the state after each attempt.
pub(super) struct EventSlotRecorder {
    pub(super) case: EventSlotCase,
    pub(super) context: Handle<AisInteractiveContext>,
    pub(super) view: Handle<dyn V3dViewSource>,
    depth: AtomicI32,
    armed: AtomicBool,
    pub(super) results: Mutex<Vec<i32>>,
    pub(super) records: Mutex<Vec<String>>,
}
type FrameFn = fn(
    &mut AisViewController,
    &Handle<AisInteractiveContext>,
    &Handle<dyn V3dViewSource>,
) -> Result<(), Failure>;
type SlotCall<'a> = Box<dyn FnOnce(&mut AisViewController) -> Result<bool, Failure> + 'a>;
impl EventSlotRecorder {
    pub(super) fn new(
        case: EventSlotCase,
        context: Handle<AisInteractiveContext>,
        view: Handle<dyn V3dViewSource>,
    ) -> Self {
        Self {
            case,
            context,
            view,
            depth: AtomicI32::new(0),
            armed: AtomicBool::new(true),
            results: Mutex::new(Vec::new()),
            records: Mutex::new(Vec::new()),
        }
    }
    pub(super) fn push(&self, row: String) {
        self.records.lock().unwrap().push(row);
    }
    fn emit(&self, slot: EventSlot, phase: &str, args: &str, result: i32) {
        let (name, depth) = (self.case.name, self.depth.load(Ordering::SeqCst));
        let slot = EVENT_SLOT_NAMES[slot as usize];
        self.push(format!(
            "{{\"case\":\"{name}\",\"slot\":\"{slot}\",\"phase\":\"{phase}\",\"depth\":{depth},\"args\":\"{args}\",\"result\":{result}}}"
        ));
    }
    /// Mirror the native override body; `recorder.dispatch(c, slot, args, true, reenter, base)?`.
    fn dispatch(
        &self,
        c: &mut AisViewController,
        slot: EventSlot,
        args: &str,
        has_result: bool,
        reenter: SlotCall<'_>,
        base: SlotCall<'_>,
    ) -> Result<bool, Failure> {
        self.emit(slot, "enter", args, -1);
        let mut result = true;
        if slot != self.case.slot {
            result = base(c)?;
        } else {
            self.effect(c, reenter)?;
            if self.case.mode == EventSlotMode::Delegate {
                result = base(c)?;
            }
        }
        self.emit(
            slot,
            "leave",
            args,
            if has_result { result as i32 } else { -1 },
        );
        Ok(result)
    }
    fn effect(&self, c: &mut AisViewController, reenter: SlotCall<'_>) -> Result<(), Failure> {
        if !self.armed.swap(false, Ordering::SeqCst) {
            return Ok(());
        }
        match self.case.mode {
            EventSlotMode::Typed => Err(Failure::with_type(
                "Standard_ProgramError",
                "event slot kind=typed; expected success",
                "",
            )),
            EventSlotMode::Unrelated => panic!("event slot kind=unrelated; expected success"),
            EventSlotMode::Reenter => {
                self.depth.fetch_add(1, Ordering::SeqCst);
                let result = reenter(c);
                self.depth.fetch_sub(1, Ordering::SeqCst);
                result.map(|_| ())
            }
            _ => Ok(()),
        }
    }
    fn frame_args(
        &self,
        context: &AisInteractiveContext,
        view: &Handle<dyn V3dViewSource>,
    ) -> String {
        let same = std::ptr::eq(context, &*self.context)
            && view.object_address() == self.view.object_address();
        (same as i32).to_string()
    }
    fn frame(
        &self,
        c: &mut AisViewController,
        slot: EventSlot,
        context: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
        call: FrameFn,
        base: FrameFn,
    ) -> Result<(), Failure> {
        let args = self.frame_args(context, view);
        let reenter: SlotCall<'_> = Box::new(move |c| call(c, context, view).map(|_| false));
        let base: SlotCall<'_> = Box::new(move |c| base(c, context, view).map(|_| false));
        self.dispatch(c, slot, &args, false, reenter, base)
            .map(|_| ())
    }
}
fn mouse_args(
    point: Vec2<i32>,
    buttons: AspectVKeyMouse,
    flags: AspectVKeyFlags,
    flag: bool,
) -> String {
    format!(
        "{},{},{},{},{}",
        point.x, point.y, buttons, flags, flag as i32
    )
}
type MouseFn = fn(
    &mut AisViewController,
    Vec2<i32>,
    AspectVKeyMouse,
    AspectVKeyFlags,
    bool,
) -> Result<bool, Failure>;
impl EventSlotRecorder {
    fn mouse(
        &self,
        c: &mut AisViewController,
        slot: EventSlot,
        (point, buttons, flags, flag): (Vec2<i32>, AspectVKeyMouse, AspectVKeyFlags, bool),
        call: MouseFn,
        base: MouseFn,
    ) -> Result<bool, Failure> {
        let args = mouse_args(point, buttons, flags, flag);
        let reenter: SlotCall<'_> = Box::new(move |c| call(c, point, buttons, flags, flag));
        let base: SlotCall<'_> = Box::new(move |c| base(c, point, buttons, flags, flag));
        self.dispatch(c, slot, &args, true, reenter, base)
    }
}
impl AisViewControllerCameraHooks for EventSlotRecorder {
    fn update_mouse_click(
        &self,
        c: &mut AisViewController,
        p: Vec2<i32>,
        b: AspectVKeyMouse,
        f: AspectVKeyFlags,
        d: bool,
    ) -> Result<bool, Failure> {
        use AisViewController as A;
        self.mouse(
            c,
            EventSlot::Click,
            (p, b, f, d),
            A::update_mouse_click,
            A::update_mouse_click_base,
        )
    }
    fn update_mouse_buttons(
        &self,
        c: &mut AisViewController,
        p: Vec2<i32>,
        b: AspectVKeyMouse,
        f: AspectVKeyFlags,
        e: bool,
    ) -> Result<bool, Failure> {
        use AisViewController as A;
        self.mouse(
            c,
            EventSlot::Buttons,
            (p, b, f, e),
            A::update_mouse_buttons,
            A::update_mouse_buttons_base,
        )
    }
    fn update_mouse_position(
        &self,
        c: &mut AisViewController,
        p: Vec2<i32>,
        b: AspectVKeyMouse,
        f: AspectVKeyFlags,
        e: bool,
    ) -> Result<bool, Failure> {
        use AisViewController as A;
        self.mouse(
            c,
            EventSlot::Position,
            (p, b, f, e),
            A::update_mouse_position,
            A::update_mouse_position_base,
        )
    }
    fn handle_move_to(
        &self,
        c: &mut AisViewController,
        ctx: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        use AisViewController as A;
        self.frame(
            c,
            EventSlot::MoveTo,
            ctx,
            view,
            A::handle_move_to,
            A::handle_move_to_base,
        )
    }
    fn handle_selection_pick(
        &self,
        c: &mut AisViewController,
        ctx: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        use AisViewController as A;
        self.frame(
            c,
            EventSlot::Pick,
            ctx,
            view,
            A::handle_selection_pick,
            A::handle_selection_pick_base,
        )
    }
    fn handle_dynamic_highlight(
        &self,
        c: &mut AisViewController,
        ctx: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        use AisViewController as A;
        let (call, base) = (
            A::handle_dynamic_highlight,
            A::handle_dynamic_highlight_base,
        );
        self.frame(c, EventSlot::Highlight, ctx, view, call, base)
    }
    fn handle_selection_poly(
        &self,
        c: &mut AisViewController,
        ctx: &Handle<AisInteractiveContext>,
        view: &Handle<dyn V3dViewSource>,
    ) -> Result<(), Failure> {
        use AisViewController as A;
        self.frame(
            c,
            EventSlot::Poly,
            ctx,
            view,
            A::handle_selection_poly,
            A::handle_selection_poly_base,
        )
    }
    fn context_lazy_move_to(
        &self,
        c: &mut AisViewController,
        ctx: &AisInteractiveContext,
        view: &Handle<dyn V3dViewSource>,
        pixel: &Vec2<i32>,
    ) -> Result<(), Failure> {
        let args = format!("{},{},{}", self.frame_args(ctx, view), pixel.x, pixel.y);
        let reenter: SlotCall<'_> =
            Box::new(move |c| c.context_lazy_move_to(ctx, view, pixel).map(|_| false));
        let base: SlotCall<'_> =
            Box::new(move |c| c.context_lazy_move_to_base(ctx, view, pixel).map(|_| false));
        self.dispatch(c, EventSlot::Lazy, &args, false, reenter, base)
            .map(|_| ())
    }
}
