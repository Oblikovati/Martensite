use super::{CaseRunner, CaseSpec, FactorProfile, spec::CHILD_ENV};
use std::{cell::Cell, error::Error, io, path::PathBuf, rc::Rc};

fn receipt_root() -> PathBuf {
    std::env::temp_dir().join("martensite-prepared-case-runner-live-controls")
}

fn prepare_thread_confined_owner() -> Result<Rc<Cell<u32>>, Box<dyn Error>> {
    assert_eq!(crate::reported_process_limit(), 0);
    println!("prepared owner before policy");
    Ok(Rc::new(Cell::new(41)))
}

fn consume_thread_confined_owner(owned: Rc<Cell<u32>>) -> Result<(), Box<dyn Error>> {
    assert_eq!(crate::reported_process_limit(), 60);
    assert_eq!(owned.replace(42), 41);
    println!("consumed prepared owner after policy");
    Ok(())
}

#[test]
fn actual_prepared_child_orders_policy_and_keeps_parent_unchanged() {
    let before = crate::reported_process_limit();
    let root = receipt_root();
    let spec = CaseSpec::new(
        "case_runner::prepared_live_controls::actual_prepared_child_orders_policy_and_keeps_parent_unchanged",
        60,
        FactorProfile::RequireUnset,
    );
    let receipt = CaseRunner::new(&root, None)
        .run_prepared(
            spec,
            prepare_thread_confined_owner,
            consume_thread_confined_owner,
        )
        .unwrap();
    verify_parent_receipt(receipt, before);
}

fn verify_parent_receipt(receipt: super::CaseReceipt, before: i64) {
    if std::env::var_os(CHILD_ENV).is_some() {
        return;
    }
    assert_eq!(crate::reported_process_limit(), before);
    let output = std::fs::read_to_string(receipt.directory.join("stdout.log")).unwrap();
    assert!(output.contains("prepared owner before policy"));
    assert!(output.contains("consumed prepared owner after policy"));
}

fn named_preparation_refusal() -> Result<Rc<Cell<u32>>, Box<dyn Error>> {
    assert_eq!(crate::reported_process_limit(), 0);
    Err(Box::new(io::Error::new(
        io::ErrorKind::InvalidInput,
        "original prepared-state failure",
    )))
}

#[test]
fn actual_preparation_error_is_reported_without_body_or_installed_policy() {
    let before = crate::reported_process_limit();
    let root = receipt_root();
    let spec = CaseSpec::new(
        "case_runner::prepared_live_controls::actual_preparation_error_is_reported_without_body_or_installed_policy",
        60,
        FactorProfile::RequireUnset,
    );
    let result = CaseRunner::new(&root, None).run_prepared(spec, named_preparation_refusal, |_| {
        panic!("body must not run")
    });
    verify_preparation_failure(result, before);
}

fn verify_preparation_failure(
    result: Result<super::CaseReceipt, super::CaseRunError>,
    before: i64,
) {
    if std::env::var_os(CHILD_ENV).is_some() {
        assert_eq!(crate::reported_process_limit(), 0);
        result.unwrap();
        return;
    }
    let failure = result.unwrap_err();
    assert!(
        failure
            .to_string()
            .contains("original prepared-state failure")
    );
    assert_eq!(crate::reported_process_limit(), before);
}
