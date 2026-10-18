//! Successful native-query baseline for #89; injected failures need separate evidence.
#![forbid(unsafe_code)]

#[test]
fn successful_time_of_day_preserves_raw_errno_and_microsecond_shape() {
    use nix::errno::Errno;
    let original = Errno::last_raw();
    for sentinel in [0, libc::EIO, 12345] {
        Errno::set_raw(sentinel);
        let stamp = super::time_of_day();
        let observed = Errno::last_raw();
        Errno::set_raw(original);
        let (_, micros) = stamp.expect("native gettimeofday should succeed with valid storage");
        assert!(
            (0..1_000_000).contains(&micros),
            "tv_usec={micros}; expected normalized microseconds in 0..1_000_000"
        );
        assert_eq!(observed, sentinel, "successful query changed raw errno");
    }
}
