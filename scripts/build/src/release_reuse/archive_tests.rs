//! Linux is the workflow's artifact verification host; no production service is contacted.
use super::*;
use crate::{
    context::BuildContext,
    fake_runner::RecordingRunner,
    git::GitRepo,
    process::{CommandOutput, CommandRunner, CommandSpec, ProcessRunner},
    report::{LogFlavor, StepReporter},
};
use sha2::{Digest, Sha256};
use std::path::Path;

pub(super) struct ArtifactRunner(pub(super) RecordingRunner);
impl CommandRunner for ArtifactRunner {
    fn run(&self, spec: &CommandSpec) -> BuildResult<()> {
        ProcessRunner.run(spec)
    }
    fn capture(&self, spec: &CommandSpec) -> BuildResult<String> {
        if spec.display().starts_with("python3 ") {
            ProcessRunner.capture(spec)
        } else {
            self.0.capture(spec)
        }
    }
    fn exit_code(&self, spec: &CommandSpec) -> BuildResult<i32> {
        ProcessRunner.exit_code(spec)
    }
    fn capture_output(&self, spec: &CommandSpec) -> BuildResult<CommandOutput> {
        if spec.display().starts_with("python3 ") {
            ProcessRunner.capture_output(spec)
        } else {
            self.0.capture_output(spec)
        }
    }
}

fn zip_bytes(root: &Path, name: &str, copies: usize) -> Vec<u8> {
    let path = root.join("fixture.zip");
    let script = "import sys,zipfile\nwith zipfile.ZipFile(sys.argv[1],'w') as z:\n for _ in range(int(sys.argv[3])): z.writestr(sys.argv[2],b'payload')";
    ProcessRunner
        .run(&CommandSpec::new("python3", root).args([
            "-I",
            "-c",
            script,
            &path.display().to_string(),
            name,
            &copies.to_string(),
        ]))
        .unwrap();
    std::fs::read(path).unwrap()
}

#[test]
fn verified_archive_round_trip_reads_exact_payload_and_refuses_traversal() {
    let root = tempfile::tempdir().unwrap();
    for name in ["package.crate", "../outside", "nested/file", "a\nb"] {
        let result = read_fixture_archive(root.path(), name);
        if name == "package.crate" {
            assert_eq!(result.unwrap()[name], b"payload");
        } else {
            assert!(result.is_err(), "{name}");
        }
    }
    assert!(!root.path().join("outside").exists());
}

fn read_fixture_archive(
    root: &Path,
    name: &str,
) -> BuildResult<std::collections::BTreeMap<String, Vec<u8>>> {
    read_archive_bytes(root, zip_bytes(root, name, 1))
}

fn read_archive_bytes(
    root: &Path,
    bytes: Vec<u8>,
) -> BuildResult<std::collections::BTreeMap<String, Vec<u8>>> {
    let artifact = Artifact {
        id: 7,
        name: "dist-linux".into(),
        digest: format!("sha256:{:x}", Sha256::digest(&bytes)),
        size_in_bytes: bytes.len() as u64,
        expired: false,
    };
    let runner = ArtifactRunner(
        RecordingRunner::default()
            .with_stdout("git rev-parse --show-toplevel", &root.display().to_string())
            .with_bytes("gh api repos/owner/repo/actions/artifacts/7/zip", bytes),
    );
    let context = fixture_context(root, &runner);
    archive::files(&context, "owner/repo", &artifact)
}

fn fixture_context<'a>(root: &Path, runner: &'a ArtifactRunner) -> BuildContext<'a> {
    BuildContext {
        runner,
        git: GitRepo::discover(runner, root).unwrap(),
        reporter: StepReporter::new(LogFlavor::Terminal),
    }
}

#[test]
fn authenticated_zip_with_duplicate_entries_is_rejected_by_real_reader() {
    let root = tempfile::tempdir().unwrap();
    let duplicate = zip_bytes(root.path(), "package.crate", 2);
    assert_reader_rejected(root.path(), duplicate);
}

#[test]
fn authenticated_zip_with_encryption_flags_is_rejected_by_real_reader() {
    let root = tempfile::tempdir().unwrap();
    let mut encrypted = zip_bytes(root.path(), "package.crate", 1);
    patch_zip_field(&mut encrypted, b"PK\x03\x04", 6, &1_u16.to_le_bytes());
    patch_zip_field(&mut encrypted, b"PK\x01\x02", 8, &1_u16.to_le_bytes());
    assert_reader_rejected(root.path(), encrypted);
}

#[test]
fn authenticated_zip_with_oversized_declared_payload_is_rejected_without_allocation() {
    let root = tempfile::tempdir().unwrap();
    let mut oversized = zip_bytes(root.path(), "package.crate", 1);
    let declared = (1_073_741_824_u32 + 1).to_le_bytes();
    patch_zip_field(&mut oversized, b"PK\x03\x04", 22, &declared);
    patch_zip_field(&mut oversized, b"PK\x01\x02", 24, &declared);
    assert!(oversized.len() < 1024);
    assert_reader_rejected(root.path(), oversized);
}

fn patch_zip_field(bytes: &mut [u8], signature: &[u8; 4], offset: usize, value: &[u8]) {
    let header = bytes
        .windows(4)
        .position(|window| window == signature)
        .unwrap();
    let start = header + offset;
    bytes[start..start + value.len()].copy_from_slice(value);
}

fn assert_reader_rejected(root: &Path, bytes: Vec<u8>) {
    let error = read_archive_bytes(root, bytes).unwrap_err();
    assert!(
        matches!(error, BuildError::CommandFailed { ref command, .. }
        if command.starts_with("python3 ")),
        "{error}"
    );
}
