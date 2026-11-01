//! Owned raw headers pin metadata ordering without relying on a logical tar iterator.
use super::bounded;
use std::{io::Write, path::Path};

struct InnerArchiveFake;

impl InnerArchiveFake {
    fn write(path: &Path, headers: &[(tar::EntryType, Vec<u8>, &str, u64, bool)]) {
        let file = std::fs::File::create(path).unwrap();
        let mut gzip = flate2::write::GzEncoder::new(file, flate2::Compression::default());
        for (kind, bytes, name, size, magic) in headers {
            Self::append(&mut gzip, *kind, bytes, name, *size, *magic);
        }
        gzip.write_all(&[0; 1024]).unwrap();
        gzip.finish().unwrap();
    }

    fn append<W: Write>(
        gzip: &mut W,
        kind: tar::EntryType,
        bytes: &[u8],
        name: &str,
        size: u64,
        magic: bool,
    ) {
        let header = fake_header(kind, name, size, magic);
        gzip.write_all(header.as_bytes()).unwrap();
        gzip.write_all(bytes).unwrap();
        let padding = (512 - bytes.len() % 512) % 512;
        gzip.write_all(&vec![0; padding]).unwrap();
    }
}

fn fake_header(kind: tar::EntryType, name: &str, size: u64, magic: bool) -> tar::Header {
    let mut header = tar::Header::new_gnu();
    header.set_path(name).unwrap();
    header.set_entry_type(kind);
    header.set_size(size);
    if !magic {
        header.as_mut_bytes()[257..265].fill(0);
    }
    header.set_cksum();
    header
}

type FakeHeader = (tar::EntryType, Vec<u8>, &'static str, u64, bool);

fn long(name: &[u8]) -> FakeHeader {
    (
        tar::EntryType::GNULongName,
        name.to_vec(),
        "././@LongLink",
        name.len() as u64,
        true,
    )
}

fn regular() -> FakeHeader {
    (
        tar::EntryType::Regular,
        vec![],
        "martensite-example-0.1.0/member",
        0,
        true,
    )
}

fn evaluate(headers: &[FakeHeader], expected: &str) -> bool {
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("owned.crate");
    InnerArchiveFake::write(&path, headers);
    bounded::inner_archive(&path, expected).is_ok()
}

#[test]
fn genuine_header_names_of_102_to_122_bytes_are_supported() {
    let names = [
        "martensite-tkernel-0.4.0/src/osd/timing/signal_native/cpu_budget/darwin/inventory/descriptor_tests.rs",
        "martensite-tkgeombase-0.3.5/tests/support/construction_corpus/elementary_construction/transformations.rs",
        "martensite-tkshhealing-0.3.6/src/shape_upgrade/shape_convert_to_bezier/postprocess/edges/shape_convert_to_bezier.expected",
    ];
    for (name, expected_size) in names.into_iter().zip([102, 105, 122]) {
        genuine_structure(name, expected_size);
    }
}

fn genuine_structure(name: &'static str, expected_size: usize) {
    let bytes = format!("{name}\0").into_bytes();
    assert_eq!(bytes.len(), expected_size);
    let mut following = regular();
    following.2 = &name[..100];
    let root = name.split('/').next().unwrap();
    assert!(evaluate(
        &[long(&bytes), following.clone()],
        &format!("{root}/Cargo.toml")
    ));
    assert_eq!(logical_name(&[long(&bytes), following]), name);
}

fn logical_name(headers: &[FakeHeader]) -> String {
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("logical.crate");
    InnerArchiveFake::write(&path, headers);
    let input = flate2::read::GzDecoder::new(std::fs::File::open(path).unwrap());
    let mut tar = tar::Archive::new(input);
    let entry = tar.entries().unwrap().next().unwrap().unwrap();
    entry.path().unwrap().to_str().unwrap().to_owned()
}

#[test]
fn metadata_requires_canonical_package_root_and_exact_terminal_nul() {
    for name in [
        b"".as_slice(),
        b"member",
        b"\0",
        b"../escape\0",
        b"/absolute\0",
        b"other/member\0",
        b"martensite-example-0.1.0/../escape\0",
        b"martensite-example-0.1.0/a\\b\0",
        b"martensite-example-0.1.0/a\0b\0",
        b"martensite-example-0.1.0/\xff\0",
        b"martensite-example-0.1.0/Cargo.toml\0",
    ] {
        assert!(!evaluate(
            &[long(name), regular()],
            "martensite-example-0.1.0/Cargo.toml"
        ));
    }
}

#[test]
fn metadata_must_be_recognized_and_followed_once_by_a_regular_member() {
    let header = long(b"martensite-example-0.1.0/long-member\0");
    assert!(!evaluate(
        std::slice::from_ref(&header),
        "martensite-example-0.1.0/Cargo.toml"
    ));
    assert!(!evaluate(
        &[header.clone(), header.clone(), regular()],
        "martensite-example-0.1.0/Cargo.toml"
    ));
}

#[test]
fn unrecognized_longname_magic_is_not_a_valid_alias() {
    let mut wrong_magic = long(b"martensite-example-0.1.0/long-member\0");
    wrong_magic.4 = false;
    assert!(!evaluate(
        &[wrong_magic, regular()],
        "martensite-example-0.1.0/Cargo.toml"
    ));
}

#[test]
fn metadata_cannot_describe_links_sparse_or_unsupported_extensions() {
    let header = long(b"martensite-example-0.1.0/long-member\0");
    for kind in [
        tar::EntryType::Symlink,
        tar::EntryType::Link,
        tar::EntryType::GNUSparse,
        tar::EntryType::XHeader,
        tar::EntryType::GNULongLink,
    ] {
        let mut next = regular();
        next.0 = kind;
        assert!(!evaluate(
            &[header.clone(), next],
            "martensite-example-0.1.0/Cargo.toml"
        ));
    }
}

#[test]
fn metadata_bombs_and_manifest_size_aliases_refuse_before_payload_reads() {
    let mut huge = long(b"martensite-example-0.1.0/member\0");
    huge.3 = 4097;
    assert!(!evaluate(
        &[huge, regular()],
        "martensite-example-0.1.0/Cargo.toml"
    ));
    let mut manifest = regular();
    manifest.2 = "martensite-example-0.1.0/Cargo.toml";
    manifest.3 = 1024 * 1024 + 1;
    let name = long(b"martensite-example-0.1.0/non-manifest\0");
    assert!(!evaluate(
        &[name, manifest],
        "martensite-example-0.1.0/Cargo.toml"
    ));
}

#[test]
fn exact_longname_resource_limit_is_accepted() {
    let mut bytes = b"martensite-example-0.1.0/".to_vec();
    bytes.resize(4095, b'a');
    bytes.push(0);
    assert!(evaluate(
        &[long(&bytes), regular()],
        "martensite-example-0.1.0/Cargo.toml"
    ));
}

#[test]
fn inner_header_resource_limit_refuses_without_iteration() {
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("count.crate");
    InnerArchiveFake::write(&path, &[regular()]);
    let input = flate2::read::GzDecoder::new(std::fs::File::open(&path).unwrap());
    let mut tar = tar::Archive::new(input);
    let mut entry = tar.entries().unwrap().raw(true).next().unwrap().unwrap();
    let mut sequence = super::long_name::InnerSequence::new("martensite-example-0.1.0/Cargo.toml");
    assert!(sequence.inspect(&mut entry, &path, 65536).is_err());
}
