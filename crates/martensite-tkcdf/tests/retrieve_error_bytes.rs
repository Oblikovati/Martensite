//! The CDF filename catch preserves raw failure bytes and its status threshold.
#[path = "retrieve_error_bytes/callbacks.rs"]
mod callbacks;
#[path = "support/mod.rs"]
mod document_fixture;
#[path = "xml_header/support.rs"]
mod header_fixture;
use callbacks::{FaultApplication, FaultReader};
use martensite_tkcdf::{
    cdf::{CdfApplicationBase, CdfApplicationServices},
    cdm::{
        CdmApplication, CdmDocument, CdmDocumentLifecycle, CdmDocumentStorage, CdmMetaData,
        MetaDataLookUpTable,
    },
    pcdm::{PcdmReaderBase, PcdmReaderFilter, PcdmReaderFilterAppendMode, PcdmReaderStatus},
};
use martensite_tkernel::{
    message::ProgressRange,
    standard::{Failure, Handle, StandardTransient},
    tcollection::ExtendedString,
};
use std::sync::Mutex;

struct RawCase {
    message: &'static [u8],
    stack: &'static [u8],
    wrapped: &'static [u8],
}
const CASES: [RawCase; 3] = [
    RawCase {
        message: b"probe sentinel",
        stack: b"",
        wrapped: b"Standard_DomainError: probe sentinel\n",
    },
    RawCase {
        message: b"device\xffcause",
        stack: b"stack\xfetail",
        wrapped: b"Standard_DomainError: device\xffcausestack\xfetail\n",
    },
    RawCase {
        message: b"",
        stack: b"stack\xfetail",
        wrapped: b"Standard_DomainErrorstack\xfetail\n",
    },
];

fn bound_application(reader: Handle<FaultReader>) -> Handle<FaultApplication> {
    let app = Handle::new(FaultApplication {
        transient: StandardTransient::default(),
        base: CdfApplicationBase::default(),
        reader,
    });
    app.application_base().bind_self(&app.cast());
    app
}
fn metadata(file: &ExtendedString) -> Handle<CdmMetaData> {
    let name = ExtendedString::new("owned");
    CdmMetaData::look_up(
        &mut MetaDataLookUpTable::default(),
        &name,
        &name,
        file,
        file,
        false,
    )
}
fn fault_reader(
    case: &RawCase,
    status: PcdmReaderStatus,
    document: &Handle<dyn CdmDocument>,
    filter: &Handle<PcdmReaderFilter>,
) -> Handle<FaultReader> {
    Handle::new(FaultReader {
        transient: StandardTransient::default(),
        base: PcdmReaderBase::default(),
        failure: Failure::with_type_bytes("Standard_DomainError", case.message, case.stack),
        status,
        expected_document: document.clone(),
        expected_filter: filter.clone(),
        calls: Mutex::default(),
    })
}
fn run_case(case: &RawCase, status: PcdmReaderStatus) {
    let file = header_fixture::HeaderFixture::new(
        &format!(
            "retrieve-error-bytes-{}-{}",
            status as i32,
            case.message.len()
        ),
        Some(b"<document format='Owned'/>"),
    );
    let (document, _) = document_fixture::new_document();
    let filter = Handle::new(PcdmReaderFilter::new());
    filter.set_mode(PcdmReaderFilterAppendMode::Protect);
    let app = bound_application(fault_reader(case, status, &document, &filter));
    let metadata = metadata(&file.path);
    document.open(&app.cast());
    document.set_meta_data(&metadata).unwrap();
    check_result(&app, &metadata, &filter, case, status);
    assert!(metadata.document() == document);
    document.close().unwrap();
}
fn check_result(
    app: &FaultApplication,
    metadata: &Handle<CdmMetaData>,
    filter: &Handle<PcdmReaderFilter>,
    case: &RawCase,
    status: PcdmReaderStatus,
) {
    let result = app.retrieve_meta_data(metadata, true, false, filter, &ProgressRange::default());
    assert_eq!(app.get_retrieve_status(), status);
    assert_eq!(
        *app.reader.calls.lock().unwrap(),
        ["reader:Owned", "read-file"]
    );
    assert_outcome(result, app, case, status);
}
fn assert_outcome(
    result: Result<Handle<dyn CdmDocument>, Failure>,
    app: &FaultApplication,
    case: &RawCase,
    status: PcdmReaderStatus,
) {
    if status <= PcdmReaderStatus::AlreadyRetrieved {
        assert!(result.unwrap() == app.reader.expected_document);
        return;
    }
    let error = result.unwrap_err();
    assert_eq!(error.exception_type(), "Standard_Failure");
    assert_eq!(error.message_bytes(), case.wrapped);
    assert!(error.stack_bytes().is_empty());
}

#[test]
fn filename_retrieve_preserves_ascii_failure_printing() {
    for status in [
        PcdmReaderStatus::UnknownDocument,
        PcdmReaderStatus::ReaderException,
    ] {
        run_case(&CASES[0], status);
    }
}
#[test]
fn filename_retrieve_preserves_non_utf8_message_and_stack_printing() {
    for case in &CASES[1..] {
        for status in [
            PcdmReaderStatus::UnknownDocument,
            PcdmReaderStatus::ReaderException,
        ] {
            run_case(case, status);
        }
    }
}
#[test]
fn filename_retrieve_suppresses_failures_at_or_below_already_retrieved_status() {
    for case in &CASES {
        for status in [PcdmReaderStatus::Ok, PcdmReaderStatus::AlreadyRetrieved] {
            run_case(case, status);
        }
    }
}
