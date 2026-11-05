//! Actual operation-local view overrides observe callbacks; no renderer or clock is substituted.
use crate::hatch_surface::scene::{
    HatchBoxScene, HatchSourceView, NamedSourceScene, unit_box_shape,
};
use martensite_tkernel::standard::{Failure, Handle, SharedState, WeakHandle};
use martensite_tkservice::{
    aspect::{AspectNeutralWindow, AspectWindowMembers as _},
    null_driver::NullGraphicDriver,
};
use martensite_tkv3d::{ais::AisInteractiveContext, v3d::*};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

/// Fixed source-compatible CPU input; `mode_cases()[0]`.
#[derive(Clone, Copy, Debug)]
pub(super) struct BareModeCase {
    pub initial: i32,
    pub requested: i32,
    pub immediate: bool,
    pub fail_at: usize,
}
/// Choose one actual existing scene route; `BareModeFacade::Named`.
#[derive(Clone, Copy, Debug)]
pub(super) enum BareModeFacade {
    Box,
    Named,
}

struct ModeCommandTrace {
    armed: AtomicBool,
    fail_at: usize,
    context: SharedState<WeakHandle<AisInteractiveContext>>,
    rows: SharedState<Vec<[i32; 3]>>,
}
struct ModeCommandView {
    base: V3dView,
    trace: Arc<ModeCommandTrace>,
}
martensite_tkernel::impl_transient!(ModeCommandView,"Martensite_ModeCommandView",V3dView,base,
    interfaces:[dyn V3dViewSource]);
impl V3dViewSource for ModeCommandView {
    fn view_base(&self) -> &V3dView {
        // The override and command share one real retained base state.
        &self.base
    }
    fn redraw(&self) {
        if self.trace.armed.load(Ordering::Relaxed) {
            self.trace.record(&self.base);
        }
        v3d_view_redraw(&self.base);
    }
}
impl ModeCommandTrace {
    fn record(&self, view: &V3dView) {
        let ordinal = self.rows.read().len() + 1;
        let context = self.context.read().observe();
        let mode = context.display_mode();
        self.rows
            .write()
            .push([ordinal as i32, observed_immediate(view), mode]);
        if ordinal == self.fail_at {
            std::panic::panic_any(Failure::new(&failure_message(ordinal)));
        }
    }
}

/// Invocation-local UI owners; never published or moved between UI threads.
pub(super) struct ModeCommandFixture {
    stage: Option<HatchSourceView>,
    pub view: Handle<dyn V3dViewSource>,
    pub context: Handle<AisInteractiveContext>,
    trace: Arc<ModeCommandTrace>,
}
impl ModeCommandFixture {
    /// Build the real null-driver context and actual source subclass; `ModeCommandFixture::new(case)`.
    pub(super) fn new(case: BareModeCase) -> Self {
        let viewer = V3dViewer::new(&NullGraphicDriver::new_handle()).unwrap();
        let context = AisInteractiveContext::new(&viewer).unwrap();
        let trace = new_trace(&context, case.fail_at);
        let view = bind_mode_view(&viewer, &trace);
        let stage = Some(prepare_mode_stage(
            viewer,
            context.clone(),
            view.clone(),
            case,
        ));
        trace.armed.store(true, Ordering::Relaxed);
        Self {
            stage,
            view,
            context,
            trace,
        }
    }
    /// Move exactly these owners into a genuine facade; `fixture.facade(BareModeFacade::Box)`.
    pub(super) fn facade(&mut self, route: BareModeFacade) -> ModeCommandFacade {
        let stage = self
            .stage
            .take()
            .expect("mode fixture owners=None; expected unconsumed stage");
        match route {
            BareModeFacade::Box => ModeCommandFacade::Box(stage.with_shape(unit_box_shape())),
            BareModeFacade::Named => ModeCommandFacade::Named(NamedSourceScene::new(stage)),
        }
    }
    /// Start observation after unrelated display setup; `fixture.restart_trace()`.
    pub(super) fn restart_trace(&self) {
        self.trace.armed.store(false, Ordering::Relaxed);
        self.trace.rows.write().clear();
        self.trace.armed.store(true, Ordering::Relaxed);
    }
    /// Observe and disarm before owner cleanup; `let rows=fixture.finish()`.
    pub(super) fn finish(&self) -> Vec<[i32; 3]> {
        // Both fixture hosts retain this exact reset signature, even when no reset is needed.
        let _: fn(&Self) = Self::restart_trace;
        self.trace.armed.store(false, Ordering::Relaxed);
        self.trace.rows.read().clone()
    }
}

