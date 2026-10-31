//! Signal support describes available mechanisms, not installed handlers or trap masks.

pub(super) const FLOATING_POINT_TRAP_CONTROL: bool = cfg!(any(
    windows,
    all(
        target_os = "linux",
        target_env = "gnu",
        any(
            target_arch = "x86",
            target_arch = "x86_64",
            target_arch = "aarch64"
        )
    )
));

/// Native signal mechanisms exposed by OSD on the current target.
///
/// ```
/// let capabilities = martensite_tkernel::osd::Osd::signal_capabilities();
/// assert!(!capabilities.recoverable_native_faults);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SignalCapabilities {
    /// SIGINT or console Ctrl-C/Ctrl-Break can arm the ordinary cancellation flag.
    pub console_interruptions: bool,
    /// The calling thread's floating-point trap mask can be configured and read.
    pub floating_point_trap_control: bool,
    /// Hardware faults can become Rust failures without terminating the process.
    /// Currently false on every target; installing handlers does not provide recovery.
    pub recoverable_native_faults: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::osd::Osd;

    #[test]
    fn native_capabilities_report_mechanisms_without_changing_signal_state() {
        fn require_send_sync<T: Send + Sync>() {}
        require_send_sync::<SignalCapabilities>();
        let before = (Osd::signal_mode(), Osd::to_catch_floating_signals());
        let capabilities = Osd::signal_capabilities();
        assert!(!capabilities.recoverable_native_faults);
        assert_eq!(
            before,
            (Osd::signal_mode(), Osd::to_catch_floating_signals())
        );
    }
}
