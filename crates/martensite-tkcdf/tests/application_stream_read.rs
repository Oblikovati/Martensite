//! `CDF_Application::Read(stream)`: known formats reach their reader after the probe, with
//! the native document preparation, status and exception catch.
#[path = "support/mod.rs"]
mod document_fixture;
#[path = "application_stream_read/fixture.rs"]
mod fixture;
#[allow(dead_code)] // The shared wire builder also serves the owned-stream cases.
#[path = "owned_stream_format/wire.rs"]
mod wire;
use document_fixture::new_document;
use fixture::{StreamApplication, StreamReader};
use martensite_tkcdf::{
    cdf::CdfApplicationServices,
    cdm::CdmDocument,
    pcdm::{PcdmInputStream, PcdmReaderFilter, PcdmReaderFilterAppendMode, PcdmReaderStatus},
};
use martensite_tkernel::{
    message::ProgressRange,
    standard::{Failure, Handle},
    storage::state,
};

const XML: &[u8] = b"<?xml version=\"1.0\"?><document format=\"last\"><label/></document>";

fn binary_with_body() -> Vec<u8> {
    let mut bytes = wire::ordinary_binary(0);
    bytes.extend_from_slice(b"document body");
    bytes
}

fn read(
    application: &StreamApplication,
    bytes: &[u8],
    document: &mut Handle<dyn CdmDocument>,
    filter: &Handle<PcdmReaderFilter>,
) -> Result<(), Failure> {
    let mut source = bytes;
    application.read_stream(&mut source, document, filter, &ProgressRange::default())
}

fn append_filter() -> Handle<PcdmReaderFilter> {
    let filter = Handle::new(PcdmReaderFilter::new());
    filter.set_mode(PcdmReaderFilterAppendMode::Protect);
    filter
}

#[test]
fn binary_stream_makes_a_new_document_and_reader_reads_the_body() {
    let (created, _) = new_document();
    let app = StreamApplication::bound(
        StreamReader::new(PcdmReaderStatus::Ok, None),
        created.clone(),
    );
    let mut document = Handle::default();
    read(&app, &binary_with_body(), &mut document, &Handle::default()).unwrap();
    assert_eq!(app.trace(), ["reader:last", "new:last"]);
    assert!(document == created);
    let calls = app.reader.calls.lock().unwrap();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].body, b"document body");
    assert_eq!(calls[0].format_lines, 2);
    assert!(calls[0].document == created);
    assert_eq!(app.get_retrieve_status(), PcdmReaderStatus::Ok);
}

#[test]
fn xml_stream_reader_continues_after_the_document_start_tag() {
    let (created, _) = new_document();
    let app = StreamApplication::bound(StreamReader::new(PcdmReaderStatus::Ok, None), created);
    read(&app, XML, &mut Handle::default(), &Handle::default()).unwrap();
    let calls = app.reader.calls.lock().unwrap();
    assert_eq!(calls[0].body, b"<label/></document>");
    assert_eq!(calls[0].format_lines, 0);
}

#[test]
fn append_mode_without_document_stops_with_no_document() {
    let app = StreamApplication::bound(
        StreamReader::new(PcdmReaderStatus::Ok, None),
        Handle::default(),
    );
    read(&app, XML, &mut Handle::default(), &append_filter()).unwrap();
    assert_eq!(app.get_retrieve_status(), PcdmReaderStatus::NoDocument);
    assert_eq!(app.trace(), ["reader:last"]);
    assert!(app.reader.calls.lock().unwrap().is_empty());
}

#[test]
fn append_mode_rejects_a_document_of_another_format() {
    let app = StreamApplication::bound(
        StreamReader::new(PcdmReaderStatus::Ok, None),
        Handle::default(),
    );
    let (mut document, _) = new_document();
    read(&app, XML, &mut document, &append_filter()).unwrap();
    assert_eq!(app.get_retrieve_status(), PcdmReaderStatus::FormatFailure);
    assert!(app.reader.calls.lock().unwrap().is_empty());
}

