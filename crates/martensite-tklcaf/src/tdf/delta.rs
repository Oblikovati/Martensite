//! `TDF_Delta`: the attribute deltas of one committed transaction, valid between two times.
use super::{TdfAttributeDelta, TdfLabel};
use crate::live;
use martensite_tkcdf::json_dump::{class_begin, field_dumped, field_integer, field_string};
use martensite_tkernel::{
    impl_transient, impl_transient_interface_target,
    ncollection::Map,
    standard::{
        Failure, Handle, RuntimeType, RuntimeTyped, SharedState, Transient, dump::DumpStream,
    },
    tcollection::ExtendedString,
};
use std::io;

#[derive(Default)]
struct DeltaState {
    begin_time: i32,
    end_time: i32,
    attribute_deltas: Vec<Handle<dyn TdfAttributeDelta>>,
    name: ExtendedString,
}

/// `TDF_Delta`; made by `TdfData` commits (`TdfDelta::new()` is the empty delta).
#[derive(Default)]
pub struct TdfDelta {
    base: TdfDeltaBase,
}
impl_transient!(TdfDelta, "TDF_Delta", dyn Transient, interfaces: [dyn TdfDeltaSource]);

/// The shared state of a transaction delta. Example: `TdfDeltaBase::default()`.
#[derive(Default)]
pub struct TdfDeltaBase {
    state: SharedState<DeltaState>,
}

impl RuntimeTyped for dyn TdfDeltaSource {
    const TYPE_NAME: &'static str = TdfDelta::TYPE_NAME;
    fn static_type() -> &'static RuntimeType {
        TdfDelta::static_type()
    }
}
impl_transient_interface_target!(dyn TdfDeltaSource);

impl TdfDeltaSource for TdfDelta {
    fn delta_base(&self) -> &TdfDeltaBase {
        &self.base
    }
}

impl TdfDelta {
    /// `TDF_Delta()`: empty, valid from time 0 to 0.
    pub fn new() -> Self {
        Self::default()
    }
}

/// The operations shared by regular and compound transaction deltas.
/// Example: `let end = delta.end_time();`.
pub trait TdfDeltaSource: Transient {
    /// Borrow the fields of this delta. Example: `let base = delta.delta_base();`.
    fn delta_base(&self) -> &TdfDeltaBase;

