//! Release-versioned production source workspaces, independently of crate versions (#279).
mod commands;
mod consumer;
mod files;
mod integrity;
mod metadata;
mod paths;
mod snapshot;
mod vendor;

use crate::{
    archive,
    context::BuildContext,
    error::{BuildResult, io_error},
    release::ReleaseRequest,
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
/// Identifies an SDK archive only after its extracted consumer gate succeeds.
pub(crate) struct SdkRecord {
    pub archive: String,
    pub sha256: String,
    pub revision: String,
    pub manifest_sha256: String,
    pub verification: String,
}

/// Creates and verifies the actual SDK archive; e.g. `let record = create(context, request)?;`.
pub(crate) fn create(
    context: &BuildContext<'_>,
    request: &ReleaseRequest,
) -> BuildResult<SdkRecord> {
    let temporary =
        tempfile::tempdir().map_err(|cause| io_error(Path::new("SDK staging"), cause))?;
    let stage = temporary.path().join("stage");
    let snapshot = snapshot::stage(context, &stage)?;
    vendor::prepare(context.runner, &stage)?;
    let manifest = integrity::write_manifest(&stage, &snapshot, &request.version)?;
    let archive = write_archive(&stage, request)?;
    let extracted = temporary.path().join("extracted");
    integrity::extract_verified(&archive, &extracted, &manifest)?;
    consumer::verify(context.runner, &extracted, temporary.path())?;
    archive::write_checksum(&archive)?;
    record(&archive, snapshot.revision, &manifest)
}

fn write_archive(stage: &Path, request: &ReleaseRequest) -> BuildResult<PathBuf> {
    let top = format!("martensite-{}-source-workspace", request.version);
    let archive = request.dist_dir.join(format!("{top}.tar.gz"));
    archive::write_tar_gz(&archive, &top, &files::entries(stage)?)?;
    Ok(archive)
}

fn record(
    archive: &Path,
    revision: String,
    manifest: &integrity::Manifest,
) -> BuildResult<SdkRecord> {
    Ok(SdkRecord {
        archive: archive::file_name(archive)?.into(),
        sha256: archive::sha256_hex(archive)?,
        revision,
        manifest_sha256: integrity::manifest_hash(manifest)?,
        verification: "offline-production-libraries-all-features-and-external-consumer".into(),
    })
}

#[cfg(test)]
mod security_tests;
#[cfg(test)]
mod tests;

#[cfg(test)]
mod files_tests;
