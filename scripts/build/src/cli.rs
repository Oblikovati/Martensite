//! Command-line surface. Each command resolves its inputs once and calls one
//! pipeline; no build logic lives here.

use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

use crate::cargo::{BuildTarget, CargoTasks};
use crate::context::BuildContext;
use crate::error::{BuildError, BuildResult};
use crate::git::GitRepo;
use crate::plan::{CiPlan, CiTrigger, plan_for, plan_outputs, write_outputs};
use crate::pr::validate_pull_request;
use crate::process::ProcessRunner;
use crate::publish::publish_release;
use crate::release::{ReleaseRequest, default_dist_dir, run_release, run_release_checks};
use crate::report::{LogFlavor, StepReporter};
use crate::targets::is_primary;
use crate::version::parse_version_argument;

/// Martensite PR validation and release driver.
#[derive(Parser)]
#[command(name = "cargo xtask", version)]
pub struct Cli {
    /// The pipeline to run.
    #[command(subcommand)]
    pub command: BuildCommand,
}

/// Pipelines a contributor runs locally.
#[derive(Subcommand)]
pub enum BuildCommand {
    /// Validate this branch (with uncommitted work) the way CI validates a PR.
    Pr {
        /// The branch the PR merges into.
        #[arg(long, default_value = "origin/main")]
        base: String,
    },
    /// Rehearse a release on this machine: semver gate, lint, tests, packages.
    Release {
        /// The version to release, e.g. `0.3.0` or `v0.3.0`.
        #[arg(long)]
        version: String,
        /// Target triple; defaults to the host.
        #[arg(long)]
        target: Option<String>,
    },
    /// Entry points used by `.github/workflows/martensite.yml`.
    #[command(subcommand)]
    Ci(CiCommand),
}

/// Commands that read the GitHub Actions trigger from the environment.
#[derive(Subcommand)]
pub enum CiCommand {
    /// Write the pipeline mode and target matrix to `GITHUB_OUTPUT`.
    Plan,
    /// Run the planned pipeline for one matrix target.
    Run {
        /// The matrix target triple.
        #[arg(long)]
        target: String,
        /// Delegate tests to the required package or release-shard matrix.
        #[arg(long)]
        checks_only: bool,
    },
    /// Run one affected package or release test shard.
    Test {
        #[arg(long, required_unless_present = "shard", conflicts_with = "shard")]
        package: Option<String>,
        #[arg(long, required_unless_present = "package", conflicts_with = "package")]
        shard: Option<usize>,
        #[arg(long)]
        target: String,
    },
    /// Require every validation and test job to succeed.
    Gate {
        #[arg(long)]
        checks: String,
        #[arg(long)]
        tests: String,
        #[arg(long)]
        has_tests: bool,
    },
    /// Publish the packaged assets of a tag-push release.
    Publish {
        /// Directory holding every target's packaged assets.
        #[arg(long)]
        dist: PathBuf,
    },
}

/// Runs one command against the repository containing the current directory.
///
/// ```ignore
/// execute(BuildCommand::Pr { base: "origin/main".into() })?;
/// ```
pub fn execute(command: BuildCommand) -> BuildResult<()> {
    let cwd =
        std::env::current_dir().map_err(|cause| crate::error::io_error(Path::new("."), cause))?;
    let runner = ProcessRunner;
    let flavor = LogFlavor::from_env_value(std::env::var("GITHUB_ACTIONS").ok().as_deref());
    let context = BuildContext {
        runner: &runner,
        git: GitRepo::discover(&runner, &cwd)?,
        reporter: StepReporter::new(flavor),
    };
    dispatch(&context, command)
}

fn dispatch(context: &BuildContext<'_>, command: BuildCommand) -> BuildResult<()> {
    match command {
        BuildCommand::Pr { base } => validate_pull_request(context, &base, &BuildTarget::Host),
        BuildCommand::Release { version, target } => rehearse_release(context, &version, target),
        BuildCommand::Ci(command) => dispatch_ci(context, command),
    }
}

fn dispatch_ci(context: &BuildContext<'_>, command: CiCommand) -> BuildResult<()> {
    match command {
        CiCommand::Run {
            target,
            checks_only,
        } => run_planned(context, &target, checks_only),
        CiCommand::Test {
            package,
            shard,
            target,
        } => test_planned_selection(context, package.as_deref(), shard, target),
        CiCommand::Gate {
            checks,
            tests,
            has_tests,
        } => crate::ci_tests::require_success(&checks, &tests, has_tests),
        CiCommand::Publish { dist } => publish_planned(context, &dist),
        CiCommand::Plan => emit_plan(context),
    }
}

fn test_planned_selection(
    context: &BuildContext<'_>,
    package: Option<&str>,
    shard: Option<usize>,
    target: String,
) -> BuildResult<()> {
    let plan = current_plan()?;
    let target = BuildTarget::Triple(target);
    match (package, shard) {
        (Some(package), None) => crate::ci_tests::test_package(context, &plan, package, &target),
        (None, Some(shard)) => crate::ci_tests::test_shard(context, &plan, shard, &target),
        _ => Err(BuildError::Usage(format!(
            "package {package:?}, shard {shard:?}; expected exactly one of --package or --shard"
        ))),
    }
}

fn current_plan() -> BuildResult<CiPlan> {
    plan_for(&CiTrigger::from_lookup(|key| std::env::var(key).ok()))
}

fn emit_plan(context: &BuildContext<'_>) -> BuildResult<()> {
    let plan = current_plan()?;
    let mut outputs = plan_outputs(&plan);
    outputs.extend(crate::ci_tests::matrix_outputs(context, &plan)?);
    let github_output = std::env::var_os("GITHUB_OUTPUT").map(PathBuf::from);
    write_outputs(&outputs, github_output.as_deref())
}

