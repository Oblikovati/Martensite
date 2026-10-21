use super::{
    CaseFailure, CaseSpec, FactorProfile,
    child_policy::{PolicyInstaller, execute_with},
    context::CaseContext,
    fake_controls::OwnedArtifactRoot,
};
use std::{cell::RefCell, error::Error, io};

struct RecordingChildPolicy<'a> {
    events: &'a RefCell<Vec<&'static str>>,
    refuse: bool,
}

impl PolicyInstaller for RecordingChildPolicy<'_> {
    fn install(&self, seconds: i64) -> Result<(), Box<dyn Error>> {
        assert_eq!(seconds, 60);
        self.events.borrow_mut().push("policy");
        if self.refuse {
            return Err(Box::new(io::Error::from_raw_os_error(libc::EPERM)));
        }
        Ok(())
    }
}

fn spec() -> CaseSpec<'static> {
    CaseSpec::new("controls::policy", 60, FactorProfile::RequireUnset)
}

fn assert_errno(error: &CaseFailure, expected: i32) {
    let cause = error.source().unwrap().downcast_ref::<io::Error>().unwrap();
    assert_eq!(cause.raw_os_error(), Some(expected));
}

fn policy<'a>(events: &'a RefCell<Vec<&'static str>>, refuse: bool) -> RecordingChildPolicy<'a> {
    RecordingChildPolicy { events, refuse }
}

fn successful_body(context: &CaseContext, events: &RefCell<Vec<&'static str>>) {
    assert_eq!(*events.borrow(), ["policy"]);
    assert!(!context.directory.join("complete.txt").exists());
    events.borrow_mut().push("body");
}

#[test]
fn installation_refusal_retains_typed_cause_and_prevents_body_completion() {
    let root = OwnedArtifactRoot::new();
    let context = CaseContext::create(root.path(), spec()).unwrap();
    let events = RefCell::new(Vec::new());
    let error = execute_with(
        &context,
        spec(),
        || panic!("fixture must not run"),
        &policy(&events, true),
    )
    .unwrap_err();
    assert!(matches!(error, CaseFailure::Policy(_)));
    assert_errno(&error, libc::EPERM);
    assert_eq!(*events.borrow(), ["policy"]);
    assert!(!context.directory.join("complete.txt").exists());
}

#[test]
fn successful_policy_precedes_body_and_completion_is_last() {
    let root = OwnedArtifactRoot::new();
    let context = CaseContext::create(root.path(), spec()).unwrap();
    let events = RefCell::new(Vec::new());
    execute_with(
        &context,
        spec(),
        || {
            successful_body(&context, &events);
            Ok(())
        },
        &policy(&events, false),
    )
    .unwrap();
    assert_eq!(*events.borrow(), ["policy", "body"]);
    context.require_record("complete.txt").unwrap();
}

#[test]
fn returned_body_error_remains_the_original_source_without_completion() {
    let root = OwnedArtifactRoot::new();
    let context = CaseContext::create(root.path(), spec()).unwrap();
    let events = RefCell::new(Vec::new());
    let error = execute_with(
        &context,
        spec(),
        || Err(Box::new(io::Error::from_raw_os_error(libc::EINVAL))),
        &policy(&events, false),
    )
    .unwrap_err();
    assert!(matches!(error, CaseFailure::Body(_)));
    assert_errno(&error, libc::EINVAL);
    assert!(!context.directory.join("complete.txt").exists());
}
