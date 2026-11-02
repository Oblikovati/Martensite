use martensite_tkernel::standard::Failure;
use martensite_tkmath::math::{RootSearchObservation, RootSearchTermination as Exit};

/// Owned local numerical work; accepted calls do not certify a global root search.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NumericalSearchCounts {
    /// Isoparametric solves dispatched.
    pub scheduled: u64,
    /// Solves accepted by the existing solver stopping policy.
    pub accepted: u64,
    /// Accepted solves subsequently rejected by the distance criterion.
    pub residual_declined: u64,
    /// Progress criteria stopped without accepting a solution.
    pub locally_declined: u64,
    /// Function evaluation declined or exceeded its supported magnitude.
    pub evaluation_declined: u64,
    /// Requested divergence stopping rejected a candidate.
    pub divergent: u64,
    /// Solves that exhausted their outer iteration budget.
    pub exhausted: u64,
    /// Calls that returned an error.
    pub failed: u64,
    /// Calls for which no terminal observation was available.
    pub unobserved: u64,
    /// Starts rejected by the existing differential tangency test.
    pub tangency_declined: u64,
    /// Actual outer solver iterations entered.
    pub iterations_entered: u64,
    /// Scalar repair searches actually entered.
    pub scalar_searches: u64,
    /// Scalar repairs whose own stopping criterion was reached.
    pub scalar_accepted: u64,
    /// Scalar repair iteration-limit exits.
    pub scalar_exhausted: u64,
    /// Declined scalar function evaluations.
    pub scalar_evaluation_declined: u64,
    /// Scalar stopping-criterion evaluations actually entered.
    pub scalar_iterations_entered: u64,
    /// Scalar function evaluations actually entered.
    pub scalar_evaluations: u64,
    /// Returned repair candidates rejected by the improvement filter.
    pub scalar_result_declined: u64,
    /// Existing standard or boundary descent-limit events.
    pub descent_limits: u64,
}
impl NumericalSearchCounts {
    pub(crate) fn returned<T>(
        &mut self,
        result: &Result<T, Failure>,
        actual: RootSearchObservation,
    ) {
        self.iterations_entered += actual.iterations_entered as u64;
        self.observe_inner(actual.inner);
        if result.is_err() {
            self.failed += 1;
            return;
        }
        *match actual.termination {
            Exit::Accepted => &mut self.accepted,
            Exit::LocalDecline | Exit::Stationary | Exit::Stagnation => &mut self.locally_declined,
            Exit::EvaluationDeclined => &mut self.evaluation_declined,
            Exit::Divergence => &mut self.divergent,
            Exit::IterationLimit => &mut self.exhausted,
            _ => &mut self.unobserved,
        } += 1;
    }
    pub(crate) fn all_accepted(&self) -> bool {
        self.scheduled == self.accepted
            && self.residual_declined == 0
            && self.tangency_declined == 0
            && self.scalar_searches == self.scalar_accepted
            && self.scalar_evaluation_declined == 0
            && self.descent_limits == 0
    }
    pub(crate) fn accumulate(&mut self, actual: Self) {
        self.scheduled += actual.scheduled;
        self.accepted += actual.accepted;
        self.residual_declined += actual.residual_declined;
        self.locally_declined += actual.locally_declined;
        self.evaluation_declined += actual.evaluation_declined;
        self.divergent += actual.divergent;
        self.exhausted += actual.exhausted;
        self.failed += actual.failed;
        self.unobserved += actual.unobserved;
        self.tangency_declined += actual.tangency_declined;
        self.iterations_entered += actual.iterations_entered;
        self.accumulate_inner(actual);
    }
    fn accumulate_inner(&mut self, actual: Self) {
        self.scalar_searches += actual.scalar_searches;
        self.scalar_accepted += actual.scalar_accepted;
        self.scalar_exhausted += actual.scalar_exhausted;
        self.scalar_evaluation_declined += actual.scalar_evaluation_declined;
        self.scalar_iterations_entered += actual.scalar_iterations_entered;
        self.scalar_evaluations += actual.scalar_evaluations;
        self.scalar_result_declined += actual.scalar_result_declined;
        self.descent_limits += actual.descent_limits;
    }
}

impl NumericalSearchCounts {
    fn observe_inner(&mut self, actual: martensite_tkmath::math::RootSearchInnerWork) {
        self.scalar_searches += actual.searches;
        self.scalar_accepted += actual.accepted;
        self.scalar_exhausted += actual.exhausted;
        self.scalar_evaluation_declined += actual.evaluation_declined;
        self.scalar_iterations_entered += actual.iterations_entered;
        self.scalar_evaluations += actual.evaluations;
        self.scalar_result_declined += actual.result_declined;
        self.descent_limits += actual.standard_descent_limits + actual.boundary_descent_limits;
    }
}

#[cfg(test)]
#[path = "search_counts_tests.rs"]
mod tests;
