use super::{format_from_driver, select_driver};
use crate::pcdm::file_header_test_support::{FROZEN_FILES, FakeHeaderFiles};
use martensite_tkernel::{
    fsd::FileOpenMode,
    storage::OpenMode,
    tcollection::{AsciiString, ExtendedString},
};

#[path = "../../tests/file_header/cases.rs"]
mod cases;

#[test]
fn frozen_formats_use_first_marker_then_first_type_and_native_utf8_conversion() {
    for (kind, cases) in FROZEN_FILES {
        for (scenario, bytes) in cases.into_iter().enumerate() {
            let files = FakeHeaderFiles::new(bytes, None);
            let mut driver = select_driver(&AsciiString::new("owned.fsd"), files.clone())
                .unwrap()
                .unwrap();
            assert_eq!(
                format_from_driver(&mut *driver, &ExtendedString::new("owned.fsd")).unwrap(),
                ExtendedString::from_utf16(cases::EXPECTED_UNITS[scenario]),
                "{kind}-{scenario}"
            );
            assert_eq!(driver.open_mode(), OpenMode::None);
            assert_eq!(
                files
                    .memory
                    .contents(&AsciiString::new("owned.fsd"))
                    .unwrap(),
                bytes
            );
        }
    }
}

#[test]
fn detection_order_reopens_once_and_closes_every_successful_probe_and_query() {
    for (index, (_, cases)) in FROZEN_FILES.into_iter().enumerate() {
        let files = FakeHeaderFiles::new(cases[0], None);
        let mut driver = select_driver(&AsciiString::new("owned.fsd"), files.clone())
            .unwrap()
            .unwrap();
        format_from_driver(&mut *driver, &ExtendedString::new("owned.fsd")).unwrap();
        assert_eq!(
            files.trace.opens.borrow().as_slice(),
            vec![FileOpenMode::READ; index + 2]
        );
        assert_eq!(files.trace.closed.get(), index + 2);
        assert!(files.trace.reads.borrow().iter().all(|count| *count > 0));
    }
}

#[test]
fn a_failed_reopen_is_caught_without_reading_or_changing_the_file() {
    let bytes = FROZEN_FILES[0].1[0];
    let files = FakeHeaderFiles::new(bytes, Some(2));
    let mut driver = select_driver(&AsciiString::new("owned.fsd"), files.clone())
        .unwrap()
        .unwrap();
    let read_calls = files.trace.reads.borrow().len();
    assert!(
        format_from_driver(&mut *driver, &ExtendedString::new("owned.fsd"))
            .unwrap()
            .is_empty()
    );
    assert_eq!(files.trace.reads.borrow().len(), read_calls);
    assert_eq!(files.trace.closed.get(), 1);
    assert_eq!(driver.open_mode(), OpenMode::None);
    assert_eq!(
        files
            .memory
            .contents(&AsciiString::new("owned.fsd"))
            .unwrap(),
        bytes
    );
}

#[test]
fn unrecognized_and_truncated_header_probes_close_every_open_device() {
    for bytes in [
        b"unknown".as_slice(),
        b"CMPFILE\nbroken".as_slice(),
        b"FSDFILE\nbroken".as_slice(),
        b"BINFILE\0".as_slice(),
    ] {
        let files = FakeHeaderFiles::new(bytes, None);
        if let Some(mut driver) =
            select_driver(&AsciiString::new("owned.fsd"), files.clone()).unwrap()
        {
            assert!(
                format_from_driver(&mut *driver, &ExtendedString::new("owned.fsd"))
                    .unwrap()
                    .is_empty()
            );
            assert_eq!(driver.open_mode(), OpenMode::None);
        }
        assert_eq!(files.trace.closed.get(), files.trace.opens.borrow().len());
    }
}

fn truncated_binary_format(cut: usize) -> ExtendedString {
    let bytes = &FROZEN_FILES[2].1[0][..cut];
    let files = FakeHeaderFiles::new(bytes, None);
    let mut format = ExtendedString::default();
    if let Some(mut driver) = select_driver(&AsciiString::new("owned.fsd"), files.clone()).unwrap()
    {
        format = format_from_driver(&mut *driver, &ExtendedString::new("owned.fsd")).unwrap();
        assert_eq!(driver.open_mode(), OpenMode::None);
    }
    assert_eq!(files.trace.closed.get(), files.trace.opens.borrow().len());
    assert_eq!(
        files
            .memory
            .contents(&AsciiString::new("owned.fsd"))
            .unwrap(),
        bytes
    );
    format
}

#[test]
fn every_binary_prefix_preserves_completed_user_info_despite_later_header_failure() {
    for cut in 0..=167 {
        let expected = if cut >= 139 {
            cases::EXPECTED_UNITS[0]
        } else {
            &[]
        };
        assert_eq!(
            truncated_binary_format(cut).as_utf16(),
            expected,
            "binary prefix {cut}"
        );
    }
}
