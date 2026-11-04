//! Named filter double records real ordered virtual calls without external I/O.
use martensite_tkernel::standard::{Failure, Handle, SharedState, StandardTransient};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkv3d::select_mgr::{SelectMgrEntityOwnerSource, SelectMgrFilter};
use std::sync::Arc;

pub(super) struct ReadinessDecisionFilter {
    transient: StandardTransient,
    accepted: bool,
    identity: u8,
    calls: Arc<SharedState<Vec<u8>>>,
}
martensite_tkernel::impl_transient!(ReadinessDecisionFilter, "ReadinessDecisionFilter", dyn SelectMgrFilter, transient,
    interfaces: [dyn SelectMgrFilter]);

impl ReadinessDecisionFilter {
    /// Retain a named decision and call log; `ReadinessDecisionFilter::new(true, 7, &calls)`.
    pub(super) fn new(
        accepted: bool,
        identity: u8,
        calls: &Arc<SharedState<Vec<u8>>>,
    ) -> Handle<Self> {
        Handle::new(Self {
            transient: StandardTransient::default(),
            accepted,
            identity,
            calls: calls.clone(),
        })
    }
}

impl SelectMgrFilter for ReadinessDecisionFilter {
    fn is_ok(&self, _owner: &Handle<dyn SelectMgrEntityOwnerSource>) -> Result<bool, Failure> {
        self.calls.write().push(self.identity);
        Ok(self.accepted)
    }
    fn acts_on(&self, mode: ShapeEnum) -> bool {
        let expected = ShapeEnum::Face;
        mode == expected
    }
}
