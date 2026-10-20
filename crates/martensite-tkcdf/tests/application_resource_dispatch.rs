//! Resource callbacks run only on the source branch that uses them, once per virtual call.
use martensite_tkcdf::{
    SharedResourceManager,
    cdf::{CdfApplication, CdfApplicationBase, CdfApplicationServices},
    cdf_application_cdm_members,
    cdm::CdmApplication,
    pcdm::PcdmReaderStatus,
};
use martensite_tkernel::{
    impl_transient,
    resource::{ResourceHost, ResourceManager},
    standard::{Failure, Handle, SharedState, StandardTransient, exceptions},
    tcollection::ExtendedString,
};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

struct SequencedResourceApplication {
    transient: StandardTransient,
    base: CdfApplicationBase,
    managers: Vec<SharedResourceManager>,
    calls: AtomicUsize,
    fail_on: Option<usize>,
}
impl_transient!(SequencedResourceApplication, "SequencedResourceApplication", dyn CdfApplication,
    transient, interfaces: [dyn CdmApplication, dyn CdfApplication]);

impl CdmApplication for SequencedResourceApplication {
    cdf_application_cdm_members!();
    fn resources(&self) -> Result<SharedResourceManager, Failure> {
        let call = self.calls.fetch_add(1, Ordering::SeqCst);
        if self.fail_on == Some(call) {
            return Err(exceptions::domain_error(&format!(
                "Resources callback {call} failed; expected an available owned resource manager"
            )));
        }
        Ok(self.managers[call.min(self.managers.len() - 1)].clone())
    }
}
impl CdfApplication for SequencedResourceApplication {
    fn cdf_application_base(&self) -> &CdfApplicationBase {
        &self.base
    }
}

fn resource_manager(entries: &[(&str, &str)]) -> SharedResourceManager {
    let mut manager = ResourceManager::empty(ResourceHost::native());
    for (key, value) in entries {
        manager.set_resource(key, value.as_bytes());
    }
    Arc::new(SharedState::new(manager))
}

fn application(
    managers: Vec<SharedResourceManager>,
    fail_on: Option<usize>,
) -> SequencedResourceApplication {
    SequencedResourceApplication {
        transient: StandardTransient::default(),
        base: CdfApplicationBase::default(),
        managers,
        calls: AtomicUsize::new(0),
        fail_on,
    }
}

fn unavailable_resources() -> SequencedResourceApplication {
    application(vec![resource_manager(&[])], Some(0))
}

#[test]
fn extensionless_format_skips_resources_and_clears_the_output() {
    let app = unavailable_resources();
    let mut found = ExtendedString::new("old output");
    assert!(
        !app.format(&ExtendedString::new("owned-document"), &mut found)
            .unwrap()
    );
    assert!(found.is_empty());
    assert_eq!(app.calls.load(Ordering::SeqCst), 0);
}

#[test]
fn empty_extension_also_skips_resources() {
    let app = unavailable_resources();
    let mut found = ExtendedString::new("old output");
    assert!(
        !app.format(&ExtendedString::new("owned-document."), &mut found)
            .unwrap()
    );
    assert!(found.is_empty());
    assert_eq!(app.calls.load(Ordering::SeqCst), 0);
}

#[test]
fn format_clears_output_before_a_resource_callback_failure() {
    let app = unavailable_resources();
    let mut found = ExtendedString::new("old output");
    let error = app
        .format(&ExtendedString::new("owned.MiXeD"), &mut found)
        .unwrap_err();
    assert_eq!(error.exception_type(), "Standard_DomainError");
    assert!(found.is_empty());
    assert_eq!(app.calls.load(Ordering::SeqCst), 1);
}

#[test]
fn format_uses_the_second_virtual_resource_result_for_value() {
    let app = application(
        vec![
            resource_manager(&[("MiXeD.FileFormat", "first")]),
            resource_manager(&[("MiXeD.FileFormat", "second")]),
        ],
        None,
    );
    let mut found = ExtendedString::default();
    assert!(
        app.format(&ExtendedString::new("owned.MiXeD"), &mut found)
            .unwrap()
    );
    assert_eq!(found, ExtendedString::new("second"));
    assert_eq!(app.calls.load(Ordering::SeqCst), 2);
}

#[test]
fn missing_resource_is_a_single_lookup_and_does_not_fetch_a_value() {
    let app = application(vec![resource_manager(&[])], Some(1));
    let mut found = ExtendedString::new("old output");
    assert!(
        !app.format(&ExtendedString::new("owned.MiXeD"), &mut found)
            .unwrap()
    );
    assert!(found.is_empty());
    assert_eq!(app.calls.load(Ordering::SeqCst), 1);
}

