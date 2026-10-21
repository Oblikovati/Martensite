use super::{CaseSpec, context::CaseContext, spec::CHILD_ENV};
use std::{io, process::Command};

pub(crate) struct ChildRequest<'a> {
    pub(crate) context: &'a CaseContext,
    pub(crate) spec: CaseSpec<'a>,
}

impl ChildRequest<'_> {
    pub(crate) fn arguments(&self) -> Vec<String> {
        [
            "--exact",
            self.spec.identity,
            "--test-threads=1",
            "--format=pretty",
            "--color=never",
            "--show-output",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect()
    }

    pub(crate) fn command(&self) -> Command {
        let mut command = Command::new(&self.context.executable);
        command
            .args(self.arguments())
            .env(CHILD_ENV, &self.context.directory)
            .env_remove("RUST_TEST_NOCAPTURE");
        command
    }
}

#[derive(Debug)]
pub(crate) struct ChildObservation {
    pub(crate) code: Option<i32>,
    pub(crate) signal: Option<i32>,
    pub(crate) stdout: Vec<u8>,
    pub(crate) stderr: Vec<u8>,
}

impl ChildObservation {
    pub(crate) fn from_output(output: std::process::Output) -> Self {
        #[cfg(unix)]
        let signal = std::os::unix::process::ExitStatusExt::signal(&output.status);
        #[cfg(not(unix))]
        let signal = None;
        Self {
            code: output.status.code(),
            signal,
            stdout: output.stdout,
            stderr: output.stderr,
        }
    }
}

pub(crate) trait ChildLauncher {
    fn launch(&self, request: &ChildRequest<'_>) -> Result<ChildObservation, io::Error>;
}

pub(crate) struct ExecutableChild;

impl ChildLauncher for ExecutableChild {
    fn launch(&self, request: &ChildRequest<'_>) -> Result<ChildObservation, io::Error> {
        let output = request.command().output()?;
        Ok(ChildObservation::from_output(output))
    }
}
