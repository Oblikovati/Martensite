use super::*;
use martensite_tkernel::{message::ProgressRange, standard::Failure};

impl SurfaceSelfSearchReceipt {
    pub(crate) fn begin(&mut self) {
        let invocation = self.invocation + 1;
        *self = Self {
            invocation,
            termination: SelfSearchTermination::Running,
            ..Self::default()
        };
    }
    pub(crate) fn finish(&mut self, result: &Result<(), Failure>) {
        if self.termination == SelfSearchTermination::Cancelled {
            return;
        }
        self.termination = if result.is_err() {
            SelfSearchTermination::Failed
        } else if self.calls_with_unobserved_inner_work != 0
            || !self.discovery_finished
            || !self.work_accounted()
            || self.limits != SelfSearchLimitCounts::default()
        {
            SelfSearchTermination::Incomplete
        } else {
            SelfSearchTermination::Finished
        };
    }
    pub(crate) fn check_cancelled(&mut self, progress: &ProgressRange<'_>) -> Result<(), Failure> {
        if !progress.user_break() {
            return Ok(());
        }
        self.termination = SelfSearchTermination::Cancelled;
        Err(Failure::with_message(
            "SurfaceSelfSearchCancelled",
            "surface self-search progress.user_break=true; expected an uncancelled range",
        ))
    }
    pub(crate) fn discovered(&mut self, sections: i32, zones: i32) {
        self.discovery = SelfSearchDiscovery::SampledPolyhedron;
        self.discovery_finished = true;
        self.sections.scheduled = sections as u64;
        self.tangent_zones.scheduled = zones as u64;
    }
    pub(crate) fn analytic(&mut self) {
        self.discovery = SelfSearchDiscovery::AnalyticPolicy;
        self.discovery_finished = true;
    }
}
impl SelfSearchWorkCounts {
    pub(crate) fn entered(&mut self) {
        self.scheduled += 1;
        self.visited += 1;
    }
    pub(crate) fn returned(&mut self, result: &Result<(), Failure>, cancelled: bool) {
        if cancelled {
            self.cancelled += 1;
        } else if result.is_err() {
            self.failed += 1;
        } else {
            self.finished += 1;
        }
    }
}

impl SurfaceSelfSearchReceipt {
    pub(crate) fn observe_walking(&mut self, actual: crate::int_walk::WalkingSearchObservation) {
        self.marching_steps += actual.steps;
        self.limits.no_append += actual.no_append_limit;
        self.limits.repeated_start += actual.repeated_start_limit;
        self.limits.point_count += actual.point_count_limit;
        self.calls_with_unobserved_inner_work += u64::from(actual.inner_termination_unobserved);
        if actual.cancelled {
            self.termination = SelfSearchTermination::Cancelled;
        }
    }
}

impl SurfaceSelfSearchReceipt {
    fn work_accounted(&self) -> bool {
        [
            &self.sections,
            &self.tangent_zones,
            &self.starts,
            &self.refinements,
            &self.walks,
        ]
        .into_iter()
        .all(SelfSearchWorkCounts::accounted)
    }
}
impl SelfSearchWorkCounts {
    fn accounted(&self) -> bool {
        let returned = self
            .finished
            .checked_add(self.pruned)
            .and_then(|n| n.checked_add(self.failed))
            .and_then(|n| n.checked_add(self.cancelled));
        self.scheduled == self.visited
            && returned == Some(self.visited)
            && self.failed == 0
            && self.cancelled == 0
    }
}
