//! Injected providers preserve reference loading without requiring a native container codec.
mod support;
use martensite_tkcdf::{
    cdm::{CdmDocumentReferences, CdmMetaData, MetaDataLookUpTable},
    pcdm::{PcdmDefaultReferenceIteratorHooks, PcdmReferenceIterator, PcdmReferenceIteratorHooks},
};
use martensite_tkernel::{
    message::Messenger,
    standard::{Failure, Handle, exceptions},
};
use std::sync::{Arc, Mutex};
use support::{new_application, new_document, text};

#[derive(Clone, Copy, Debug, PartialEq)]
enum ProviderFailure {
    Init,
    MetaData,
    Identifier,
    Version,
}

#[derive(Default)]
struct ProviderCursor {
    index: usize,
    calls: Vec<&'static str>,
    configurations: Vec<bool>,
}

#[derive(Default)]
struct NamedReferenceProvider {
    cursor: Mutex<ProviderCursor>,
    failure: Option<ProviderFailure>,
    failure_index: usize,
}

fn lookup(table: &mut MetaDataLookUpTable, name: &str) -> Handle<CdmMetaData> {
    CdmMetaData::look_up(
        table,
        &text("owned"),
        &text(name),
        &text(name),
        &text(name),
        false,
    )
}

impl NamedReferenceProvider {
    fn visit(&self, step: ProviderFailure, label: &'static str) -> Result<(), Failure> {
        let mut cursor = self.cursor.lock().unwrap();
        cursor.calls.push(label);
        if self.failure == Some(step) && cursor.index == self.failure_index {
            return Err(exceptions::domain_error(&format!(
                "reference provider failed at {step:?}; expected a readable reference record"
            )));
        }
        Ok(())
    }
}

impl PcdmReferenceIteratorHooks for NamedReferenceProvider {
    fn init(
        &self,
        _: &PcdmReferenceIterator,
        metadata: &Handle<CdmMetaData>,
    ) -> Result<(), Failure> {
        assert_eq!(metadata.file_name(), text("source"));
        self.visit(ProviderFailure::Init, "init")?;
        self.cursor.lock().unwrap().index = 0;
        Ok(())
    }
    fn more(&self, _: &PcdmReferenceIterator) -> bool {
        let mut cursor = self.cursor.lock().unwrap();
        cursor.calls.push("more");
        cursor.index < 2
    }
    fn next(&self, _: &PcdmReferenceIterator) {
        let mut cursor = self.cursor.lock().unwrap();
        cursor.calls.push("next");
        cursor.index += 1;
    }
    fn meta_data(
        &self,
        _: &PcdmReferenceIterator,
        table: &mut MetaDataLookUpTable,
        use_storage_configuration: bool,
    ) -> Result<Handle<CdmMetaData>, Failure> {
        self.visit(ProviderFailure::MetaData, "metadata")?;
        let mut cursor = self.cursor.lock().unwrap();
        cursor.configurations.push(use_storage_configuration);
        Ok(lookup(table, ["first", "second"][cursor.index]))
    }
    fn reference_identifier(&self, _: &PcdmReferenceIterator) -> Result<i32, Failure> {
        self.visit(ProviderFailure::Identifier, "identifier")?;
        Ok([17, 41][self.cursor.lock().unwrap().index])
    }
    fn document_version(&self, _: &PcdmReferenceIterator) -> Result<i32, Failure> {
        self.visit(ProviderFailure::Version, "version")?;
        Ok([3, 9][self.cursor.lock().unwrap().index])
    }
}

fn cursor_of(provider: Arc<NamedReferenceProvider>) -> PcdmReferenceIterator {
    PcdmReferenceIterator::with_hooks(&Arc::new(Messenger::new()), provider)
}

#[test]
fn custom_provider_creates_both_references_with_their_identifiers_and_versions() {
    let provider = Arc::new(NamedReferenceProvider::default());
    let iterator = cursor_of(provider.clone());
    let (document, _) = new_document();
    let (application, _) = new_application(0);
    let metadata = lookup(&mut MetaDataLookUpTable::default(), "source");
    iterator
        .load_references(&document, &metadata, &application, true)
        .unwrap();
    assert_eq!(document.to_references_number(), 2);
    assert_eq!(document.reference(17).document_version(), 3);
    assert_eq!(document.reference(41).document_version(), 9);
    assert_eq!(document.name(17).unwrap(), text("first"));
    assert_eq!(document.name(41).unwrap(), text("second"));
    assert_provider_calls(&provider);
}

fn assert_provider_calls(provider: &NamedReferenceProvider) {
    let cursor = provider.cursor.lock().unwrap();
    assert_eq!(cursor.configurations, [true, true]);
    assert_eq!(cursor.calls.first(), Some(&"init"));
    assert_eq!(cursor.calls.last(), Some(&"more"));
    for callback in ["metadata", "identifier", "version", "next"] {
        assert_eq!(
            cursor
                .calls
                .iter()
                .filter(|&&call| call == callback)
                .count(),
            2
        );
    }
    assert_eq!(
        cursor.calls.iter().filter(|&&call| call == "more").count(),
        3
    );
}

