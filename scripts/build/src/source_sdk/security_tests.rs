use super::*;
use crate::error::BuildError;
use std::{fs, io, path::Path};

struct SealedArchiveFixture {
    directory: tempfile::TempDir,
    stage: PathBuf,
    manifest: integrity::Manifest,
}

impl SealedArchiveFixture {
    fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let stage = staged_payloads(directory.path());
        let snapshot = snapshot::Snapshot {
            revision: "a".repeat(40),
            gitlinks: Default::default(),
            executable_paths: Vec::new(),
            committed_sha256: Default::default(),
        };
        let manifest =
            integrity::write_manifest(&stage, &snapshot, &semver::Version::new(0, 4, 0)).unwrap();
        Self {
            directory,
            stage,
            manifest,
        }
    }

    fn archive(&self, top: &str, duplicate: bool) -> PathBuf {
        let path = self.directory.path().join("input.tar.gz");
        let mut entries = files::entries(&self.stage).unwrap();
        if duplicate {
            entries.push(entries[0].clone());
        }
        archive::write_tar_gz(&path, top, &entries).unwrap();
        path
    }

    fn rejected(&self, path: &Path, expected: &str) {
        let output = self.directory.path().join("extracted");
        let error = integrity::extract_verified(path, &output, &self.manifest).unwrap_err();
        assert!(error.to_string().contains(expected), "{error}");
        assert!(!self.directory.path().join("escape").exists());
    }

    // Host parsing may reject traversal before portable spelling, but must not extract (#313).
    fn rejected_raw_member(&self, member: &[u8]) {
        let output = self.directory.path().join("extracted");
        let archive = self.raw_member(member);
        let error = integrity::extract_verified(&archive, &output, &self.manifest).unwrap_err();
        assert_raw_path_rule(error, member);
        assert!(!output.exists());
        assert!(!self.directory.path().join("escape").exists());
    }

    fn crafted(&self, symlink: bool) -> PathBuf {
        let path = self.directory.path().join("crafted.tar.gz");
        let file = fs::File::create(&path).unwrap();
        let mut builder = tar::Builder::new(flate2::write::GzEncoder::new(
            file,
            flate2::Compression::fast(),
        ));
        let mut header = crafted_header(symlink);
        header.set_cksum();
        builder.append(&header, io::empty()).unwrap();
        builder.into_inner().unwrap().finish().unwrap();
        path
    }

    fn raw_member(&self, member: &[u8]) -> PathBuf {
        let path = self.directory.path().join("raw-member.tar.gz");
        let encoder = flate2::write::GzEncoder::new(
            fs::File::create(&path).unwrap(),
            flate2::Compression::fast(),
        );
        let mut builder = tar::Builder::new(encoder);
        let mut header = tar::Header::new_gnu();
        header.set_size(0);
        header.set_mode(0o644);
        header.as_mut_bytes()[..member.len()].copy_from_slice(member);
        header.set_cksum();
        builder.append(&header, io::empty()).unwrap();
        builder.into_inner().unwrap().finish().unwrap();
        path
    }

    fn altered_modes(&self) -> PathBuf {
        let input = self.archive("martensite-0.4.0-source-workspace", false);
        let output = self.directory.path().join("altered-modes.tar.gz");
        copy_with_mode(&input, &output, 0o4644);
        output
    }
}

fn assert_raw_path_rule(error: BuildError, member: &[u8]) {
    let BuildError::ReleaseRule(rule) = error else {
        panic!("expected archive path rejection, got {error}");
    };
    let member = Path::new(std::str::from_utf8(member).unwrap());
    assert!(
        rule.starts_with(&format!("SDK path {member:?}; expected ")),
        "{rule}"
    );
    assert!(
        rule.ends_with("portable path components")
            || rule.ends_with("a nonempty relative regular-file path"),
        "{rule}"
    );
}

fn copy_with_mode(input: &Path, output: &Path, mode: u32) {
    let mut reader =
        tar::Archive::new(flate2::read::GzDecoder::new(fs::File::open(input).unwrap()));
    let encoder = flate2::write::GzEncoder::new(
        fs::File::create(output).unwrap(),
        flate2::Compression::fast(),
    );
    let mut writer = tar::Builder::new(encoder);
    for item in reader.entries().unwrap() {
        let mut item = item.unwrap();
        let mut header = item.header().clone();
        header.set_mode(mode);
        header.set_cksum();
        writer.append(&header, &mut item).unwrap();
    }
    writer.into_inner().unwrap().finish().unwrap();
}

fn staged_payloads(directory: &Path) -> PathBuf {
    let stage = directory.join("stage");
    files::write(&stage, Path::new("Cargo.toml"), b"[workspace]").unwrap();
    files::write(&stage, Path::new("Cargo.lock"), b"version=4").unwrap();
    files::write(&stage, Path::new("fixture.txt"), b"sealed fixture").unwrap();
    files::write(
        &stage,
        Path::new("vendor/dependency/payload.rs"),
        b"sealed vendor",
    )
    .unwrap();
    stage
}

