use super::{action, interrupt_handler};
use nix::errno::Errno;

#[test]
fn action_starts_with_an_empty_signal_mask() {
    let expected = nix::sys::signal::SigSet::empty();
    let record = action(libc::SIG_DFL, libc::SA_RESTART);
    assert_eq!(&record.sa_mask, expected.as_ref());
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
