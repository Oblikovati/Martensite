//! A real transient retains one shared operation; borrowed views never escape a guard.
use super::*;
use martensite_tkernel::{
    impl_transient,
    standard::{RuntimeType, RuntimeTyped},
};
use std::sync::{Arc, Mutex, RwLock};

/// Stable shared ownership of the actual operation builder; `Arc::new(RwLock::new(filler))`.
pub type TopOpeBRepFacesFillerContext = Arc<RwLock<TopOpeBRepFacesFiller<'static>>>;

/// Handle-capable diagnostics with shared operation identity; `Handle::new(TopOpeBRepFFDumper::new(None))`.
pub struct TopOpeBRepFFDumper {
    parent: StandardTransient,
    state: Mutex<OwnedDiagnosticState>,
}
#[derive(Default)]
struct OwnedDiagnosticState {
    context: Option<TopOpeBRepFacesFillerContext>,
    cache: DiagnosticCache,
}
impl_transient!(
    TopOpeBRepFFDumper,
    "TopOpeBRepFFDumper",
    StandardTransient,
    parent
);

impl TopOpeBRepFFDumper {
    /// Creates the native silent release configuration; `Self::new(None)` retains no filler.
    pub fn new(_context: Option<TopOpeBRepFacesFillerContext>) -> Self {
        Self {
            parent: StandardTransient::default(),
            state: Mutex::new(OwnedDiagnosticState::default()),
        }
    }
    /// Retains the actual operation in debug configuration; `Self::new_debug(context.clone())?`.
    pub fn new_debug(context: TopOpeBRepFacesFillerContext) -> Result<Self, Failure> {
        let result = Self::new(None);
        result.state.lock().map_err(poisoned)?.cache.debug = true;
        result.init(Some(context))?;
        Ok(result)
    }
    /// Rebinds debug context, retaining cache for equal operand faces; `dumper.init(Some(context))?`.
    pub fn init(&self, context: Option<TopOpeBRepFacesFillerContext>) -> Result<(), Failure> {
        let mut state = self.state.lock().map_err(poisoned)?;
        if !state.cache.debug {
            return Ok(());
        }
        state.context = context;
        with_view(&mut state, |view| view.init(view.filler))
    }
    /// Returns the exact shared operation identity; `Arc::ptr_eq(&dumper.p_faces_filler_dummy()?.unwrap(), &context)`.
    pub fn p_faces_filler_dummy(&self) -> Result<Option<TopOpeBRepFacesFillerContext>, Failure> {
        Ok(self.state.lock().map_err(poisoned)?.context.clone())
    }
    /// Reads the runtime class name; `Self::get_type_name()`.
    pub fn get_type_name() -> &'static str {
        Self::TYPE_NAME
    }
    /// Reads the descriptor used by real handle casts; `Self::get_type_descriptor()`.
    pub fn get_type_descriptor() -> &'static RuntimeType {
        Self::static_type()
    }
    /// Reads the same descriptor exposed through `dyn Transient`; `dumper.dynamic_type()`.
    pub fn dynamic_type(&self) -> &'static RuntimeType {
        Self::static_type()
    }
    /// Writes one prepared intersection line; `dumper.dump_line_index(1, &mut output)?`.
    pub fn dump_line_index(&self, index: i32, output: &mut impl Write) -> Result<(), Failure> {
        self.run(|view| view.dump_line_index(index, output))
    }
    /// Writes one source line using the retained context; `dumper.dump_line(&line, &mut output)?`.
    pub fn dump_line(
        &self,
        line: &TopOpeBRepLineInter,
        output: &mut impl Write,
    ) -> Result<(), Failure> {
        self.run(|view| view.dump_line(line, output))
    }
    /// Writes one point diagnostic; `dumper.dump_vp(&point, &mut output)?`.
    pub fn dump_vp(
        &self,
        point: &TopOpeBRepVPointInter<'_>,
        output: &mut impl Write,
    ) -> Result<(), Failure> {
        self.run(|view| view.dump_vp(point, output))
    }
    /// Writes one operand's boundary diagnostic; `dumper.dump_vp_on_surface(&point, 1, &mut output)?`.
    pub fn dump_vp_on_surface(
        &self,
        point: &TopOpeBRepVPointInter<'_>,
        operand: i32,
        output: &mut impl Write,
    ) -> Result<(), Failure> {
        self.run(|view| view.dump_vp_on_surface(point, operand, output))
    }
    /// Looks up the retained edge exploration index; `dumper.explore_index(&edge, 1)?`.
    pub fn explore_index(&self, edge: &Shape, operand: i32) -> Result<i32, Failure> {
        self.run(|view| view.explore_index(edge, operand))
    }
    /// Reads current operation storage while writing identity and tolerance; `dumper.dump_dsp(&point, kind, 1, true, &mut output)?`.
    pub fn dump_dsp(
        &self,
        point: &TopOpeBRepVPointInter<'_>,
        kind: TopOpeBRepDSKind,
        index: i32,
        is_new: bool,
        output: &mut impl Write,
    ) -> Result<(), Failure> {
        self.run(|view| view.dump_dsp(point, kind, index, is_new, output))
    }
    fn run<R>(
        &self,
        action: impl FnOnce(&mut TopOpeBRepFFDumperView<'_, 'static>) -> Result<R, Failure>,
    ) -> Result<R, Failure> {
        let mut state = self.state.lock().map_err(poisoned)?;
        with_view(&mut state, action)
    }
}
fn with_view<R>(
    state: &mut OwnedDiagnosticState,
    action: impl FnOnce(&mut TopOpeBRepFFDumperView<'_, 'static>) -> Result<R, Failure>,
) -> Result<R, Failure> {
    let context = state.context.clone();
    let guard = context
        .as_ref()
        .map(|context| context.read().map_err(poisoned))
        .transpose()?;
    let mut view = TopOpeBRepFFDumperView {
        filler: guard.as_deref(),
        cache: std::mem::take(&mut state.cache),
    };
    let result = action(&mut view);
    state.cache = view.cache;
    result
}
fn poisoned<T>(error: std::sync::PoisonError<T>) -> Failure {
    Failure::with_message(
        "Standard_Failure",
        &format!("FFDumper: poisoned operation lock={error}; expected consistent diagnostic state"),
    )
}
