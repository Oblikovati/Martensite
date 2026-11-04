//! Strictly validate the sequence index and immutable occurrence bytes before appending.
use super::{
    INDEX_SCHEMA, OCCURRENCE_SCHEMA, PIECES, hash_file, invalid_path, occurrence_name,
    read_ordinal, require_regular,
};
use serde_json::Value;
use std::{ffi::OsString, fs, io, path::Path};

pub(super) fn validate_index(
    index: &Value,
    basename: &str,
    ordinal: usize,
) -> io::Result<Vec<Value>> {
    validate_index_identity(index, basename)?;
    let prior = index_rows(index)?;
    validate_latest_pointer(index, &prior, ordinal)?;
    validate_increasing_rows(&prior, ordinal)?;
    Ok(prior)
}

fn validate_index_identity(index: &Value, basename: &str) -> io::Result<()> {
    let keys = [
        "schema",
        "image",
        "latest_ordinal",
        "latest_snapshot",
        "occurrences",
    ];
    if exact_keys(index, &keys)
        && index.get("schema").and_then(Value::as_str) == Some(INDEX_SCHEMA)
        && index.get("image").and_then(Value::as_str) == Some(basename)
    {
        return Ok(());
    }
    Err(invalid_path("latest.json", "owned witness sequence index"))
}

fn index_rows(index: &Value) -> io::Result<Vec<Value>> {
    index
        .get("occurrences")
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| invalid_path("occurrences", "ordered witness occurrence array"))
}

fn validate_latest_pointer(index: &Value, prior: &[Value], ordinal: usize) -> io::Result<()> {
    if latest_pointer_matches(index, prior, ordinal) {
        return Ok(());
    }
    Err(invalid_path(
        "latest_ordinal",
        "last completed earlier dump",
    ))
}

fn latest_pointer_matches(index: &Value, prior: &[Value], ordinal: usize) -> bool {
    let latest = prior
        .last()
        .and_then(|row| row.get("ordinal"))
        .and_then(Value::as_u64);
    let snapshot = latest.map(|n| format!("occurrences/{}", occurrence_name(n as usize)));
    let value = index
        .get("latest_ordinal")
        .and_then(Value::as_u64)
        .map(|n| n as usize);
    !prior.is_empty()
        && latest.map(|n| n as usize) == value
        && latest.is_none_or(|n| (n as usize) < ordinal)
        && index.get("latest_snapshot").and_then(Value::as_str) == snapshot.as_deref()
}
fn validate_increasing_rows(prior: &[Value], ordinal: usize) -> io::Result<()> {
    let mut previous = 0;
    for entry in prior {
        let current = read_ordinal(entry)?;
        if !valid_occurrence_row(entry, current, ordinal, previous) {
            return Err(invalid_path(
                "occurrences",
                "strictly increasing earlier dump ordinals",
            ));
        }
        previous = current;
    }
    Ok(())
}

fn valid_occurrence_row(entry: &Value, current: usize, ordinal: usize, previous: usize) -> bool {
    let name = occurrence_name(current);
    exact_keys(entry, &["ordinal", "directory", "image_sha256"])
        && entry.get("directory").and_then(Value::as_str) == Some(&name)
        && valid_sha256(entry.get("image_sha256").and_then(Value::as_str))
        && current > previous
        && current < ordinal
}

pub(super) fn validate_occurrence(directory: &Path, basename: &str, name: &str) -> io::Result<()> {
    let path = directory.join("occurrence.json");
    let manifest = read_manifest(&path)?;
    validate_manifest(&manifest, &path, basename, name)?;
    validate_manifest_pieces(directory, &manifest)
}

fn read_manifest(path: &Path) -> io::Result<Value> {
    require_regular(path)?;
    serde_json::from_slice(&fs::read(path)?)
        .map_err(|error| invalid_path(path.display().to_string(), error.to_string()))
}

fn validate_manifest(manifest: &Value, path: &Path, basename: &str, name: &str) -> io::Result<()> {
    if manifest_matches(manifest, basename, name) {
        return Ok(());
    }
    Err(invalid_path(
        path.display().to_string(),
        "matching complete occurrence manifest",
    ))
}

fn manifest_matches(manifest: &Value, basename: &str, name: &str) -> bool {
    let keys = [
        "schema",
        "image",
        "ordinal",
        "directory",
        "image_sha256",
        "piece_sha256",
    ];
    exact_keys(manifest, &keys) && manifest_identity_matches(manifest, basename, name)
}

fn manifest_identity_matches(manifest: &Value, basename: &str, name: &str) -> bool {
    let ordinal = name
        .strip_prefix("dump-")
        .and_then(|value| value.parse::<usize>().ok());
    manifest.get("schema").and_then(Value::as_str) == Some(OCCURRENCE_SCHEMA)
        && manifest.get("image").and_then(Value::as_str) == Some(basename)
        && manifest.get("directory").and_then(Value::as_str) == Some(name)
        && manifest
            .get("ordinal")
            .and_then(Value::as_u64)
            .map(|value| value as usize)
            == ordinal
}
fn validate_manifest_pieces(directory: &Path, manifest: &Value) -> io::Result<()> {
    let hashes = manifest
        .get("piece_sha256")
        .and_then(Value::as_object)
        .ok_or_else(|| invalid_path("piece_sha256", "complete piece hash map"))?;
    if !exact_object_keys(hashes, &PIECES)
        || !valid_sha256(manifest.get("image_sha256").and_then(Value::as_str))
    {
        return Err(invalid_path(
            "piece_sha256",
            "exact SHA-256 map and image digest",
        ));
    }
    validate_piece_directory(directory, hashes)
}

fn validate_piece_directory(
    directory: &Path,
    hashes: &serde_json::Map<String, Value>,
) -> io::Result<()> {
    let expected = piece_names();
    if sorted_names(directory)? != expected {
        return Err(invalid_path(
            directory.display().to_string(),
            "exact complete witness piece set",
        ));
    }
    validate_piece_hashes(directory, hashes)
}

fn piece_names() -> Vec<OsString> {
    let mut names = PIECES
        .iter()
        .copied()
        .chain(["occurrence.json"])
        .map(OsString::from)
        .collect::<Vec<_>>();
    names.sort();
    names
}

fn sorted_names(directory: &Path) -> io::Result<Vec<OsString>> {
    let mut names = fs::read_dir(directory)?
        .map(|entry| entry.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, _>>()?;
    names.sort();
    Ok(names)
}

fn validate_piece_hashes(
    directory: &Path,
    hashes: &serde_json::Map<String, Value>,
) -> io::Result<()> {
    for name in PIECES {
        let path = directory.join(name);
        require_regular(&path)?;
        let expected = hashes
            .get(name)
            .and_then(Value::as_str)
            .ok_or_else(|| invalid_path(name, "SHA-256 in occurrence manifest"))?;
        if hash_file(&path)? != expected {
            return Err(invalid_path(
                path.display().to_string(),
                "matching immutable occurrence hash",
            ));
        }
    }
    Ok(())
}

fn exact_keys(value: &Value, expected: &[&str]) -> bool {
    value
        .as_object()
        .is_some_and(|object| exact_object_keys(object, expected))
}
fn exact_object_keys(object: &serde_json::Map<String, Value>, expected: &[&str]) -> bool {
    object.len() == expected.len() && expected.iter().all(|key| object.contains_key(*key))
}
fn valid_sha256(value: Option<&str>) -> bool {
    value.is_some_and(|text| text.len() == 64 && text.bytes().all(|byte| byte.is_ascii_hexdigit()))
}