fn crafted_header(symlink: bool) -> tar::Header {
    let mut header = tar::Header::new_gnu();
    header.set_size(0);
    header.set_mode(0o644);
    if symlink {
        header
            .set_path("martensite-0.4.0-source-workspace/link")
            .unwrap();
        header.set_entry_type(tar::EntryType::Symlink);
        header.set_link_name("../escape").unwrap();
        return header;
    }
    let path = b"martensite-0.4.0-source-workspace/../escape";
    header.as_mut_bytes()[..path.len()].copy_from_slice(path);
    header.set_entry_type(tar::EntryType::Regular);
    header
}

#[test]
fn duplicate_members_and_wrong_top_directories_are_rejected() {
    let duplicate = SealedArchiveFixture::new();
    duplicate.rejected(
        &duplicate.archive("martensite-0.4.0-source-workspace", true),
        "unique regular file",
    );
    let wrong_top = SealedArchiveFixture::new();
    wrong_top.rejected(
        &wrong_top.archive("unexpected", false),
        "expected top directory",
    );
}

#[test]
fn symbolic_link_headers_and_traversal_paths_are_rejected() {
    for symlink in [true, false] {
        let fixture = SealedArchiveFixture::new();
        let expected = if symlink {
            "unique regular file"
        } else {
            "nonempty relative regular-file path"
        };
        fixture.rejected(&fixture.crafted(symlink), expected);
        assert!(fs::symlink_metadata(fixture.directory.path().join("extracted/link")).is_err());
    }
}

#[test]
fn special_permission_bits_cannot_pass_as_an_ordinary_sealed_payload() {
    let fixture = SealedArchiveFixture::new();
    fixture.rejected(&fixture.altered_modes(), "expected 644 or 755");
}

#[test]
fn missing_or_changed_sealed_vendor_files_are_rejected() {
    for remove in [true, false] {
        let fixture = SealedArchiveFixture::new();
        let vendor = fixture.stage.join("vendor/dependency/payload.rs");
        if remove {
            fs::remove_file(vendor).unwrap();
        } else {
            fs::write(vendor, "changed dependency").unwrap();
        }
        fixture.rejected(
            &fixture.archive("martensite-0.4.0-source-workspace", false),
            "expected exact manifest and payload hashes/modes",
        );
    }
}

#[test]
fn inherited_ancestor_cargo_configuration_is_rejected() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().join("workspace");
    fs::create_dir(&root).unwrap();
    files::write(
        directory.path(),
        Path::new(".cargo/config.toml"),
        b"[build]\nrustc-wrapper='unused-canary'",
    )
    .unwrap();
    assert!(
        commands::reject_ancestor_configuration(&root)
            .unwrap_err()
            .to_string()
            .contains("ancestor Cargo configuration")
    );
}

#[cfg(unix)]
#[test]
fn physical_ancestor_configuration_is_checked_through_symlinked_temporary_parents() {
    let directory = tempfile::tempdir().unwrap();
    let physical = directory.path().join("physical");
    let workspace = physical.join("temporary/workspace");
    fs::create_dir_all(&workspace).unwrap();
    files::write(
        &physical,
        Path::new(".cargo/config"),
        b"[build]\nrustflags=['unused-canary']",
    )
    .unwrap();
    std::os::unix::fs::symlink(physical.join("temporary"), directory.path().join("alias")).unwrap();
    let alias = directory.path().join("alias/workspace");
    assert!(
        commands::reject_ancestor_configuration(&alias)
            .unwrap_err()
            .to_string()
            .contains("ancestor Cargo configuration")
    );
}

#[test]
fn the_sealed_archive_control_passes_before_any_payload_or_header_mutation() {
    let fixture = SealedArchiveFixture::new();
    let archive = fixture.archive("martensite-0.4.0-source-workspace", false);
    let output = fixture.directory.path().join("positive-control");
    integrity::extract_verified(&archive, &output, &fixture.manifest).unwrap();
    assert_eq!(
        fs::read(output.join("vendor/dependency/payload.rs")).unwrap(),
        b"sealed vendor"
    );
}

#[test]
fn portable_member_roundtrip_keeps_all_sealed_payload_bytes_and_names() {
    let fixture = SealedArchiveFixture::new();
    let before = files::entries(&fixture.stage).unwrap();
    let archive = fixture.archive("martensite-0.4.0-source-workspace", false);
    let output = fixture.directory.path().join("portable-roundtrip");
    integrity::extract_verified(&archive, &output, &fixture.manifest).unwrap();
    let after = files::entries(&output).unwrap();
    assert_eq!(before.len(), after.len());
    for (before, after) in before.iter().zip(after.iter()) {
        assert_eq!(before.archive_path, after.archive_path);
        assert_eq!(before.executable, after.executable);
        assert_eq!(
            fs::read(&before.source).unwrap(),
            fs::read(&after.source).unwrap()
        );
    }
}

#[test]
fn raw_member_backslashes_and_drive_names_fail_before_native_reconstruction() {
    for member in [
        br"martensite-0.4.0-source-workspace/vendor\payload.rs".as_slice(),
        br"martensite-0.4.0-source-workspace/..\escape",
        b"martensite-0.4.0-source-workspace/vendor/C:payload",
    ] {
        let fixture = SealedArchiveFixture::new();
        fixture.rejected_raw_member(member);
    }
}
