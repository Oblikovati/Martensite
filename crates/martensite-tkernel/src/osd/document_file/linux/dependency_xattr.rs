//! Dependency qualification for the RUSTSEC-2026-0486 update (#445).
use std::{collections::BTreeSet, mem::MaybeUninit};
use tempfile::NamedTempFile;
use xattr::FileExt;
const ATTRIBUTE: &str = "user.martensite_445";

#[test]
fn zero_capacity_value_buffer_never_claims_initialized_attribute_bytes() {
    let file = NamedTempFile::new().unwrap();
    xattr::set(file.path(), ATTRIBUTE, b"owned attribute").unwrap();
    let mut buffer: [MaybeUninit<u8>; 0] = [];
    let (initialized, spare) =
        rustix::fs::getxattr(file.path(), ATTRIBUTE, &mut buffer[..]).unwrap();
    assert!(initialized.is_empty() && spare.is_empty());
    assert_eq!(
        xattr::get(file.path(), ATTRIBUTE).unwrap(),
        Some(b"owned attribute".to_vec())
    );
}
#[test]
fn zero_capacity_name_buffer_never_claims_initialized_attribute_names() {
    let file = NamedTempFile::new().unwrap();
    xattr::set(file.path(), ATTRIBUTE, b"owned attribute").unwrap();
    let mut buffer: [MaybeUninit<u8>; 0] = [];
    let (initialized, spare) = rustix::fs::listxattr(file.path(), &mut buffer[..]).unwrap();
    assert!(initialized.is_empty() && spare.is_empty());
    assert!(
        xattr::list(file.path())
            .unwrap()
            .any(|name| name == ATTRIBUTE)
    );
}
fn capable_file() -> NamedTempFile {
    let root = std::env::var_os("MARTENSITE_LARGE_XATTR_ROOT")
        .expect("MARTENSITE_LARGE_XATTR_ROOT missing; expected explicit filesystem root accepting attributes over 4096 bytes");
    NamedTempFile::new_in(&root)
        .unwrap_or_else(|error| panic!("root={root:?}; expected writable owned tempfile: {error}"))
}
#[test]
#[ignore = "requires explicit MARTENSITE_LARGE_XATTR_ROOT accepting >4KiB attributes"]
fn oversized_values_roundtrip_through_all_three_query_paths() {
    let file = capable_file();
    let expected: Vec<_> = (0..8192).map(|i| (i % 251) as u8).collect();
    xattr::set(file.path(), ATTRIBUTE, &expected).unwrap();
    for actual in [
        xattr::get(file.path(), ATTRIBUTE),
        xattr::get_deref(file.path(), ATTRIBUTE),
        file.as_file().get_xattr(ATTRIBUTE),
    ] {
        let actual = actual.unwrap().unwrap();
        assert_eq!(actual.len(), expected.len());
        assert_eq!(actual, expected);
    }
    println!(
        "qualified xattr exact_bytes={} across 3 query paths",
        expected.len()
    );
}
fn large_name_set(file: &NamedTempFile) -> BTreeSet<std::ffi::OsString> {
    (0..200)
        .map(|i| {
            let name = format!("user.martensite_445_name_{i:04}");
            xattr::set(file.path(), &name, b"").unwrap();
            name.into()
        })
        .collect()
}
#[test]
#[ignore = "requires explicit MARTENSITE_LARGE_XATTR_ROOT accepting >4KiB attribute names"]
fn oversized_name_lists_roundtrip_through_all_three_query_paths() {
    let file = capable_file();
    let mut expected: BTreeSet<_> = xattr::list(file.path()).unwrap().collect();
    expected.extend(large_name_set(&file));
    let bytes: usize = expected.iter().map(|name| name.len() + 1).sum();
    assert!(bytes > 4096);
    for actual in [
        xattr::list(file.path()),
        xattr::list_deref(file.path()),
        file.as_file().list_xattr(),
    ] {
        assert_eq!(actual.unwrap().collect::<BTreeSet<_>>(), expected);
    }
    println!(
        "qualified xattr names={} encoded_bytes={} across 3 query paths",
        expected.len(),
        bytes
    );
}
