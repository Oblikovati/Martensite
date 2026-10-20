//! Default-mask storage containers: original cursor, live header and precise failure effects.
#[path = "owned_stream_format/fixture.rs"]
mod fixture;
#[path = "owned_stream_format/wire.rs"]
mod wire;
use fixture::FormatProbe;
use martensite_tkernel::{
    standard::Handle,
    storage::{Data, state},
    tcollection::AsciiString,
};

#[test]
fn recognized_text_and_compatible_containers_only_scan_existing_storage() {
    for magic in [b"FSDFILEremainder".as_slice(), b"CMPFILEremainder"] {
        let mut probe = FormatProbe::new(magic.to_vec(), true, 0);
        assert_eq!(probe.query().unwrap().to_utf8(), "previous");
        probe.assert_stream(0, 1, 7);
        probe.assert_storage(true, true, 1, 99);
    }
}

#[test]
fn binary_container_allocates_after_offsets_and_returns_last_format() {
    let mut probe = FormatProbe::new(wire::ordinary_binary(0), false, 0);
    assert_eq!(probe.query().unwrap().to_utf8(), "last");
    probe.assert_stream(0, 4, 154);
    probe.assert_storage(true, false, 2, 0);
}

#[test]
fn binary_container_reuses_storage_and_appends_information() {
    let mut probe = FormatProbe::new(wire::ordinary_binary(0), true, 0);
    assert_eq!(probe.query().unwrap().to_utf8(), "last");
    probe.assert_stream(0, 4, 154);
    probe.assert_storage(true, true, 3, 0);
}

#[test]
fn binary_complete_read_leaves_trailing_caller_bytes_unconsumed() {
    let mut bytes = wire::ordinary_binary(0);
    bytes.extend_from_slice(b"unconsumed-tail");
    let mut probe = FormatProbe::new(bytes, false, 0);
    assert_eq!(probe.query().unwrap().to_utf8(), "last");
    probe.assert_stream(0, 4, 154);
    let mut remainder = [0; 15];
    probe.stream.read(&mut remainder);
    assert_eq!(&remainder, b"unconsumed-tail");
}

#[test]
fn binary_without_format_does_not_use_the_filename_type_fallback() {
    let mut probe = FormatProbe::new(wire::binary(&[b"plain"], 0), false, 0);
    probe.storage = Handle::new(Data::default());
    probe
        .storage
        .type_data()
        .get()
        .unwrap()
        .add_type(&AsciiString::new("UnusedFallback"), 1);
    probe.before = probe.storage.clone();
    assert!(probe.query().unwrap().is_empty());
    probe.assert_stream(0, 4, 120);
    probe.assert_storage(true, true, 1, 0);
}

#[test]
fn embedded_format_key_preserves_native_second_token_selection() {
    let mut probe = FormatProbe::new(wire::binary(&[b"prefix FILE_FORMAT: actual"], 0), false, 0);
    assert_eq!(probe.query().unwrap().to_utf8(), "FILE_FORMAT:");
    probe.assert_stream(0, 4, 141);
}

#[test]
fn incomplete_offsets_do_not_allocate_and_incomplete_info_does_not_publish_header() {
    for existing in [false, true] {
        verify_short_field(existing, false);
        verify_short_field(existing, true);
    }
}

fn verify_short_field(existing: bool, information: bool) {
    let bytes = if information {
        wire::ordinary_binary(0)[..65].to_vec()
    } else {
        b"BINFILExy".to_vec()
    };
    let mut probe = FormatProbe::new(bytes, existing, 0);
    let error = probe.query().unwrap_err();
    assert_eq!(error.exception_type(), "Storage_StreamTypeMismatchError");
    assert!(error.what().is_empty());
    probe.assert_stream(
        state::EOF | state::FAIL,
        2,
        if information { 65 } else { 9 },
    );
    verify_retained_storage(&probe, existing, information);
}

fn verify_retained_storage(probe: &FormatProbe, existing: bool, information: bool) {
    let allocated = existing || information;
    let identical = existing || !information;
    let objects = if existing { 99 } else { 0 };
    probe.assert_storage(allocated, identical, i32::from(existing), objects);
}

#[test]
fn short_ascii_and_extended_strings_keep_native_empty_error_payload() {
    for extended in [false, true] {
        let mut probe = FormatProbe::new(wire::short_string(extended), true, 0);
        let error = probe.query().unwrap_err();
        assert_eq!(error.exception_type(), "Storage_StreamReadError");
        assert!(error.what().is_empty());
        probe.assert_stream(state::EOF | state::FAIL, 3, if extended { 86 } else { 70 });
        probe.assert_storage(true, true, 1, 99);
    }
}

#[test]
fn actual_nonzero_stream_origin_is_not_rebased() {
    let mut bytes = b"xx".to_vec();
    bytes.extend_from_slice(&wire::binary(&[b"FILE_FORMAT: shifted"], 2));
    let mut probe = FormatProbe::new(bytes, false, 2);
    assert_eq!(probe.query().unwrap().to_utf8(), "shifted");
    probe.assert_stream(0, 4, 137);
}

#[test]
fn unknown_and_initially_failed_streams_preserve_storage() {
    let mut unknown = FormatProbe::new(b"unknown!remainder".to_vec(), true, 0);
    assert!(unknown.query().unwrap().is_empty());
    unknown.assert_stream(0, 1, 7);
    unknown.assert_storage(true, true, 1, 99);
    let mut failed = FormatProbe::new(wire::ordinary_binary(0), true, 0);
    failed.stream.set_state(state::FAIL);
    assert!(failed.query().unwrap().is_empty());
    failed.assert_stream(state::FAIL, 0, 0);
    failed.assert_storage(true, true, 1, 99);
}

#[test]
fn xml_reads_the_document_format_and_stops_after_its_start_tag() {
    let declaration = b"<?xml version=\"1.0\"?>".len() as i64;
    let tag = b"<document format=\"X\">".len() as i64;
    let mut probe = FormatProbe::new(
        b"<?xml version=\"1.0\"?><document format=\"X\"><label/></document>".to_vec(),
        true,
        0,
    );
    assert_eq!(probe.query().unwrap().to_utf8(), "X");
    probe.assert_stream(0, 1, declaration + tag);
    probe.assert_storage(true, true, 1, 99);
    assert_eq!(
        probe.storage.user_info().value(1).unwrap(),
        &AsciiString::new("FILE_FORMAT: previous")
    );
}

#[test]
fn xml_other_root_or_unterminated_declaration_has_no_format() {
    let other = b"<?xml version=\"1.0\"?><other format=\"X\"/>".to_vec();
    let mut other = FormatProbe::new(other, true, 0);
    assert!(other.query().unwrap().is_empty());
    let mut cut = FormatProbe::new(b"<?xml version".to_vec(), true, 0);
    assert!(cut.query().unwrap().is_empty());
    cut.assert_stream(state::EOF | state::FAIL, 0, 13);
    cut.assert_storage(true, true, 1, 99);
}

#[test]
fn binary_without_any_format_allocates_storage_and_returns_empty() {
    let mut probe = FormatProbe::new(wire::binary(&[b"plain"], 0), false, 0);
    assert!(probe.query().unwrap().is_empty());
    probe.assert_stream(0, 4, 120);
    probe.assert_storage(true, false, 1, 0);
}
