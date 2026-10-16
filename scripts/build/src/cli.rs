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
use crate::release::{ReleaseRequest, default_dist_dir, run_release};
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
    if let BuildCommand::Ci(CiCommand::Plan) = command {
        return emit_plan();
    }
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
        BuildCommand::Ci(CiCommand::Run { target }) => run_planned(context, &target),
        BuildCommand::Ci(CiCommand::Publish { dist }) => publish_planned(context, &dist),
        BuildCommand::Ci(CiCommand::Plan) => emit_plan(),
    }
}

fn current_plan() -> BuildResult<CiPlan> {
    plan_for(&CiTrigger::from_lookup(|key| std::env::var(key).ok()))
}

fn emit_plan() -> BuildResult<()> {
    let outputs = plan_outputs(&current_plan()?);
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
    release_for(context, parse_version_argument(version)?, &triple)
}

fn release_for(
    context: &BuildContext<'_>,
    version: semver::Version,
    triple: &str,
) -> BuildResult<()> {
    let request = ReleaseRequest {
        dist_dir: default_dist_dir(context.root(), &version),
        version,
        triple: triple.to_owned(),
        source_packages: is_primary(triple),
    };
    run_release(context, &request)?;
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

fn run_planned(context: &BuildContext<'_>, triple: &str) -> BuildResult<()> {
    match current_plan()? {
        CiPlan::PullRequest { base } => {
            validate_pull_request(context, &base, &BuildTarget::Triple(triple.to_owned()))
        }
        CiPlan::Release { version, .. } => release_for(context, version, triple),
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
}
