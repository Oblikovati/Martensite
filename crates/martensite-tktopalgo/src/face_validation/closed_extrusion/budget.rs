//! One request owns all counters; the next edge cannot reset a spent allowance.
use super::types::*;
use martensite_tkmath::predicates::roots::{RootIsolationOptions, metric_bounds::*};
pub(super) type CheckResult<T> = Result<T, ClosedFaceCompletion>;
pub(super) struct Budget<'a, F: FnMut() -> bool> {
    pub policy: ClosedFacePolicy,
    pub work: ClosedFaceWork,
    pub cancel: &'a mut F,
}
impl<F: FnMut() -> bool> Budget<'_, F> {
    pub fn poll(&mut self) -> CheckResult<()> {
        if (self.cancel)() {
            return Err(ClosedFaceCompletion::Incomplete(
                ClosedFaceIncomplete::Cancelled,
            ));
        }
        Ok(())
    }
    pub fn query(&mut self) -> CheckResult<()> {
        self.poll()?;
        self.work.queries = self.work.queries.saturating_add(1);
        limit("queries", self.policy.max_queries, self.work.queries)
    }
    pub fn location_step(&mut self, power: i32) -> CheckResult<()> {
        self.poll()?;
        self.work.location_steps = self.work.location_steps.saturating_add(1);
        limit(
            "location steps",
            self.policy.max_location_steps as u64,
            self.work.location_steps as u64,
        )?;
        limit(
            "location power",
            self.policy.max_location_power as u64,
            power.unsigned_abs() as u64,
        )
    }
    pub fn residual(
        &mut self,
        mut query: TransportedResidualQuery<'_>,
    ) -> CheckResult<ResidualDecision> {
        self.query()?;
        let remaining = self
            .policy
            .max_bisections
            .saturating_sub(self.work.bisections);
        query.residual.options = RootIsolationOptions {
            max_bisections: remaining.min(u16::MAX as u64) as u16,
            ..RootIsolationOptions::new(1.)
        };
        let report = certify_transported_residual(query, &mut *self.cancel);
        self.work.bisections = self.work.bisections.saturating_add(report.bisections);
        report
            .decision
            .map_err(|cause| ClosedFaceCompletion::Incomplete(ClosedFaceIncomplete::Metric(cause)))
    }
}
pub(super) fn limit(kind: &'static str, requested: u64, actual: u64) -> CheckResult<()> {
    if actual > requested {
        return Err(ClosedFaceCompletion::Incomplete(
            ClosedFaceIncomplete::Limit {
                kind,
                requested,
                actual,
            },
        ));
    }
    Ok(())
}
pub(super) fn unsupported(
    owner: usize,
    field: &'static str,
    actual: impl Into<String>,
    expected: &'static str,
) -> ClosedFaceCompletion {
    ClosedFaceCompletion::Unsupported(ClosedFaceUnsupported {
        owner,
        field,
        actual: actual.into(),
        expected,
    })
}
pub(super) fn invalid(
    owner: usize,
    invariant: &'static str,
    actual: impl Into<String>,
) -> ClosedFaceCompletion {
    ClosedFaceCompletion::Invalid(ClosedFaceViolation {
        owner,
        invariant,
        actual: actual.into(),
    })
}
