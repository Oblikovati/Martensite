//! Real Drop callbacks inspect the live parent while intrusive ownership reaches zero.
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient, WeakHandle};
use martensite_tkv3d::select_mgr::{
    SelectMgrCompositionFilter, SelectMgrEntityOwnerSource, SelectMgrFilter,
};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

#[derive(Clone, Copy, Default)]
pub(super) enum DropAction {
    #[default]
    None,
    Append,
    Remove,
    RecursiveClear,
    InspectTraversal,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct DropRecord {
    pub identity: u8,
    pub parent_length: Option<usize>,
    pub parent_empty: Option<bool>,
    pub references: i32,
}

#[derive(Default)]
pub(super) struct DropTrace {
    pub parent: SharedState<WeakHandle<dyn SelectMgrCompositionFilter>>,
    pub extra: SharedState<WeakHandle<dyn SelectMgrFilter>>,
    pub action: SharedState<DropAction>,
    pub panic_once: AtomicBool,
    pub records: SharedState<Vec<DropRecord>>,
    pub traversal_failed: AtomicBool,
}

pub(super) struct DropProbeFilter {
    transient: StandardTransient,
    trace: Arc<DropTrace>,
    identity: u8,
}
martensite_tkernel::impl_transient!(DropProbeFilter, "DropProbeFilter", dyn SelectMgrFilter, transient,
    interfaces: [dyn SelectMgrFilter]);

impl DropProbeFilter {
    /// Bind a real destructor observation; `DropProbeFilter::new(&trace, 1)`.
    pub(super) fn new(trace: &Arc<DropTrace>, identity: u8) -> Handle<Self> {
        Handle::new(Self {
            transient: StandardTransient::default(),
            trace: trace.clone(),
            identity,
        })
    }

    fn observe_drop(&self) {
        let parent = self.trace.parent.read().observe();
        self.trace.records.write().push(self.drop_record(&parent));
        if self.identity != 1 || parent.is_null() {
            return;
        }
        self.perform_action(&parent);
        if self.trace.panic_once.swap(false, Ordering::AcqRel) {
            std::panic::panic_any(Failure::with_message(
                "Standard_ProgramError",
                "Drop id=1; expected non-panicking destructor",
            ));
        }
    }

    fn drop_record(&self, parent: &Handle<dyn SelectMgrCompositionFilter>) -> DropRecord {
        DropRecord {
            identity: self.identity,
            parent_length: parent.get().map(|owner| owner.stored_filters().len()),
            parent_empty: parent.get().map(|owner| owner.is_empty()),
            references: self.transient.ref_count(),
        }
    }

    fn perform_action(&self, parent: &Handle<dyn SelectMgrCompositionFilter>) {
        let action = *self.trace.action.read();
        let extra = self.trace.extra.read().observe();
        match action {
            DropAction::None => {}
            DropAction::Append => parent.add(&extra),
            DropAction::Remove => parent.remove(&extra),
            DropAction::RecursiveClear => parent.clear(),
            DropAction::InspectTraversal => {
                let failed = parent.stored_filters().iter().try_next().is_err();
                self.trace.traversal_failed.store(failed, Ordering::Release);
            }
        }
    }
}

impl SelectMgrFilter for DropProbeFilter {
    fn is_ok(&self, owner: &Handle<dyn SelectMgrEntityOwnerSource>) -> Result<bool, Failure> {
        // This fixture's meaningful observation is Drop; a live owner is accepted before Clear.
        Ok(owner.is_some())
    }
}

impl Drop for DropProbeFilter {
    fn drop(&mut self) {
        // Observe while the actual Rust destructor is running, never through a deletion surrogate.
        self.observe_drop();
    }
}
