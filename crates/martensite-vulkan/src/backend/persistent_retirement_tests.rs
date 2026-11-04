//! CPU-only limitation witness: an abandoned unknown wait stays alive instead of releasing its fake owner.
use super::device_retirement::{CompletionDispatch, retire_until_stopped, retirement_retry_pause};
use crate::RenderError;
use std::{
    io::{BufRead, Write},
    process::{Child, Command, Stdio},
    sync::mpsc,
    thread,
    time::Duration,
};
const CHILD_SELECTOR: &str =
    "backend::persistent_retirement_tests::persistent_unknown_fixture_entry";
const CHILD_MARKER: &str = "MARTENSITE_RESOURCE_PERSISTENT_WAIT_CHILD";
struct PersistentUnknownDispatch;
impl CompletionDispatch for PersistentUnknownDispatch {
    fn wait_completion(&mut self) -> Result<(), RenderError> {
        // This is a named synthetic observation, not a failed real driver or GPU.
        Err(RenderError::device_backend(
            "synthetic persistent wait",
            -1,
            "injected unknown completion".into(),
        ))
    }
    fn retry_pause(&mut self) {
        emit_marker(b"{\"event\":\"pending-fake-owner\"}\n");
        retirement_retry_pause();
    }
}
struct FakePendingOwner;
impl Drop for FakePendingOwner {
    fn drop(&mut self) {
        // An early return from the production wait loop must make the parent control fail.
        emit_marker(b"{\"event\":\"destroyed-fake-owner\"}\n");
    }
}
fn emit_marker(marker: &[u8]) {
    let mut output = std::io::stdout().lock();
    output.write_all(marker).unwrap();
    output.flush().unwrap();
}
#[test]
fn persistent_unknown_fixture_entry() {
    // This registered subprocess entry is not credited as an independent passing shutdown test.
    if std::env::var_os(CHILD_MARKER).is_none() {
        return;
    }
    let _owner = FakePendingOwner;
    let _outcome = retire_until_stopped(&mut PersistentUnknownDispatch);
}
struct PersistentWaitChild {
    child: Child,
    lines: mpsc::Receiver<String>,
    reader: Option<thread::JoinHandle<()>>,
    trace: Vec<String>,
}
impl PersistentWaitChild {
    fn start() -> Self {
        let (sender, lines) = mpsc::channel();
        let child = persistent_child_command().spawn().unwrap();
        let mut owned = Self {
            child,
            lines,
            reader: None,
            trace: Vec::new(),
        };
        let stdout = owned
            .child
            .stdout
            .take()
            .expect("a configured piped child stdout");
        owned.reader = Some(thread::spawn(move || forward_child_lines(stdout, sender)));
        owned
    }
    fn await_pending(&mut self) {
        loop {
            let line = self
                .lines
                .recv_timeout(Duration::from_secs(5))
                .expect("a live named wait-loop observation within five seconds");
            let pending = line.contains("pending-fake-owner");
            self.trace.push(line);
            if pending {
                return;
            }
        }
    }
    fn assert_alive_without_release(&mut self) {
        thread::sleep(Duration::from_millis(30));
        assert!(
            self.child.try_wait().unwrap().is_none(),
            "received terminated child; expected retained unresolved owner"
        );
        self.trace.extend(self.lines.try_iter());
        assert!(
            !self
                .trace
                .iter()
                .any(|line| line.contains("destroyed-fake-owner"))
        );
    }
    fn terminate_limit(&mut self) {
        self.child.kill().unwrap();
        let status = self.child.wait().unwrap();
        assert!(
            !status.success(),
            "external termination is not successful retirement"
        );
        self.collect_terminated_trace();
    }
    fn collect_terminated_trace(&mut self) {
        self.reader.take().unwrap().join().unwrap();
        self.trace.extend(self.lines.try_iter());
        assert!(
            !self
                .trace
                .iter()
                .any(|line| line.contains("destroyed-fake-owner"))
        );
        println!(
            "{{\"event\":\"persistent-unknown-limitation\",\"child\":{},\"shutdown_pass\":false,\"raw_stdout\":{:?}}}",
            self.child.id(),
            self.trace
        );
    }
}
fn forward_child_lines(stdout: std::process::ChildStdout, sender: mpsc::Sender<String>) {
    for line in std::io::BufReader::new(stdout)
        .lines()
        .map_while(Result::ok)
    {
        if sender.send(line).is_err() {
            return;
        }
    }
}
impl Drop for PersistentWaitChild {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
    }
}
#[test]
fn persistent_unknown_retirement_is_observed_as_a_live_non_shutdown_limit() {
    let mut fixture = PersistentWaitChild::start();
    fixture.await_pending();
    fixture.assert_alive_without_release();
    fixture.terminate_limit();
}

fn persistent_child_command() -> Command {
    let mut command = Command::new(std::env::current_exe().unwrap());
    command.args(["--exact", CHILD_SELECTOR, "--nocapture", "--test-threads=1"]);
    command
        .env(CHILD_MARKER, "fixture-only")
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());
    command
}
