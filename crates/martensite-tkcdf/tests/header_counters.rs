//! Literal counter expectations and warning bytes over owned frozen input containers.
#[path = "retrieval_contract/support.rs"]
mod callbacks;
#[path = "header_counters/support.rs"]
mod support;
use martensite_tkcdf::{cdf::CdfApplicationServices, pcdm::PcdmRetrievalDriverBase};
use martensite_tkernel::{message::Gravity, tcollection::ExtendedString};
use support::{EXPECTED_COUNTERS, fixture, recorder};

fn check_counter_fixture(kind: &str, scenario: usize, counter: i32, version: i32, warns: bool) {
    let file = fixture(kind, scenario);
    let (messenger, printer) = recorder();
    assert_eq!(
        PcdmRetrievalDriverBase::reference_counter_checked(&file, Some(&messenger)).unwrap(),
        counter
    );
    assert_eq!(
        PcdmRetrievalDriverBase::document_version_checked(&file, Some(&messenger)).unwrap(),
        version
    );
    assert_warnings(&file, &printer.messages.lock().unwrap(), warns, 1);
    assert_eq!(
        PcdmRetrievalDriverBase::reference_counter(&file, &messenger),
        counter
    );
    assert_eq!(
        PcdmRetrievalDriverBase::document_version(&file, &messenger),
        version
    );
    assert_warnings(&file, &printer.messages.lock().unwrap(), warns, 2);
}

fn assert_warnings(
    file: &ExtendedString,
    messages: &[(Vec<u8>, Gravity)],
    expected: bool,
    repeats: usize,
) {
    if !expected {
        assert!(messages.is_empty());
        return;
    }
    assert_eq!(messages.len(), 2 * repeats);
    for ((bytes, gravity), kind) in messages
        .iter()
        .zip(["reference counter", "version"].into_iter().cycle())
    {
        assert_eq!(*gravity, Gravity::Warning);
        assert_eq!(
            bytes,
            format!("Warning: could not read the {kind} in {}", file.to_utf8()).as_bytes()
        );
    }
}

#[test]
fn all_container_counters_preserve_defaults_last_valid_values_and_warning_bytes() {
    for kind in ["cmp", "text", "binary"] {
        for (scenario, &(counter, version, warns)) in EXPECTED_COUNTERS.iter().enumerate() {
            check_counter_fixture(kind, scenario, counter, version, warns);
        }
    }
}

#[test]
fn a_null_messenger_suppresses_warnings_without_changing_the_counter_scan() {
    for kind in ["cmp", "text", "binary"] {
        for (scenario, &(counter, version, _)) in EXPECTED_COUNTERS.iter().enumerate() {
            let file = fixture(kind, scenario);
            assert_eq!(
                PcdmRetrievalDriverBase::reference_counter_checked(&file, None).unwrap(),
                counter
            );
            assert_eq!(
                PcdmRetrievalDriverBase::document_version_checked(&file, None).unwrap(),
                version
            );
        }
    }
}

#[test]
fn application_document_version_reaches_the_real_stored_counter_without_resources() {
    let file = fixture("binary", 0);
    let meta = callbacks::metadata(&file);
    let app = callbacks::application(meta.clone(), true, true, false, false);
    assert_eq!(app.cdf_document_version(&meta).unwrap(), 23);
    assert!(app.trace.lock().unwrap().is_empty());
}
