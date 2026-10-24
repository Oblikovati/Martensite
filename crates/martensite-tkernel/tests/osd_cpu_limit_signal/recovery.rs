//! A temporary descriptor shortage must not disable later process limits (#278).
use super::{CHILD_MODE, wait_before_deadline};
use martensite_tkernel::osd::{reported_process_cpu_limit, set_process_cpu_limit};
use rustix::process::{Resource, Rlimit, getrlimit, setrlimit};
use std::fs::File;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[test]
fn exhausted_descriptors_allow_later_budget_initialization() {
    if std::env::var_os(CHILD_MODE).is_some() {
        run_descriptor_recovery();
        return;
    }
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "recovery::exhausted_descriptors_allow_later_budget_initialization",
            "--nocapture",
        ])
        .env(CHILD_MODE, "recovery")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    wait_before_deadline(&mut child, Instant::now() + Duration::from_secs(12));
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{output:?}");
}

fn run_descriptor_recovery() {
    let before = getrlimit(Resource::Cpu);
    let handles = exhaust_child_descriptors();
    let cause = set_process_cpu_limit(Some(200)).unwrap_err();
    assert_eq!(cause.raw_os_error(), Some(libc::EMFILE));
    assert_eq!(reported_process_cpu_limit(), 200);
    assert_eq!(getrlimit(Resource::Cpu), before);
    drop(handles);
    set_process_cpu_limit(Some(201)).unwrap();
    assert_eq!(reported_process_cpu_limit(), 201);
    assert_eq!(getrlimit(Resource::Cpu).current, Some(201));
    set_process_cpu_limit(None).unwrap();
    assert_eq!(reported_process_cpu_limit(), -1);
}

fn exhaust_child_descriptors() -> Vec<File> {
    bound_child_descriptor_count();
    let mut handles = Vec::new();
    loop {
        match File::open("/dev/null") {
            Ok(file) => handles.push(file),
            Err(cause) => {
                assert_eq!(cause.raw_os_error(), Some(libc::EMFILE));
                return handles;
            }
        }
    }
}

fn bound_child_descriptor_count() {
    let original = getrlimit(Resource::Nofile);
    setrlimit(
        Resource::Nofile,
        Rlimit {
            current: Some(64),
            maximum: original.maximum,
        },
    )
    .unwrap();
}
