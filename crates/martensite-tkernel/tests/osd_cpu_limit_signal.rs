//! The CPU exit action must survive a closed output pipe without changing global SIGPIPE.
#![cfg(unix)]
#![allow(unsafe_code)]
use martensite_tkernel::osd::set_process_cpu_limit;
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc::{Sender, channel};
use std::time::{Duration, Instant};

const CHILD_MODE: &str = "MARTENSITE_CPU_PIPE_CHILD";

#[test]
fn closed_stdout_keeps_cpu_budget_exit_code() {
    if std::env::var_os(CHILD_MODE).is_some() {
        run_pipe_child();
        return;
    }
    let deadline = Instant::now() + Duration::from_secs(12);
    let mut child = spawn_pipe_child();
    close_stdout_after_ready(&mut child, deadline);
    child.stdin.take().unwrap().write_all(b"1").unwrap();
    wait_before_deadline(&mut child, deadline);
    let output = child.wait_with_output().unwrap();
    assert_eq!(output.status.code(), Some(2), "{output:?}");
}

fn run_pipe_child() {
    reset_pipe_action();
    set_process_cpu_limit(Some(200)).unwrap();
    assert_eq!(reset_pipe_action(), libc::SIG_DFL);
    set_process_cpu_limit(None).unwrap();
    assert_eq!(reset_pipe_action(), libc::SIG_DFL);
    println!("budget-ready");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_exact(&mut [0]).unwrap();
    set_process_cpu_limit(Some(1)).unwrap();
    let mut accumulator = 1_u64;
    loop {
        accumulator = std::hint::black_box(accumulator.wrapping_mul(3).wrapping_add(1));
    }
}

fn reset_pipe_action() -> libc::sighandler_t {
    // SAFETY: only the explicitly selected fresh child calls this OS control;
    // SIGPIPE and SIG_DFL are valid, and no parent test signal action is touched.
    unsafe { libc::signal(libc::SIGPIPE, libc::SIG_DFL) }
}

fn spawn_pipe_child() -> Child {
    Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "closed_stdout_keeps_cpu_budget_exit_code",
            "--nocapture",
        ])
        .env(CHILD_MODE, "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap()
}

fn close_stdout_after_ready(child: &mut Child, deadline: Instant) {
    let stdout = child.stdout.take().unwrap();
    let (send, receive) = channel();
    let reader = std::thread::spawn(move || observe_readiness(stdout, send));
    if receive
        .recv_timeout(deadline.saturating_duration_since(Instant::now()))
        .is_err()
    {
        terminate_and_reap(child);
        reader.join().unwrap();
        panic!("pipe child: expected readiness inside 12-second safety guard");
    }
    reader.join().unwrap();
}

fn observe_readiness(stdout: impl Read, send: Sender<()>) {
    let mut reader = BufReader::new(stdout);
    let mut line = String::new();
    while reader.read_line(&mut line).unwrap() != 0 {
        if line.ends_with("budget-ready\n") {
            drop(reader); // Close the last pipe reader before acknowledging readiness.
            send.send(()).unwrap();
            return;
        }
        line.clear();
    }
}

fn wait_before_deadline(child: &mut Child, deadline: Instant) {
    while child.try_wait().unwrap().is_none() {
        if Instant::now() >= deadline {
            terminate_and_reap(child);
            panic!("pipe child: expected termination inside 12-second safety guard");
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn terminate_and_reap(child: &mut Child) {
    // The child can exit between the last poll and kill; reap either outcome.
    let _already_exited = child.kill();
    child.wait().unwrap();
}

#[test]
fn forced_child_shutdown_is_reaped() {
    let mut child = spawn_pipe_child();
    terminate_and_reap(&mut child);
    assert!(child.try_wait().unwrap().is_some());
}

#[path = "osd_cpu_limit_signal/backpressure.rs"]
mod backpressure;

#[path = "osd_cpu_limit_signal/recovery.rs"]
mod recovery;
