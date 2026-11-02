use super::FunctionSetRoot;

/// Observed local solver exit, not a statement about all roots.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RootSearchTermination {
    /// No solve was entered.
    #[default]
    NotRun,
    /// The solve has not returned yet.
    Running,
    /// The existing local-solution stopping criterion accepted the iterate.
    Accepted,
    /// The existing progress criteria declined to continue.
    LocalDecline,
    /// A vanishing search slope stopped the existing solver.
    Stationary,
    /// The existing repeated-progress test stopped the search.
    Stagnation,
    /// Function evaluation declined or exceeded its supported magnitude.
    EvaluationDeclined,
    /// Requested divergence stopping rejected an out-of-bounds iterate.
    Divergence,
    /// The configured outer iteration budget was exhausted.
    IterationLimit,
    /// Input validation or preparation failed.
    InvalidInput,
}

/// Owned observation available even when a root was not accepted.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RootSearchObservation {
    /// Last observed exit reason.
    pub termination: RootSearchTermination,
    /// Outer iterations actually entered, excluding the exhaustion sentinel.
    pub iterations_entered: i32,
    /// Configured outer iteration budget.
    pub iteration_limit: i32,
    /// Actual bounded scalar-repair and descent work.
    pub inner: RootSearchInnerWork,
}
impl FunctionSetRoot {
    /// Reads local work without requiring convergence; `solver.search_observation()`.
    pub fn search_observation(&self) -> RootSearchObservation {
        self.observation
    }
    pub(super) fn record_finish(&mut self, accepted: bool) {
        if accepted && self.run.objective > 1e-32 && self.run.gradient_norm <= 1e-64 {
            self.observation.termination = RootSearchTermination::Stationary;
            return;
        }
        if matches!(
            self.observation.termination,
            RootSearchTermination::Stationary | RootSearchTermination::Stagnation
        ) || (!accepted && self.observation.termination != RootSearchTermination::Running)
        {
            return;
        }
        self.observation.termination = if accepted {
            RootSearchTermination::Accepted
        } else {
            RootSearchTermination::LocalDecline
        };
    }
}

/// Local scalar repairs and bounded descent events within one root search.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RootSearchInnerWork {
    /// Scalar repair searches entered.
    pub searches: u64,
    /// Scalar repairs whose existing stopping criterion was reached.
    pub accepted: u64,
    /// Scalar repairs that exhausted their iteration budget.
    pub exhausted: u64,
    /// Scalar iterations whose stopping criterion was evaluated.
    pub iterations_entered: u64,
    /// Line-function evaluations entered, including direct interpolation checks.
    pub evaluations: u64,
    /// Line-function evaluations that declined or exceeded their supported magnitude.
    pub evaluation_declined: u64,
    /// Returned repair candidates rejected by the existing improvement filter.
    pub result_declined: u64,
    /// Existing standard descent-limit events.
    pub standard_descent_limits: u64,
    /// Existing boundary descent-limit events.
    pub boundary_descent_limits: u64,
}
