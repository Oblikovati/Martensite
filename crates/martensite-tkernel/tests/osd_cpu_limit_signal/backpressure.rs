//! Full live output must never hold a budget-expired process alive (#278).
use super::{CHILD_MODE, observe_readiness, terminate_and_reap, wait_before_deadline};
use martensite_tkernel::osd::set_process_cpu_limit;
use rustix::fs::{OFlags, fcntl_getfl, fcntl_setfl};
use std::io::{PipeWriter, Read, Write, pipe};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

#[test]
fn full_stdout_keeps_cpu_budget_exit_code() {
    if std::env::var_os(CHILD_MODE).is_some() {
        run_full_pipe_child();
        return;
    }
    let deadline = Instant::now() + Duration::from_secs(12);
    let (_reader, writer) = pipe().unwrap();
    let mut child = spawn_full_pipe_child(writer.try_clone().unwrap());
    wait_for_full_pipe(&mut child, deadline);
    // Darwin records FWASWRITTEN during intentional setup writes (#289).
    // Compare the same prepared descriptor state across expiry, without masking flags.
    let flags = fcntl_getfl(&writer).unwrap();
    child.stdin.take().unwrap().write_all(b"1").unwrap();
    wait_before_deadline(&mut child, deadline);
    assert_eq!(fcntl_getfl(&writer).unwrap(), flags);
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(2), "{output:?}");
}

fn run_full_pipe_child() {
    let before_install = fcntl_getfl(std::io::stdout()).unwrap();
    set_process_cpu_limit(Some(200)).unwrap();
    assert_eq!(fcntl_getfl(std::io::stdout()).unwrap(), before_install);
    let original_flags = fill_stdout_to_capacity();
    eprintln!("budget-ready");
    std::io::stderr().flush().unwrap();
    std::io::stdin().read_exact(&mut [0]).unwrap();
    set_process_cpu_limit(Some(1)).unwrap();
    assert_eq!(fcntl_getfl(std::io::stdout()).unwrap(), original_flags);
    let mut accumulator = 1_u64;
    loop {
        accumulator = std::hint::black_box(accumulator.wrapping_mul(3).wrapping_add(1));
    }
}

fn fill_stdout_to_capacity() -> OFlags {
    let stdout = std::io::stdout();
    let original_flags = fcntl_getfl(&stdout).unwrap();
    assert!(!original_flags.contains(OFlags::NONBLOCK));
    fcntl_setfl(&stdout, original_flags | OFlags::NONBLOCK).unwrap();
    exhaust_stdout(&stdout, &[b'x'; 4096]);
    exhaust_stdout(&stdout, b"x");
    fcntl_setfl(&stdout, original_flags).unwrap();
    assert_eq!(fcntl_getfl(&stdout).unwrap(), original_flags);
    original_flags
}

fn exhaust_stdout(stdout: &std::io::Stdout, bytes: &[u8]) {
    loop {
        match rustix::io::write(stdout, bytes) {
            Ok(count) => assert_ne!(count, 0),
            Err(rustix::io::Errno::AGAIN) => return,
            Err(cause) => panic!("stdout fill: expected capacity, got {cause}"),
        }
    }
}

fn spawn_full_pipe_child(writer: PipeWriter) -> Child {
    Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "backpressure::full_stdout_keeps_cpu_budget_exit_code",
            "--nocapture",
        ])
        .env(CHILD_MODE, "full")
        .stdin(Stdio::piped())
        .stdout(Stdio::from(writer))
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
}

fn wait_for_full_pipe(child: &mut Child, deadline: Instant) {
    let stderr = child.stderr.take().unwrap();
    let (send, receive) = channel();
    let reader = std::thread::spawn(move || observe_readiness(stderr, send));
    if receive
        .recv_timeout(deadline.saturating_duration_since(Instant::now()))
        .is_err()
    {
        terminate_and_reap(child);
        reader.join().unwrap();
        panic!("full pipe child: expected readiness inside 12-second safety guard");
    }
    reader.join().unwrap();
}
