//! Pin the native tick unit and successful-query errno behavior before replacement.

#[test]
fn clock_ticks_match_independent_getconf() {
    let output = std::process::Command::new("getconf")
        .arg("CLK_TCK")
        .output()
        .unwrap();
    assert!(output.status.success(), "getconf failed: {output:?}");
    let expected: f64 = std::str::from_utf8(&output.stdout)
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    assert!(expected > 0.0);
    assert_eq!(super::clock_ticks(), expected);
}

#[test]
fn clock_ticks_preserve_callers_errno() {
    use nix::errno::Errno;
    let original = Errno::last_raw();
    for sentinel in [0, libc::EIO, 12345] {
        Errno::set_raw(sentinel);
        let ticks = super::clock_ticks();
        let observed = Errno::last_raw();
        Errno::set_raw(original);
        assert!(ticks > 0.0);
        assert_eq!(observed, sentinel);
    }
}

#[test]
fn clock_tick_results_preserve_value_and_errno_contracts() {
    use nix::errno::Errno;
    assert_clock_tick_result(Ok(Some(100)), 0, 100.0, 12345);
    assert_clock_tick_result(Ok(None), 0, -1.0, 12345);
    assert_clock_tick_result(Err(Errno::EINVAL), libc::EINVAL, -1.0, libc::EINVAL);
    assert_clock_tick_result(Ok(Some(100)), libc::EIO, 100.0, libc::EIO);
}

fn assert_clock_tick_result(
    queried: nix::Result<Option<libc::c_long>>,
    query_errno: i32,
    expected_ticks: f64,
    expected_errno: i32,
) {
    use nix::errno::Errno;
    let original = Errno::last_raw();
    Errno::set_raw(query_errno);
    let ticks = super::clock_ticks_result(12345, queried);
    let observed = Errno::last_raw();
    Errno::set_raw(original);
    assert_eq!((ticks, observed), (expected_ticks, expected_errno));
}
