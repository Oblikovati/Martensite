//! Recorded images are exact only on the CPU vendor they were recorded on.
//!
//! Lavapipe's output differs between CPU vendors at some pixels, for the same driver build and
//! settings. The recordings come from AMD CPUs, so these checks are exact there. On other vendors
//! a mismatch prints a notice instead of failing. Everything else in a test still applies.
use std::sync::OnceLock;

/// Fails on a recording CPU, or prints a notice elsewhere;
/// `require_exact(differing.is_empty(), || format!("{differing:?}"))`.
pub(crate) fn require_exact(exact: bool, message: impl FnOnce() -> String) {
    if exact {
        return;
    }
    let message = message();
    if let Some(vendor) = other_vendor() {
        let notice: String = message.chars().take(400).collect();
        eprintln!("recorded image check skipped on {vendor}: {notice}");
        return;
    }
    panic!("{message}");
}

/// The CPU vendor when it is known not to be the recording one.
fn other_vendor() -> Option<&'static str> {
    static VENDOR: OnceLock<Option<String>> = OnceLock::new();
    let vendor = VENDOR.get_or_init(|| {
        let info = std::fs::read_to_string("/proc/cpuinfo").ok()?;
        let line = info.lines().find(|line| line.starts_with("vendor_id"))?;
        Some(line.split(':').nth(1)?.trim().to_owned())
    });
    vendor.as_deref().filter(|vendor| *vendor != "AuthenticAMD")
}
