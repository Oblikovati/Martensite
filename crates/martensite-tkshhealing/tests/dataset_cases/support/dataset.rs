//! `locate_data_file` for the TKBRep-owned scenario cases: the exact public dataset member.
//!
//! The members are read from the `external/martensite-test-data` submodule (its
//! unchanged `dataset-7.9.0/` tree), or from `$MARTENSITE_TEST_DATA_DIR` when set. A
//! missing submodule, or a missing or changed member, fails the case with a message
//! naming `git submodule update --init`: it never passes without its original input.
use super::dataset_manifest::PINNED_MEMBERS;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

/// One pinned dataset member; `pinned("OCC141.draw").bytes`.
#[derive(Clone, Copy, Debug)]
pub struct PinnedMember<'a> {
    pub name: &'a str,
    pub directory: &'a str,
    pub bytes: usize,
    pub sha256: &'a str,
}

/// The manifest row of `name`; panics for a name the cases never pinned.
pub fn pinned(name: &str) -> PinnedMember<'static> {
    PINNED_MEMBERS
        .lines()
        .map(parse_row)
        .find(|member| member.name == name)
        .unwrap_or_else(|| panic!("dataset member {name:?}: expected a pinned manifest row"))
}

fn parse_row(row: &'static str) -> PinnedMember<'static> {
    let fields: Vec<&str> = row.split(' ').collect();
    let [name, directory, bytes, sha256] = fields[..] else {
        panic!("manifest row {row:?}: expected `name directory bytes sha256`");
    };
    let bytes = bytes.parse().expect("manifest byte count");
    PinnedMember {
        name,
        directory,
        bytes,
        sha256,
    }
}

/// The `dataset-7.9.0` directory: `$MARTENSITE_TEST_DATA_DIR` when set, else the
/// `external/martensite-test-data` submodule of the repository.
pub fn dataset_directory() -> PathBuf {
    if let Some(directory) = std::env::var_os("MARTENSITE_TEST_DATA_DIR") {
        return PathBuf::from(directory).join("dataset-7.9.0");
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/martensite-test-data/dataset-7.9.0")
}

/// The exact bytes of the pinned member `name`; `located_bytes("OCC141.draw")`.
pub fn located_bytes(name: &str) -> Vec<u8> {
    let member = pinned(name);
    let directory = dataset_directory();
    assert!(
        directory.is_dir(),
        "test data directory {} is missing: run `git submodule update --init \
         external/martensite-test-data` (or set MARTENSITE_TEST_DATA_DIR)",
        directory.display()
    );
    let path = directory.join(member.directory).join(member.name);
    let bytes = std::fs::read(&path).unwrap_or_else(|error| {
        panic!(
            "dataset member {}: {error}; run `git submodule update --init \
             external/martensite-test-data`",
            path.display()
        )
    });
    assert_eq!(bytes.len(), member.bytes, "size of dataset member {name}");
    assert_eq!(
        sha256_hex(&bytes),
        member.sha256,
        "SHA-256 of dataset member {name}"
    );
    bytes
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
