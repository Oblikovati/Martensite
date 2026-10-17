//! Pin successful native system-name queries before changing their adapter (#89).

#[cfg(target_os = "linux")]
#[test]
#[ignore = "requires an isolated process with uname rejected as EIO by strace"]
fn rejected_system_name_query_preserves_empty_output_and_errno() {
    use nix::errno::Errno;
    let original = Errno::last_raw();
    Errno::set_raw(12345);
    let names = super::system_names();
    let observed = Errno::last_raw();
    Errno::set_raw(original);
    assert_eq!(names, super::SystemNames::default());
    assert_eq!(observed, libc::EIO);
}

#[test]
fn system_names_match_native_uname_fields() {
    let expected = super::SystemNames {
        sysname: uname_field("-s"),
        release: uname_field("-r"),
    };
    assert_eq!(super::system_names(), expected);
}

fn uname_field(option: &str) -> Vec<u8> {
    let output = std::process::Command::new("uname")
        .arg(option)
        .output()
        .unwrap();
    assert!(output.status.success(), "uname {option} failed: {output:?}");
    let mut bytes = output.stdout;
    assert_eq!(bytes.pop(), Some(b'\n'));
    bytes
}

#[test]
fn system_names_preserve_callers_errno_on_success() {
    use nix::errno::Errno;
    let original = Errno::last_raw();
    for sentinel in [0, libc::EIO, 12345] {
        Errno::set_raw(sentinel);
        let names = super::system_names();
        let observed = Errno::last_raw();
        Errno::set_raw(original);
        assert!(!names.sysname.is_empty(), "empty system name: {names:?}");
        assert!(!names.release.is_empty(), "empty release: {names:?}");
        assert_eq!(observed, sentinel);
    }
}
