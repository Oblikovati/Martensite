//! A sealed payload inventory binds the extracted files actually used by Cargo.
use super::{files, snapshot::Snapshot};
use crate::{
    archive,
    error::{BuildError, BuildResult, io_error},
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, io::Read, path::Path};

const NAME: &str = "martensite-source-workspace.json";

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Manifest {
    schema: u32,
    release: String,
    snapshot: Snapshot,
    scope: String,
    inputs: super::metadata::InputMetadata,
    files: BTreeMap<String, Payload>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Payload {
    sha256: String,
    executable: bool,
}

pub(super) fn write_manifest(
    stage: &Path,
    snapshot: &Snapshot,
    release: &semver::Version,
) -> BuildResult<Manifest> {
    let manifest = Manifest {
        schema: 1,
        inputs: super::metadata::inspect(stage)?,
        release: release.to_string(),
        snapshot: snapshot.clone(),
        scope: "production-libraries; datasets separately provisioned; installed toolchain and native libraries required".into(),
        files: payloads(stage)?,
    };
    files::write(stage, Path::new(NAME), &encode(&manifest)?)?;
    Ok(manifest)
}

fn payloads(root: &Path) -> BuildResult<BTreeMap<String, Payload>> {
    let mut payloads = BTreeMap::new();
    for entry in files::entries(root)? {
        if entry.archive_path == NAME {
            continue;
        }
        payloads.insert(
            entry.archive_path,
            Payload {
                sha256: archive::sha256_hex(&entry.source)?,
                executable: entry.executable,
            },
        );
    }
    Ok(payloads)
}

fn encode(manifest: &Manifest) -> BuildResult<Vec<u8>> {
    serde_json::to_vec_pretty(manifest).map_err(|cause| {
        BuildError::ReleaseRule(format!(
            "SDK manifest: {cause}; expected serializable payload inventory"
        ))
    })
}

pub(super) fn manifest_hash(manifest: &Manifest) -> BuildResult<String> {
    use sha2::{Digest, Sha256};
    let digest = Sha256::digest(encode(manifest)?);
    Ok(format!("{digest:x}"))
}

pub(super) fn extract_verified(
    archive: &Path,
    destination: &Path,
    expected: &Manifest,
) -> BuildResult<()> {
    let file = fs::File::open(archive).map_err(|cause| io_error(archive, cause))?;
    let mut tar = tar::Archive::new(flate2::read::GzDecoder::new(file));
    let mut seen = std::collections::BTreeSet::new();
    let top = format!("martensite-{}-source-workspace", expected.release);
    for item in tar.entries().map_err(|cause| io_error(archive, cause))? {
        extract_file(
            item.map_err(|cause| io_error(archive, cause))?,
            destination,
            &mut seen,
            &top,
        )?;
    }
    let manifest_path = destination.join(NAME);
    let bytes = fs::read(&manifest_path).map_err(|cause| io_error(&manifest_path, cause))?;
    if bytes != encode(expected)? || payloads(destination)? != expected.files {
        return Err(BuildError::ReleaseRule(format!(
            "SDK archive {archive:?}; expected exact manifest and payload hashes/modes"
        )));
    }
    Ok(())
}

fn extract_file<R: Read>(
    mut entry: tar::Entry<'_, R>,
    root: &Path,
    seen: &mut std::collections::BTreeSet<String>,
    top: &str,
) -> BuildResult<()> {
    let relative = relative_member_name(&entry, root, top)?;
    if !entry.header().entry_type().is_file() || !seen.insert(relative.clone()) {
        return Err(BuildError::ReleaseRule(format!(
            "SDK member {relative:?}; expected unique regular file"
        )));
    }
    let bytes = entry_bytes(&mut entry, Path::new(&relative))?;
    files::write(root, Path::new(&relative), &bytes)?;
    restore_mode(&entry, &root.join(relative))
}

// Raw archive spelling is checked before native reconstruction can hide separators (#290).
fn relative_member_name<R: Read>(
    entry: &tar::Entry<'_, R>,
    root: &Path,
    top: &str,
) -> BuildResult<String> {
    let path = entry.path().map_err(|cause| io_error(root, cause))?;
    files::relative(&path)?;
    require_top(&path, top)?;
    files::native_archive_path(&path.components().skip(1).collect::<std::path::PathBuf>())
}

fn entry_bytes<R: Read>(entry: &mut tar::Entry<'_, R>, path: &Path) -> BuildResult<Vec<u8>> {
    let mut bytes = Vec::new();
    entry
        .read_to_end(&mut bytes)
        .map_err(|cause| io_error(path, cause))?;
    Ok(bytes)
}

fn require_top(path: &Path, top: &str) -> BuildResult<()> {
    if path.components().next() != Some(std::path::Component::Normal(std::ffi::OsStr::new(top))) {
        return Err(BuildError::ReleaseRule(format!(
            "SDK member {path:?}; expected top directory {top:?}"
        )));
    }
    Ok(())
}

fn restore_mode<R: Read>(entry: &tar::Entry<'_, R>, path: &Path) -> BuildResult<()> {
    let mode = entry
        .header()
        .mode()
        .map_err(|cause| io_error(path, cause))?;
    if !matches!(mode, 0o644 | 0o755) {
        return Err(BuildError::ReleaseRule(format!(
            "SDK member mode {mode:o} at {path:?}; expected 644 or 755"
        )));
    }
    files::set_mode(path, mode & 0o111 != 0)
}
