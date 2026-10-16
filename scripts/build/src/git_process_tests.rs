//! Real Git regressions: exit 1 can mean either a clean diff or an unreadable input.
use super::*;
use crate::process::ProcessRunner;

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
        ProcessRunner.run(&command).unwrap();
    }

    fn git(&self) -> GitRepo<'_> {
        GitRepo::discover(&ProcessRunner, self.directory.path()).unwrap()
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
fn real_windows_line_conversion_settings_are_reset_by_fixture_configuration() {
    let fixture = TemporaryGitFixture::new();
    fixture.run(&["config", "core.autocrlf", "true"]);
    fixture.run(&["config", "core.safecrlf", "warn"]);
    fixture.write("new.txt", "clean\n");
    let files = [untracked_file("new.txt")];
    let warning = fixture.git().whitespace_clean("HEAD", &files).unwrap_err();
    assert!(warning.to_string().contains("CRLF"), "{warning}");
    fixture.configure();
    assert!(fixture.git().whitespace_clean("HEAD", &files).unwrap());
    fixture.write("new.txt", "trailing space \n");
    assert!(!fixture.git().whitespace_clean("HEAD", &files).unwrap());
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
