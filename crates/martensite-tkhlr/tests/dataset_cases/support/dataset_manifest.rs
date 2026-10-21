//! The exact public dataset members (release 7.9.0) the hidden-line regression cases
//! restore.
//!
//! Taken from the dataset archive of release 7.9.0 (archive SHA-256
//! a92ed91c3271c299287c1c404bb9454d463251094bfc848acc44aee60e6a026c). One row per member: file name, directory under
//! `dataset-7.9.0/` of the `external/martensite-test-data` submodule, size in bytes and
//! SHA-256.

/// `name directory bytes sha256` per pinned member, sorted by name.
pub const PINNED_MEMBERS: &str = "\
bug25908_hlr-bspline-clone2-tcl-h1.brep brep 544275 6a5fd2cef864d82c0e60eb7a6182b253d682155c4da179cf108a0e246b56c2de";
