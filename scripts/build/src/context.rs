//! What every pipeline needs: a process runner, the repository, a reporter.

use std::path::Path;

use crate::git::GitRepo;
use crate::process::CommandRunner;
use crate::report::StepReporter;

/// Dependencies shared by the PR, release and publish pipelines.
pub struct BuildContext<'a> {
    /// Runs git, cargo, rustfmt and gh.
    pub runner: &'a dyn CommandRunner,
    /// The repository whose root is also the cargo workspace root.
    pub git: GitRepo<'a>,
    /// Prints step banners.
    pub reporter: StepReporter,
}

impl BuildContext<'_> {
    /// The repository and workspace root.
    ///
    /// ```ignore
    /// let changelog = context.root().join("CHANGELOG.md");
    /// ```
    pub fn root(&self) -> &Path {
        self.git.root()
    }
}

/// The package name of this driver; it is tested like any member but never
/// shipped or version-checked as part of a kernel release.
pub const BUILD_TOOL_PACKAGE: &str = env!("CARGO_PKG_NAME");

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::fake_runner::RecordingRunner;
    use crate::report::LogFlavor;

    /// A context rooted at `root`, as `git rev-parse` would report it.
    pub(crate) fn context_at<'a>(runner: &'a RecordingRunner, root: &Path) -> BuildContext<'a> {
        let git = GitRepo::discover(runner, root).unwrap();
        BuildContext {
            runner,
            git,
            reporter: StepReporter::new(LogFlavor::Terminal),
        }
    }

    #[test]
    fn root_comes_from_git() {
        let runner =
            RecordingRunner::default().with_stdout("git rev-parse --show-toplevel", "/repo\n");
        assert_eq!(
            context_at(&runner, Path::new("/repo/sub")).root(),
            Path::new("/repo")
        );
    }
}
