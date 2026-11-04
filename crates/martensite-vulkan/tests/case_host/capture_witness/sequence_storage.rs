//! Immutable per-dump frames with a checked pointer to the latest image-matched snapshot.
use super::encoding::WitnessPiece;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
};

const PIECES: [&str; 6] = [
    "capture.rgba",
    "capture.depth-f32le",
    "submitted-geometry.bin",
    "submitted-draws.txt",
    "camera-state.txt",
    "complete.txt",
];
const INDEX_SCHEMA: &str = "martensite-original-capture-witness-sequence-v1";
const OCCURRENCE_SCHEMA: &str = "martensite-original-capture-witness-occurrence-v1";
#[path = "sequence_storage_validation.rs"]
mod validation;

pub(super) fn publish(
    root: &Path,
    basename: &str,
    ordinal: usize,
    image_path: &Path,
    pieces: &[WitnessPiece],
) -> io::Result<()> {
    validate_request(basename, ordinal, pieces)?;
    ensure_real_parent(image_path)?;
    ensure_real_parent(root)?;
    let image_sha = hash_file(image_path)?;
    let prior = prepare_root(root, basename, ordinal)?;
    publish_occurrence(root, basename, ordinal, &image_sha, pieces, &prior)
}

fn validate_request(basename: &str, ordinal: usize, pieces: &[WitnessPiece]) -> io::Result<()> {
    if basename.is_empty()
        || Path::new(basename).file_name().and_then(|v| v.to_str()) != Some(basename)
    {
        return Err(invalid_path(basename, "single filename component"));
    }
    if ordinal == 0 || pieces.iter().map(|piece| piece.name).collect::<Vec<_>>() != PIECES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "ordinal={ordinal}, witness pieces={:?}; expected positive ordinal and ordered {PIECES:?}",
                pieces.iter().map(|piece| piece.name).collect::<Vec<_>>()
            ),
        ));
    }
    Ok(())
}

fn prepare_root(root: &Path, basename: &str, ordinal: usize) -> io::Result<Vec<Value>> {
    ensure_directory(root)?;
    let occurrences = root.join("occurrences");
    let index_path = root.join("latest.json");
    if !index_path.exists() {
        require_empty_first_root(root)?;
        ensure_directory(&occurrences)?;
        return Ok(Vec::new());
    }
    ensure_directory(&occurrences)?;
    let prior = validate_existing_root(root, &occurrences, &index_path, basename, ordinal)?;
    Ok(prior)
}

fn require_empty_first_root(root: &Path) -> io::Result<()> {
    if fs::read_dir(root)?.next().is_none() {
        return Ok(());
    }
    Err(invalid_path(
        root.display().to_string(),
        "fresh witness directory",
    ))
}

fn validate_existing_root(
    root: &Path,
    occurrences: &Path,
    index_path: &Path,
    basename: &str,
    ordinal: usize,
) -> io::Result<Vec<Value>> {
    require_regular(index_path)?;
    let index: Value = serde_json::from_slice(&fs::read(index_path)?)
        .map_err(|error| invalid_path(index_path.display().to_string(), error.to_string()))?;
    let prior = validation::validate_index(&index, basename, ordinal)?;
    validate_occurrence_entries(occurrences, &prior, basename)?;
    validate_flat_latest(root, &index, basename)?;
    validate_root_entries(root, &prior)?;
    Ok(prior)
}

fn validate_occurrence_entries(root: &Path, prior: &[Value], basename: &str) -> io::Result<()> {
    let expected = occurrence_names(prior)?;
    for (entry, name) in prior.iter().zip(expected.iter()) {
        validate_occurrence_entry(root, entry, basename, name)?;
    }
    validate_occurrence_names(root, &expected)
}

fn occurrence_names(prior: &[Value]) -> io::Result<Vec<String>> {
    prior
        .iter()
        .map(|entry| read_ordinal(entry).map(occurrence_name))
        .collect()
}

