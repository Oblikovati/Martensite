//! Named callbacks borrow real parents without adding observable intrusive ownership.
use martensite_tkernel::standard::{Failure, Handle, SharedState, WeakHandle};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkv3d::select_mgr::{
    SelectMgrCompositionFilter, SelectMgrEntityOwnerSource, SelectMgrFilter,
};
use std::sync::{
    Arc, Weak,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

#[derive(Default)]
pub(super) struct LiveTrace {
    pub calls: SharedState<Vec<(u8, i32)>>,
    pub parent: SharedState<WeakHandle<dyn SelectMgrCompositionFilter>>,
    pub append: SharedState<WeakHandle<dyn SelectMgrFilter>>,
    pub remove: SharedState<WeakHandle<dyn SelectMgrFilter>>,
    changed: AtomicBool,
    pub fail: AtomicBool,
    pub destructions: AtomicUsize,
}

pub(super) struct LiveProbeFilter {
    trace: Arc<LiveTrace>,
    identity: u8,
    accepted: bool,
    owner: Weak<Self>,
}
martensite_tkernel::impl_transient!(LiveProbeFilter, "LiveProbeFilter", dyn SelectMgrFilter,
    interfaces: [dyn SelectMgrFilter]);

impl LiveProbeFilter {
    /// Bind real callback observations without owning its parent; `LiveProbeFilter::new(&trace, 1, true)`.
    pub(super) fn new(trace: &Arc<LiveTrace>, identity: u8, accepted: bool) -> Handle<Self> {
        Handle::from_arc(Arc::new_cyclic(|owner| Self {
            trace: trace.clone(),
            identity,
            accepted,
            owner: owner.clone(),
        }))
    }

    fn observed_callback(&self) -> bool {
        self.trace.calls.write().push((
            self.identity,
            i32::try_from(self.owner.strong_count()).unwrap(),
        ));
        if self.identity == 1 && !self.trace.changed.swap(true, Ordering::AcqRel) {
            self.mutate_following();
        }
        self.accepted
    }

    fn mutate_following(&self) {
        let parent = self.trace.parent.read().upgrade();
        let append = self.trace.append.read().upgrade();
        let remove = self.trace.remove.read().upgrade();
        if append.is_some() {
            parent.add(&append);
        }
        if remove.is_some() {
            parent.remove(&remove);
        }
    }
}

impl SelectMgrFilter for LiveProbeFilter {
    fn is_ok(&self, _owner: &Handle<dyn SelectMgrEntityOwnerSource>) -> Result<bool, Failure> {
        let accepted = self.observed_callback();
        if self.trace.fail.load(Ordering::Acquire) {
            return Err(Failure::with_message(
                "LiveProbeFailure",
                "fail=true; expected a completed callback",
            ));
        }
        Ok(accepted)
    }
    fn acts_on(&self, _mode: ShapeEnum) -> bool {
        // Keep the same observation point as IsOk without adding an owning temporary.
        self.observed_callback()
    }
}

impl Drop for LiveProbeFilter {
    fn drop(&mut self) {
        let count = &self.trace.destructions;
        count.fetch_add(1, Ordering::AcqRel);
    }
}
