use std::{error::Error, fmt, io, path::PathBuf};

/// The failure that prevented an exact case from being accepted.
///
/// Preparation, body and policy causes retain their original child-side error objects. Such
/// objects may be thread confined, so this development error makes no Send/Sync
/// promise. Parent failures retain process status and raw log paths instead of
/// pretending to reconstruct an error object across a process boundary.
/// Example: `matches!(failure, CaseFailure::Protocol { .. })`.
#[derive(Debug)]
pub enum CaseFailure {
    /// A declaration did not describe an executable supported case.
    InvalidSpec {
        /// The actual identity or budget supplied by the caller.
        observed: String,
        /// The supported declaration shape.
        expected: &'static str,
    },
    /// The actual factor environment is outside the explicitly unset profile.
    FactorPresent(std::ffi::OsString),
    /// A filesystem or process operation failed.
    Io {
        /// The filesystem or child-process action that failed.
        operation: &'static str,
        /// The original operating-system error, retained as the source.
        cause: io::Error,
    },
    /// Child identity, marker or libtest framing did not match the request.
    Protocol {
        /// The actual marker or harness content.
        observed: String,
        /// The required identity, budget or framing.
        expected: String,
    },
    /// A child did not exit successfully, including termination by signal.
    ChildExit {
        /// The normal exit code, absent when the child was signalled.
        code: Option<i32>,
        /// The observed Unix signal, absent for normal exits.
        signal: Option<i32>,
    },
    /// Preparing child-local state failed before policy installation.
    Preparation(Box<dyn Error>),
    /// Installing the child policy failed before the body received prepared state.
    Policy(Box<dyn Error>),
    /// The original body returned its own error in the child.
    Body(Box<dyn Error>),
}

impl fmt::Display for CaseFailure {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidSpec { observed, expected } => {
                write!(out, "value {observed:?}: expected {expected}")
            }
            Self::FactorPresent(value) => {
                write!(out, "CSF_CPULIMIT_FACTOR={value:?}: expected unset")
            }
            Self::Io { operation, cause } => write!(out, "{operation}: {cause}"),
            Self::Protocol { observed, expected } => {
                write!(out, "observed {observed:?}: expected {expected}")
            }
            Self::ChildExit { code, signal } => format_child_status(out, *code, *signal),
            Self::Preparation(cause) => write!(out, "child preparation: {cause}"),
            Self::Policy(cause) => write!(out, "child policy installation: {cause}"),
            Self::Body(cause) => write!(out, "original child body: {cause}"),
        }
    }
}

impl Error for CaseFailure {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { cause, .. } => Some(cause),
            Self::Preparation(cause) | Self::Policy(cause) | Self::Body(cause) => {
                Some(cause.as_ref())
            }
            _ => None,
        }
    }
}

/// Case, budget, primary cause and retained diagnostic locations.
///
/// Recorder failures are secondary when the child already failed. Child body
/// causes can be thread confined; this error is not promised Send or Sync.
/// Example: `eprintln!("{error}");` reports the case and its raw child log paths.
pub struct CaseRunError {
    pub(crate) identity: String,
    pub(crate) seconds: i64,
    pub(crate) failure: Box<CaseFailure>,
    pub(crate) artifacts: Option<PathBuf>,
    pub(crate) recording_failures: Vec<io::Error>,
    pub(crate) diagnostics: Option<Box<(Vec<u8>, Vec<u8>)>>,
}

impl CaseRunError {
    pub(crate) fn new(spec: super::CaseSpec<'_>, failure: CaseFailure) -> Self {
        Self {
            identity: spec.identity.into(),
            seconds: spec.seconds,
            failure: Box::new(failure),
            artifacts: None,
            recording_failures: Vec::new(),
            diagnostics: None,
        }
    }
}

impl fmt::Display for CaseRunError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            out,
            "case {:?}, budget {} seconds: {}; raw stdout/stderr and receipt directory {:?}",
            self.identity, self.seconds, self.failure, self.artifacts
        )?;
        for failure in &self.recording_failures {
            write!(out, "; secondary recorder failure: {failure}")?;
        }
        if let Some(diagnostics) = &self.diagnostics {
            let (stdout, stderr) = diagnostics.as_ref();
            write!(
                out,
                "\nraw child stdout:\n{}\nraw child stderr:\n{}",
                diagnostic_text(stdout),
                diagnostic_text(stderr)
            )?;
        }
        Ok(())
    }
}

impl fmt::Debug for CaseRunError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, out)
    }
}

fn diagnostic_text(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(text) => text.into(),
        Err(_) => format!("non-UTF-8 exact bytes: {}", super::context::hex(bytes)),
    }
}

impl Error for CaseRunError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.failure.as_ref())
    }
}

pub(crate) fn io_failure(operation: &'static str, cause: io::Error) -> CaseFailure {
    CaseFailure::Io { operation, cause }
}

fn format_child_status(
    out: &mut fmt::Formatter<'_>,
    code: Option<i32>,
    signal: Option<i32>,
) -> fmt::Result {
    write!(
        out,
        "child code={code:?}, signal={signal:?}: expected normal exit 0"
    )
}
