//! The exact the retained behavior 7.9.0 public dataset members the TKShHealing-owned scenario cases restore.
//!
//! Taken from `opencascade-dataset-7.9.0.tar.xz` (archive SHA-256
//! a92ed91c3271c299287c1c404bb9454d463251094bfc848acc44aee60e6a026c, published with the
//! the retained behavior V7_9_0 release). `dataset_manifest.txt` has one row per member: file name,
//! directory under `dataset-7.9.0/` of the `external/martensite-test-data` submodule,
//! size in bytes and SHA-256, sorted by name.

/// `name directory bytes sha256` per pinned member, sorted by name.
pub const PINNED_MEMBERS: &str = include_str!("dataset_manifest.txt");
