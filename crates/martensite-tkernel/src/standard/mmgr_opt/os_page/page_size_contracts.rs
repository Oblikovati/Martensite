//! Pin the system query before replacing its direct FFI (#89).

#[test]
fn page_size_matches_independent_getconf() {
    let output = std::process::Command::new("getconf")
        .arg("PAGESIZE")
        .output()
        .unwrap();
    assert!(output.status.success(), "getconf failed: {output:?}");
    let expected: usize = std::str::from_utf8(&output.stdout)
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    assert!(expected > 0);
    assert_eq!(super::os_page_size(), expected);
}

#[test]
fn page_size_preserves_callers_errno() {
    use nix::errno::Errno;
    let original = Errno::last_raw();
    for sentinel in [0, libc::EIO, 12345] {
        Errno::set_raw(sentinel);
        let size = super::os_page_size();
        let observed = Errno::last_raw();
        Errno::set_raw(original);
        assert!(size > 0);
        assert_eq!(observed, sentinel);
    }
}
