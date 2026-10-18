//! GNU/Linux `times` baseline for #89; no injected invalid storage is used.
#![forbid(unsafe_code)]

#[test]
fn process_ticks_preserve_incoming_errno() {
    use nix::errno::Errno;
    let original = Errno::last_raw();
    for sentinel in [0, libc::EIO, 12345] {
        Errno::set_raw(sentinel);
        let (user, system) = super::tms_seconds(100.0);
        let observed = Errno::last_raw();
        Errno::set_raw(original);
        assert!(
            user.is_finite() && user >= 0.0,
            "user={user}; expected nonnegative finite seconds"
        );
        assert!(
            system.is_finite() && system >= 0.0,
            "system={system}; expected nonnegative finite seconds"
        );
        assert_eq!(observed, sentinel);
    }
}

#[test]
#[ignore = "requires externally verified strace times error injection; see #89"]
fn injected_process_ticks_failure_keeps_zero_output_and_errno() {
    use nix::errno::Errno;
    let original = Errno::last_raw();
    Errno::set_raw(12345);
    let seconds = super::tms_seconds(100.0);
    let observed = Errno::last_raw();
    Errno::set_raw(original);
    assert_eq!(seconds, (0.0, 0.0));
    assert_eq!(observed, 12345);
}
