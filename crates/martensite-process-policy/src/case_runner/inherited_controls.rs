use super::{
    CaseFailure, CaseSpec, FactorProfile,
    artifacts::{FilesystemReceipts, retain_result},
    child_policy::{PolicyInstaller, execute_prepared_with, execute_with},
    context::CaseContext,
    fake_controls::OwnedArtifactRoot,
    launch::{ChildObservation, ChildRequest},
    lifecycle::ChildLifecycle,
};
use std::{cell::Cell, error::Error};

struct NamedForbiddenInstaller(Cell<usize>);
impl PolicyInstaller for NamedForbiddenInstaller {
    fn install(&self, _seconds: i64) -> Result<(), Box<dyn Error>> {
        self.0.set(self.0.get() + 1);
        Err("inheritance must not install a CPU/elapsed policy".into())
    }
}

fn inherited_spec() -> CaseSpec<'static> {
    let identity = "controls::inherited";
    CaseSpec::inherit_process_policy(identity, FactorProfile::RequireUnset)
}

#[test]
fn direct_inheritance_skips_installer_and_retains_complete_v2_request() {
    let root = OwnedArtifactRoot::new();
    let context = CaseContext::create(root.path(), inherited_spec()).unwrap();
    let installer = NamedForbiddenInstaller(Cell::new(0));
    let body = Cell::new(0);
    execute_with(
        &context,
        inherited_spec(),
        || {
            body.set(body.get() + 1);
            Ok(())
        },
        &installer,
    )
    .unwrap();
    assert_eq!((installer.0.get(), body.get()), (0, 1));
    assert_bound_inherited_request(&context);
}

#[test]
fn prepared_inheritance_preserves_order_without_a_policy_installation() {
    let root = OwnedArtifactRoot::new();
    let context =
        CaseContext::create_for(root.path(), inherited_spec(), ChildLifecycle::Prepared).unwrap();
    let installer = NamedForbiddenInstaller(Cell::new(0));
    execute_prepared_with(
        &context,
        inherited_spec(),
        || Ok(17),
        |owned| require_owned_preparation(&context, &installer, owned),
        &installer,
    )
    .unwrap();
    context.require_record("complete.txt").unwrap();
    assert_eq!(installer.0.get(), 0);
}

#[test]
fn preparation_error_in_inherited_case_prevents_body_and_completion() {
    let root = OwnedArtifactRoot::new();
    let context =
        CaseContext::create_for(root.path(), inherited_spec(), ChildLifecycle::Prepared).unwrap();
    let installer = NamedForbiddenInstaller(Cell::new(0));
    let result = execute_prepared_with(
        &context,
        inherited_spec(),
        || Err::<(), _>("prepare refused".into()),
        |_| panic!("body must not run"),
        &installer,
    );
    assert!(matches!(result, Err(CaseFailure::Preparation(_))));
    assert_eq!(installer.0.get(), 0);
    assert!(!context.directory.join("complete.txt").exists());
}

#[test]
fn inherited_body_error_retains_failure_without_complete_record() {
    let root = OwnedArtifactRoot::new();
    let context = CaseContext::create(root.path(), inherited_spec()).unwrap();
    let installer = NamedForbiddenInstaller(Cell::new(0));
    let result = execute_with(
        &context,
        inherited_spec(),
        || Err("owned body refusal".into()),
        &installer,
    );
    assert!(matches!(result, Err(CaseFailure::Body(_))));
    assert_eq!(installer.0.get(), 0);
    assert!(!context.directory.join("complete.txt").exists());
}

#[test]
fn finite_v1_and_inherited_v2_contexts_cannot_be_exchanged() {
    let root = OwnedArtifactRoot::new();
    let finite = CaseSpec::new("controls::inherited", 60, FactorProfile::RequireUnset);
    let context = CaseContext::create(root.path(), finite).unwrap();
    let request = String::from_utf8(context.request.clone()).unwrap();
    assert!(request.starts_with("MARTENSITE_EXACT_CASE_V1\n"));
    assert!(request.contains("\nseconds=60\n"));
    assert!(CaseContext::load(context.directory, inherited_spec()).is_err());
}

#[test]
fn inherited_receipt_records_no_fabricated_budget_or_installed_policy() {
    let root = OwnedArtifactRoot::new();
    let context = CaseContext::create(root.path(), inherited_spec()).unwrap();
    let receipt = inherited_receipt_text(&context);
    for expected in [
        "\"schema_version\":2",
        "\"budget_seconds\":null",
        "\"process_policy\":\"inherit\"",
        "\"runner_policy_installed\":false",
    ] {
        assert!(
            receipt.contains(expected),
            "receipt={receipt}; expected {expected}"
        );
    }
}

fn assert_bound_inherited_request(context: &CaseContext) {
    context.require_record("entry.txt").unwrap();
    context.require_record("complete.txt").unwrap();
    let request = String::from_utf8(context.request.clone()).unwrap();
    assert!(request.starts_with("MARTENSITE_EXACT_CASE_V2\n"));
    assert!(request.contains("\nprocess_policy=inherit\n"));
    assert!(!request.contains("seconds="));
}

fn require_owned_preparation(
    context: &CaseContext,
    installer: &NamedForbiddenInstaller,
    owned: i32,
) -> Result<(), Box<dyn Error>> {
    assert_eq!(owned, 17);
    context.require_record("prepared.txt").unwrap();
    assert_eq!(installer.0.get(), 0);
    Ok(())
}

fn inherited_receipt_text(context: &CaseContext) -> String {
    let request = ChildRequest {
        context,
        spec: inherited_spec(),
    };
    let observation = ChildObservation {
        code: Some(0),
        signal: None,
        stdout: vec![],
        stderr: vec![],
    };
    retain_result(&request, Some(&observation), None, &FilesystemReceipts).unwrap();
    std::fs::read_to_string(context.directory.join("receipt.json")).unwrap()
}

#[test]
fn inherited_v2_cannot_be_loaded_as_finite_v1() {
    let root = OwnedArtifactRoot::new();
    let context = CaseContext::create(root.path(), inherited_spec()).unwrap();
    let finite = CaseSpec::new("controls::inherited", 60, FactorProfile::RequireUnset);
    assert!(CaseContext::load(context.directory, finite).is_err());
}

#[test]
fn inherited_mode_refuses_invalid_identity_and_unknown_request_header() {
    let invalid = CaseSpec::inherit_process_policy("", FactorProfile::RequireUnset);
    assert!(invalid.validate().is_err());
    let root = OwnedArtifactRoot::new();
    let context = CaseContext::create(root.path(), inherited_spec()).unwrap();
    let request = String::from_utf8(context.request.clone())
        .unwrap()
        .replace("EXACT_CASE_V2", "EXACT_CASE_V99");
    std::fs::write(context.directory.join("request.txt"), request).unwrap();
    assert!(CaseContext::load(context.directory, inherited_spec()).is_err());
}
