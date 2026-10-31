//! Real Git regressions: exit 1 can mean either a clean diff or an unreadable input.
use super::*;
use crate::process::ProcessRunner;

struct IsolatedGitRunner;
fn isolated(spec: &CommandSpec) -> CommandSpec {
    spec.clone()
        .environment("GIT_CONFIG_NOSYSTEM", Some("1"))
        .environment(
            "GIT_CONFIG_GLOBAL",
            Some(if cfg!(windows) { "NUL" } else { "/dev/null" }),
        )
}
impl CommandRunner for IsolatedGitRunner {
    fn run(&self, spec: &CommandSpec) -> BuildResult<()> {
        ProcessRunner.run(&isolated(spec))
    }
    fn capture(&self, spec: &CommandSpec) -> BuildResult<String> {
        ProcessRunner.capture(&isolated(spec))
    }
    fn exit_code(&self, spec: &CommandSpec) -> BuildResult<i32> {
        ProcessRunner.exit_code(&isolated(spec))
    }
    fn capture_output(&self, spec: &CommandSpec) -> BuildResult<CommandOutput> {
        ProcessRunner.capture_output(&isolated(spec))
    }
}

struct TemporaryGitFixture {
    directory: tempfile::TempDir,
}

impl TemporaryGitFixture {
    fn new() -> Self {
        let fixture = Self {
            directory: tempfile::tempdir().unwrap(),
        };
        fixture.run(&["init", "--quiet"]);
        fixture.configure();
        fixture.run(&[
            "commit",
            "--quiet",
            "--no-gpg-sign",
            "--allow-empty",
            "-m",
            "fixture",
        ]);
        fixture
    }

    fn configure(&self) {
        for (name, value) in [
            ("user.name", "WhitespaceFixture"),
            ("user.email", "fixture@example.invalid"),
            ("core.hooksPath", ".fixture-hooks-disabled"),
            ("core.fsmonitor", "false"),
            // #75: Git for Windows enables conversion warnings in otherwise clean diffs.
            ("core.autocrlf", "false"),
            ("core.safecrlf", "false"),
            ("core.eol", "lf"),
            ("core.whitespace", "trailing-space,space-before-tab"),
        ] {
            self.run(&["config", name, value]);
        }
    }

    fn run(&self, args: &[&str]) {
        let command = CommandSpec::new("git", self.directory.path()).args(args.iter().copied());
        IsolatedGitRunner.run(&command).unwrap();
    }

    fn git(&self) -> GitRepo<'_> {
        GitRepo::discover(&IsolatedGitRunner, self.directory.path()).unwrap()
    }

    fn write(&self, path: &str, content: &str) {
        std::fs::write(self.directory.path().join(path), content).unwrap();
    }
}

#[test]
fn real_clean_untracked_file_passes_and_whitespace_fails() {
    let fixture = TemporaryGitFixture::new();
    fixture.write("new.txt", "clean\n");
    let files = [untracked_file("new.txt")];
    assert!(fixture.git().whitespace_clean("HEAD", &files).unwrap());
    fixture.write("new.txt", "trailing space \n");
    assert!(!fixture.git().whitespace_clean("HEAD", &files).unwrap());
}

#[test]
fn real_windows_line_conversion_warnings_do_not_fail_the_whitespace_check() {
    let fixture = TemporaryGitFixture::new();
    fixture.run(&["config", "core.autocrlf", "true"]);
    fixture.run(&["config", "core.safecrlf", "warn"]);
    fixture.write("new.txt", "clean\n");
    let files = [untracked_file("new.txt")];
    assert!(fixture.git().whitespace_clean("HEAD", &files).unwrap());
    fixture.write("new.txt", "trailing space \n");
    assert!(!fixture.git().whitespace_clean("HEAD", &files).unwrap());
}

#[test]
fn tracked_windows_line_conversion_preserves_config_and_whitespace_checks() {
    let fixture = TemporaryGitFixture::new();
    fixture.run(&["config", "core.autocrlf", "true"]);
    fixture.run(&["config", "core.safecrlf", "warn"]);
    fixture.write("tracked.txt", "first\r\n");
    fixture.run(&["add", "tracked.txt"]);
    fixture.run(&["commit", "--no-gpg-sign", "--quiet", "-m", "tracked"]);
    fixture.write("tracked.txt", "changed\n");
    assert!(fixture.git().whitespace_clean("HEAD", &[]).unwrap());
    let query = CommandSpec::new("git", fixture.directory.path()).args(["config", "core.safecrlf"]);
    assert_eq!(IsolatedGitRunner.capture(&query).unwrap().trim(), "warn");
    fixture.write("tracked.txt", "changed \n");
    assert!(!fixture.git().whitespace_clean("HEAD", &[]).unwrap());
}

#[test]
fn real_missing_untracked_file_is_an_error_not_clean() {
    let fixture = TemporaryGitFixture::new();
    let result = fixture
        .git()
        .whitespace_clean("HEAD", &[untracked_file("missing.txt")]);
    let error = result.expect_err("Git's missing-file exit 1 must not be accepted as clean");
    assert!(error.to_string().contains("missing.txt"), "{error}");
}

#[test]
fn real_invalid_base_is_a_command_error_not_a_whitespace_violation() {
    let fixture = TemporaryGitFixture::new();
    let result = fixture.git().whitespace_clean("missing-base", &[]);
    assert!(
        matches!(result, Err(BuildError::CommandFailed { .. })),
        "{result:?}"
    );
}

#[cfg(unix)]
#[test]
fn real_directory_symlink_is_rejected_without_changing_it() {
    let fixture = TemporaryGitFixture::new();
    let link = fixture.directory.path().join("self-link");
    std::os::unix::fs::symlink(".", &link).unwrap();
    let result = fixture
        .git()
        .whitespace_clean("HEAD", &[untracked_file("self-link")]);
    assert!(
        matches!(result, Err(BuildError::CommandFailed { .. })),
        "{result:?}"
    );
    assert_eq!(std::fs::read_link(&link).unwrap(), Path::new("."));
}

#[cfg(unix)]
#[test]
fn real_file_symlink_uses_git_link_semantics_without_modifying_its_target() {
    let fixture = TemporaryGitFixture::new();
    fixture.write("target.txt", "trailing space \n");
    let link = fixture.directory.path().join("file-link");
    std::os::unix::fs::symlink("target.txt", &link).unwrap();
    let result = fixture
        .git()
        .whitespace_clean("HEAD", &[untracked_file("file-link")]);
    assert!(result.unwrap());
    assert_eq!(std::fs::read_link(&link).unwrap(), Path::new("target.txt"));
    assert_eq!(
        std::fs::read_to_string(fixture.directory.path().join("target.txt")).unwrap(),
        "trailing space \n"
    );
}

#[path = "fixture_checkout_tests.rs"]
mod fixture_checkout_tests;
