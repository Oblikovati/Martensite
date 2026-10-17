use super::{action, interrupt_handler};
use nix::errno::Errno;

#[cfg(target_os = "macos")]
#[test]
fn darwin_action_starts_with_a_zero_signal_mask() {
    assert_eq!(action(libc::SIG_DFL, libc::SA_RESTART).sa_mask, 0);
}

#[test]
fn action_preserves_requested_handler_and_flags() {
    let interrupt = interrupt_handler as *const () as usize;
    for (handler, flags) in [
        (libc::SIG_DFL, 0),
        (libc::SIG_DFL, libc::SA_RESTART),
        (interrupt, libc::SA_RESTART | libc::SA_SIGINFO),
    ] {
        let record = action(handler, flags);
        assert_eq!(record.sa_sigaction, handler);
        assert_eq!(record.sa_flags, flags);
    }
}

#[test]
fn action_construction_preserves_callers_errno() {
    for sentinel in [0, libc::EIO, 12345] {
        let previous = Errno::last_raw();
        Errno::set_raw(sentinel);
        let _record = action(libc::SIG_DFL, libc::SA_RESTART);
        let observed = Errno::last_raw();
        Errno::set_raw(previous);
        assert_eq!(observed, sentinel);
    }
}

#[cfg(all(
    target_os = "linux",
    target_env = "gnu",
    any(target_arch = "x86_64", target_arch = "aarch64")
))]
#[test]
fn action_starts_without_a_restorer() {
    assert!(
        action(libc::SIG_DFL, libc::SA_RESTART)
            .sa_restorer
            .is_none()
    );
}
