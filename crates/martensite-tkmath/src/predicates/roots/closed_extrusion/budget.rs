//! One invocation charges every preparation and separating-direction attempt globally.
use super::types::{ClosedExtrusionCause as Cause, ClosedExtrusionPolicy, ClosedExtrusionWork};
pub(super) struct ProofBudget<'a> {
    pub work: ClosedExtrusionWork,
    pub policy: ClosedExtrusionPolicy,
    cancelled: &'a mut dyn FnMut() -> bool,
}
impl<'a> ProofBudget<'a> {
    pub fn new(policy: ClosedExtrusionPolicy, cancelled: &'a mut dyn FnMut() -> bool) -> Self {
        Self {
            work: ClosedExtrusionWork::default(),
            policy,
            cancelled,
        }
    }
    pub fn checkpoint(&mut self) -> Result<(), Cause> {
        if (self.cancelled)() {
            return Err(Cause::Cancelled);
        }
        if self.work.started_proof_steps >= self.policy.max_proof_steps {
            return Err(Cause::Limit {
                kind: "proof steps",
                requested: self.policy.max_proof_steps,
                actual: self.work.started_proof_steps,
            });
        }
        self.work.started_proof_steps += 1;
        Ok(())
    }
    pub fn start_obligation(&mut self) -> Result<(), Cause> {
        self.checkpoint()?;
        self.work.started_obligations += 1;
        Ok(())
    }
    pub fn finish_obligation(&mut self) {
        self.work.finished_obligations += 1;
    }
}