fn validate_occurrence_entry(
    root: &Path,
    entry: &Value,
    basename: &str,
    name: &str,
) -> io::Result<()> {
    let directory = root.join(name);
    require_directory(&directory)?;
    validation::validate_occurrence(&directory, basename, name)?;
    let manifest: Value = serde_json::from_slice(&fs::read(directory.join("occurrence.json"))?)
        .map_err(|error| invalid_path(directory.display().to_string(), error.to_string()))?;
    if entry.get("image_sha256") != manifest.get("image_sha256") {
        return Err(invalid_path(
            name,
            "index image hash matching immutable occurrence",
        ));
    }
    Ok(())
}

fn validate_occurrence_names(root: &Path, expected: &[String]) -> io::Result<()> {
    let actual = sorted_names(root)?;
    let wanted = expected.iter().map(OsString::from).collect::<Vec<_>>();
    if actual == wanted {
        return Ok(());
    }
    Err(invalid_path(
        root.display().to_string(),
        "only indexed completed occurrence directories",
    ))
}

fn sorted_names(root: &Path) -> io::Result<Vec<OsString>> {
    let mut names = fs::read_dir(root)?
        .map(|entry| entry.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, _>>()?;
    names.sort();
    Ok(names)
}

fn validate_flat_latest(root: &Path, index: &Value, basename: &str) -> io::Result<()> {
    let hashes = latest_piece_hashes(root, index, basename)?;
    validate_flat_pieces(root, &hashes)
}

fn latest_piece_hashes(root: &Path, index: &Value, basename: &str) -> io::Result<Value> {
    let name = latest_directory(index)?;
    let path = root.join("occurrences").join(&name);
    validation::validate_occurrence(&path, basename, &name)?;
    read_piece_hashes(&path)
}

fn latest_directory(index: &Value) -> io::Result<String> {
    index
        .get("occurrences")
        .and_then(Value::as_array)
        .and_then(|rows| rows.last())
        .and_then(|row| row.get("directory"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| invalid_path("directory", "latest occurrence directory"))
}

fn read_piece_hashes(directory: &Path) -> io::Result<Value> {
    let path = directory.join("occurrence.json");
    let manifest: Value = serde_json::from_slice(&fs::read(&path)?)
        .map_err(|error| invalid_path("occurrence.json", error.to_string()))?;
    manifest
        .get("piece_sha256")
        .cloned()
        .ok_or_else(|| invalid_path("piece_sha256", "latest occurrence hashes"))
}
fn validate_flat_pieces(root: &Path, hashes: &Value) -> io::Result<()> {
    let hashes = hashes
        .as_object()
        .ok_or_else(|| invalid_path("piece_sha256", "latest occurrence hash map"))?;
    for piece in PIECES {
        let flat = root.join(piece);
        require_regular(&flat)?;
        let expected = hashes
            .get(piece)
            .and_then(Value::as_str)
            .ok_or_else(|| invalid_path(piece, "latest occurrence hash"))?;
        if hash_file(&flat)? != expected {
            return Err(invalid_path(
                flat.display().to_string(),
                "latest completed occurrence bytes",
            ));
        }
    }
    Ok(())
}

fn validate_root_entries(root: &Path, _prior: &[Value]) -> io::Result<()> {
    let mut expected = PIECES
        .iter()
        .copied()
        .chain(["latest.json", "occurrences"])
        .map(OsString::from)
        .collect::<Vec<_>>();
    expected.sort();
    if sorted_names(root)? == expected {
        return Ok(());
    }
    Err(invalid_path(
        root.display().to_string(),
        "exact latest witness files and occurrence index",
    ))
}

fn publish_occurrence(
    root: &Path,
    basename: &str,
    ordinal: usize,
    image_sha: &str,
    pieces: &[WitnessPiece],
    prior: &[Value],
) -> io::Result<()> {
    let (stage, final_dir) = stage_paths(root, ordinal);
    fs::create_dir(&stage).map_err(|error| write_error(&stage, error))?;
    write_complete_stage(&stage, basename, ordinal, image_sha, pieces)?;
    fs::rename(&stage, &final_dir).map_err(|error| write_error(&final_dir, error))?;
    publish_latest_occurrence(root, basename, ordinal, image_sha, pieces, prior)
}

fn stage_paths(root: &Path, ordinal: usize) -> (PathBuf, PathBuf) {
    let occurrences = root.join("occurrences");
    (
        occurrences.join(format!(".pending-{ordinal:06}")),
        occurrences.join(occurrence_name(ordinal)),
    )
}

fn write_complete_stage(
    stage: &Path,
    basename: &str,
    ordinal: usize,
    image_sha: &str,
    pieces: &[WitnessPiece],
) -> io::Result<()> {
    write_stage(stage, pieces)?;
    let hashes = piece_hashes(pieces);
    write_occurrence_manifest(stage, basename, ordinal, image_sha, &hashes)
}

fn publish_latest_occurrence(
    root: &Path,
    basename: &str,
    ordinal: usize,
    image_sha: &str,
    pieces: &[WitnessPiece],
    prior: &[Value],
) -> io::Result<()> {
    let mut rows = prior.to_vec();
    rows.push(json!({"ordinal": ordinal, "directory": occurrence_name(ordinal), "image_sha256": image_sha}));
    let completion = replace_flat_latest(root, pieces)?;
    publish_latest_index(root, basename, ordinal, rows)?;
    publish_flat_completion(root, completion)
}

fn write_stage(stage: &Path, pieces: &[WitnessPiece]) -> io::Result<()> {
    for piece in pieces {
        write_new(&stage.join(piece.name), &piece.bytes)?;
    }
    Ok(())
}

fn piece_hashes(pieces: &[WitnessPiece]) -> BTreeMap<String, String> {
    pieces
        .iter()
        .map(|piece| (piece.name.to_owned(), digest(&piece.bytes)))
        .collect()
}

fn write_occurrence_manifest(
    stage: &Path,
    basename: &str,
    ordinal: usize,
    image_sha: &str,
    hashes: &BTreeMap<String, String>,
) -> io::Result<()> {
    let value = json!({"schema": OCCURRENCE_SCHEMA, "image": basename,
        "ordinal": ordinal, "directory": occurrence_name(ordinal),
        "image_sha256": image_sha, "piece_sha256": hashes});
    let bytes = serde_json::to_vec(&value).map_err(io::Error::other)?;
    write_new(&stage.join("occurrence.json"), &bytes)
}

fn replace_flat_latest(root: &Path, pieces: &[WitnessPiece]) -> io::Result<Vec<u8>> {
    let complete = root.join("complete.txt");
    if complete.exists() {
        remove_regular(&complete)?;
    }
    for piece in pieces.iter().filter(|piece| piece.name != "complete.txt") {
        replace_regular(root, piece.name, &piece.bytes)?;
    }
    let marker = pieces
        .iter()
        .find(|piece| piece.name == "complete.txt")
        .ok_or_else(|| invalid_path("complete.txt", "final witness piece"))?;
    Ok(marker.bytes.clone())
}

fn publish_flat_completion(root: &Path, bytes: Vec<u8>) -> io::Result<()> {
    replace_regular(root, "complete.txt", &bytes)
}

fn publish_latest_index(
    root: &Path,
    basename: &str,
    ordinal: usize,
    rows: Vec<Value>,
) -> io::Result<()> {
    let temp = root.join(format!(".latest-{:06}.pending", ordinal));
    let index = json!({"schema": INDEX_SCHEMA, "image": basename, "latest_ordinal": ordinal,
        "latest_snapshot": format!("occurrences/{}", occurrence_name(ordinal)), "occurrences": rows});
    let bytes = serde_json::to_vec_pretty(&index).map_err(io::Error::other)?;
    write_new(&temp, &bytes)?;
    fs::rename(&temp, root.join("latest.json"))
        .map_err(|error| write_error(&root.join("latest.json"), error))
}

fn occurrence_name(ordinal: usize) -> String {
    format!("dump-{ordinal:06}")
}
fn read_ordinal(entry: &Value) -> io::Result<usize> {
    entry
        .get("ordinal")
        .and_then(Value::as_u64)
        .map(|n| n as usize)
        .filter(|n| *n > 0)
        .ok_or_else(|| invalid_path("ordinal", "positive dump occurrence"))
}
fn ensure_real_parent(path: &Path) -> io::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| invalid_path(path.display().to_string(), "path with parent"))?;
    let absolute = absolute_path(parent)?;
    validate_real_directory_components(&absolute)
}

