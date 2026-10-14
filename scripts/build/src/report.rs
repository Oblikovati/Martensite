//! Plain-text step banners for the user-facing CLI. Inside GitHub Actions the
//! same banners become collapsible log groups.

use crate::error::BuildResult;

/// Where step banners are rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFlavor {
    /// A local terminal.
    Terminal,
    /// A GitHub Actions log, which understands `::group::` markers.
    GithubActions,
}

impl LogFlavor {
    /// Detects GitHub Actions from the `GITHUB_ACTIONS` variable it always sets.
    ///
    /// ```ignore
    /// let flavor = LogFlavor::from_env_value(std::env::var("GITHUB_ACTIONS").ok().as_deref());
    /// ```
    pub fn from_env_value(github_actions: Option<&str>) -> Self {
        match github_actions {
            Some("true") => Self::GithubActions,
            _ => Self::Terminal,
        }
    }

    fn opening(self, title: &str) -> String {
        match self {
            Self::Terminal => format!("==> {title}"),
            Self::GithubActions => format!("::group::{title}"),
        }
    }

    fn closing(self) -> Option<&'static str> {
        match self {
            Self::Terminal => None,
            Self::GithubActions => Some("::endgroup::"),
        }
    }
}

/// Announces pipeline steps and keeps their output grouped.
pub struct StepReporter {
    flavor: LogFlavor,
}

impl StepReporter {
    /// Creates a reporter for one log flavor.
    pub fn new(flavor: LogFlavor) -> Self {
        Self { flavor }
    }

    /// Runs `body` under a titled banner and returns its result.
    ///
    /// ```ignore
    /// reporter.step("Run tests", || cargo.test(&selection))?;
    /// ```
    pub fn step<T>(&self, title: &str, body: impl FnOnce() -> BuildResult<T>) -> BuildResult<T> {
        println!("{}", self.flavor.opening(title));
        let result = body();
        if let Some(closing) = self.flavor.closing() {
            println!("{closing}");
        }
        result
    }

    /// Prints one informational line outside any step.
    pub fn note(&self, message: &str) {
        println!("{message}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn github_actions_is_detected_only_from_true() {
        assert_eq!(
            LogFlavor::from_env_value(Some("true")),
            LogFlavor::GithubActions
        );
        assert_eq!(LogFlavor::from_env_value(Some("1")), LogFlavor::Terminal);
        assert_eq!(LogFlavor::from_env_value(None), LogFlavor::Terminal);
    }

    #[test]
    fn banners_match_the_log_flavor() {
        assert_eq!(LogFlavor::Terminal.opening("Lint"), "==> Lint");
        assert_eq!(LogFlavor::GithubActions.opening("Lint"), "::group::Lint");
        assert_eq!(LogFlavor::GithubActions.closing(), Some("::endgroup::"));
    }

    #[test]
    fn step_returns_the_body_result() {
        let reporter = StepReporter::new(LogFlavor::Terminal);
        assert_eq!(reporter.step("Count", || Ok(3)).unwrap(), 3);
    }
}
