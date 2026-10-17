//! Pin native host-name bytes and errno before replacing the call (#89).

#[cfg(all(target_os = "linux", target_env = "gnu"))]
#[test]
#[ignore = "requires an isolated UTS namespace with a 63- or 64-byte hostname"]
fn hostname_boundary_in_isolated_namespace() {
    use nix::errno::Errno;
    let length = std::env::var("MARTENSITE_HOSTNAME_LENGTH").unwrap();
    let (expected, expected_errno) = match length.as_str() {
        "63" => (Ok(vec![b'h'; 63]), 12345),
        "64" => (Err(libc::ENAMETOOLONG), libc::ENAMETOOLONG),
        other => panic!("expected length 63 or 64, got {other}"),
    };
    let original = Errno::last_raw();
    Errno::set_raw(12345);
    let result = super::host_name();
    let observed = Errno::last_raw();
    Errno::set_raw(original);
    assert_eq!((result, observed), (expected, expected_errno));
}

#[test]
fn hostname_matches_native_uname_command() {
    let output = std::process::Command::new("uname")
        .arg("-n")
        .output()
        .unwrap();
    assert!(output.status.success(), "uname failed: {output:?}");
    let mut expected = output.stdout;
    assert_eq!(expected.pop(), Some(b'\n'));
    #[cfg(target_vendor = "apple")]
    expected.truncate(63);
    assert!(
        expected.len() < 64,
        "fixture requires short hostname: {expected:?}"
    );
    assert_eq!(super::host_name(), Ok(expected));
}

#[test]
fn hostname_preserves_callers_errno_on_success() {
    use nix::errno::Errno;
    let original = Errno::last_raw();
    for sentinel in [0, libc::EIO, 12345] {
        Errno::set_raw(sentinel);
        let name = super::host_name();
        let observed = Errno::last_raw();
        Errno::set_raw(original);
        assert!(name.is_ok(), "native hostname failed: {name:?}");
        assert_eq!(observed, sentinel);
    }
}
