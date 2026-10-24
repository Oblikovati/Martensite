//! Dedicated children exercise process-global budgets without mutating the test harness.
use martensite_process_policy::{reported_process_limit, set_process_limit};
use martensite_tkernel::osd::{CpuTimes, NativeCpuTimes};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};

const CHILD_MODE: &str = "MARTENSITE_BUDGET_CHILD";

fn budget_child(test: &str, mode: &str) -> Output {
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", test, "--nocapture"])
        .env(CHILD_MODE, mode)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    await_child(&mut child, mode);
    child.wait_with_output().unwrap()
}

fn await_child(child: &mut Child, mode: &str) {
    let deadline = Instant::now() + Duration::from_secs(12);
    while child.try_wait().unwrap().is_none() {
        if Instant::now() >= deadline {
            child.kill().unwrap();
            panic!("child {mode}: expected termination within 12-second safety guard");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn exhaust_cpu() {
    let mut accumulator = 1_u64;
    loop {
        accumulator = std::hint::black_box(accumulator.wrapping_mul(3).wrapping_add(1));
    }
}

fn expected_termination(test: &str, mode: &str, reason: &str) {
    if std::env::var(CHILD_MODE).as_deref() == Ok(mode) {
        run_expiring_child(mode);
        return;
    }
    let child = budget_child(test, mode);
    assert_eq!(child.status.code(), Some(2), "{mode}: {child:?}");
    let diagnostic = format!("Process killed by {reason} limit  (1 sec)");
    assert!(
        String::from_utf8(child.stdout)
            .unwrap()
            .contains(&diagnostic)
    );
}

fn run_expiring_child(mode: &str) {
    if mode == "late" {
        consume_before_installation();
    }
    set_process_limit(Some(1)).unwrap();
    if mode == "elapsed" {
        std::thread::sleep(Duration::from_secs(20));
        panic!("elapsed budget failed to terminate child");
    }
    if mode == "workers" {
        for _ in 0..3 {
            std::thread::spawn(exhaust_cpu);
        }
    }
    exhaust_cpu();
}

fn consume_before_installation() {
    loop {
        let (user, system) = NativeCpuTimes.process_cpu();
        if user + system >= 1.1 {
            return;
        }
        std::hint::black_box((user + system).sqrt());
    }
}

#[cfg(unix)]
#[test]
fn cpu_budget_reports_diagnostic_and_exit_code() {
    expected_termination("cpu_budget_reports_diagnostic_and_exit_code", "cpu", "CPU");
}

#[cfg(unix)]
#[test]
fn cpu_budget_on_workers_uses_the_same_process_contract() {
    expected_termination(
        "cpu_budget_on_workers_uses_the_same_process_contract",
        "workers",
        "CPU",
    );
}

#[cfg(unix)]
#[test]
fn already_consumed_cpu_cannot_expire_before_action_installation() {
    expected_termination(
        "already_consumed_cpu_cannot_expire_before_action_installation",
        "late",
        "CPU",
    );
}

#[test]
fn elapsed_budget_reports_diagnostic_and_exit_code() {
    expected_termination(
        "elapsed_budget_reports_diagnostic_and_exit_code",
        "elapsed",
        "elapsed",
    );
}

#[test]
fn removing_a_budget_preserves_unlimited_query_state() {
    if std::env::var(CHILD_MODE).as_deref() == Ok("reset") {
        set_process_limit(Some(200)).unwrap();
        assert_eq!(reported_process_limit(), 200);
        set_process_limit(None).unwrap();
        assert_eq!(reported_process_limit(), -1);
        return;
    }
    assert!(
        budget_child("removing_a_budget_preserves_unlimited_query_state", "reset")
            .status
            .success()
    );
}

#[cfg(unix)]
#[test]
fn concurrent_setters_leave_reported_and_effective_caps_coherent() {
    if std::env::var(CHILD_MODE).as_deref() == Ok("setters") {
        check_concurrent_setters();
        return;
    }
    let child = budget_child(
        "concurrent_setters_leave_reported_and_effective_caps_coherent",
        "setters",
    );
    assert!(child.status.success(), "{child:?}");
}

#[cfg(unix)]
fn check_concurrent_setters() {
    let workers = (0..8)
        .map(|worker| {
            std::thread::spawn(move || {
                for request in 0..16 {
                    set_process_limit(Some(200 + worker * 16 + request)).unwrap();
                }
            })
        })
        .collect::<Vec<_>>();
    for worker in workers {
        worker.join().unwrap();
    }
    let installed = rustix::process::getrlimit(rustix::process::Resource::Cpu);
    assert_eq!(Some(reported_process_limit() as u64), installed.current);
    set_process_limit(None).unwrap();
}

#[cfg(unix)]
#[test]
fn refused_os_cap_retains_request_and_typed_cause() {
    if std::env::var(CHILD_MODE).as_deref() == Ok("refusal") {
        check_refused_cap();
        return;
    }
    let child = budget_child("refused_os_cap_retains_request_and_typed_cause", "refusal");
    assert!(child.status.success(), "{child:?}");
}

#[cfg(unix)]
fn check_refused_cap() {
    use rustix::process::{Resource, Rlimit, getrlimit, setrlimit};
    setrlimit(
        Resource::Cpu,
        Rlimit {
            current: Some(2),
            maximum: Some(2),
        },
    )
    .unwrap();
    let error = set_process_limit(Some(200)).unwrap_err();
    assert_eq!(reported_process_limit(), 200);
    assert_eq!(getrlimit(Resource::Cpu).current, Some(2));
    assert_refused_cause(&error);
}

#[cfg(unix)]
fn assert_refused_cause(error: &martensite_process_policy::ProcessLimitError) {
    assert_ne!(error.raw_status(), 0);
    let cause = std::error::Error::source(&error).unwrap();
    assert!(
        cause
            .downcast_ref::<std::io::Error>()
            .unwrap()
            .raw_os_error()
            .is_some()
    );
    assert!(error.to_string().contains("Some(200) seconds"));
}
