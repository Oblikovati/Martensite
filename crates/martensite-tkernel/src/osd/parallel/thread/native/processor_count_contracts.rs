//! Pin the online CPU query independently of affinity-limited parallelism (#89).

#[cfg(target_os = "linux")]
#[test]
fn online_processors_match_getconf() {
    let output = std::process::Command::new("getconf")
        .arg("_NPROCESSORS_ONLN")
        .output()
        .unwrap();
    assert!(output.status.success(), "getconf failed: {output:?}");
    let expected: i32 = std::str::from_utf8(&output.stdout)
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    assert!(expected > 0);
    assert_eq!(super::online_processors(), expected);
}

#[test]
fn online_processors_preserve_callers_errno() {
    use nix::errno::Errno;
    let original = Errno::last_raw();
    for sentinel in [0, libc::EIO, 12345] {
        Errno::set_raw(sentinel);
        let count = super::online_processors();
        let observed = Errno::last_raw();
        Errno::set_raw(original);
        assert!(count > 0);
        assert_eq!(observed, sentinel);
    }
}
