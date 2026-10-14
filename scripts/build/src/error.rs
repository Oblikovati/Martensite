//! Typed failures for every pipeline step, so CI logs name the command or
//! value that broke and the shape that was expected.

use std::fmt;
use std::io;
use std::path::PathBuf;

/// Result alias used across the build driver.
pub type BuildResult<T> = Result<T, BuildError>;

/// A failure of one build-driver step.
#[derive(Debug)]
pub enum BuildError {
    /// A child process could not be started.
    CommandSpawn {
        /// The command line as shown to the user.
        command: String,
        /// The operating-system cause.
        cause: io::Error,
    },
    /// A child process exited unsuccessfully.
    CommandFailed {
        /// The command line as shown to the user.
        command: String,
        /// The exit status as reported by the operating system.
        status: String,
    },
    /// A tool printed output this driver cannot interpret.
    UnexpectedOutput {
        /// The tool or document that produced the output.
        origin: String,
        /// The offending value and the shape that was expected.
        detail: String,
    },
    /// A filesystem operation failed.
    Io {
        /// The path being read or written.
        path: PathBuf,
        /// The operating-system cause.
        cause: io::Error,
    },
    /// The requested release breaks a versioning rule.
    ReleaseRule(String),
    /// The CI trigger or arguments do not select a known pipeline.
    Usage(String),
    /// One or more lint checks reported problems.
    LintFailed(String),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandSpawn { command, cause } => {
                write!(f, "cannot start `{command}`: {cause}")
            }
            Self::CommandFailed { command, status } => {
                write!(f, "`{command}` failed with {status}")
            }
            Self::UnexpectedOutput { origin, detail } => {
                write!(f, "unexpected output from {origin}: {detail}")
            }
            Self::Io { path, cause } => write!(f, "{}: {cause}", path.display()),
            Self::ReleaseRule(rule) => write!(f, "release rejected: {rule}"),
            Self::Usage(message) => write!(f, "{message}"),
            Self::LintFailed(message) => write!(f, "lint failed: {message}"),
        }
    }
}

impl std::error::Error for BuildError {}

/// Wraps an I/O error with the path it concerns.
///
/// ```ignore
/// std::fs::read(&path).map_err(|cause| io_error(&path, cause))?;
/// ```
pub fn io_error(path: &std::path::Path, cause: io::Error) -> BuildError {
    BuildError::Io {
        path: path.to_path_buf(),
        cause,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_failure_names_the_command_and_status() {
        let error = BuildError::CommandFailed {
            command: "cargo test".into(),
            status: "exit status: 101".into(),
        };
        assert_eq!(
            error.to_string(),
            "`cargo test` failed with exit status: 101"
        );
    }

    #[test]
    fn io_error_keeps_the_offending_path() {
        let error = io_error(
            std::path::Path::new("CHANGELOG.md"),
            io::Error::from(io::ErrorKind::NotFound),
        );
        assert!(error.to_string().starts_with("CHANGELOG.md: "));
    }
}
