//! Invocation-local observations; nested solver termination remains explicit unknown.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct WalkingSearchObservation {
    pub steps: u64,
    pub no_append_limit: u64,
    pub repeated_start_limit: u64,
    pub point_count_limit: u64,
    pub inner_termination_unobserved: bool,
    pub cancelled: bool,
}
