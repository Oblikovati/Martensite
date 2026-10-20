//! Frozen container bytes exercise the public filename paths without a native runtime.
#[path = "retrieval_contract/support.rs"]
mod callbacks;
#[path = "file_header/cases.rs"]
mod cases;
use martensite_tkcdf::{
    cdf::CdfApplicationServices,
    pcdm::{PcdmReadWriterBase, PcdmReaderStatus},
};
use martensite_tkernel::tcollection::ExtendedString;

fn path(kind: &str, scenario: usize) -> ExtendedString {
    let path = format!(
        "{}/tests/file_header/fixtures/{kind}-{scenario}.fsd",
        env!("CARGO_MANIFEST_DIR")
    );
    ExtendedString::from_c_string(path.as_bytes(), true)
}

#[test]
fn public_filename_queries_read_all_frozen_text_compatible_and_binary_headers() {
    for kind in ["cmp", "text", "binary"] {
        for (scenario, units) in cases::EXPECTED_UNITS.iter().enumerate() {
            let file = path(kind, scenario);
            assert_eq!(
                PcdmReadWriterBase::file_format_checked(&file)
                    .unwrap()
                    .as_utf16(),
                *units,
                "{kind}-{scenario}"
            );
            assert_eq!(
                PcdmReadWriterBase::file_format(&file).as_utf16(),
                *units,
                "{kind}-{scenario}"
            );
        }
    }
}

fn check_framework_header(kind: &str, scenario: usize, units: &[u16]) {
    let file = path(kind, scenario);
    let app = callbacks::application(callbacks::metadata(&file), true, true, false, false);
    let mut found = callbacks::text("stale");
    if units.is_empty() {
        assert_eq!(
            app.format(&file, &mut found).unwrap_err().exception_type(),
            "Standard_DomainError"
        );
        assert!(found.is_empty());
        assert_eq!(*app.trace.lock().unwrap(), ["resources"]);
        return;
    }
    assert!(app.format(&file, &mut found).unwrap());
    assert_eq!(found.as_utf16(), units);
    assert!(app.trace.lock().unwrap().is_empty());
    assert_eq!(
        app.can_retrieve(&callbacks::text("folder"), &callbacks::text("name"), false)
            .unwrap(),
        PcdmReaderStatus::NoDriver
    );
    assert_eq!(
        app.trace.lock().unwrap().last().unwrap(),
        &format!("reader:{}", ExtendedString::from_utf16(units).to_utf8())
    );
}

#[test]
fn framework_uses_all_real_container_headers_before_resources_and_reader_dispatch() {
    for kind in ["cmp", "text", "binary"] {
        for (scenario, units) in cases::EXPECTED_UNITS.iter().enumerate() {
            check_framework_header(kind, scenario, units);
        }
    }
}
