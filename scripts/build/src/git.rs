//! Read-only git queries: the change set of a branch and the release tags.

use std::path::{Path, PathBuf};

use crate::error::{BuildError, BuildResult};
use crate::process::{CommandOutput, CommandRunner, CommandSpec};

/// One path that differs between the merge base and the working tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangedFile {
    /// Path relative to the repository root, `/`-separated as git prints it.
    pub path: String,
    /// The file no longer exists, so file-level linters must skip it.
    pub deleted: bool,
    /// The file is not tracked yet; `git diff` does not see its content.
    pub untracked: bool,
}

/// Git queries rooted at one repository.
pub struct GitRepo<'a> {
    runner: &'a dyn CommandRunner,
    root: PathBuf,
}

impl<'a> GitRepo<'a> {
    /// Opens the repository that contains `cwd`.
    ///
    /// ```ignore
    /// let git = GitRepo::discover(&ProcessRunner, Path::new("."))?;
    /// ```
    pub fn discover(runner: &'a dyn CommandRunner, cwd: &Path) -> BuildResult<Self> {
        let spec = CommandSpec::new("git", cwd).args(["rev-parse", "--show-toplevel"]);
        let root = PathBuf::from(runner.capture(&spec)?.trim());
        Ok(Self { runner, root })
    }

    /// The repository root directory.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The commit where `HEAD` forked from `base` (a branch, tag or SHA).
    pub fn merge_base(&self, base: &str) -> BuildResult<String> {
        let stdout = self.capture(&["merge-base", base, "HEAD"])?;
        Ok(stdout.trim().to_owned())
    }

    /// Files changed since `base_commit`, including uncommitted and untracked
    /// work, so a local run checks exactly what the pushed PR will contain.
    pub fn changed_files(&self, base_commit: &str) -> BuildResult<Vec<ChangedFile>> {
        let diff = self.capture(&["diff", "--name-status", "--no-renames", "-z", base_commit])?;
        let mut files = parse_name_status(&diff)?;
        let untracked = self.capture(&["ls-files", "--others", "--exclude-standard", "-z"])?;
        files.extend(untracked.split_terminator('\0').map(untracked_file));
        files.sort_by(|left, right| left.path.cmp(&right.path));
        Ok(files)
    }

    /// Reports whitespace errors added since `base_commit`; true when clean.
    pub fn whitespace_clean(&self, base_commit: &str, files: &[ChangedFile]) -> BuildResult<bool> {
        let tracked = self.diff_clean(&["diff", "--check", base_commit], false)?;
        let mut clean = tracked;
        for file in files.iter().filter(|file| file.untracked) {
            clean &= self.untracked_whitespace_clean(&file.path)?;
        }
        Ok(clean)
    }

    /// Every tag that starts with `v`, the release tag convention.
    pub fn release_tags(&self) -> BuildResult<Vec<String>> {
        let stdout = self.capture(&["tag", "--list", "v*"])?;
        Ok(stdout.lines().map(str::to_owned).collect())
    }

    fn capture(&self, args: &[&str]) -> BuildResult<String> {
        self.runner.capture(&self.command(args))
    }

    fn diff_clean(&self, args: &[&str], no_index: bool) -> BuildResult<bool> {
        let spec = self.command(args);
        let output = self.runner.capture_output(&spec)?;
        let clean = diff_whitespace_result(&spec, &output, no_index)?;
        if !clean {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
        Ok(clean)
    }

    fn untracked_whitespace_clean(&self, path: &str) -> BuildResult<bool> {
        self.diff_clean(
            &["diff", "--no-index", "--check", "--", "/dev/null", path],
            true,
        )
    }

    fn command(&self, args: &[&str]) -> CommandSpec {
        CommandSpec::new("git", &self.root).args(args.iter().copied())
    }
}

fn diff_whitespace_result(
    spec: &CommandSpec,
    output: &CommandOutput,
    no_index: bool,
) -> BuildResult<bool> {
    // #75: Git uses exit 1 for both a clean new file and fatal no-index read errors.
    match (
        output.code,
        output.stdout.is_empty(),
        output.stderr.is_empty(),
    ) {
        (0, true, true) => Ok(true),
        (1, true, true) if no_index => Ok(true),
        (2 | 3, _, true) => Ok(false),
        _ => Err(diff_command_failure(spec, output)),
    }
}

fn diff_command_failure(spec: &CommandSpec, output: &CommandOutput) -> BuildError {
    BuildError::CommandFailed {
        command: spec.display(),
        status: format!(
            "exit code {}; expected clean output or a whitespace diagnostic; stdout={:?}; stderr={:?}",
            output.code,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
    }
}

/// Parses `git diff --name-status -z`: alternating status and path fields.
fn parse_name_status(stdout: &str) -> BuildResult<Vec<ChangedFile>> {
    let fields: Vec<&str> = stdout.split_terminator('\0').collect();
    if !fields.len().is_multiple_of(2) {
        return Err(BuildError::UnexpectedOutput {
            origin: "git diff --name-status -z".into(),
            detail: format!("{fields:?}; expected status/path pairs"),
        });
    }
    let files = fields.chunks(2).map(|pair| ChangedFile {
        path: pair[1].to_owned(),
        deleted: pair[0] == "D",
        untracked: false,
    });
    Ok(files.collect())
}

fn untracked_file(path: &str) -> ChangedFile {
    ChangedFile {
        path: path.to_owned(),
        deleted: false,
        untracked: true,
    }
}

#[cfg(test)]
#[path = "git_process_tests.rs"]
mod process_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fake_runner::RecordingRunner;

    fn repo(runner: &RecordingRunner) -> GitRepo<'_> {
        GitRepo {
            runner,
            root: PathBuf::from("/repo"),
        }
    }

