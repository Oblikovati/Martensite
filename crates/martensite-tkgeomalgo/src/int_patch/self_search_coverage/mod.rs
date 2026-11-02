//! Observed work from one surface self-search; discovery remains sampled (#213).
mod recording;
#[cfg(test)]
mod tests;

/// The operation's observed termination, independent of its geometric result.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SelfSearchTermination {
    #[default]
    /// No self-search has run on this receiver.
    NotRun,
    /// The invocation has entered its search.
    Running,
    /// Every observed scheduled call returned; this does not certify global absence.
    Finished,
    /// An inner return or termination condition was not fully observed.
    Incomplete,
    /// A search call returned its existing error.
    Failed,
    /// The supplied progress range requested cancellation.
    Cancelled,
}
/// How the invocation chose its search domain.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SelfSearchDiscovery {
    #[default]
    /// The search domain has not been established.
    Unobserved,
    /// The existing analytic/extrusion policy bypassed the sampled search.
    AnalyticPolicy,
    /// Candidate discovery completed on a sampled surface polyhedron.
    SampledPolyhedron,
}
/// Calls accounted for at their actual dispatch and return sites.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SelfSearchWorkCounts {
    /// Calls made available for this invocation.
    pub scheduled: u64,
    /// Calls entered at their dispatch site.
    pub visited: u64,
    /// Calls that returned normally without being counted as pruned.
    pub finished: u64,
    /// Starts rejected by an existing geometric or result filter.
    pub pruned: u64,
    /// Entered calls that returned an error.
    pub failed: u64,
    /// Entered calls interrupted by the supplied progress range.
    pub cancelled: u64,
}
/// Existing limits observed during marching; zero does not certify other nested solvers.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SelfSearchLimitCounts {
    /// Times marching reached its existing no-append threshold.
    pub no_append: u64,
    /// Times marching stopped at the repeated-start limit.
    pub repeated_start: u64,
    /// Times marching stopped at its point-count limit.
    pub point_count: u64,
}
/// Owned work receipt; `search.self_search_receipt().termination` reads its outcome.
///
/// A finished sampled invocation accounts for dispatched work, not an exact global
/// self-intersection exclusion. Unobserved inner termination keeps it incomplete.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SurfaceSelfSearchReceipt {
    /// Receiver-local invocation number, incremented when self-search begins.
    pub invocation: u64,
    /// Observed operation termination, separate from geometric validity.
    pub termination: SelfSearchTermination,
    /// Policy used to establish the candidate domain.
    pub discovery: SelfSearchDiscovery,
    /// Whether sampled candidate discovery was actually entered.
    pub discovery_started: bool,
    /// Whether candidate discovery or the existing analytic policy returned.
    pub discovery_finished: bool,
    /// Discovered section-line calls.
    pub sections: SelfSearchWorkCounts,
    /// Discovered tangent-zone calls.
    pub tangent_zones: SelfSearchWorkCounts,
    /// Actual section or tangent start attempts.
    pub starts: SelfSearchWorkCounts,
    /// First-point numerical refinement calls.
    pub refinements: SelfSearchWorkCounts,
    /// Marching calls dispatched from accepted first points.
    pub walks: SelfSearchWorkCounts,
    /// Observed existing marching-limit events.
    pub limits: SelfSearchLimitCounts,
    /// Marching iterations actually entered.
    pub marching_steps: u64,
    /// Calls whose nested numerical termination was not available.
    pub calls_with_unobserved_inner_work: u64,
}
