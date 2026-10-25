//! Exact archive identities and normalized requirements are verified before reuse.
use super::invalid;
use crate::{
    archive,
    error::{BuildResult, io_error},
    package_manifest::parse_table,
};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub(crate) struct ArchiveOrigin {
    pub release_id: u64,
    pub release_tag: String,
    pub asset_id: u64,
    pub archive: String,
    pub sha256: String,
    pub manifest_sha256: String,
}

#[derive(Debug)]
pub(crate) struct PublishedArchive {
    pub name: String,
    pub version: Version,
    pub path: PathBuf,
    pub origins: Vec<ArchiveOrigin>,
}

pub(super) fn identity(filename: &str) -> BuildResult<(String, Version)> {
    let stem = filename
        .strip_suffix(".crate")
        .ok_or_else(|| invalid(filename, "name-version.crate"))?;
    let candidates: Vec<_> = stem
        .match_indices('-')
        .filter_map(|(offset, _)| identity_candidate(stem, offset))
        .collect();
    if candidates.len() != 1 {
        return Err(invalid(
            filename,
            "one canonical Martensite package name/version",
        ));
    }
    candidates
        .into_iter()
        .next()
        .ok_or_else(|| invalid(filename, "package identity"))
}

fn identity_candidate(stem: &str, offset: usize) -> Option<(String, Version)> {
    let (name, suffix) = stem.split_at(offset);
    let version = Version::parse(&suffix[1..]).ok()?;
    let valid = name.starts_with("martensite-")
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_'));
    valid.then(|| (name.to_owned(), version))
}

/// Reads the exact normalized package manifest; e.g. `manifest(path, name, &version)?`.
pub(crate) fn manifest(path: &Path, name: &str, version: &Version) -> BuildResult<String> {
    let file = fs::File::open(path).map_err(|cause| io_error(path, cause))?;
    let mut tar = tar::Archive::new(flate2::read::GzDecoder::new(file));
    let expected = format!("{name}-{version}/Cargo.toml");
    let mut found = None;
    for item in tar.entries().map_err(|cause| io_error(path, cause))? {
        let mut item = item.map_err(|cause| io_error(path, cause))?;
        if item.path().map_err(|cause| io_error(path, cause))?.as_ref() != Path::new(&expected) {
            continue;
        }
        if found.is_some() || !item.header().entry_type().is_file() {
            return Err(invalid(&expected, "one regular normalized manifest"));
        }
        let mut text = String::new();
        item.read_to_string(&mut text)
            .map_err(|cause| io_error(path, cause))?;
        found = Some(text);
    }
    let text = found.ok_or_else(|| invalid(&expected, "a normalized archive manifest"))?;
    check_identity(&text, name, version)?;
    Ok(text)
}

fn check_identity(text: &str, name: &str, version: &Version) -> BuildResult<()> {
    let table = parse_table(text)?;
    let expected_version = version.to_string();
    let package = table.get("package");
    if package.and_then(|v| v.get("name")).and_then(|v| v.as_str()) != Some(name)
        || package
            .and_then(|v| v.get("version"))
            .and_then(|v| v.as_str())
            != Some(expected_version.as_str())
    {
        return Err(invalid(
            &format!("{name}-{version}"),
            "matching normalized archive package identity",
        ));
    }
    Ok(())
}

pub(super) fn manifest_hash(text: &str) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(text.as_bytes()))
}

pub(super) fn verify_companion(path: &Path) -> BuildResult<()> {
    archive::verify_checksum(path)
}
