//! Bounds artifact subprocess output while reading, before allocating the full stream.
use super::{BuildError, BuildResult, CommandOutput, CommandSpec, Stdio, failure, spawn_error};
use std::{io::Read, process::Child};

pub(super) fn check(spec: &CommandSpec, size: usize, limit: usize) -> BuildResult<()> {
    if size > limit {
        return Err(BuildError::UnexpectedOutput {
            origin: spec.display(),
            detail: format!("stdout exceeded {limit} bytes; expected at most {limit}"),
        });
    }
    Ok(())
}

pub(super) fn capture(spec: &CommandSpec, limit: usize) -> BuildResult<CommandOutput> {
    let bound = limit.checked_add(1).ok_or_else(|| {
        BuildError::Usage("stdout limit overflow; expected less than usize::MAX".into())
    })?;
    let mut child = spawn_piped(spec)?;
    let output = read_stdout(&mut child, spec, bound, limit);
    if output.is_err() {
        let _ = child.kill();
        let _ = child.wait();
    }
    let stdout = output?;
    let status = child.wait().map_err(|error| spawn_error(spec, error))?;
    let code = status.code().ok_or_else(|| failure(spec, status))?;
    Ok(CommandOutput {
        code,
        stdout,
        stderr: Vec::new(),
    })
}

fn spawn_piped(spec: &CommandSpec) -> BuildResult<Child> {
    spec.to_command()
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|error| spawn_error(spec, error))
}

fn read_stdout(
    child: &mut Child,
    spec: &CommandSpec,
    bound: usize,
    limit: usize,
) -> BuildResult<Vec<u8>> {
    let pipe = child
        .stdout
        .take()
        .ok_or_else(|| BuildError::UnexpectedOutput {
            origin: spec.display(),
            detail: "missing stdout pipe; expected piped child output".into(),
        })?;
    let mut bytes = Vec::new();
    pipe.take(bound as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| spawn_error(spec, error))?;
    check(spec, bytes.len(), limit)?;
    Ok(bytes)
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use crate::process::{CommandRunner, ProcessRunner};

    #[test]
    fn bounded_capture_accepts_exact_limit_and_refuses_more_output() {
        let root = tempfile::tempdir().unwrap();
        let spec = CommandSpec::new("python3", root.path()).args([
            "-I",
            "-c",
            "import sys; sys.stdout.buffer.write(b'x'*1024)",
        ]);
        assert_eq!(
            ProcessRunner
                .capture_bounded(&spec, 1024)
                .unwrap()
                .stdout
                .len(),
            1024
        );
        assert!(ProcessRunner.capture_bounded(&spec, 16).is_err());
        assert!(ProcessRunner.capture_bounded(&spec, usize::MAX).is_err());
    }
}
