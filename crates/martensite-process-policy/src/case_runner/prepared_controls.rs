use super::{
    CaseFailure,
    child_policy::execute_prepared_with,
    context::CaseContext,
    fake_controls::OwnedArtifactRoot,
    lifecycle::ChildLifecycle,
    prepared_control_support::{Events, OrderedPolicy, PreparedOwner, assert_errno, prepare, spec},
};
use std::{error::Error, io};

fn context(root: &OwnedArtifactRoot) -> CaseContext {
    CaseContext::create_for(root.path(), spec(), ChildLifecycle::Prepared).unwrap()
}

fn policy<'a>(context: &'a CaseContext, events: &Events, refuse: bool) -> OrderedPolicy<'a> {
    OrderedPolicy {
        context,
        events: events.clone(),
        refuse,
    }
}

fn assert_not_completed(context: &CaseContext) {
    context.require_record("entry.txt").unwrap();
    assert!(!context.directory.join("complete.txt").exists());
}

#[test]
fn prepared_owner_moves_once_after_policy_and_drops_before_completion() {
    let root = OwnedArtifactRoot::new();
    let context = context(&root);
    let events = Events::default();
    execute_prepared_with(
        &context,
        spec(),
        || prepare(&events),
        |owned| successful_body(&context, owned),
        &policy(&context, &events, false),
    )
    .unwrap();
    assert_eq!(*events.borrow(), ["prepare", "policy", "body", "drop"]);
    context.require_record("prepared.txt").unwrap();
    context.require_record("complete.txt").unwrap();
}

fn successful_body(context: &CaseContext, owned: PreparedOwner) -> Result<(), Box<dyn Error>> {
    assert_eq!(*owned.events.borrow(), ["prepare", "policy"]);
    assert!(!context.directory.join("complete.txt").exists());
    owned.events.borrow_mut().push("body");
    Ok(())
}

fn refusing_preparation(events: &Events) -> Result<PreparedOwner, Box<dyn Error>> {
    events.borrow_mut().push("prepare");
    Err(Box::new(io::Error::from_raw_os_error(libc::EIO)))
}

#[test]
fn preparation_error_retains_typed_cause_without_policy_body_or_completion() {
    let root = OwnedArtifactRoot::new();
    let context = context(&root);
    let events = Events::default();
    let error = execute_prepared_with(
        &context,
        spec(),
        || refusing_preparation(&events),
        |_| panic!("body must not run"),
        &policy(&context, &events, false),
    )
    .unwrap_err();
    assert!(matches!(error, CaseFailure::Preparation(_)));
    assert_errno(&error, libc::EIO);
    assert_eq!(*events.borrow(), ["prepare"]);
    assert!(!context.directory.join("prepared.txt").exists());
    assert_not_completed(&context);
}

#[test]
fn policy_failure_drops_prepared_owner_and_retains_its_original_cause() {
    let root = OwnedArtifactRoot::new();
    let context = context(&root);
    let events = Events::default();
    let error = execute_prepared_with(
        &context,
        spec(),
        || prepare(&events),
        |_| panic!("body must not run"),
        &policy(&context, &events, true),
    )
    .unwrap_err();
    assert!(matches!(error, CaseFailure::Policy(_)));
    assert_errno(&error, libc::EPERM);
    assert_eq!(*events.borrow(), ["prepare", "policy", "drop"]);
    context.require_record("prepared.txt").unwrap();
    assert_not_completed(&context);
}

fn refusing_body(owned: PreparedOwner) -> Result<(), Box<dyn Error>> {
    owned.events.borrow_mut().push("body");
    Err(Box::new(io::Error::from_raw_os_error(libc::EINVAL)))
}

#[test]
fn body_failure_drops_owner_and_never_publishes_completion() {
    let root = OwnedArtifactRoot::new();
    let context = context(&root);
    let events = Events::default();
    let error = execute_prepared_with(
        &context,
        spec(),
        || prepare(&events),
        refusing_body,
        &policy(&context, &events, false),
    )
    .unwrap_err();
    assert!(matches!(error, CaseFailure::Body(_)));
    assert_errno(&error, libc::EINVAL);
    assert_eq!(*events.borrow(), ["prepare", "policy", "body", "drop"]);
    assert_not_completed(&context);
}

#[test]
fn prepared_record_refusal_drops_owner_before_installing_policy() {
    let root = OwnedArtifactRoot::new();
    let context = context(&root);
    let events = Events::default();
    std::fs::write(context.directory.join("prepared.txt"), b"already claimed").unwrap();
    let error = execute_prepared_with(
        &context,
        spec(),
        || prepare(&events),
        |_| panic!("body must not run"),
        &policy(&context, &events, false),
    )
    .unwrap_err();
    assert_record_refused(&context, &events, &error);
}

fn assert_record_refused(context: &CaseContext, events: &Events, error: &CaseFailure) {
    assert!(matches!(error, CaseFailure::Io { .. }));
    let cause = error.source().unwrap().downcast_ref::<io::Error>().unwrap();
    assert_eq!(cause.kind(), io::ErrorKind::AlreadyExists);
    assert_eq!(*events.borrow(), ["prepare", "drop"]);
    assert_not_completed(context);
}

#[test]
fn lifecycle_is_bound_and_legacy_direct_request_bytes_remain_unchanged() {
    let root = OwnedArtifactRoot::new();
    let prepared = context(&root);
    let direct = CaseContext::create(root.path(), spec()).unwrap();
    assert!(CaseContext::load(prepared.directory.clone(), spec()).is_err());
    assert!(
        CaseContext::load_for(direct.directory.clone(), spec(), ChildLifecycle::Prepared).is_err()
    );
    let suffix = ChildLifecycle::Prepared.request_suffix().as_bytes();
    assert!(prepared.request.ends_with(suffix));
    assert!(!String::from_utf8_lossy(&direct.request).contains("lifecycle="));
    let expected = legacy_request(&direct);
    assert_eq!(direct.request, expected);
}

fn legacy_request(context: &CaseContext) -> Vec<u8> {
    format!("MARTENSITE_EXACT_CASE_V1\nidentity={}\nseconds=60\nprofile=rust-1.99.0-pretty-show-output\nfactor=unset\nexecutable={}\ninvocation={}\n", super::context::hex(spec().identity.as_bytes()), context.fingerprint, super::context::hex(context.directory.as_os_str().as_encoded_bytes())).into_bytes()
}
