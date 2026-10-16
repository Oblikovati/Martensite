//! Test double for [`CommandRunner`]: records every command and answers with
//! scripted stdout or exit codes keyed by the command line; every command
//! succeeds unless an exit code says otherwise.

use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::error::BuildResult;
use crate::process::{CommandOutput, CommandRunner, CommandSpec};

/// Records commands; replies from scripted tables, defaulting to success.
#[derive(Default)]
pub struct RecordingRunner {
    stdout: BTreeMap<String, String>,
    stderr: BTreeMap<String, String>,
    exit_codes: BTreeMap<String, i32>,
    recorded: RefCell<Vec<String>>,
}

impl RecordingRunner {
    /// Scripts the stdout returned for an exact command line.
    pub fn with_stdout(mut self, command: &str, stdout: &str) -> Self {
        self.stdout.insert(command.to_owned(), stdout.to_owned());
        self
    }

    /// Scripts the exit code returned by `exit_code` for a command line.
    pub fn with_exit_code(mut self, command: &str, code: i32) -> Self {
        self.exit_codes.insert(command.to_owned(), code);
        self
    }

    /// Scripts diagnostics for a captured completed command.
    pub fn with_stderr(mut self, command: &str, stderr: &str) -> Self {
        self.stderr.insert(command.to_owned(), stderr.to_owned());
        self
    }

    /// Every command line seen so far, in call order.
    pub fn commands(&self) -> Vec<String> {
        self.recorded.borrow().clone()
    }

    fn record(&self, spec: &CommandSpec) -> String {
        let line = spec.display();
        self.recorded.borrow_mut().push(line.clone());
        line
    }
}

impl CommandRunner for RecordingRunner {
    fn run(&self, spec: &CommandSpec) -> BuildResult<()> {
        self.record(spec);
        Ok(())
    }

    fn capture(&self, spec: &CommandSpec) -> BuildResult<String> {
        let line = self.record(spec);
        Ok(self.stdout.get(&line).cloned().unwrap_or_default())
    }

    fn exit_code(&self, spec: &CommandSpec) -> BuildResult<i32> {
        let line = self.record(spec);
        Ok(self.exit_codes.get(&line).copied().unwrap_or(0))
    }

    fn capture_output(&self, spec: &CommandSpec) -> BuildResult<CommandOutput> {
        let line = self.record(spec);
        let stdout = self.stdout.get(&line).cloned().unwrap_or_default();
        let stderr = self.stderr.get(&line).cloned().unwrap_or_default();
        Ok(CommandOutput {
            code: self.exit_codes.get(&line).copied().unwrap_or(0),
            stdout: stdout.into_bytes(),
            stderr: stderr.into_bytes(),
        })
    }
}
