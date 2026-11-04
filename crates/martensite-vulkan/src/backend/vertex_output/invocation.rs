//! Recording the target twice must not overwrite the first observed invocation.
use std::sync::atomic::{AtomicUsize, Ordering};
#[derive(Default)]
pub(in crate::backend) struct InvocationCount(AtomicUsize);
impl InvocationCount {
    pub(in crate::backend) fn begin(&self) -> bool {
        // Command recording is serial, while the owner remains Send + Sync.
        self.0.fetch_add(1, Ordering::Relaxed) == 0
    }
    pub(in crate::backend) fn ending(&self) -> bool {
        self.0.load(Ordering::Relaxed) == 1
    }
    pub(in crate::backend) fn actual(&self) -> usize {
        self.0.load(Ordering::Relaxed)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_and_repeated_dispatches_refuse_without_overwriting_first_capture() {
        let count = InvocationCount::default();
        assert_eq!(count.actual(), 0);
        assert!(!count.ending());
        assert!(count.begin());
        assert!(count.ending());
        assert_eq!(count.actual(), 1);
        assert!(!count.begin());
        assert!(!count.ending());
        assert_eq!(count.actual(), 2);
    }
}
