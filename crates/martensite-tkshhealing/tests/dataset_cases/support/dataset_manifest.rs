//! Exact public dataset members used by healing regression cases.
//! The manifest stores each member’s directory, byte count and SHA-256 digest
//! so fixture loading detects missing or changed inputs.
/// `name directory bytes sha256` per pinned member, sorted by name.
pub const PINNED_MEMBERS: &str = include_str!("dataset_manifest.txt");
