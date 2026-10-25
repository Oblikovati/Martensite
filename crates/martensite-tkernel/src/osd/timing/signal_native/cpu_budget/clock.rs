//! A failed clock or an unavailable diagnostic must never hold up process exit.
#![allow(unsafe_code)]
use super::OUTPUT_COMPLETE;
use std::sync::atomic::Ordering;

pub(super) const DIAGNOSTIC_GRACE_NS: u64 = 100_000_000;

#[cfg(not(target_os = "macos"))]
pub(super) fn wait_for_diagnostic() {
    let Some(start) = monotonic_nanoseconds() else {
        return;
    };
    while !OUTPUT_COMPLETE.load(Ordering::Acquire) {
        let Some(now) = monotonic_nanoseconds() else {
            return;
        };
        if now.saturating_sub(start) >= DIAGNOSTIC_GRACE_NS {
            return;
        }
        std::hint::spin_loop();
    }
}

/// Queries the signal-safe clock; `clock::monotonic_nanoseconds()` returns no fabricated time on failure.
pub(super) fn monotonic_nanoseconds() -> Option<u64> {
    let mut instant = libc::timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    // SAFETY: the initialized timespec is writable; CLOCK_MONOTONIC is supported
    // on the Unix targets. clock_gettime is async-signal-safe and failure exits.
    if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut instant) } != 0 {
        return None;
    }
    Some((instant.tv_sec as u64).saturating_mul(1_000_000_000) + instant.tv_nsec as u64)
}

#[cfg(target_os = "macos")]
/// Starts the existing grace once; `clock::diagnostic_deadline()` also bounds the isolated writer.
pub(super) fn diagnostic_deadline() -> Option<u64> {
    monotonic_nanoseconds().map(|now| now.saturating_add(DIAGNOSTIC_GRACE_NS))
}

#[cfg(target_os = "macos")]
/// Waits for completion or the original deadline; `clock::wait_until_deadline(deadline)` never extends it.
pub(super) fn wait_until_deadline(deadline: u64) {
    while !OUTPUT_COMPLETE.load(Ordering::Acquire) {
        let Some(now) = monotonic_nanoseconds() else {
            return;
        };
        if now >= deadline {
            return;
        }
        std::hint::spin_loop();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn monotonic_clock_advances_without_output() {
        let before = super::monotonic_nanoseconds().unwrap();
        let after = super::monotonic_nanoseconds().unwrap();
        assert!(after >= before);
        assert_eq!(super::DIAGNOSTIC_GRACE_NS, 100_000_000);
    }
}
