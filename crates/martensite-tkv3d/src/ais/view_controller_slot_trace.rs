//! Shared recorder of the derived-controller witnesses: every recorded virtual emits an enter
//! and a leave row, only the case slot applies its mode (direct, delegate, reentry, typed
//! failure, unrelated panic), and slots outside the witness pass through unrecorded, as the
//! native witnesses do for virtuals they do not override.
use super::AisViewController;
use super::view_controller_event_slot_fixture::{EventSlotMode, event_slot_mode};
use martensite_tkernel::standard::Failure;
use martensite_tkmath::gp::{Direction3, Point3};
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

/// Exact text of a double, as the witnesses write it; `bits(1.0) == "3ff0000000000000"`.
pub(super) fn bits(value: f64) -> String {
    format!("{:016x}", value.to_bits())
}
pub(super) fn point_bits(point: &Point3) -> String {
    format!(
        "{},{},{}",
        bits(point.x()),
        bits(point.y()),
        bits(point.z())
    )
}
pub(super) fn dir_bits(dir: &Direction3) -> String {
    format!("{},{},{}", bits(dir.x()), bits(dir.y()), bits(dir.z()))
}
/// Whether a call reenters the virtual or runs its base body.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Via {
    Reenter,
    Base,
}
pub(super) type SlotCall<'a> =
    &'a mut dyn FnMut(&mut AisViewController, Via) -> Result<bool, Failure>;
pub(super) type VoidCall<'a> =
    &'a mut dyn FnMut(&mut AisViewController, Via) -> Result<(), Failure>;
