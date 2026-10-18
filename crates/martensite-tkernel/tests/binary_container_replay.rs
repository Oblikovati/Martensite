//! Literal container fixtures pin the owned reader before adapter extraction.
#[path = "binary_container_replay/fixture.rs"]
mod fixture;
use martensite_tkernel::fsd::{BinaryFile, MemoryFileSystem};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::storage::{Data, HeaderSections, IoStream, state};
use std::rc::Rc;

fn driver() -> BinaryFile {
    BinaryFile::new(Rc::new(MemoryFileSystem::default()))
}
fn input(bytes: Vec<u8>, position: i64) -> IoStream {
    let mut stream = IoStream::from_bytes(bytes);
    stream.seekg(position);
    stream
}

#[test]
fn literal_empty_sections_keep_body_and_actual_origin() {
    let mut stream = input(fixture::empty_container(), 7);
    let mut set = Handle::<Data>::default();
    driver().read_complete_info(&mut stream, &mut set).unwrap();
    assert_eq!(stream.tellg(), 111);
    assert!(stream.good());
    assert_eq!(set.get().unwrap().number_of_objects(), 0);
    assert_eq!(set.get().unwrap().number_of_types(), 0);
    assert_eq!(set.get().unwrap().number_of_roots(), 0);
    let mut body = [0; 4];
    stream.read(&mut body);
    assert_eq!(&body, b"body");
}

#[test]
fn shifted_offsets_use_caller_origin_instead_of_consumption() {
    let mut stream = input(fixture::shifted_container(), 12);
    let mut set = Handle::<Data>::default();
    driver().read_complete_info(&mut stream, &mut set).unwrap();
    assert_eq!(stream.tellg(), 116);
    assert!(stream.good());
    assert_eq!(stream.gcount(), 4);
    assert!(!set.is_null());
}

#[test]
fn short_header_keeps_the_dataset_null() {
    let mut bytes = fixture::empty_container();
    bytes.truncate(9);
    let mut stream = input(bytes, 7);
    let mut set = Handle::<Data>::default();
    let failure = driver()
        .read_complete_info(&mut stream, &mut set)
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Storage_StreamTypeMismatchError");
    assert!(set.is_null());
    assert_eq!(stream.rdstate(), state::EOF | state::FAIL);
    assert_eq!(stream.gcount(), 2);
}

#[test]
fn short_info_allocates_dataset_without_partial_header_publication() {
    let mut bytes = fixture::empty_container();
    bytes[59..63].copy_from_slice(&12_i32.to_ne_bytes());
    bytes.truncate(65);
    let mut stream = input(bytes, 7);
    let mut set = Handle::<Data>::default();
    let failure = driver()
        .read_complete_info(&mut stream, &mut set)
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Storage_StreamTypeMismatchError");
    assert_eq!(set.get().unwrap().number_of_objects(), 0);
    assert_eq!(stream.rdstate(), state::EOF | state::FAIL);
    assert_eq!(stream.gcount(), 2);
}

#[test]
fn short_info_keeps_existing_identity_and_header() {
    let mut bytes = fixture::empty_container();
    bytes[59..63].copy_from_slice(&12_i32.to_ne_bytes());
    bytes.truncate(65);
    let mut stream = input(bytes, 7);
    let mut set = Handle::new(Data::default());
    let identity = set.as_ptr();
    set.get()
        .unwrap()
        .header_data()
        .get()
        .unwrap()
        .set_number_of_objects(99);
    assert!(driver().read_complete_info(&mut stream, &mut set).is_err());
    assert_eq!(set.as_ptr(), identity);
    assert_eq!(set.get().unwrap().number_of_objects(), 99);
}

#[test]
fn complete_info_reuses_existing_dataset_identity() {
    let mut stream = input(fixture::empty_container(), 7);
    let mut set = Handle::new(Data::default());
    let identity = set.as_ptr();
    set.get()
        .unwrap()
        .header_data()
        .get()
        .unwrap()
        .set_number_of_objects(99);
    driver().read_complete_info(&mut stream, &mut set).unwrap();
    assert_eq!(set.as_ptr(), identity);
    assert_eq!(set.get().unwrap().number_of_objects(), 0);
}
