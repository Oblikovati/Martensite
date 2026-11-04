//! One request owns all work limits and cancellation checkpoints.
use super::{Ratio, types::*};
pub(super) struct Budget<'a> {
    pub policy: SurfaceQueryPolicy,
    pub work: SurfaceQueryWork,
    cancelled: &'a mut dyn FnMut() -> bool,
}
impl<'a> Budget<'a> {
    pub fn new(policy: SurfaceQueryPolicy, cancelled: &'a mut dyn FnMut() -> bool) -> Self {
        Self {
            policy,
            work: SurfaceQueryWork::default(),
            cancelled,
        }
    }
    pub fn step(&mut self) -> Result<(), SurfaceQueryCause> {
        if (self.cancelled)() {
            return Err(SurfaceQueryCause::Cancelled);
        }
        let actual = self.work.steps.saturating_add(1);
        if actual > self.policy.max_steps {
            return Err(SurfaceQueryCause::Limit {
                field: "steps",
                requested: self.policy.max_steps,
                actual,
            });
        }
        self.work.steps = actual;
        Ok(())
    }
    pub fn retain(&mut self, value: Ratio) -> Result<Ratio, SurfaceQueryCause> {
        let bits = value.numerator().bits().max(value.denominator().bits());
        self.work.coefficient_bits = self.work.coefficient_bits.max(bits);
        if bits > self.policy.max_coefficient_bits {
            return Err(SurfaceQueryCause::Limit {
                field: "coefficient bits",
                requested: self.policy.max_coefficient_bits,
                actual: bits,
            });
        }
        Ok(value)
    }
}