fn rehearse_release(
    context: &BuildContext<'_>,
    version: &str,
    target: Option<String>,
) -> BuildResult<()> {
    let triple = match target {
        Some(triple) => triple,
        None => CargoTasks::new(context.runner, context.root()).host_triple()?,
    };
    release_for(context, parse_version_argument(version)?, &triple, true)
}

fn release_for(
    context: &BuildContext<'_>,
    version: semver::Version,
    triple: &str,
    run_tests: bool,
) -> BuildResult<()> {
    let request = release_request(context, version, triple);
    if run_tests {
        run_release(context, &request)?;
    } else {
        run_release_checks(context, &request, false)?;
    }
    capture_release_inventory(context, triple)
}

fn release_for_using(
    context: &BuildContext<'_>,
    version: semver::Version,
    triple: &str,
    run_tests: bool,
    sources: &dyn crate::release_sources::SourceRelease,
) -> BuildResult<()> {
    let request = release_request(context, version, triple);
    crate::release::run_release_using(context, &request, run_tests, sources)?;
    capture_release_inventory(context, triple)
}

fn release_request(
    context: &BuildContext<'_>,
    version: semver::Version,
    triple: &str,
) -> ReleaseRequest {
    ReleaseRequest {
        dist_dir: default_dist_dir(context.root(), &version),
        version,
        triple: triple.into(),
        source_packages: is_primary(triple),
    }
}

fn capture_release_inventory(context: &BuildContext<'_>, triple: &str) -> BuildResult<()> {
    context
        .reporter
        .step("Check expanded unsafe inventory", || {
            crate::compiler_inventory::capture_inventory(
                context.runner,
                context.root(),
                &BuildTarget::Triple(triple.to_owned()),
            )
        })
}

fn run_planned(context: &BuildContext<'_>, triple: &str, checks_only: bool) -> BuildResult<()> {
    dispatch_plan(context, current_plan()?, triple, checks_only)
}

fn dispatch_plan(
    context: &BuildContext<'_>,
    plan: CiPlan,
    triple: &str,
    checks_only: bool,
) -> BuildResult<()> {
    dispatch_plan_using(
        context,
        plan,
        triple,
        checks_only,
        &crate::release_sources::VerifiedSourceRelease,
    )
}

fn dispatch_plan_using(
    context: &BuildContext<'_>,
    plan: CiPlan,
    triple: &str,
    checks_only: bool,
    sources: &dyn crate::release_sources::SourceRelease,
) -> BuildResult<()> {
    match plan {
        CiPlan::PullRequest { base } => crate::pr::validate_pr_checks(
            context,
            &base,
            &BuildTarget::Triple(triple.to_owned()),
            !checks_only,
        ),
        CiPlan::Release { version, .. } => {
            release_for_using(context, version, triple, !checks_only, sources)
        }
    }
}

fn publish_planned(context: &BuildContext<'_>, dist: &Path) -> BuildResult<()> {
    match current_plan()? {
        CiPlan::Release {
            version,
            publish: true,
        } => publish_release(context, &version, dist),
        other => Err(BuildError::Usage(format!(
            "{other:?} does not publish; expected a push of a vX.Y.Z tag"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn command_line_definition_is_consistent() {
        Cli::command().debug_assert();
    }

    #[test]
    fn pr_base_defaults_to_origin_main() {
        let cli = Cli::try_parse_from(["cargo xtask", "pr"]).unwrap();
        assert!(matches!(cli.command, BuildCommand::Pr { ref base } if base == "origin/main"));
    }

    #[test]
    fn ci_run_requires_a_target() {
        assert!(Cli::try_parse_from(["cargo xtask", "ci", "run"]).is_err());
    }

    #[test]
    fn ci_test_requires_exactly_one_selector() {
        let prefix = [
            "cargo xtask",
            "ci",
            "test",
            "--target",
            "x86_64-unknown-linux-gnu",
        ];
        for selector in [vec!["--package", "martensite-tkmath"], vec!["--shard", "0"]] {
            assert!(Cli::try_parse_from(prefix.into_iter().chain(selector)).is_ok());
        }
        assert!(Cli::try_parse_from(prefix).is_err());
        let both = ["--package", "martensite-tkmath", "--shard", "0"];
        assert!(Cli::try_parse_from(prefix.into_iter().chain(both)).is_err());
    }

    #[test]
    fn release_dispatch_delegates_only_tests_and_keeps_inventory_required() {
        for checks_only in [false, true] {
            assert_release_dispatch(checks_only);
        }
    }

    fn assert_release_dispatch(checks_only: bool) {
        let fixture = crate::release::tests::ReleaseFixture::new();
        let runner = fixture.runner("v0.2.7\n", "0.3.0");
        let context = crate::context::tests::context_at(&runner, fixture.root());
        let plan = CiPlan::Release {
            version: semver::Version::new(0, 3, 0),
            publish: false,
        };
        let result = dispatch_plan_using(
            &context,
            plan,
            "x86_64-unknown-linux-gnu",
            checks_only,
            &crate::release::tests::FixtureSourceRelease,
        );
        // The fake returns no compiler stream: reaching this error proves the gate still runs.
        let error = result.unwrap_err().to_string();
        assert!(error.contains("missing build-finished"));
        assert_release_commands(&runner.commands(), checks_only);
    }

    fn assert_release_commands(commands: &[String], checks_only: bool) {
        assert_eq!(
            commands.iter().any(|c| c.starts_with("cargo test ")),
            !checks_only
        );
        assert!(commands.iter().any(|c| c.starts_with("cargo package ")));
        assert!(
            commands
                .iter()
                .any(|c| c.contains("target/unsafe-inventory/build"))
        );
    }
}
