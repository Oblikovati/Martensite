use super::{CounterSpec, REFERENCES, VERSION, read_counter};
use crate::pcdm::file_header::{query_catching_read_failure, select_driver};
use crate::pcdm::file_header_test_support::{FROZEN_FILES, FakeHeaderFiles};
use martensite_tkernel::{
    storage::OpenMode,
    tcollection::{AsciiString, ExtendedString},
};

fn read_counter_bytes(bytes: &[u8], spec: CounterSpec, refuse: Option<usize>) -> i32 {
    let files = FakeHeaderFiles::new(bytes, refuse);
    let driver = select_driver(&AsciiString::new("owned.fsd"), files.clone()).unwrap();
    let Some(mut driver) = driver else {
        assert_closed_unchanged(&files, bytes);
        return spec.fallback;
    };
    let value = query_catching_read_failure(
        &mut *driver,
        &ExtendedString::new("owned.fsd"),
        spec.fallback,
        |driver, found| read_counter(driver, found, &ExtendedString::new("owned.fsd"), None, spec),
    )
    .unwrap();
    assert_eq!(driver.open_mode(), OpenMode::None);
    assert_closed_unchanged(&files, bytes);
    value
}

fn assert_closed_unchanged(files: &FakeHeaderFiles, bytes: &[u8]) {
    let successful = files.trace.opens.borrow().len() - usize::from(files.refuse_open.is_some());
    assert_eq!(files.trace.closed.get(), successful);
    assert_eq!(
        files
            .memory
            .contents(&AsciiString::new("owned.fsd"))
            .unwrap(),
        bytes
    );
}

#[test]
fn format_only_containers_retain_both_counter_defaults() {
    for (_, fixtures) in FROZEN_FILES {
        for bytes in fixtures {
            assert_eq!(read_counter_bytes(bytes, REFERENCES, None), 0);
            assert_eq!(read_counter_bytes(bytes, VERSION, None), -1);
        }
    }
}

#[test]
fn every_binary_prefix_preserves_exact_default_or_completed_header_values() {
    let bytes = include_bytes!("../../tests/header_counters/fixtures/binary-0.fsd");
    assert_eq!(bytes.len(), 167);
    for cut in 0..=167 {
        assert_eq!(
            read_counter_bytes(&bytes[..cut], REFERENCES, None),
            if (135..=138).contains(&cut) || cut >= 163 {
                17
            } else {
                0
            },
            "reference prefix {cut}"
        );
        assert_eq!(
            read_counter_bytes(&bytes[..cut], VERSION, None),
            if cut < 163 { -1 } else { 23 },
            "version prefix {cut}"
        );
    }
}

#[test]
fn a_refused_binary_reopen_is_caught_and_leaves_both_defaults() {
    let bytes = include_bytes!("../../tests/header_counters/fixtures/binary-0.fsd");
    assert_eq!(read_counter_bytes(bytes, REFERENCES, Some(4)), 0);
    assert_eq!(read_counter_bytes(bytes, VERSION, Some(4)), -1);
}