fn absolute_path(path: &Path) -> io::Result<PathBuf> {
    if path.is_absolute() {
        return Ok(path.to_path_buf());
    }
    Ok(std::env::current_dir()?.join(path))
}

fn validate_real_directory_components(path: &Path) -> io::Result<()> {
    let mut prefix = PathBuf::new();
    for component in path.components() {
        prefix.push(component);
        validate_directory_component(&prefix)?;
    }
    Ok(())
}

fn validate_directory_component(path: &Path) -> io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_dir() => Ok(()),
        Ok(_) => Err(invalid_path(
            path.display().to_string(),
            "real directory ancestry without symlinks",
        )),
        Err(error) => Err(write_error(path, error)),
    }
}

fn ensure_directory(path: &Path) -> io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_dir() => Ok(()),
        Ok(_) => Err(invalid_path(
            path.display().to_string(),
            "real directory, not symlink or file",
        )),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            fs::create_dir(path).map_err(|e| write_error(path, e))
        }
        Err(error) => Err(write_error(path, error)),
    }
}
fn require_directory(path: &Path) -> io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_dir() => Ok(()),
        Ok(_) => Err(invalid_path(
            path.display().to_string(),
            "real occurrence directory",
        )),
        Err(error) => Err(write_error(path, error)),
    }
}
fn require_regular(path: &Path) -> io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_file() => Ok(()),
        Ok(_) => Err(invalid_path(
            path.display().to_string(),
            "regular file, not symlink",
        )),
        Err(error) => Err(write_error(path, error)),
    }
}
fn hash_file(path: &Path) -> io::Result<String> {
    require_regular(path)?;
    Ok(digest(&fs::read(path)?))
}
fn digest(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
fn write_new(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| write_error(path, error))?;
    use std::io::Write as _;
    file.write_all(bytes)
        .map_err(|error| write_error(path, error))?;
    file.sync_all().map_err(|error| write_error(path, error))
}
fn replace_regular(root: &Path, name: &str, bytes: &[u8]) -> io::Result<()> {
    let destination = root.join(name);
    match fs::symlink_metadata(&destination) {
        Ok(_) => require_regular(&destination)?,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(write_error(&destination, error)),
    }
    let temp = root.join(format!(".{name}.pending"));
    write_new(&temp, bytes)?;
    fs::rename(&temp, &destination).map_err(|error| write_error(&destination, error))
}
fn remove_regular(path: &Path) -> io::Result<()> {
    require_regular(path)?;
    fs::remove_file(path).map_err(|error| write_error(path, error))
}
fn invalid_path(path: impl std::fmt::Display, expected: impl std::fmt::Display) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        format!("path={path}; expected {expected}"),
    )
}
fn write_error(path: &Path, error: io::Error) -> io::Error {
    io::Error::new(
        error.kind(),
        format!("path={path:?}: {error}; expected fresh regular witness output"),
    )
}

#[cfg(test)]
#[path = "sequence_storage_controls.rs"]
mod controls;
