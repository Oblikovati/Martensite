//! Shared-owner overrides of the four selection-input virtuals, driven through the public
//! click, button, position and sequence entrypoints as the native witness
//! the recorded controller_selection_input capture does.
use super::*;
use martensite_tkernel::ncollection::{Sequence, Vec2};
use martensite_tkernel::standard::Failure;
use martensite_tkservice::aspect::ASPECT_VKEY_MOUSE_LEFT_BUTTON as LEFT;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum SelectionInputSlot {
    Pick,
    Points,
    Rubber,
    Lasso,
}
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum SelectionInputMode {
    Direct,
    Delegate,
    Reenter,
    Typed,
    Unrelated,
}
/// One witnessed scenario, named `<slot>-<mode>` exactly as the native trace.
#[derive(Clone, Copy)]
pub(super) struct SelectionInputCase {
    pub(super) name: &'static str,
    pub(super) slot: SelectionInputSlot,
    pub(super) mode: SelectionInputMode,
}
/// Parse a recorded case name; `selection_input_case("lasso-typed")`.
pub(super) fn selection_input_case(name: &'static str) -> SelectionInputCase {
    use SelectionInputMode as M;
    use SelectionInputSlot as S;
    let (slot, mode) = name
        .split_once('-')
        .unwrap_or_else(|| panic!("case={name}; expected <slot>-<mode>"));
    let slot = match slot {
        "pick" => S::Pick,
        "points" => S::Points,
        "rubber" => S::Rubber,
        "lasso" => S::Lasso,
        _ => panic!("case slot={slot}; expected pick, points, rubber or lasso"),
    };
    let mode = match mode {
        "direct" => M::Direct,
        "delegate" => M::Delegate,
        "reenter" => M::Reenter,
        "typed" => M::Typed,
        "unrelated" => M::Unrelated,
        _ => panic!("case mode={mode}; expected direct, delegate, reenter, typed or unrelated"),
    };
    SelectionInputCase { name, slot, mode }
}
/// Records every override call and the controller state after each attempt.
pub(super) struct SelectionInputRecorder {
    pub(super) case: SelectionInputCase,
    depth: AtomicI32,
    armed: AtomicBool,
    pub(super) results: Mutex<Vec<i32>>,
    pub(super) records: Mutex<Vec<String>>,
}
impl SelectionInputRecorder {
    pub(super) fn new(case: SelectionInputCase) -> Self {
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
    fn emit(&self, phase: &str, args: &str) {
        let depth = self.depth.load(Ordering::SeqCst);
        let name = self.case.name;
        self.push(format!(
            "{{\"case\":\"{name}\",\"phase\":\"{phase}\",\"depth\":{depth},\"args\":\"{args}\"}}"
        ));
    }
    /// Mirror the native override body; `recorder.dispatch(c, args, reenter, base)?`.
    fn dispatch(
        &self,
        c: &mut AisViewController,
        args: &str,
        reenter: impl FnOnce(&mut AisViewController) -> Result<(), Failure>,
        base: impl FnOnce(&mut AisViewController),
    ) -> Result<(), Failure> {
        self.emit("enter", args);
        self.effect(c, reenter)?;
        if self.case.mode == SelectionInputMode::Delegate {
            base(c);
        }
        self.emit("leave", args);
        Ok(())
    }
    fn effect(
        &self,
        c: &mut AisViewController,
        reenter: impl FnOnce(&mut AisViewController) -> Result<(), Failure>,
    ) -> Result<(), Failure> {
        if !self.armed.swap(false, Ordering::SeqCst) {
            return Ok(());
        }
        match self.case.mode {
            SelectionInputMode::Typed => Err(Failure::with_type(
                "Standard_ProgramError",
                "selection input kind=typed; expected success",
                "",
            )),
            SelectionInputMode::Unrelated => {
                panic!("selection input kind=unrelated; expected success")
            }
            SelectionInputMode::Reenter => {
                self.depth.fetch_add(1, Ordering::SeqCst);
                let result = reenter(c);
                self.depth.fetch_sub(1, Ordering::SeqCst);
                result
            }
            _ => Ok(()),
        }
    }
}
impl AisViewControllerCameraHooks for SelectionInputRecorder {
    fn select_in_viewer(
        &self,
        c: &mut AisViewController,
        point: Vec2<i32>,
        scheme: AisSelectionScheme,
    ) -> Result<(), Failure> {
        let args = format!("{},{},{}", point.x, point.y, scheme as i32);
        self.dispatch(
            c,
            &args,
            |c| c.select_in_viewer(point, scheme),
            |c| c.select_in_viewer_base(point, scheme),
        )
    }
    fn select_points_in_viewer(
        &self,
        c: &mut AisViewController,
        points: &Sequence<Vec2<i32>>,
        scheme: AisSelectionScheme,
    ) -> Result<(), Failure> {
        let coordinates: String = points.iter().map(|p| format!("{},{},", p.x, p.y)).collect();
        let args = format!("{coordinates}{}", scheme as i32);
        self.dispatch(
            c,
            &args,
            |c| c.select_points_in_viewer(points, scheme),
            |c| c.select_points_in_viewer_base(points, scheme),
        )
    }
    fn update_rubber_band(
        &self,
        c: &mut AisViewController,
        first: Vec2<i32>,
        second: Vec2<i32>,
    ) -> Result<(), Failure> {
        let args = format!("{},{},{},{}", first.x, first.y, second.x, second.y);
        self.dispatch(
            c,
            &args,
            |c| c.update_rubber_band(first, second),
            |c| c.update_rubber_band_base(first, second),
        )
    }
    fn update_poly_selection(
        &self,
        c: &mut AisViewController,
        point: Vec2<i32>,
        append: bool,
    ) -> Result<(), Failure> {
        let args = format!("{},{},{}", point.x, point.y, append as i32);
        self.dispatch(
            c,
            &args,
            |c| c.update_poly_selection(point, append),
            |c| c.update_poly_selection_base(point, append),
        )
    }
}
/// Bind the native witness maps: left click picks with XOR, left drag is the case gesture.
pub(super) fn selection_input_controller(case: SelectionInputCase) -> AisViewController {
    let mut c = AisViewController::new().unwrap();
    c.change_mouse_selection_schemes().clear(true);
    c.change_mouse_selection_schemes()
        .bind(LEFT, AisSelectionScheme::Xor);
    let gesture = match case.slot {
        SelectionInputSlot::Lasso => AisMouseGesture::SelectLasso,
        _ => AisMouseGesture::SelectRectangle,
    };
    c.change_mouse_gesture_map().clear(true);
    c.change_mouse_gesture_map().bind(LEFT, gesture);
    c
}
