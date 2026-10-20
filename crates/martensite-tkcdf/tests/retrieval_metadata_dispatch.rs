//! Metadata callbacks preserve their typed failures and the application's live driver binding.
#[path = "retrieval_contract/support.rs"]
mod callbacks;
#[path = "retrieval_contract/fallible_metadata.rs"]
mod fallible_metadata;
use callbacks::{FakeHeaderApplication, application, metadata, text};
use fallible_metadata::{QueryHook, configured_driver};
use martensite_tkcdf::{
    cdf::{CdfApplicationServices, CdfMetaDataDriver},
    pcdm::PcdmReaderStatus,
};
use martensite_tkernel::standard::{Failure, Handle};
use std::sync::Arc;

fn callback_failure() -> Failure {
    Failure::with_type_bytes("Standard_DomainError", b"metadata\xffcause", b"owned-stack")
}
fn prepared_application() -> Handle<FakeHeaderApplication> {
    let app = application(metadata(&text("unused")), true, true, false, false);
    app.base
        .set_retrievable_status(PcdmReaderStatus::ReaderException);
    app
}
fn retrieve(app: &FakeHeaderApplication, version: &str) -> Result<PcdmReaderStatus, Failure> {
    if version.is_empty() {
        return app.can_retrieve(&text("folder"), &text("name"), true);
    }
    app.can_retrieve_version(&text("folder"), &text("name"), &text(version), true)
}
fn expected_trace(version: &str, phases: &[&str]) -> Vec<String> {
    let mut trace = vec![format!("find:folder/name/{version}")];
    trace.extend(phases.iter().map(|phase| (*phase).to_owned()));
    trace
}
fn assert_unchanged_status(app: &FakeHeaderApplication) {
    assert_eq!(app.get_retrieve_status(), PcdmReaderStatus::ReaderException);
    assert_eq!(app.read_calls.load(std::sync::atomic::Ordering::SeqCst), 0);
}

fn checked_query_hook(
    expected_version: &str,
    callback: impl Fn(&str) -> Result<(), Failure> + Send + Sync + 'static,
) -> QueryHook {
    let expected_version = text(expected_version);
    Arc::new(move |phase, folder, name, version| {
        assert_eq!(folder, &text("folder"));
        assert_eq!(name, &text("name"));
        assert_eq!(version, &expected_version);
        callback(phase)
    })
}

fn injected_driver(
    app: &FakeHeaderApplication,
    readable: bool,
    hook: Option<QueryHook>,
) -> Handle<fallible_metadata::FallibleRetrievalDriver> {
    configured_driver(app, metadata(&text("unused")), true, readable, false, hook)
}
fn failure_query_hook(version: &str, stage: &'static str) -> QueryHook {
    checked_query_hook(version, move |phase| {
        if phase == stage {
            return Err(callback_failure());
        }
        Ok(())
    })
}
fn install_replacement(
    app: &Handle<FakeHeaderApplication>,
    version: &str,
    stage: &'static str,
    replacement: &Handle<fallible_metadata::FallibleRetrievalDriver>,
) -> Handle<fallible_metadata::FallibleRetrievalDriver> {
    let weak = app.downgrade();
    let replacement = replacement.clone();
    let hook = checked_query_hook(version, move |phase| {
        if phase == stage {
            weak.upgrade()
                .base
                .set_meta_data_driver(&replacement.cast());
        }
        Ok(())
    });
    let first = injected_driver(app, true, Some(hook));
    app.base.set_meta_data_driver(&first.cast());
    first
}
fn assert_trace(app: &FakeHeaderApplication, version: &str, phases: &[&str]) {
    assert_eq!(*app.trace.lock().unwrap(), expected_trace(version, phases));
    assert_unchanged_status(app);
}

#[test]
fn successful_metadata_queries_keep_the_retrieval_status_and_trace() {
    for version in ["v7", ""] {
        let app = prepared_application();
        let driver = injected_driver(&app, true, Some(checked_query_hook(version, |_| Ok(()))));
        app.base.set_meta_data_driver(&driver.cast());
        assert_eq!(
            retrieve(&app, version).unwrap(),
            PcdmReaderStatus::NoDocument
        );
        assert_trace(&app, version, &["permission", "metadata"]);
    }
}

#[test]
fn typed_provider_failures_stop_at_the_original_metadata_boundary() {
    for version in ["v7", ""] {
        for (stage, phases) in [
            ("find", vec![]),
            ("permission", vec!["permission"]),
            ("metadata", vec!["permission", "metadata"]),
        ] {
            let app = prepared_application();
            let driver = injected_driver(&app, true, Some(failure_query_hook(version, stage)));
            app.base.set_meta_data_driver(&driver.cast());
            assert_eq!(retrieve(&app, version).unwrap_err(), callback_failure());
            assert_trace(&app, version, &phases);
        }
    }
}

#[test]
fn find_replacement_supplies_the_following_permission_query() {
    for version in ["v7", ""] {
        let app = prepared_application();
        let replacement =
            injected_driver(&app, false, Some(checked_query_hook(version, |_| Ok(()))));
        let _old_driver_lease = install_replacement(&app, version, "find", &replacement);
        assert_eq!(
            retrieve(&app, version).unwrap(),
            PcdmReaderStatus::PermissionDenied
        );
        assert_trace(&app, version, &["permission"]);
        assert!(app.base.meta_data_driver_field() == replacement);
    }
}

#[test]
fn permission_replacement_supplies_the_following_metadata_query() {
    for version in ["v7", ""] {
        let app = prepared_application();
        let replacement =
            injected_driver(&app, true, Some(failure_query_hook(version, "metadata")));
        let _old_driver_lease = install_replacement(&app, version, "permission", &replacement);
        assert_eq!(retrieve(&app, version).unwrap_err(), callback_failure());
        assert_trace(&app, version, &["permission", "metadata"]);
        assert!(app.base.meta_data_driver_field() == replacement);
    }
}

#[test]
fn unversioned_find_forwards_empty_version_and_the_original_failure() {
    for (found, fails) in [(false, false), (true, false), (true, true)] {
        let app = prepared_application();
        let hook = if fails {
            failure_query_hook("", "find")
        } else {
            checked_query_hook("", |_| Ok(()))
        };
        let driver = configured_driver(
            &app,
            metadata(&text("unused")),
            found,
            true,
            false,
            Some(hook),
        );
        let driver: Handle<dyn CdfMetaDataDriver> = driver.cast();
        let outcome = driver.try_find_without_version(&text("folder"), &text("name"));
        assert_eq!(
            outcome,
            if fails {
                Err(callback_failure())
            } else {
                Ok(found)
            }
        );
        assert_trace(&app, "", &[]);
    }
}
