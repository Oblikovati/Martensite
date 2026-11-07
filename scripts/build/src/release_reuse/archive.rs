//! Reads only flat regular payload names from authenticated artifact ZIP bytes.
use super::{Artifact, api, refused};
use crate::{
    context::BuildContext,
    error::{BuildResult, io_error},
    process::CommandSpec,
};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs};

const MAX_ARCHIVE: u64 = 1_073_741_824;
const MAX_PAYLOAD: usize = 1_073_741_824;

pub(super) fn files(
    context: &BuildContext<'_>,
    repo: &str,
    artifact: &Artifact,
) -> BuildResult<BTreeMap<String, Vec<u8>>> {
    if artifact.expired || artifact.size_in_bytes > MAX_ARCHIVE || artifact.id == 0 {
        return Err(refused(format!(
            "expired or oversized artifact {}",
            artifact.id
        )));
    }
    let bytes = api::zip(context, repo, artifact)?;
    verify_bytes(artifact, &bytes)?;
    let directory = tempfile::tempdir()
        .map_err(|error| io_error(std::path::Path::new("artifact tempdir"), error))?;
    let path = directory.path().join("artifact.zip");
    fs::write(&path, bytes).map_err(|error| io_error(&path, error))?;
    let listing = context.runner.capture(&zip_command(context, &path))?;
    let members: Vec<String> = serde_json::from_str(&listing).map_err(refused)?;
    extract(context, &path, &names(&members.join("\n"))?)
}

pub(super) fn verify_bytes(artifact: &Artifact, bytes: &[u8]) -> BuildResult<()> {
    let digest = format!("sha256:{:x}", Sha256::digest(bytes));
    if artifact.digest != digest || artifact.size_in_bytes != bytes.len() as u64 {
        return Err(refused(format!(
            "artifact {} digest or size mismatch",
            artifact.id
        )));
    }
    Ok(())
}

pub(super) fn names(listing: &str) -> BuildResult<Vec<String>> {
    let mut names = std::collections::BTreeSet::new();
    for name in listing.lines() {
        if name.len() > 255
            || name.is_empty()
            || name.starts_with('.')
            || !name
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"-_.".contains(&c))
            || !names.insert(name.to_owned())
        {
            return Err(refused(format!("artifact entry {name:?}")));
        }
    }
    if names.is_empty() || names.len() > 4096 {
        return Err(refused("artifact entry count"));
    }
    Ok(names.into_iter().collect())
}

fn extract(
    context: &BuildContext<'_>,
    path: &std::path::Path,
    names: &[String],
) -> BuildResult<BTreeMap<String, Vec<u8>>> {
    let mut result = BTreeMap::new();
    let mut size = 0;
    for name in names {
        let command = zip_command(context, path).arg(name);
        let output = context
            .runner
            .capture_bounded(&command, MAX_PAYLOAD - size)?;
        size += output.stdout.len();
        if output.code != 0 || size > MAX_PAYLOAD {
            return Err(refused("artifact extraction status or size"));
        }
        result.insert(name.clone(), output.stdout);
    }
    Ok(result)
}

fn zip_command(context: &BuildContext<'_>, path: &std::path::Path) -> CommandSpec {
    CommandSpec::new("python3", context.root()).args([
        "-I",
        "-c",
        include_str!("artifact_zip.py"),
        &path.display().to_string(),
    ])
}