    /// `IsEmpty`: no attribute delta.
    fn is_empty(&self) -> bool {
        self.delta_base().state.read().attribute_deltas.is_empty()
    }
    /// `IsApplicable(aCurrentTime)`: the delta ends at that time.
    fn is_applicable(&self, current_time: i32) -> bool {
        self.delta_base().state.read().end_time == current_time
    }
    /// `BeginTime`.
    fn begin_time(&self) -> i32 {
        self.delta_base().state.read().begin_time
    }
    /// `EndTime`.
    fn end_time(&self) -> i32 {
        self.delta_base().state.read().end_time
    }
    /// `Labels(aLabelList)`: the labels touched by the delta, merged with the labels already
    /// in the list, in the iteration order of the native label map.
    fn labels(&self, labels: &mut Vec<TdfLabel>) {
        let mut map: Map<TdfLabel> = Map::default();
        for label in labels.iter() {
            map.add(label.clone());
        }
        for delta in self.attribute_deltas().iter().filter_map(Handle::get) {
            map.add(delta.label());
        }
        labels.clear();
        labels.extend(map.iter().cloned());
    }
    /// `AttributeDeltas`: a copy of the list, in commit order.
    fn attribute_deltas(&self) -> Vec<Handle<dyn TdfAttributeDelta>> {
        self.delta_base().state.read().attribute_deltas.clone()
    }
    /// `Name`; `delta.name()`.
    fn name(&self) -> ExtendedString {
        self.delta_base().state.read().name.clone()
    }
    /// `SetName`; `delta.set_name(&name)`.
    fn set_name(&self, name: &ExtendedString) {
        self.delta_base().state.write().name = name.clone();
    }
    /// Protected `Validity`: the times between which the delta applies.
    fn validity(&self, begin_time: i32, end_time: i32) {
        let mut state = self.delta_base().state.write();
        state.begin_time = begin_time;
        state.end_time = end_time;
    }
    /// Protected `AddAttributeDelta`: appends a non-null attribute delta.
    fn add_attribute_delta(&self, attribute_delta: &Handle<dyn TdfAttributeDelta>) {
        if attribute_delta.is_some() {
            self.delta_base()
                .state
                .write()
                .attribute_deltas
                .push(attribute_delta.clone());
        }
    }
    /// `Dump`: the validity, the count and one line per attribute delta.
    fn dump(&self, stream: &mut dyn io::Write) -> io::Result<()> {
        let deltas = self.attribute_deltas();
        let (begin, end) = (self.begin_time(), self.end_time());
        writeln!(
            stream,
            "DELTA available from time \t#{begin} to time \t#{end}"
        )?;
        writeln!(stream, "Nb Attribute Delta(s): {}", deltas.len())?;
        for delta in deltas.iter().filter_map(Handle::get) {
            write!(stream, "| ")?;
            delta.dump(stream)?;
            writeln!(stream)?;
        }
        Ok(())
    }
    /// `DumpJson`: the times, the attribute deltas and the name.
    fn dump_json(&self, stream: &mut DumpStream, depth: i32) -> io::Result<()> {
        class_begin(stream, "TDF_Delta")?;
        field_integer(stream, "myBeginTime", self.begin_time())?;
        field_integer(stream, "myEndTime", self.end_time())?;
        for delta in self.attribute_deltas().iter().filter_map(Handle::get) {
            let child = |s: &mut DumpStream, d: i32| delta.dump_json(s, d);
            field_dumped(stream, depth, "anAttDeltaList.get()", Some(&child))?;
        }
        field_string(stream, "myName", &self.name().to_utf8())
    }
}

impl TdfDeltaBase {
    /// Private `ReplaceDeltaList`.
    pub(crate) fn replace_delta_list(&self, list: Vec<Handle<dyn TdfAttributeDelta>>) {
        self.state.write().attribute_deltas = list;
    }
    /// Private `Apply`: applies each attribute delta in order.
    pub(crate) fn apply(&self) -> Result<(), Failure> {
        let deltas = self.state.read().attribute_deltas.clone();
        for delta in deltas {
            live(&delta, "TDF_Delta::Apply")?.apply()?;
        }
        Ok(())
    }
    /// Private `BeforeOrAfterApply`: calls `BeforeUndo` (or `AfterUndo`) until every
    /// attribute accepts; when a round accepts none (a dead lock), forces the rest.
    pub(crate) fn before_or_after_apply(&self, before: bool) -> Result<(), Failure> {
        let mut pending = self.state.read().attribute_deltas.clone();
        let mut count = pending.len();
        let mut no_dead_lock = true;
        while no_dead_lock && count != 0 {
            let mut kept = Vec::with_capacity(pending.len());
            for delta in pending {
                if !undo_callback(&delta, before, false)? {
                    kept.push(delta);
                }
            }
            pending = kept;
            no_dead_lock = count > pending.len();
            count = pending.len();
        }
        if !no_dead_lock {
            for delta in &pending {
                undo_callback(delta, before, true)?;
            }
        }
        Ok(())
    }
}

/// One `BeforeUndo`/`AfterUndo` call on the attribute of a delta.
fn undo_callback(
    delta: &Handle<dyn TdfAttributeDelta>,
    before: bool,
    force: bool,
) -> Result<bool, Failure> {
    let attribute = live(delta, "TDF_Delta::BeforeOrAfterApply")?.attribute();
    let attribute = live(&attribute, "TDF_Delta::BeforeOrAfterApply")?;
    if before {
        attribute.before_undo(delta, force)
    } else {
        attribute.after_undo(delta, force)
    }
}
