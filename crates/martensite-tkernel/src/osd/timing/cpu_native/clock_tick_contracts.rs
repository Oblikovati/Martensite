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
