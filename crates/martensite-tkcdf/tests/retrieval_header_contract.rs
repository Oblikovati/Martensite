//! Header precedence and metadata callback ordering follow the actual framework branches.
#[path = "retrieval_contract/support.rs"]
mod callbacks;
#[path = "support/mod.rs"]
mod document_fixture;
#[path = "xml_header/support.rs"]
mod header_fixture;
use callbacks::{FakeHeaderApplication, application, metadata, text};
use header_fixture::HeaderFixture;
use martensite_tkcdf::{
    cdf::CdfApplicationServices,
    cdm::{CdmDocument, CdmDocumentLifecycle, CdmDocumentStorage, CdmMetaData},
    pcdm::PcdmReaderStatus,
};
use martensite_tkernel::standard::Handle;

const METADATA_TRACE: &[&str] = &["find:folder/name/", "permission", "metadata"];
const READER_TRACE: &[&str] = &[
    "find:folder/name/",
    "permission",
    "metadata",
    "reader:Header",
];

fn assert_status(app: &FakeHeaderApplication, append: bool, expected: PcdmReaderStatus) {
    assert_eq!(
        app.can_retrieve(&text("folder"), &text("name"), append)
            .unwrap(),
        expected
    );
}
fn assert_trace(app: &FakeHeaderApplication, expected: &[&str]) {
    assert_eq!(*app.trace.lock().unwrap(), expected);
}
fn attach_document(
    app: &Handle<FakeHeaderApplication>,
    meta: &Handle<CdmMetaData>,
    modified: bool,
) -> Handle<dyn CdmDocument> {
    let (document, _) = document_fixture::new_document();
    document.open(&app.cast());
    document.set_meta_data(meta).unwrap();
    if modified {
        document.modify();
    }
    document
}

#[test]
fn format_uses_xml_header_before_the_failing_resources_callback() {
    let file = HeaderFixture::new(
        "format-precedence",
        Some(b"<document format='Header'/><broken"),
    );
    let app = application(metadata(&file.path), true, true, false, false);
    let mut found = text("stale");
    assert!(app.format(&file.path, &mut found).unwrap());
    assert_eq!(found, text("Header"));
    assert_trace(&app, &[]);
}

#[test]
fn empty_xml_format_keeps_the_extension_resource_failure_and_cleared_output() {
    let file = HeaderFixture::new("format-fallback", Some(b"<document format=''/>"));
    let app = application(metadata(&file.path), true, true, false, false);
    let mut found = text("stale");
    let error = app.format(&file.path, &mut found).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_DomainError");
    assert!(found.is_empty());
    assert_trace(&app, &["resources"]);
}

fn assert_metadata_rejection(
    found: bool,
    readable: bool,
    expected: PcdmReaderStatus,
    trace: &[&str],
) {
    let app = application(metadata(&text("unused")), found, readable, false, false);
    assert_eq!(
        app.can_retrieve_version(&text("folder"), &text("name"), &text("v7"), false)
            .unwrap(),
        expected
    );
    assert_trace(&app, trace);
}

#[test]
fn metadata_callbacks_short_circuit_unknown_and_permission_denied_documents() {
    assert_metadata_rejection(
        false,
        true,
        PcdmReaderStatus::UnknownDocument,
        &["find:folder/name/v7"],
    );
    assert_metadata_rejection(
        true,
        false,
        PcdmReaderStatus::PermissionDenied,
        &["find:folder/name/v7", "permission"],
    );
}

#[test]
fn metadata_failure_propagates_before_any_header_or_reader_callback() {
    let app = application(metadata(&text("unused")), true, true, true, false);
    let error = app
        .can_retrieve(&text("folder"), &text("name"), false)
        .unwrap_err();
    assert_eq!(error.exception_type(), "Standard_DomainError");
    assert_trace(&app, METADATA_TRACE);
}

#[test]
fn append_without_a_retrieved_document_returns_before_header_dispatch() {
    let app = application(metadata(&text("unused")), true, true, false, false);
    assert_status(&app, true, PcdmReaderStatus::NoDocument);
    assert_trace(&app, METADATA_TRACE);
}

#[test]
fn retrieved_document_modification_state_precedes_header_dispatch() {
    for (modified, expected) in [
        (false, PcdmReaderStatus::AlreadyRetrieved),
        (true, PcdmReaderStatus::AlreadyRetrievedAndModified),
    ] {
        let meta = metadata(&text("unused"));
        let app = application(meta.clone(), true, true, false, false);
        let _document = attach_document(&app, &meta, modified);
        assert_status(&app, false, expected);
        assert_trace(&app, METADATA_TRACE);
    }
}

#[test]
fn xml_header_reaches_the_overridden_reader_and_null_means_no_driver() {
    let file = HeaderFixture::new("retrieve-null", Some(b"<document format='Header'/>"));
    let app = application(metadata(&file.path), true, true, false, false);
    assert_status(&app, false, PcdmReaderStatus::NoDriver);
    assert_trace(&app, READER_TRACE);
}

#[test]
fn reader_failure_is_suppressed_after_header_dispatch_without_resetting_its_status() {
    let file = HeaderFixture::new("retrieve-failure", Some(b"<document format='Header'/>"));
    let app = application(metadata(&file.path), true, true, false, true);
    assert_status(&app, false, PcdmReaderStatus::Ok);
    assert_eq!(app.get_retrieve_status(), PcdmReaderStatus::ReaderException);
    assert_trace(&app, READER_TRACE);
}

#[test]
fn live_reader_availability_returns_ok_without_invoking_its_read_callbacks() {
    let file = HeaderFixture::new("retrieve-live", Some(b"<document format='Header'/>"));
    let app = application(metadata(&file.path), true, true, false, false);
    app.live_reader
        .store(true, std::sync::atomic::Ordering::SeqCst);
    assert_status(&app, false, PcdmReaderStatus::Ok);
    assert_trace(&app, READER_TRACE);
    assert_eq!(app.read_calls.load(std::sync::atomic::Ordering::SeqCst), 0);
}

#[test]
fn append_to_a_retrieved_modified_document_continues_to_header_dispatch() {
    let file = HeaderFixture::new("append-retrieved", Some(b"<document format='Header'/>"));
    let meta = metadata(&file.path);
    let app = application(meta.clone(), true, true, false, false);
    let _document = attach_document(&app, &meta, true);
    assert_status(&app, true, PcdmReaderStatus::NoDriver);
    assert_trace(&app, READER_TRACE);
}

#[test]
fn legacy_metadata_queries_keep_their_default_fallible_adapters() {
    for (found, readable) in [(false, true), (true, false), (true, true)] {
        let app = callbacks::application(
            callbacks::metadata(&text("unused")),
            found,
            readable,
            false,
            false,
        );
        let driver = app.base.meta_data_driver_field();
        assert_eq!(
            driver.try_find(&text("f"), &text("n"), &text("v")).unwrap(),
            found
        );
        assert_eq!(
            driver
                .try_has_read_permission(&text("f"), &text("n"), &text("v"))
                .unwrap(),
            readable
        );
        assert_eq!(*app.trace.lock().unwrap(), ["find:f/n/v", "permission"]);
        assert_eq!(
            driver
                .try_find_without_version(&text("f"), &text("n"))
                .unwrap(),
            found
        );
        assert_eq!(
            *app.trace.lock().unwrap(),
            ["find:f/n/v", "permission", "find:f/n/"]
        );
    }
}