    #[test]
    fn changed_files_merge_diff_and_untracked_sorted() {
        let runner = RecordingRunner::default()
            .with_stdout(
                "git diff --name-status --no-renames -z abc",
                "M\0src/b.rs\0D\0old.rs\0",
            )
            .with_stdout("git ls-files --others --exclude-standard -z", "a.rs\0");
        let files = repo(&runner).changed_files("abc").unwrap();
        let paths: Vec<_> = files
            .iter()
            .map(|f| (f.path.as_str(), f.deleted, f.untracked))
            .collect();
        assert_eq!(
            paths,
            [
                ("a.rs", false, true),
                ("old.rs", true, false),
                ("src/b.rs", false, false)
            ]
        );
    }

    #[test]
    fn odd_name_status_output_is_rejected() {
        let error = parse_name_status("M\0").unwrap_err();
        assert!(error.to_string().contains("expected status/path pairs"));
    }

    #[test]
    fn untracked_whitespace_problem_is_read_from_bit_two() {
        let file = untracked_file("new.txt");
        let command = "git diff --no-index --check -- /dev/null new.txt";
        let differs_only = RecordingRunner::default().with_exit_code(command, 1);
        assert!(
            repo(&differs_only)
                .whitespace_clean("abc", std::slice::from_ref(&file))
                .unwrap()
        );
        let problem = RecordingRunner::default().with_exit_code(command, 3);
        assert!(!repo(&problem).whitespace_clean("abc", &[file]).unwrap());
    }

    #[test]
    fn tracked_whitespace_problem_fails_the_check() {
        let runner = RecordingRunner::default().with_exit_code("git diff --check abc", 2);
        assert!(!repo(&runner).whitespace_clean("abc", &[]).unwrap());
    }

    #[test]
    fn fatal_no_index_exit_one_preserves_the_command_and_diagnostic() {
        let command = "git diff --no-index --check -- /dev/null missing.txt";
        let runner = RecordingRunner::default()
            .with_exit_code(command, 1)
            .with_stderr(command, "error: Could not access 'missing.txt'");
        let error = repo(&runner)
            .whitespace_clean("abc", &[untracked_file("missing.txt")])
            .unwrap_err();
        assert!(matches!(error, BuildError::CommandFailed { .. }));
        assert!(error.to_string().contains(command));
        assert!(error.to_string().contains("Could not access 'missing.txt'"));
    }

    #[test]
    fn unexpected_exit_status_is_never_accepted_as_clean() {
        let command = "git diff --no-index --check -- /dev/null new.txt";
        for code in [4, 128, 255] {
            let runner = RecordingRunner::default().with_exit_code(command, code);
            assert!(
                repo(&runner)
                    .whitespace_clean("abc", &[untracked_file("new.txt")])
                    .is_err()
            );
        }
    }

    #[test]
    fn unexpected_output_is_never_accepted_as_clean() {
        let command = "git diff --no-index --check -- /dev/null new.txt";
        let runner = RecordingRunner::default()
            .with_exit_code(command, 1)
            .with_stdout(command, "unexpected");
        assert!(
            repo(&runner)
                .whitespace_clean("abc", &[untracked_file("new.txt")])
                .is_err()
        );
    }

    #[test]
    fn merge_base_is_trimmed() {
        let runner =
            RecordingRunner::default().with_stdout("git merge-base origin/main HEAD", "f00\n");
        assert_eq!(repo(&runner).merge_base("origin/main").unwrap(), "f00");
    }

    #[test]
    fn release_tags_are_listed_one_per_line() {
        let runner =
            RecordingRunner::default().with_stdout("git tag --list v*", "v0.1.0\nv0.2.0\n");
        assert_eq!(repo(&runner).release_tags().unwrap(), ["v0.1.0", "v0.2.0"]);
    }
}