fn check_provider_failure(point: ProviderFailure) {
    let provider = Arc::new(NamedReferenceProvider {
        failure: Some(point),
        ..Default::default()
    });
    let (document, _) = new_document();
    let (application, _) = new_application(0);
    let metadata = lookup(&mut MetaDataLookUpTable::default(), "source");
    let error = cursor_of(provider.clone())
        .load_references(&document, &metadata, &application, false)
        .unwrap_err();
    assert_eq!(error.exception_type(), "Standard_DomainError");
    assert!(error.message().contains(&format!("{point:?}")));
    assert_eq!(document.to_references_number(), 0);
    assert!(!provider.cursor.lock().unwrap().calls.contains(&"next"));
}

#[test]
fn provider_failures_propagate_before_any_reference_is_created_or_cursor_advanced() {
    for point in [
        ProviderFailure::Init,
        ProviderFailure::MetaData,
        ProviderFailure::Identifier,
        ProviderFailure::Version,
    ] {
        check_provider_failure(point);
    }
}

#[test]
fn default_provider_initializes_an_empty_unknown_format_and_can_be_reused() {
    let iterator = PcdmReferenceIterator::new(&Arc::new(Messenger::new()));
    let (document, _) = new_document();
    let (application, _) = new_application(0);
    let metadata = lookup(&mut MetaDataLookUpTable::default(), "source");
    iterator.init(&metadata).unwrap();
    iterator
        .load_references(&document, &metadata, &application, false)
        .unwrap();
    assert_eq!(document.to_references_number(), 0);
    assert!(!PcdmDefaultReferenceIteratorHooks.more(&iterator));
    assert!(
        PcdmDefaultReferenceIteratorHooks
            .reference_identifier(&iterator)
            .is_err()
    );
    assert!(
        PcdmDefaultReferenceIteratorHooks
            .document_version(&iterator)
            .is_err()
    );
}

#[test]
fn a_later_provider_failure_preserves_the_completed_reference_without_advancing() {
    let provider = Arc::new(NamedReferenceProvider {
        failure: Some(ProviderFailure::MetaData),
        failure_index: 1,
        ..Default::default()
    });
    let (document, _) = new_document();
    let (application, _) = new_application(0);
    let metadata = lookup(&mut MetaDataLookUpTable::default(), "source");
    assert!(
        cursor_of(provider.clone())
            .load_references(&document, &metadata, &application, false)
            .is_err()
    );
    assert_eq!(document.to_references_number(), 1);
    assert_eq!(document.reference(17).document_version(), 3);
    assert!(document.reference(41).is_null());
    assert_eq!(provider.cursor.lock().unwrap().index, 1);
}

#[test]
fn default_provider_reports_absent_current_records_and_advances_its_own_cursor() {
    let iterator = PcdmReferenceIterator::new(&Arc::new(Messenger::new()));
    let metadata = lookup(&mut MetaDataLookUpTable::default(), "source");
    iterator.init(&metadata).unwrap();
    let mut table = MetaDataLookUpTable::default();
    let defaults = PcdmDefaultReferenceIteratorHooks;
    assert!(defaults.meta_data(&iterator, &mut table, false).is_err());
    defaults.next(&iterator);
    let error = defaults.reference_identifier(&iterator).unwrap_err();
    assert!(error.message().contains("index 2 outside 1..0"));
    assert!(!defaults.more(&iterator));
}

#[test]
fn public_reference_iterator_and_provider_contracts_are_send_and_sync() {
    fn require_send_sync<T: Send + Sync + ?Sized>() {}
    require_send_sync::<PcdmReferenceIterator>();
    require_send_sync::<dyn PcdmReferenceIteratorHooks>();
}

#[derive(Default)]
struct EmptyReferenceProvider {
    calls: Mutex<Vec<&'static str>>,
}

impl PcdmReferenceIteratorHooks for EmptyReferenceProvider {
    fn init(&self, _: &PcdmReferenceIterator, _: &Handle<CdmMetaData>) -> Result<(), Failure> {
        self.calls.lock().unwrap().push("init");
        Ok(())
    }
    fn more(&self, _: &PcdmReferenceIterator) -> bool {
        self.calls.lock().unwrap().push("more");
        false
    }
}

#[test]
fn empty_provider_does_not_dereference_unused_document_or_application_handles() {
    let provider = Arc::new(EmptyReferenceProvider::default());
    let iterator = PcdmReferenceIterator::with_hooks(&Arc::new(Messenger::new()), provider.clone());
    iterator
        .load_references(
            &Handle::default(),
            &Handle::default(),
            &Handle::default(),
            false,
        )
        .unwrap();
    assert_eq!(*provider.calls.lock().unwrap(), ["init", "more"]);
}

#[test]
fn nonempty_provider_initializes_before_rejecting_a_missing_document() {
    let provider = Arc::new(NamedReferenceProvider::default());
    let (application, _) = new_application(0);
    let metadata = lookup(&mut MetaDataLookUpTable::default(), "source");
    let error = cursor_of(provider.clone())
        .load_references(&Handle::default(), &metadata, &application, false)
        .unwrap_err();
    assert_eq!(error.exception_type(), "Standard_NullObject");
    assert_eq!(*provider.cursor.lock().unwrap().calls, ["init", "more"]);
}
