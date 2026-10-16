//! Element destructors are `noexcept`, so a throwing one terminates the process.
//! Raw-pointer engines keep that outcome: an unwinding Rust `Drop` inside them aborts instead of
//! leaving dangling links for a later destruction pass (no recoverable state exists).

/// Run `destroy` and abort if it unwinds; `terminate_on_unwind(|| unsafe { drop_node(node) })`.
pub(super) fn terminate_on_unwind<R>(destroy: impl FnOnce() -> R) -> R {
    let terminate = TerminateOnUnwind;
    let result = destroy();
    std::mem::forget(terminate);
    result
}

/// Dropped only while `destroy` unwinds; `std::process::abort` terminates the process.
struct TerminateOnUnwind;

impl Drop for TerminateOnUnwind {
    fn drop(&mut self) {
        std::process::abort();
    }
}

#[cfg(test)]
mod tests {
    use super::terminate_on_unwind;

    #[test]
    fn completed_destruction_returns_its_result() {
        assert_eq!(terminate_on_unwind(|| 7), 7);
    }
}
