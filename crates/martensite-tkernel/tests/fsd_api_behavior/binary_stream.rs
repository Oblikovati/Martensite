//! The binary stream helpers use the owned input and output paths.
use crate::fsd::{BinaryFile, FileHeader, MemoryFileSystem};
use crate::ncollection::Sequence;
use crate::standard::Handle;
use crate::storage::{HeaderData, IoStream, StorageError};
use crate::tcollection::{AsciiString, ExtendedString};
use std::rc::Rc;

fn int(value: i32) -> Vec<u8> {
    value.to_ne_bytes().to_vec()
}

#[test]
fn stream_writers_report_sizes_and_can_only_count() {
    let mut stream = IoStream::from_bytes(Vec::new());
    let mut info = Sequence::default();
    info.append(AsciiString::new("ab"));
    let size = BinaryFile::write_info_stream(
        &mut stream,
        1,
        &"1.3".into(),
        &"".into(),
        &"".into(),
        &"".into(),
        &ExtendedString::new("A"),
        &"".into(),
        &ExtendedString::default(),
        &info,
        true,
    )
    .unwrap();
    assert_eq!(size, 4 + 7 + 4 + 4 + 4 + 6 + 4 + 4 + 4 + 6);
    assert_eq!(stream.tellp(), 0, "counting writes nothing");
    let mut comments = Sequence::default();
    comments.append(ExtendedString::new("c"));
    assert_eq!(
        BinaryFile::write_comment_stream(&mut stream, &comments, false).unwrap(),
        10
    );
    assert_eq!(
        BinaryFile::write_header_stream(&mut stream, &FileHeader::unset(), false).unwrap(),
        52
    );
    let mut driver = BinaryFile::new(Rc::new(MemoryFileSystem::default()));
    assert_eq!(
        driver.end_write_comment_section_stream(&mut stream),
        StorageError::Ok
    );
    assert_eq!(driver.file_header().end_comment, 62);
    driver.begin_write_comment_section_stream(&mut stream);
    driver.end_write_info_section_stream(&mut stream);
    assert_eq!(driver.file_header().end_info, 62);
}

#[test]
fn stream_readers_decode_the_info_section() {
    let mut stream = IoStream::from_bytes(Vec::new());
    BinaryFile::write_info_stream(
        &mut stream,
        2,
        &"1.3".into(),
        &"d".into(),
        &"S".into(),
        &"v".into(),
        &ExtendedString::new("App"),
        &"av".into(),
        &ExtendedString::new("T"),
        &Sequence::default(),
        false,
    )
    .unwrap();
    let bytes = stream.into_device().map(|_| ()).is_some();
    assert!(bytes);
    let mut encoded = Vec::new();
    encoded.extend(int(2));
    for text in ["1.3", "d", "S", "v"] {
        encoded.extend(int(text.len() as i32));
        encoded.extend_from_slice(text.as_bytes());
    }
    encoded.extend(int(1));
    encoded.extend_from_slice(&0x41_u16.to_ne_bytes());
    encoded.extend(int(0));
    encoded.extend(int(0));
    encoded.extend(int(1));
    encoded.extend(int(1));
    encoded.push(b'u');
    let header = Handle::new(HeaderData::default());
    BinaryFile::read_header_data(&mut IoStream::from_bytes(encoded), &header).unwrap();
    let header = header.get().unwrap();
    assert_eq!(
        (header.number_of_objects(), header.schema_name().as_bytes()),
        (2, b"S".as_slice())
    );
    assert_eq!(header.application_name().as_utf16(), &[0x41]);
    assert_eq!(header.user_info().length(), 1);
}

#[test]
fn stream_string_readers_raise_read_errors_on_short_input() {
    let mut bytes = int(4);
    bytes.extend_from_slice(b"ab");
    let mut text = AsciiString::default();
    let failure =
        BinaryFile::read_string_stream(&mut IoStream::from_bytes(bytes), &mut text).unwrap_err();
    assert_eq!(failure.exception_type(), "Storage_StreamReadError");
    let mut value = 0;
    let failure = BinaryFile::get_reference_stream(&mut IoStream::from_bytes(vec![1]), &mut value)
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Storage_StreamTypeMismatchError");
    let mut stream = IoStream::from_bytes([int(1), int(7), int(9)].concat());
    let (mut reference, mut kind) = (0, 0);
    assert_eq!(BinaryFile::ref_section_size_stream(&mut stream).unwrap(), 1);
    BinaryFile::read_reference_type_stream(&mut stream, &mut reference, &mut kind).unwrap();
    assert_eq!((reference, kind), (7, 9));
}
