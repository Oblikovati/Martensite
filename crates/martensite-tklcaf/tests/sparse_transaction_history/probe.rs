//! A named attribute fake uses the real transaction callback and backup interfaces.
use martensite_tkernel::{
    impl_transient,
    standard::{Failure, Guid, Handle},
    tcollection::AsciiString,
};
use martensite_tklcaf::{
    guid_from_text,
    tdf::{
        TdfAttribute, TdfAttributeBase, TdfAttributeState, TdfData, TdfDelta, TdfDeltaSource,
        TdfLabel, TdfRelocationTable, TdfTool, TdfTransaction,
    },
};
use std::sync::{Arc, Mutex};

const PROBE_ID: Guid = guid_from_text("c33d4900-4890-4890-8000-000000000001");
type History = Arc<Mutex<Vec<String>>>;
pub(super) enum CommitAction {
    Touch(TdfLabel),
    ClearMay(TdfLabel),
    TouchThenFail(TdfLabel),
    NestedCommit(TdfLabel),
}
pub(super) struct CommitProbeAttribute {
    base: TdfAttributeBase,
    history: History,
    action: Mutex<Option<CommitAction>>,
}
impl_transient!(CommitProbeAttribute, "CommitProbeAttribute", dyn TdfAttribute, interfaces: [dyn TdfAttribute]);

impl CommitProbeAttribute {
    fn new(history: History) -> Self {
        Self {
            base: TdfAttributeBase::default(),
            history,
            action: Mutex::new(None),
        }
    }
}
impl TdfAttribute for CommitProbeAttribute {
    fn attribute_base(&self) -> &TdfAttributeBase {
        // Backups and commits exercise the real framework state.
        &self.base
    }
    fn id(&self) -> Guid {
        // Copies must remain discoverable under the original attribute identity.
        PROBE_ID
    }
    fn restore(&self, _attribute: &Handle<dyn TdfAttribute>) -> Result<(), Failure> {
        // The probe has no persisted value; restoring must not rearm its callback.
        Ok(())
    }
    fn new_empty(&self) -> Handle<dyn TdfAttribute> {
        // A backup shares observation only, never the pending one-shot action.
        Handle::new(Self::new(self.history.clone())).into_upcast()
    }
    fn paste(
        &self,
        _into: &Handle<dyn TdfAttribute>,
        _table: &Handle<TdfRelocationTable>,
    ) -> Result<(), Failure> {
        // The fake has no application payload to copy between labels.
        Ok(())
    }
    fn before_commit_transaction(&self) -> Result<(), Failure> {
        self.history.lock().unwrap().push(entry(&self.label()));
        let action = self.action.lock().unwrap().take();
        execute_action(action)
    }
}
fn execute_action(action: Option<CommitAction>) -> Result<(), Failure> {
    match action {
        Some(CommitAction::Touch(label)) => touch_label(&label),
        Some(CommitAction::ClearMay(label)) => {
            label.node().unwrap().set_may_be_modified(false);
            Ok(())
        }
        Some(CommitAction::TouchThenFail(label)) => {
            touch_label(&label)?;
            Err(Failure::new("sparse callback refusal"))
        }
        Some(CommitAction::NestedCommit(label)) => nested_commit(&label),
        None => Ok(()),
    }
}
fn nested_commit(label: &TdfLabel) -> Result<(), Failure> {
    let data = label.data();
    let mut inner = TdfTransaction::with_data(&data, &AsciiString::new("callback transaction"));
    assert_eq!(inner.open()?, 2);
    touch_label(label)?;
    let delta = inner.commit(true)?;
    let labels: Vec<_> = delta
        .attribute_deltas()
        .iter()
        .map(|item| entry(&item.label()))
        .collect();
    assert_eq!(labels, [entry(label)]);
    assert_eq!(data.transaction(), 1);
    Ok(())
}
fn entry(label: &TdfLabel) -> String {
    let mut text = AsciiString::default();
    TdfTool::entry(label, &mut text);
    String::from_utf8(text.as_bytes().to_vec()).unwrap()
}
fn touch_label(label: &TdfLabel) -> Result<(), Failure> {
    let mut probe = Handle::default();
    assert!(label.find_attribute(&PROBE_ID, &mut probe)?);
    probe.get().unwrap().backup()
}
pub(super) struct HistoryFixture {
    pub(super) data: Handle<TdfData>,
    labels: Vec<TdfLabel>,
    probes: Vec<Handle<CommitProbeAttribute>>,
    history: History,
}
impl HistoryFixture {
    pub(super) fn new(paths: &[&[i32]]) -> Self {
        let data = TdfData::new();
        let history = History::default();
        let labels: Vec<_> = paths.iter().map(|path| label_at(&data, path)).collect();
        let probes = labels.iter().map(|label| attach(label, &history)).collect();
        let fixture = Self {
            data,
            labels,
            probes,
            history,
        };
        fixture.open().commit(false).unwrap();
        fixture.clear_history();
        fixture
    }
    pub(super) fn open(&self) -> TdfTransaction {
        let mut transaction =
            TdfTransaction::with_data(&self.data, &AsciiString::new("sparse history"));
        transaction.open().unwrap();
        transaction
    }
    pub(super) fn label(&self, index: usize) -> TdfLabel {
        // Labels retain identity so actions can target already traversed siblings.
        self.labels[index].clone()
    }
    pub(super) fn probe(&self, index: usize) -> &CommitProbeAttribute {
        // Keep observation on the attached object rather than its backup.
        self.probes[index].get().unwrap()
    }
    pub(super) fn touch(&self, index: usize) {
        // Normal backup marking is distinct from the explicit public-flag controls.
        self.probe(index).backup().unwrap();
    }
    pub(super) fn arm(&self, index: usize, action: CommitAction) {
        // Arming itself must not alter the transaction's dirty-label collection.
        *self.probe(index).action.lock().unwrap() = Some(action);
    }
    pub(super) fn clear_history(&self) {
        // Preparation callbacks are outside each test's asserted operation.
        self.history.lock().unwrap().clear();
    }
    pub(super) fn assert_history(&self, expected: &[&str]) {
        // Callback order exposes skipped or revisited labels independently of deltas.
        assert_eq!(*self.history.lock().unwrap(), expected);
    }
    pub(super) fn assert_delta(delta: &Handle<TdfDelta>, expected: &[&str]) {
        let labels: Vec<_> = delta
            .attribute_deltas()
            .iter()
            .map(|item| entry(&item.label()))
            .collect();
        assert_eq!(labels, expected);
    }
}
fn label_at(data: &Handle<TdfData>, path: &[i32]) -> TdfLabel {
    path.iter().fold(data.root(), |label, tag| {
        label.find_child(*tag, true).unwrap()
    })
}
fn attach(label: &TdfLabel, history: &History) -> Handle<CommitProbeAttribute> {
    let probe = Handle::new(CommitProbeAttribute::new(history.clone()));
    label.add_attribute(&probe.upcast(), true).unwrap();
    probe
}
