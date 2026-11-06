//! Each status allocation belongs to one command-recording generation and one source draw.
use crate::{RenderError, RenderWindowDepthFailure as Failure};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
#[derive(Default)]
pub(super) struct InvocationState {
    generations: AtomicUsize,
    invocations: AtomicU64,
    bad_dispatch: AtomicBool,
    overflow: AtomicBool,
    overflow_increment: AtomicU64,
    observed_indices: AtomicU32,
}
impl InvocationState {
    pub(super) fn begin(&self) {
        self.generations
            .try_update(Ordering::SeqCst, Ordering::SeqCst, |old| {
                Some(old.saturating_add(1))
            })
            .expect("saturating generation update");
    }
    pub(super) fn dispatch(&self, expected_indices: u32, indices: u32, instances: u32) {
        if indices != expected_indices {
            self.observed_indices.store(indices, Ordering::SeqCst);
            self.bad_dispatch.store(true, Ordering::SeqCst);
        }
        let increment = u64::from(indices / 3) * u64::from(instances);
        if self
            .invocations
            .try_update(Ordering::SeqCst, Ordering::SeqCst, |old| {
                old.checked_add(increment)
            })
            .is_err()
        {
            self.overflow_increment.store(increment, Ordering::SeqCst);
            self.overflow.store(true, Ordering::SeqCst);
        }
    }
    pub(super) fn expected(&self, indices: u32) -> Result<u64, RenderError> {
        let generations = self.generations.load(Ordering::SeqCst);
        if generations != 1 {
            return Err(Failure::GpuGeneration {
                expected: 1,
                observed: generations,
            }
            .into());
        }
        if self.overflow.load(Ordering::SeqCst) {
            return Err(Failure::GpuInvocationOverflow {
                current: self.invocations.load(Ordering::SeqCst),
                increment: self.overflow_increment.load(Ordering::SeqCst),
            }
            .into());
        }
        if self.bad_dispatch.load(Ordering::SeqCst) {
            return Err(Failure::GpuDispatch {
                expected_indices: indices,
                observed_indices: self.observed_indices.load(Ordering::SeqCst),
            }
            .into());
        }
        Ok(self.invocations.load(Ordering::SeqCst))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fresh_and_reused_generations_never_borrow_prior_completion() {
        let state = InvocationState::default();
        assert!(state.expected(6).is_err());
        state.begin();
        state.dispatch(6, 6, 1);
        assert_eq!(state.expected(6).unwrap(), 2);
        state.begin();
        assert!(state.expected(6).is_err());
    }
    #[test]
    fn actual_shared_index_dispatches_and_instances_determine_total_invocations() {
        let state = InvocationState::default();
        state.begin();
        state.dispatch(6, 6, 2);
        state.dispatch(6, 6, 1);
        assert_eq!(state.expected(6).unwrap(), 6);
        state.dispatch(6, 3, 1);
        assert!(state.expected(6).is_err());
    }
    #[test]
    fn invocation_overflow_is_incomplete_instead_of_wrapping_to_an_old_count() {
        let state = InvocationState::default();
        state.begin();
        for _ in 0..4 {
            state.dispatch(u32::MAX, u32::MAX, u32::MAX);
        }
        assert!(matches!(
            state.expected(u32::MAX).unwrap_err().window_depth_failure(),
            Some(Failure::GpuInvocationOverflow { .. })
        ));
    }
}