#[test]
fn append_mode_reads_into_the_callers_document_of_the_same_format() {
    let app = StreamApplication::bound(
        StreamReader::new(PcdmReaderStatus::Ok, None),
        Handle::default(),
    );
    let (mut document, _) = new_document();
    let xml = b"<?xml version=\"1.0\"?><document format=\"TestFormat\">rest";
    read(&app, xml, &mut document, &append_filter()).unwrap();
    assert_eq!(app.trace(), ["reader:TestFormat"]);
    let calls = app.reader.calls.lock().unwrap();
    assert!(calls[0].document == document);
    assert_eq!(calls[0].body, b"rest");
}

#[test]
fn reader_failure_past_already_retrieved_is_rethrown_printed() {
    let failure = Failure::with_message("Standard_DomainError", "owned reader fault");
    let reader = StreamReader::new(PcdmReaderStatus::ReaderException, Some(failure));
    let app = StreamApplication::bound(reader, Handle::default());
    let thrown = read(&app, XML, &mut Handle::default(), &Handle::default()).unwrap_err();
    assert_eq!(thrown.exception_type(), "Standard_Failure");
    assert_eq!(
        thrown.message(),
        "Standard_DomainError: owned reader fault\n"
    );
    assert_eq!(app.get_retrieve_status(), PcdmReaderStatus::ReaderException);
}

#[test]
fn reader_failure_up_to_already_retrieved_is_swallowed_with_its_status() {
    let failure = Failure::with_message("Standard_DomainError", "owned reader fault");
    let reader = StreamReader::new(PcdmReaderStatus::DriverFailure, Some(failure));
    let app = StreamApplication::bound(reader, Handle::default());
    read(&app, XML, &mut Handle::default(), &Handle::default()).unwrap();
    assert_eq!(app.get_retrieve_status(), PcdmReaderStatus::DriverFailure);
}

#[test]
fn missing_reader_propagates_unchanged_and_keeps_the_status() {
    let app = StreamApplication::bound(
        StreamReader::new(PcdmReaderStatus::Ok, None),
        Handle::default(),
    );
    app.base
        .set_retrievable_status(PcdmReaderStatus::PermissionDenied);
    let xml = b"<?xml version=\"1.0\"?><document format=\"missing\">";
    let thrown = read(&app, xml, &mut Handle::default(), &Handle::default()).unwrap_err();
    assert_eq!(thrown.exception_type(), "Standard_NoSuchObject");
    assert_eq!(
        app.get_retrieve_status(),
        PcdmReaderStatus::PermissionDenied
    );
}

#[test]
fn container_failure_sets_format_failure_and_is_rethrown_printed() {
    let app = StreamApplication::bound(
        StreamReader::new(PcdmReaderStatus::Ok, None),
        Handle::default(),
    );
    let truncated = &wire::ordinary_binary(0)[..65];
    let thrown = read(&app, truncated, &mut Handle::default(), &Handle::default()).unwrap_err();
    assert_eq!(thrown.exception_type(), "Standard_Failure");
    assert_eq!(thrown.message(), "Storage_StreamTypeMismatchError\n");
    assert_eq!(app.get_retrieve_status(), PcdmReaderStatus::FormatFailure);
    assert!(app.trace().is_empty());
}

#[test]
fn reader_stream_exception_from_the_mask_passes_the_catch() {
    let app = StreamApplication::bound(
        StreamReader::new(PcdmReaderStatus::Ok, None),
        Handle::default(),
    );
    app.base
        .set_retrievable_status(PcdmReaderStatus::PermissionDenied);
    let mut source: &[u8] = XML;
    let mut input = PcdmInputStream::with_exception_mask(&mut source, state::EOF);
    let thrown = app
        .read_input_stream(
            &mut input,
            &mut Handle::default(),
            &Handle::default(),
            &ProgressRange::default(),
        )
        .unwrap_err();
    assert_eq!(thrown.operation(), "CDF_Application::Read(stream)");
    assert!(thrown.kernel_failure().is_none());
    assert_eq!(
        app.get_retrieve_status(),
        PcdmReaderStatus::PermissionDenied
    );
}