/// Retained UI resources stay on the invoking test thread.
pub(super) enum ModeCommandFacade {
    Box(HatchBoxScene),
    Named(NamedSourceScene),
}
impl ModeCommandFacade {
    /// Invoke the existing public facade, not a local algorithm; `facade.run(1)?`.
    pub(super) fn run(&self, mode: i32) -> Result<(), Failure> {
        match self {
            Self::Box(scene) => scene.source_global_mode(mode),
            Self::Named(scene) => scene.source_global_mode(mode),
        }
    }
}

fn new_trace(context: &Handle<AisInteractiveContext>, fail_at: usize) -> Arc<ModeCommandTrace> {
    Arc::new(ModeCommandTrace {
        armed: AtomicBool::new(false),
        fail_at,
        context: SharedState::new(context.downgrade()),
        rows: SharedState::new(Vec::new()),
    })
}
fn bind_mode_view(
    viewer: &Handle<V3dViewer>,
    trace: &Arc<ModeCommandTrace>,
) -> Handle<dyn V3dViewSource> {
    let view = Handle::new(ModeCommandView {
        base: V3dView::new_embedded(viewer, V3dTypeOfView::Orthographic).unwrap(),
        trace: trace.clone(),
    })
    .cast();
    V3dView::bind_source(&view).unwrap();
    view
}

/// Read the published dump scalar without mutating the flag; `observed_immediate(view)`.
pub(super) fn observed_immediate(view: &V3dView) -> i32 {
    use martensite_tkernel::standard::dump::{DumpStream, init_integer_field};
    let mut stream = DumpStream::default();
    view.dump_json(&mut stream, 0).unwrap();
    let text = stream.text();
    let marker = "\"ImmediateUpdate\": ";
    assert_eq!(
        text.matches(marker).count(),
        1,
        "dump={text:?}; expected one immediate scalar"
    );
    let mut position = i32::try_from(text.find(marker).unwrap() + 1).unwrap();
    let mut value = -1;
    assert!(init_integer_field(text, "ImmediateUpdate", &mut position, &mut value).unwrap());
    assert!(matches!(value, 0 | 1), "flag={value}; expected 0 or 1");
    value
}
/// Keep the injected typed cause explicit; `failure_message(1)`.
pub(super) fn failure_message(ordinal: usize) -> String {
    let value = ordinal;
    format!("global-mode redraw={value}; expected no injected failure")
}
const MODE_SOURCE_CASES: [(i32, i32, bool, usize); 8] = [
    (0, 1, true, 0),
    (1, 1, true, 0),
    (0, 1, false, 0),
    (1, 1, false, 0),
    (0, 2, true, 0),
    (0, 1, true, 1),
    (0, 1, true, 2),
    (1, 1, true, 1),
];
/// The fixed matrix covers both update branches and exact stopping boundaries; `mode_cases()`.
pub(super) fn mode_cases() -> [BareModeCase; 8] {
    MODE_SOURCE_CASES.map(|(initial, requested, immediate, fail_at)| BareModeCase {
        initial,
        requested,
        immediate,
        fail_at,
    })
}

fn prepare_mode_stage(
    viewer: Handle<V3dViewer>,
    context: Handle<AisInteractiveContext>,
    view: Handle<dyn V3dViewSource>,
    case: BareModeCase,
) -> HatchSourceView {
    let window = mode_window(&view);
    context
        .set_default_display_mode(case.initial, false)
        .unwrap();
    view.set_immediate_update(case.immediate);
    HatchSourceView::mode_fixture_owners(viewer, context, window, view)
}
fn mode_window(view: &Handle<dyn V3dViewSource>) -> Handle<AspectNeutralWindow> {
    let window = Handle::new(AspectNeutralWindow::default());
    window.set_virtual(true);
    window.set_size(409, 409);
    view.set_window(&window.cast(), 0).unwrap();
    window
}