/// One witness case: its name `<slot>-<mode>` and the slots its witness overrides.
#[derive(Clone, Copy)]
pub(super) struct SlotTraceCase {
    pub(super) name: &'static str,
    pub(super) slot: &'static str,
    pub(super) mode: EventSlotMode,
    pub(super) recorded: &'static [&'static str],
    /// Witness label in failure messages, "frame" or "input".
    pub(super) label: &'static str,
}
/// Parse a recorded case name against the slots; `slot_trace_case("orbit-typed", FRAME, "frame")`.
pub(super) fn slot_trace_case(
    name: &'static str,
    recorded: &'static [&'static str],
    label: &'static str,
) -> SlotTraceCase {
    let (slot, mode) = name
        .split_once('-')
        .unwrap_or_else(|| panic!("case={name}; expected <slot>-<mode>"));
    let slot = recorded
        .iter()
        .copied()
        .find(|known| *known == slot)
        .unwrap_or_else(|| panic!("case slot={slot}; expected one of {recorded:?}"));
    SlotTraceCase {
        name,
        slot,
        mode: event_slot_mode(mode),
        recorded,
        label,
    }
}
/// Records every overridden call and the attempt results; see the module documentation.
pub(super) struct SlotTrace {
    pub(super) case: SlotTraceCase,
    depth: AtomicI32,
    armed: AtomicBool,
    pub(super) results: Mutex<Vec<i32>>,
    pub(super) records: Mutex<Vec<String>>,
}
impl SlotTrace {
    pub(super) fn new(case: SlotTraceCase) -> Self {
        Self {
            case,
            depth: AtomicI32::new(0),
            armed: AtomicBool::new(true),
            results: Mutex::new(Vec::new()),
            records: Mutex::new(Vec::new()),
        }
    }
    pub(super) fn push(&self, row: String) {
        self.records.lock().unwrap().push(row);
    }
    pub(super) fn keep(&self, value: bool) {
        self.results.lock().unwrap().push(value as i32);
    }
    /// One trace row; `result` is already JSON (a number, or a quoted value text).
    fn emit(&self, slot: &str, phase: &str, args: &str, result: &str) {
        let (name, depth) = (self.case.name, self.depth.load(Ordering::SeqCst));
        self.push(format!(
            "{{\"case\":\"{name}\",\"slot\":\"{slot}\",\"phase\":\"{phase}\",\"depth\":{depth},\"args\":\"{args}\",\"result\":{result}}}"
        ));
    }
    /// Mirror the native override body; `trace.dispatch(c, "zoom", || args, true, &mut call)?`.
    pub(super) fn dispatch(
        &self,
        c: &mut AisViewController,
        slot: &str,
        args: &dyn Fn() -> String,
        has_result: bool,
        call: SlotCall<'_>,
    ) -> Result<bool, Failure> {
        if !self.case.recorded.contains(&slot) {
            return call(c, Via::Base);
        }
        let args = args();
        self.emit(slot, "enter", &args, "-1");
        let mut result = true;
        if slot != self.case.slot {
            result = call(c, Via::Base)?;
        } else {
            self.effect(c, &mut *call)?;
            if self.case.mode == EventSlotMode::Delegate {
                result = call(c, Via::Base)?;
            }
        }
        let text = if has_result {
            (result as i32).to_string()
        } else {
            "-1".to_string()
        };
        self.emit(slot, "leave", &args, &text);
        Ok(result)
    }
    /// Dispatch a void virtual; its result column is -1.
    pub(super) fn void(
        &self,
        c: &mut AisViewController,
        slot: &str,
        args: &dyn Fn() -> String,
        call: VoidCall<'_>,
    ) -> Result<(), Failure> {
        let mut call = |c: &mut AisViewController, via| call(c, via).map(|_| false);
        self.dispatch(c, slot, args, false, &mut call).map(|_| ())
    }
    /// Mirror a value-returning override whose witness writes the result as quoted text: the
    /// direct and reentry modes return `direct`, delegation and pass-through the base value.
    pub(super) fn dispatch_value<T>(
        &self,
        c: &mut AisViewController,
        slot: &str,
        args: &dyn Fn() -> String,
        (direct, text): (T, &dyn Fn(&T) -> String),
        call: &mut dyn FnMut(&mut AisViewController, Via) -> Result<T, Failure>,
    ) -> Result<T, Failure> {
        if !self.case.recorded.contains(&slot) {
            return call(c, Via::Base);
        }
        let args = args();
        self.emit(slot, "enter", &args, "\"-1\"");
        let mut result = direct;
        if slot != self.case.slot {
            result = call(c, Via::Base)?;
        } else {
            let mut reenter = |c: &mut AisViewController, via| call(c, via).map(|_| false);
            self.effect(c, &mut reenter)?;
            if self.case.mode == EventSlotMode::Delegate {
                result = call(c, Via::Base)?;
            }
        }
        self.emit(slot, "leave", &args, &format!("\"{}\"", text(&result)));
        Ok(result)
    }
    fn effect(&self, c: &mut AisViewController, call: SlotCall<'_>) -> Result<(), Failure> {
        if !self.armed.swap(false, Ordering::SeqCst) {
            return Ok(());
        }
        let kind = self.case.label;
        match self.case.mode {
            EventSlotMode::Typed => Err(Failure::with_type(
                "Standard_ProgramError",
                &format!("{kind} slot kind=typed; expected success"),
                "",
            )),
            EventSlotMode::Unrelated => panic!("{kind} slot kind=unrelated; expected success"),
            EventSlotMode::Reenter => {
                self.depth.fetch_add(1, Ordering::SeqCst);
                let result = call(c, Via::Reenter);
                self.depth.fetch_sub(1, Ordering::SeqCst);
                result.map(|_| ())
            }
            _ => Ok(()),
        }
    }
    /// Record a failure row and the caller's state row after one attempt.
    pub(super) fn finish_attempt(&self, failure: Option<(String, String)>, state: String) {
        if let Some((kind, message)) = failure {
            let name = self.case.name;
            self.push(format!(
                "{{\"case\":\"{name}\",\"phase\":\"failure\",\"kind\":\"{kind}\",\"message\":\"{message}\"}}"
            ));
        }
        self.push(state);
    }
    /// Compare the recorded rows with the recorded baseline rows of this case.
    pub(super) fn assert_native(&self, baseline: &str) {
        self.assert_native_relabelled(baseline, &[]);
    }
    /// As `assert_native`, after replacing each listed native failure message by the Rust
    /// message of the same failure kind; every pair is an exact text, not a pattern.
    pub(super) fn assert_native_relabelled(&self, baseline: &str, messages: &[(&str, &str)]) {
        let name = self.case.name;
        let actual = self.records.lock().unwrap().join("\n") + "\n";
        println!("SLOT_TRACE_BEGIN {name}\n{actual}SLOT_TRACE_END {name}");
        let prefix = format!("{{\"case\":\"{name}\",");
        let relabel = |line: &str| {
            let mut line = line.replace("\"kind\":\"std::logic_error\"", "\"kind\":\"rust-panic\"");
            for (native, rust) in messages {
                line = line.replace(
                    &format!("\"message\":\"{native}\""),
                    &format!("\"message\":\"{rust}\""),
                );
            }
            line + "\n"
        };
        let expected: String = baseline
            .lines()
            .filter(|line| line.starts_with(&prefix))
            .map(relabel)
            .collect();
        assert!(!expected.is_empty(), "case={name}; expected a native trace");
        assert_eq!(
            actual, expected,
            "case={name}; expected the native override trace and state"
        );
    }
}
/// Run one attempt, keeping a typed failure or a panic message; `attempt_outcome(|| run())`.
pub(super) fn attempt_outcome(
    run: impl FnOnce() -> Result<(), Failure>,
) -> Option<(String, String)> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(run)) {
        Ok(Ok(())) => None,
        Ok(Err(failure)) => Some((
            failure.exception_type().to_string(),
            failure.message().to_string(),
        )),
        Err(payload) => Some(("rust-panic".to_string(), panic_text(payload))),
    }
}
fn panic_text(payload: Box<dyn std::any::Any + Send>) -> String {
    match payload.downcast::<&'static str>() {
        Ok(text) => text.to_string(),
        Err(payload) => *payload
            .downcast::<String>()
            .unwrap_or_else(|_| panic!("panic payload; expected a text message")),
    }
}
