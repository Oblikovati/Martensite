//! Pin native sleep errno and interruption without wall-clock timing assertions (#89).

#[test]
fn successful_native_sleeps_preserve_callers_errno() {
    for sentinel in [0, libc::EIO, 12345] {
        assert_sleep_errno(super::sec_sleep, 0, sentinel, sentinel);
        assert_sleep_errno(super::milli_sec_sleep, 0, sentinel, sentinel);
        assert_sleep_errno(super::milli_sec_sleep, 1, sentinel, sentinel);
    }
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
#[test]
#[ignore = "requires isolated strace EINTR injection for every sleep syscall and a timeout"]
fn rejected_native_sleeps_keep_wrapped_requests_and_eintr() {
    for argument in [0, 1, -1] {
        assert_sleep_errno(super::sec_sleep, argument, 12345, libc::EINTR);
        assert_sleep_errno(super::milli_sec_sleep, argument, 12345, libc::EINTR);
    }
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
#[test]
#[ignore = "requires isolated strace injection of raw error 4094 for every sleep syscall"]
fn rejected_native_sleeps_preserve_unnamed_errno() {
    assert_sleep_errno(super::sec_sleep, 0, 12345, 4094);
    assert_sleep_errno(super::milli_sec_sleep, 0, 12345, 4094);
}

fn assert_sleep_errno(sleep: fn(i32), argument: i32, incoming: i32, expected: i32) {
    use nix::errno::Errno;
    let original = Errno::last_raw();
    Errno::set_raw(incoming);
    sleep(argument);
    let observed = Errno::last_raw();
    Errno::set_raw(original);
    assert_eq!(observed, expected, "sleep argument {argument}");
}
