use super::support::driver;
use super::{baseline, fixture, support};
use martensite_tkernel::standard::Handle;
use martensite_tkernel::storage::{Data, HeaderData};

#[test]
fn read_only_buffer_keeps_unread_body_and_dataset_identity() {
    let mut source = support::source(&baseline::empty_container());
    let mut set = Handle::new(Data::default());
    let identity = set.as_ptr();
    support::with_input(&mut source, |input| {
        driver()
            .read_complete_info_checked(input, &mut set)
            .unwrap();
        assert_eq!(input.position().unwrap(), Some(111));
        assert_eq!(input.gcount(), 4);
        assert!(input.good());
    });
    assert_eq!(set.as_ptr(), identity);
    assert_eq!(source.cursor, 111);
    assert_eq!(&source.bytes[111..], b"body");
}

#[test]
fn actual_shifted_origin_controls_section_dispatch() {
    let mut source = support::source(&baseline::shifted_container());
    source.cursor = 12;
    let mut set = Handle::<Data>::default();
    support::with_input(&mut source, |input| {
        driver()
            .read_complete_info_checked(input, &mut set)
            .unwrap()
    });
    assert_eq!(source.cursor, 116);
    assert_eq!(&source.bytes[116..], b"body");
    assert!(!set.is_null());
}

#[test]
fn complete_metadata_preserves_text_units_comments_and_body() {
    let mut source = support::source(&fixture::metadata());
    let mut set = Handle::<Data>::default();
    support::with_input(&mut source, |input| {
        driver()
            .read_complete_info_checked(input, &mut set)
            .unwrap()
    });
    let header = set.get().unwrap().header_data();
    let header = header.get().unwrap();
    assert_metadata_text(header);
    assert_metadata_lists(header);
    assert_eq!(source.cursor, 157);
    assert_eq!(&source.bytes[157..], b"body");
    source.bytes.fill(0x5a);
    assert_metadata_text(header);
    assert_metadata_lists(header);
}

fn assert_metadata_text(header: &HeaderData) {
    assert_eq!(header.storage_version().as_bytes(), b"v");
    assert_eq!(header.creation_date().as_bytes(), b"2026");
    assert_eq!(header.schema_name().as_bytes(), b"schema");
    assert_eq!(header.schema_version().as_bytes(), b"v2");
    assert_eq!(header.application_version().as_bytes(), b"v3");
    assert_eq!(header.application_name().as_utf16(), &[65, 937]);
    assert_eq!(header.data_type().as_utf16(), &[0xd83d, 0xde00]);
}

fn assert_metadata_lists(header: &HeaderData) {
    assert_eq!(
        header
            .user_info()
            .iter()
            .map(|line| line.as_bytes().to_vec())
            .collect::<Vec<_>>(),
        [b"x".to_vec(), b"second".to_vec()]
    );
    assert_eq!(
        header
            .comments()
            .iter()
            .map(|line| line.as_utf16().to_vec())
            .collect::<Vec<_>>(),
        [vec![67]]
    );
}
