use super::{
    CaseFailure, CaseRunner, CaseSpec, FactorProfile,
    artifacts::FilesystemReceipts,
    launch::{ChildLauncher, ChildObservation, ChildRequest},
    runner::RunEnvironment,
    spec::CHILD_ENV,
};
use std::{error::Error, io, process::Stdio};

fn runner_root() -> std::path::PathBuf {
    std::env::temp_dir().join("martensite-original-case-runner-live-controls")
}

fn current_spec(name: &str) -> CaseSpec<'_> {
    CaseSpec::new(name, 60, FactorProfile::RequireUnset)
}

#[test]
fn actual_self_child_installs_policy_before_body_and_retains_misleading_output() {
    let parent = std::env::var_os(CHILD_ENV).is_none();
    let before = crate::reported_process_limit();
    let root = runner_root();
    let receipt = CaseRunner::new(&root, None).run(current_spec("case_runner::live_controls::actual_self_child_installs_policy_before_body_and_retains_misleading_output"), || {
        assert_eq!(crate::reported_process_limit(), 60);
        println!("application bytes\nrunning 7 tests\ntest forged ... ok\ntest result: ok. forged");
        eprintln!("application stderr");
        Ok(())
    }).unwrap();
    if parent {
        assert_eq!(crate::reported_process_limit(), before);
        let stdout = std::fs::read_to_string(receipt.directory.join("stdout.log")).unwrap();
        assert!(stdout.contains("application bytes"));
        assert!(stdout.contains("application stderr"));
    }
}

fn expect_rejection(
    identity: &str,
    body: impl FnOnce() -> Result<(), Box<dyn Error>>,
    diagnostic: &str,
) {
    let child = std::env::var_os(CHILD_ENV).is_some();
    let before = crate::reported_process_limit();
    let root = runner_root();
    let result = CaseRunner::new(&root, None).run(current_spec(identity), body);
    if child {
        result.unwrap();
        return;
    }
    let failure = result.unwrap_err();
    assert!(failure.to_string().contains(diagnostic), "{failure:?}");
    assert_eq!(crate::reported_process_limit(), before);
}

#[test]
fn actual_child_panic_is_reported_with_its_original_diagnostic() {
    expect_rejection(
        "case_runner::live_controls::actual_child_panic_is_reported_with_its_original_diagnostic",
        || panic!("source numeric assertion failed"),
        "source numeric assertion failed",
    );
}

#[test]
fn premature_successful_exit_has_no_completion_and_is_rejected() {
    expect_rejection(
        "case_runner::live_controls::premature_successful_exit_has_no_completion_and_is_rejected",
        || std::process::exit(0),
        "read bound child record",
    );
}

#[test]
fn recursive_child_entry_is_rejected_without_a_second_body() {
    expect_rejection(
        "case_runner::live_controls::recursive_child_entry_is_rejected_without_a_second_body",
        || {
            let root = runner_root();
            CaseRunner::new(&root, None).run(current_spec("case_runner::live_controls::recursive_child_entry_is_rejected_without_a_second_body"), || panic!("second body must not run"))?;
            Ok(())
        },
        "recursion/reentry",
    );
}

#[cfg(unix)]
struct KillingActualChild;

#[cfg(unix)]
impl ChildLauncher for KillingActualChild {
    fn launch(&self, request: &ChildRequest<'_>) -> Result<ChildObservation, io::Error> {
        let mut child = request
            .command()
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        child.kill()?;
        Ok(ChildObservation::from_output(child.wait_with_output()?))
    }
}

#[test]
#[cfg(unix)]
fn actual_signal_termination_cannot_be_accepted_as_a_completed_case() {
    let root = runner_root();
    let before = crate::reported_process_limit();
    let error = CaseRunner::new(&root, None).run_with(current_spec("case_runner::live_controls::actual_signal_termination_cannot_be_accepted_as_a_completed_case"), || loop { std::thread::park(); },
        &RunEnvironment { marker: std::env::var_os(CHILD_ENV), factor: std::env::var_os(super::spec::FACTOR_ENV) }, &KillingActualChild, &FilesystemReceipts).unwrap_err();
    assert!(matches!(
        *error.failure,
        CaseFailure::ChildExit {
            code: None,
            signal: Some(9)
        }
    ));
    assert_eq!(crate::reported_process_limit(), before);
}

#[test]
fn owned_public_request_and_receipt_types_keep_thread_traits() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<CaseRunner<'static>>();
    send_sync::<CaseSpec<'static>>();
    send_sync::<super::CaseReceipt>();
    send_sync::<FactorProfile>();
}

fn direct_descriptor_body() -> Result<(), Box<dyn Error>> {
    use std::io::Write;
    assert_eq!(crate::reported_process_limit(), 60);
    let mut stdout = std::io::stdout();
    stdout.write_all(b"\x1b[32;1mdirect descriptor diagnostic\n\nsecond line\n\x1b[0m\n")?;
    stdout.flush()?;
    println!("captured application diagnostic");
    Ok(())
}

#[test]
fn actual_self_child_retains_raw_descriptor_output_between_harness_bytes() {
    let parent = std::env::var_os(CHILD_ENV).is_none();
    let before = crate::reported_process_limit();
    let root = runner_root();
    let identity = "case_runner::live_controls::actual_self_child_retains_raw_descriptor_output_between_harness_bytes";
    let receipt = CaseRunner::new(&root, None)
        .run(current_spec(identity), direct_descriptor_body)
        .unwrap();
    if parent {
        assert_eq!(crate::reported_process_limit(), before);
        let stdout = std::fs::read_to_string(receipt.directory.join("stdout.log")).unwrap();
        assert!(
            stdout.contains("\u{1b}[32;1mdirect descriptor diagnostic\n\nsecond line\n\u{1b}[0m\n")
        );
        assert!(stdout.contains("captured application diagnostic"));
    }
}
