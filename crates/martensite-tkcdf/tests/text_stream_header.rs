//! Compatible/text complete-info is a no-op; format comes from the caller's live header.
use super::stream_driver_probe::support as input_support;
#[path = "text_stream_header/assertions.rs"]
mod assertions;
#[path = "text_stream_header/support.rs"]
mod support;
use input_support::DriverProbeDevice;
use martensite_tkcdf::pcdm::PcdmReadWriterBase;
use martensite_tkernel::{storage::state, tcollection::AsciiString};
use support::{HEADERS, storage_header};

#[test]
fn compatible_and_text_queries_scan_live_header_with_last_match_winning() {
    for magic in [b"CMPFILE".as_slice(), b"FSDFILE"] {
        for (header, expected) in HEADERS {
            let mut storage = storage_header(header);
            let identity = storage.clone();
            let mut device = DriverProbeDevice::new(magic);
            let actual = PcdmReadWriterBase::file_format_stream(&mut device, &mut storage);
            assert_eq!(
                actual.as_utf16(),
                expected.encode_utf16().collect::<Vec<u16>>()
            );
            assert!(storage == identity);
            assert_eq!((device.consumed, device.calls), (7, 7));
        }
    }
}

#[test]
fn masks_preserve_header_and_never_attempt_to_read_text_container_body() {
    for magic in [b"CMPFILEbody".as_slice(), b"FSDFILEbody"] {
        for mask in [
            0,
            state::BAD,
            state::EOF,
            state::FAIL,
            state::BAD | state::FAIL,
        ] {
            let mut storage = storage_header(HEADERS[2].0);
            let identity = storage.clone();
            let mut device = DriverProbeDevice::new(magic);
            device.failure_at = Some(7);
            let mut input = device.input(mask);
            let actual = PcdmReadWriterBase::file_format_input(&mut input, &mut storage).unwrap();
            assert_eq!(
                actual.as_utf16(),
                b"Second".iter().map(|b| u16::from(*b)).collect::<Vec<_>>()
            );
            assert_eq!(input.state(), 0);
            assert!(input.last_error().is_none());
            assert!(storage == identity);
            drop(input);
            assert_eq!((device.consumed, device.calls), (7, 7));
        }
    }
}

#[test]
fn explicit_masks_scan_the_same_literal_header_cases_without_replacing_sections() {
    for magic in [b"CMPFILEbody".as_slice(), b"FSDFILEbody"] {
        for (bytes, expected) in HEADERS {
            assertions::verify_header_matrix(magic, bytes, expected);
        }
    }
}

#[test]
fn only_recognized_text_headers_are_scanned_and_live_changes_are_observed() {
    let mut storage = storage_header(HEADERS[1].0);
    let mut unknown = DriverProbeDevice::new(b"unknown");
    assert!(PcdmReadWriterBase::file_format_stream(&mut unknown, &mut storage).is_empty());
    for expected in ["First", "New"] {
        let mut text = DriverProbeDevice::new(b"FSDFILE");
        assert_eq!(
            PcdmReadWriterBase::file_format_stream(&mut text, &mut storage).to_utf8(),
            expected
        );
        storage
            .get()
            .unwrap()
            .add_to_user_info(&AsciiString::new("FILE_FORMAT: New"));
    }
}

#[test]
fn failed_magic_does_not_scan_or_replace_the_caller_header() {
    for failure_at in 0..7 {
        for mask in [0, state::BAD, state::FAIL] {
            let mut storage = storage_header(HEADERS[1].0);
            let identity = storage.clone();
            let mut device = DriverProbeDevice::new(b"FSDFILE");
            device.failure_at = Some(failure_at);
            let mut input = device.input(mask);
            let result = PcdmReadWriterBase::file_format_input(&mut input, &mut storage);
            assert_eq!(result.is_err(), mask != 0);
            if let Ok(format) = result {
                assert!(format.is_empty());
            }
            assert!(storage == identity);
            assert_eq!(storage.get().unwrap().user_info().length(), 1);
        }
    }
}
