use super::PcdmReferenceIterator;
use crate::{
    cdm::{CdmMetaData, MetaDataLookUpTable},
    pcdm::PcdmReference,
};
use martensite_tkernel::{standard::Handle, tcollection::ExtendedString};

fn metadata(file: &str) -> Handle<CdmMetaData> {
    let file = ExtendedString::from_c_string(file.as_bytes(), true);
    CdmMetaData::look_up(
        &mut MetaDataLookUpTable::default(),
        &ExtendedString::new("folder"),
        &ExtendedString::new("header"),
        &file,
        &file,
        false,
    )
}
fn fixture(name: &str) -> Handle<CdmMetaData> {
    metadata(&format!(
        "{}/tests/header_references/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    ))
}
fn seed(iterator: &PcdmReferenceIterator) {
    let mut state = iterator.state.write();
    state.references = vec![PcdmReference::new(41, &ExtendedString::new("old"), 43)];
    state.iterator = 9;
}

#[test]
fn native_default_initialization_loads_references_and_restarts_the_cursor() {
    let iterator = PcdmReferenceIterator::with_optional_message_driver(None);
    seed(&iterator);
    iterator.init(&fixture("binary-0.fsd")).unwrap();
    assert_eq!(iterator.state.read().references.len(), 2);
    assert_eq!(iterator.state.read().iterator, 1);
    assert_eq!(iterator.current().unwrap().reference_identifier(), 1);
    iterator.next();
    assert_eq!(iterator.current().unwrap().document_version(), 7);
    iterator.next();
    assert!(!iterator.more());
}

#[test]
fn a_failing_header_read_clears_previous_references_before_the_failure_and_keeps_cursor() {
    let iterator = PcdmReferenceIterator::with_optional_message_driver(None);
    seed(&iterator);
    let failure = iterator.init(&fixture("truncated-info.fsd")).unwrap_err();
    assert_eq!(failure.exception_type(), "Storage_StreamWriteError");
    assert!(iterator.state.read().references.is_empty());
    assert_eq!(iterator.state.read().iterator, 9);
    assert!(!iterator.more());
}

#[test]
fn successful_empty_initialization_clears_references_and_resets_cursor() {
    let iterator = PcdmReferenceIterator::with_optional_message_driver(None);
    seed(&iterator);
    iterator.init(&fixture("binary-4.fsd")).unwrap();
    assert!(iterator.state.read().references.is_empty());
    assert_eq!(iterator.state.read().iterator, 1);
    assert!(!iterator.more());
}

struct ReentrantInitPrinter {
    base: martensite_tkernel::message::PrinterBase,
    target: std::sync::Mutex<martensite_tkernel::standard::WeakHandle<PcdmReferenceIterator>>,
    inner: Handle<CdmMetaData>,
    entered: std::sync::atomic::AtomicBool,
}
impl martensite_tkernel::message::Printer for ReentrantInitPrinter {
    fn printer_base(&self) -> &martensite_tkernel::message::PrinterBase {
        &self.base
    }
    fn dynamic_type(&self) -> &'static martensite_tkernel::standard::RuntimeType {
        <martensite_tkernel::message::PrinterType as martensite_tkernel::standard::RuntimeTyped>::static_type()
    }
    fn send(
        &self,
        _text: &martensite_tkernel::tcollection::AsciiString,
        _gravity: martensite_tkernel::message::Gravity,
    ) {
        if self.entered.swap(true, std::sync::atomic::Ordering::SeqCst) {
            return;
        }
        let target = self.target.lock().unwrap().upgrade();
        target.init(&self.inner).unwrap();
    }
}

#[test]
fn reentrant_warning_initialization_changes_the_current_member_sequence_before_outer_append() {
    use martensite_tkernel::{
        message::{Messenger, PrinterBase},
        standard::WeakHandle,
    };
    use std::sync::{Arc, Mutex, atomic::AtomicBool};
    let printer = Arc::new(ReentrantInitPrinter {
        base: PrinterBase::default(),
        target: Mutex::new(WeakHandle::default()),
        inner: fixture("binary-2.fsd"),
        entered: AtomicBool::new(false),
    });
    let messenger = Arc::new(Messenger::with_printer(printer.clone()));
    let iterator = Handle::new(PcdmReferenceIterator::new(&messenger));
    *printer.target.lock().unwrap() = iterator.downgrade();
    iterator.init(&fixture("binary-0.fsd")).unwrap();
    let state = iterator.state.read();
    assert_eq!(
        state
            .references
            .iter()
            .map(PcdmReference::reference_identifier)
            .collect::<Vec<_>>(),
        [7, 1, 2]
    );
    assert_eq!(state.iterator, 1);
}
