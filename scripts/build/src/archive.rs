//! Reproducible `.tar.gz` bundles and their SHA-256 checksum files. Headers
//! carry fixed owners, modes and timestamps so two runs of the same release
//! produce byte-identical archives.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use flate2::Compression;
use flate2::write::GzEncoder;
use sha2::{Digest, Sha256};

use crate::error::{BuildError, BuildResult, io_error};

/// One file placed into an archive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleEntry {
    /// The file on disk.
    pub source: PathBuf,
    /// The `/`-separated path inside the archive's top directory.
    pub archive_path: String,
    /// Stored with mode 0755 instead of 0644.
    pub executable: bool,
}

/// Writes `entries` under `top_dir/` into a gzip-compressed tar at `output`.
///
/// ```ignore
/// write_tar_gz(Path::new("dist/m.tar.gz"), "m-1.0.0", &entries)?;
/// ```
pub fn write_tar_gz(output: &Path, top_dir: &str, entries: &[BundleEntry]) -> BuildResult<()> {
    let file = fs::File::create(output).map_err(|cause| io_error(output, cause))?;
    let mut tar = tar::Builder::new(GzEncoder::new(file, Compression::best()));
    let mut sorted: Vec<&BundleEntry> = entries.iter().collect();
    sorted.sort_by(|left, right| left.archive_path.cmp(&right.archive_path));
    for entry in sorted {
        append_entry(&mut tar, top_dir, entry).map_err(|cause| io_error(&entry.source, cause))?;
    }
    let encoder = tar.into_inner().map_err(|cause| io_error(output, cause))?;
    encoder.finish().map_err(|cause| io_error(output, cause))?;
    Ok(())
}

fn append_entry<W: Write>(
    tar: &mut tar::Builder<W>,
    top_dir: &str,
    entry: &BundleEntry,
) -> std::io::Result<()> {
    let bytes = fs::read(&entry.source)?;
    let mut header = tar::Header::new_gnu();
    header.set_size(bytes.len() as u64);
    header.set_mode(if entry.executable { 0o755 } else { 0o644 });
    header.set_mtime(0);
    header.set_uid(0);
    header.set_gid(0);
    header.set_entry_type(tar::EntryType::Regular);
    tar.append_data(
        &mut header,
        format!("{top_dir}/{}", entry.archive_path),
        bytes.as_slice(),
    )
}

/// Lowercase hex SHA-256 of a file.
pub fn sha256_hex(path: &Path) -> BuildResult<String> {
    let bytes = fs::read(path).map_err(|cause| io_error(path, cause))?;
    let digest = Sha256::digest(&bytes);
    Ok(digest.iter().map(|byte| format!("{byte:02x}")).collect())
}

/// Writes `<file>.sha256` in `sha256sum` format and returns its path.
pub fn write_checksum(path: &Path) -> BuildResult<PathBuf> {
    let line = format!("{}  {}\n", sha256_hex(path)?, file_name(path)?);
    let checksum = checksum_path(path);
    fs::write(&checksum, line).map_err(|cause| io_error(&checksum, cause))?;
    Ok(checksum)
}

/// Re-hashes `path` and compares it with its `<file>.sha256` companion.
pub fn verify_checksum(path: &Path) -> BuildResult<()> {
    let checksum = checksum_path(path);
    let recorded = fs::read_to_string(&checksum).map_err(|cause| io_error(&checksum, cause))?;
    let expected = format!("{}  {}", sha256_hex(path)?, file_name(path)?);
    if recorded.trim_end() == expected {
        return Ok(());
    }
    Err(BuildError::ReleaseRule(format!(
        "{} records {:?}; expected {expected:?}",
        checksum.display(),
        recorded.trim_end()
    )))
}

fn checksum_path(path: &Path) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(".sha256");
    PathBuf::from(name)
}

/// The file name as UTF-8, which every release asset name must be.
pub fn file_name(path: &Path) -> BuildResult<&str> {
    path.file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            BuildError::ReleaseRule(format!("{} has no UTF-8 file name", path.display()))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::read::GzDecoder;

    fn entry(dir: &Path, name: &str, executable: bool) -> BundleEntry {
        let source = dir.join(name);
        fs::write(&source, name).unwrap();
        BundleEntry {
            source,
            archive_path: format!("bin/{name}"),
            executable,
        }
    }

    /// Entry paths and modes in archive order.
    fn listing(archive: &Path) -> Vec<(String, u32)> {
        let mut archive = tar::Archive::new(GzDecoder::new(fs::File::open(archive).unwrap()));
        let entries = archive.entries().unwrap().map(|entry| entry.unwrap());
        entries
            .map(|e| {
                (
                    e.path().unwrap().display().to_string(),
                    e.header().mode().unwrap(),
                )
            })
            .collect()
    }

    #[test]
    fn archive_is_sorted_and_reproducible() {
        let dir = tempfile::tempdir().unwrap();
        let entries = [
            entry(dir.path(), "zeta", true),
            entry(dir.path(), "alpha", false),
        ];
        let (first, second) = (dir.path().join("1.tar.gz"), dir.path().join("2.tar.gz"));
        write_tar_gz(&first, "top", &entries).unwrap();
        write_tar_gz(&second, "top", &entries).unwrap();
        assert_eq!(fs::read(&first).unwrap(), fs::read(&second).unwrap());
        let expected = [
            ("top/bin/alpha".to_owned(), 0o644),
            ("top/bin/zeta".to_owned(), 0o755),
        ];
        assert_eq!(listing(&first), expected);
    }

    #[test]
    fn checksum_round_trips_and_detects_tampering() {
        let dir = tempfile::tempdir().unwrap();
        let asset = dir.path().join("asset.tar.gz");
        fs::write(&asset, b"payload").unwrap();
        let checksum = write_checksum(&asset).unwrap();
        assert!(
            fs::read_to_string(&checksum)
                .unwrap()
                .ends_with("  asset.tar.gz\n")
        );
        verify_checksum(&asset).unwrap();
        fs::write(&asset, b"tampered").unwrap();
        assert!(verify_checksum(&asset).is_err());
    }

    #[test]
    fn sha256_matches_a_known_vector() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("abc");
        fs::write(&path, b"abc").unwrap();
        let expected = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
        assert_eq!(sha256_hex(&path).unwrap(), expected);
    }
}