#[test]
fn a_second_resource_callback_failure_propagates_without_publishing_a_format() {
    let app = application(
        vec![resource_manager(&[("MiXeD.FileFormat", "first")])],
        Some(1),
    );
    let mut found = ExtendedString::new("old output");
    let error = app
        .format(&ExtendedString::new("owned.MiXeD"), &mut found)
        .unwrap_err();
    assert_eq!(error.exception_type(), "Standard_DomainError");
    assert!(found.is_empty());
    assert_eq!(app.calls.load(Ordering::SeqCst), 2);
}

#[test]
fn a_changed_manager_without_the_value_propagates_the_resource_error() {
    let app = application(
        vec![
            resource_manager(&[("MiXeD.FileFormat", "first")]),
            resource_manager(&[]),
        ],
        None,
    );
    let mut found = ExtendedString::new("old output");
    let error = app
        .format(&ExtendedString::new("owned.MiXeD"), &mut found)
        .unwrap_err();
    assert_eq!(error.exception_type(), "Resource_NoSuchResource");
    assert!(found.is_empty());
    assert_eq!(app.calls.load(Ordering::SeqCst), 2);
}

#[test]
fn cached_null_reader_skips_resources_and_preserves_retrieval_status() {
    let app = unavailable_resources();
    let format = ExtendedString::new("owned");
    app.base.readers().add(format.clone(), Handle::default());
    app.base
        .set_retrievable_status(PcdmReaderStatus::AlreadyRetrieved);
    assert!(app.reader_from_format(&format).unwrap().is_null());
    assert_eq!(app.calls.load(Ordering::SeqCst), 0);
    assert_eq!(
        app.get_retrieve_status(),
        PcdmReaderStatus::AlreadyRetrieved
    );
}

#[test]
fn cached_null_writer_skips_resources_and_preserves_retrieval_status() {
    let app = unavailable_resources();
    let format = ExtendedString::new("owned");
    app.base.writers().add(format.clone(), Handle::default());
    app.base
        .set_retrievable_status(PcdmReaderStatus::AlreadyRetrieved);
    assert!(app.writer_from_format(&format).unwrap().is_null());
    assert_eq!(app.calls.load(Ordering::SeqCst), 0);
    assert_eq!(
        app.get_retrieve_status(),
        PcdmReaderStatus::AlreadyRetrieved
    );
}

#[test]
fn missing_writer_resource_caches_null_before_the_next_resource_failure() {
    let app = application(vec![resource_manager(&[])], Some(1));
    let format = ExtendedString::new("owned");
    let error = app.writer_from_format(&format).unwrap_err();
    assert_eq!(error.exception_type(), "Standard_NoSuchObject");
    assert!(app.writer_from_format(&format).unwrap().is_null());
    assert_eq!(app.calls.load(Ordering::SeqCst), 1);
    assert_eq!(app.base.writers().extent(), 1);
}

#[test]
fn resource_provider_failure_does_not_invent_a_cached_writer_or_status() {
    let app = unavailable_resources();
    app.base
        .set_retrievable_status(PcdmReaderStatus::AlreadyRetrieved);
    assert!(
        app.writer_from_format(&ExtendedString::new("owned"))
            .is_err()
    );
    assert_eq!(app.base.writers().extent(), 0);
    assert_eq!(
        app.get_retrieve_status(),
        PcdmReaderStatus::AlreadyRetrieved
    );
}

#[test]
fn retrieval_availability_uses_the_value_callback_before_the_cached_reader() {
    let app = application(
        vec![
            resource_manager(&[("MiXeD.FileFormat", "first")]),
            resource_manager(&[("MiXeD.FileFormat", "owned")]),
        ],
        None,
    );
    app.base
        .readers()
        .add(ExtendedString::new("owned"), Handle::default());
    assert_eq!(
        app.check_reader_availability(&ExtendedString::new("owned.MiXeD"))
            .unwrap(),
        PcdmReaderStatus::NoDriver
    );
    assert_eq!(app.calls.load(Ordering::SeqCst), 2);
}

#[test]
fn retrieval_availability_propagates_resource_value_failure_before_reader_lookup() {
    let app = application(
        vec![resource_manager(&[("MiXeD.FileFormat", "owned")])],
        Some(1),
    );
    app.base
        .set_retrievable_status(PcdmReaderStatus::AlreadyRetrieved);
    let error = app
        .check_reader_availability(&ExtendedString::new("owned.MiXeD"))
        .unwrap_err();
    assert_eq!(error.exception_type(), "Standard_DomainError");
    assert_eq!(app.calls.load(Ordering::SeqCst), 2);
    assert_eq!(
        app.get_retrieve_status(),
        PcdmReaderStatus::AlreadyRetrieved
    );
}

#[test]
fn a_present_empty_resource_still_reports_a_successful_format_lookup() {
    let app = application(vec![resource_manager(&[("MiXeD.FileFormat", "")])], None);
    let mut found = ExtendedString::new("old output");
    assert!(
        app.format(&ExtendedString::new("owned.MiXeD"), &mut found)
            .unwrap()
    );
    assert!(found.is_empty());
    assert_eq!(app.calls.load(Ordering::SeqCst), 2);
}
