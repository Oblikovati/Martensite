//! Native Windows tests: cross-compilation does not establish their runtime results.
use super::super::preserve_document_access;
use super::*;
use std::os::windows::ffi::OsStringExt;
use std::os::windows::fs::OpenOptionsExt;

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new() -> Self {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "martensite-security-{}-{nonce}",
            std::process::id()
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn stage(&self) -> PathBuf {
        let directory = self.0.join("private-stage");
        create_private_directory(&directory).unwrap();
        let file = directory.join("model");
        fs::write(&file, b"new document").unwrap();
        file
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn snapshot(path: &Path) -> Vec<String> {
    ACCESS_SECTIONS
        .iter()
        .map(|&section| section_text(path, section).unwrap())
        .collect()
}

fn set_dacl(path: &Path, text: &str) {
    let descriptor = parse_descriptor(text).unwrap();
    apply_section(path, DACL_SECURITY_INFORMATION, &descriptor).unwrap();
}

#[test]
fn private_stage_excludes_broad_inheritable_parent_grants() {
    let root = TestDirectory::new();
    let sid = effective_user_sid().unwrap();
    set_dacl(&root.0, "D:P(A;OICI;FA;;;WD)");
    let stage = root.stage();
    let directory = stage.parent().unwrap();
    let expected = parse_descriptor(&format!("O:{sid}D:P(A;OICI;FA;;;{sid})")).unwrap();
    for section in [OWNER_SECURITY_INFORMATION, DACL_SECURITY_INFORMATION] {
        assert_eq!(
            section_text(directory, section).unwrap(),
            normalized_section(descriptor_text(&expected, section).unwrap())
        );
    }
    assert_eq!(fs::read(&stage).unwrap(), b"new document");
    assert!(create_private_directory(directory).is_err());
}

#[test]
fn existing_restrictive_policy_preserves_owner_group_and_acl() {
    let root = TestDirectory::new();
    let destination = root.0.join("original");
    fs::write(&destination, b"old document").unwrap();
    let sid = effective_user_sid().unwrap();
    set_dacl(&destination, &format!("D:P(A;;FA;;;{sid})"));
    let before = snapshot(&destination);
    let stage = root.stage();
    preserve_access(&destination, &stage).unwrap();
    assert_eq!(snapshot(&stage), before);
    assert_eq!(snapshot(&destination), before);
    assert_eq!(fs::read(&destination).unwrap(), b"old document");
    fs::rename(stage, &destination).unwrap();
    assert_eq!(snapshot(&destination), before);
    assert_eq!(fs::read(destination).unwrap(), b"new document");
}

#[test]
fn new_destination_uses_final_parent_creation_policy() {
    let root = TestDirectory::new();
    let ordinary = root.0.join("ordinary");
    fs::write(&ordinary, []).unwrap();
    let expected = snapshot(&ordinary);
    let stage = root.stage();
    let destination = root.0.join("文書-Δ-model");
    preserve_access(&destination, &stage).unwrap();
    assert!(!destination.exists());
    assert_eq!(snapshot(&stage), expected);
    fs::rename(stage, &destination).unwrap();
    assert_eq!(snapshot(&destination), expected);
    assert_eq!(fs::read(destination).unwrap(), b"new document");
}

#[test]
fn writer_restrictions_refuse_widening_without_modifying_original() {
    let root = TestDirectory::new();
    let destination = root.0.join("original");
    fs::write(&destination, b"old document").unwrap();
    let before = snapshot(&destination);
    let stage = root.stage();
    let sid = effective_user_sid().unwrap();
    set_dacl(&stage, &format!("D:P(A;;FR;;;{sid})"));
    let restrictive = snapshot(&stage);
    let error = preserve_access(&destination, &stage).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
    assert_eq!(snapshot(&stage), restrictive);
    assert_eq!(snapshot(&destination), before);
    assert_eq!(fs::read(destination).unwrap(), b"old document");
}

#[test]
fn exclusive_destination_handle_blocks_rename_without_removing_original() {
    let root = TestDirectory::new();
    let destination = root.0.join("original");
    fs::write(&destination, b"old document").unwrap();
    let stage = root.stage();
    preserve_access(&destination, &stage).unwrap();
    let before = snapshot(&destination);
    let held = OpenOptions::new()
        .read(true)
        .share_mode(0)
        .open(&destination)
        .unwrap();
    let error = fs::rename(&stage, &destination).unwrap_err();
    assert!(error.raw_os_error().is_some());
    drop(held);
    assert_eq!(snapshot(&destination), before);
    assert_eq!(fs::read(&destination).unwrap(), b"old document");
    assert_eq!(fs::read(stage).unwrap(), b"new document");
}

#[test]
fn invalid_paths_preserve_windows_error_codes_and_unicode() {
    let unicode = PathBuf::from("文書-Δ");
    assert_eq!(
        wide_path(&unicode).unwrap(),
        "文書-Δ\0".encode_utf16().collect::<Vec<_>>()
    );
    let invalid = PathBuf::from(std::ffi::OsString::from_wide(&[65, 0, 66]));
    assert_eq!(
        wide_path(&invalid).unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    assert!(read_section(&invalid, OWNER_SECURITY_INFORMATION).is_err());
    let root = TestDirectory::new();
    let error = read_section(&root.0.join("absent"), OWNER_SECURITY_INFORMATION)
        .err()
        .unwrap();
    assert_eq!(error.kind(), io::ErrorKind::NotFound);
    assert!(error.raw_os_error().is_some());
    assert_eq!(
        normalized_section("D:PAIAR(A;;FA;;;WD)".to_owned()),
        "D:P(A;;FA;;;WD)"
    );
}

#[test]
fn nonregular_destination_refusal_leaves_staged_bytes_intact() {
    let root = TestDirectory::new();
    let stage = root.stage();
    let error = preserve_access(&root.0, &stage).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    assert_eq!(fs::read(stage).unwrap(), b"new document");
}

#[test]
fn readonly_writer_output_is_refused_without_changing_original() {
    let root = TestDirectory::new();
    let destination = root.0.join("original");
    fs::write(&destination, b"old document").unwrap();
    let before = snapshot(&destination);
    let stage = root.stage();
    let original_permissions = fs::metadata(&stage).unwrap().permissions();
    let mut permissions = original_permissions.clone();
    permissions.set_readonly(true);
    fs::set_permissions(&stage, permissions).unwrap();
    assert_eq!(
        preserve_access(&destination, &stage).unwrap_err().kind(),
        io::ErrorKind::PermissionDenied
    );
    assert!(fs::metadata(&stage).unwrap().permissions().readonly());
    assert_eq!(snapshot(&destination), before);
    assert_eq!(fs::read(&destination).unwrap(), b"old document");
    fs::set_permissions(stage, original_permissions).unwrap();
}

fn named_stream(path: &Path, name: &str) -> PathBuf {
    let mut stream = path.as_os_str().to_os_string();
    stream.push(format!(":{name}"));
    PathBuf::from(stream)
}

#[test]
fn named_destination_streams_refuse_publication_without_metadata_changes() {
    for name in ["Zone.Identifier", "user.metadata"] {
        let root = TestDirectory::new();
        let destination = root.0.join("original");
        fs::write(&destination, b"old document").unwrap();
        let stream = named_stream(&destination, name);
        fs::write(&stream, b"owned named-stream bytes").unwrap();
        let stage = root.stage();
        let before = (snapshot(&destination), snapshot(&stage));
        let error = preserve_document_access(&destination, &stage).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
        assert!(error.to_string().contains(name));
        assert_eq!(fs::read(&destination).unwrap(), b"old document");
        assert_eq!(fs::read(stream).unwrap(), b"owned named-stream bytes");
        assert_eq!(fs::read(&stage).unwrap(), b"new document");
        assert_eq!((snapshot(&destination), snapshot(&stage)), before);
    }
}

#[test]
fn stream_inspection_retains_native_missing_destination_error() {
    let root = TestDirectory::new();
    let error = streams::reject_named_streams(&root.0.join("missing")).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::NotFound);
    assert!(error.raw_os_error().is_some());
}
