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
    /// Compare a toolkit's paired performance cases with OCCT on this machine (#583).
    OcctParity(OcctParityArgs),
    /// Entry points used by `.github/workflows/martensite.yml`.
    #[command(subcommand)]
    Ci(CiCommand),
}

/// Arguments of `cargo xtask occt-parity --occt /opt/occt --cpus 8-15`.
#[derive(clap::Args)]
pub struct OcctParityArgs {
    /// The toolkit whose cases run.
    #[arg(long, value_enum, default_value = "tkernel")]
    toolkit: crate::occt_parity::ParityToolkit,
    /// The OCCT install prefix, holding `include/opencascade` and `lib`.
    #[arg(long)]
    occt: PathBuf,
    /// The C++ compiler.
    #[arg(long, default_value = "c++")]
    cxx: String,
    /// A `taskset` CPU list for both programs, e.g. `8-15`; keep builds off these CPUs.
    #[arg(long)]
    cpus: Option<String>,
    /// How many times each program runs, alternating.
    #[arg(long, default_value_t = 2, value_parser = clap::value_parser!(u16).range(1..))]
    rounds: u16,
    /// Keeps the cases whose name contains this text.
    #[arg(long)]
    filter: Option<String>,
    /// The relative difference still counted as parity.
    #[arg(long, default_value_t = 0.05)]
    tolerance: f64,
}

impl OcctParityArgs {
    fn request(self) -> crate::occt_parity::ParityRequest {
        crate::occt_parity::ParityRequest {
            toolkit: self.toolkit,
            occt_prefix: self.occt,
            compiler: self.cxx,
            cpus: self.cpus,
            rounds: usize::from(self.rounds),
            filter: self.filter,
            tolerance: self.tolerance,
        }
    }
}

/// Owned arguments for one planned CI target; `cargo xtask ci run --target x86_64-unknown-linux-gnu`.
#[derive(clap::Args)]
pub struct CiRunArgs {
    /// The matrix target triple.
    #[arg(long)]
    target: String,
    /// Delegate tests to the required package or release-shard matrix.
    #[arg(long)]
    checks_only: bool,
    /// Run one group of the PR checks; CI runs the groups as parallel jobs (#473).
    #[arg(long, value_enum, requires = "checks_only")]
    stage: Option<crate::pr::CheckStage>,
}

/// Commands that read the GitHub Actions trigger from the environment.
#[derive(Subcommand)]
pub enum CiCommand {
    /// Discover an exact-commit validated rehearsal for a tag.
    ReusePlan,
    /// Seal successful rehearsal artifact identities.
    SealValidation {
        #[arg(long)]
        directory: PathBuf,
    },
    /// Restore artifacts after revalidating successful rehearsal evidence.
    ReuseVerify {
        #[arg(long)]
        run: u64,
        #[arg(long)]
        dist: PathBuf,
    },
    /// Check geometry cache ownership with the pinned Linux thread sanitizer.
    ThreadSanitizer,
    /// Write the pipeline mode and target matrix to `GITHUB_OUTPUT`.
    Plan,
    /// Write the scheduled build-cache refresh matrix to `GITHUB_OUTPUT` (#472).
    WarmPlan,
    /// Compile one package's PR tests on `main` to refresh its build cache (#472).
    Warm {
        #[arg(long)]
        package: String,
        #[arg(long)]
        target: String,
    },
    /// Run the planned pipeline for one matrix target.
    Run(CiRunArgs),
    /// Run one affected package or release test shard.
    Test {
        #[arg(long, required_unless_present = "shard", conflicts_with = "shard")]
        package: Option<String>,
        #[arg(long, required_unless_present = "package", conflicts_with = "package")]
        shard: Option<usize>,
        /// Zero-based part of a split package's tests (#470).
        #[arg(long, conflicts_with = "shard")]
        part: Option<usize>,
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
        BuildCommand::OcctParity(args) => {
            crate::occt_parity::run_occt_parity(context, &args.request())
        }
        BuildCommand::Ci(command) => dispatch_ci(context, command),
    }
}

fn dispatch_ci(context: &BuildContext<'_>, command: CiCommand) -> BuildResult<()> {
    use CiCommand::{
        Gate, Plan, Publish, ReusePlan, ReuseVerify, Run, SealValidation, Test, ThreadSanitizer,
        Warm, WarmPlan,
    };
    match command {
        ReusePlan => crate::release_reuse::discover(context),
        ReuseVerify { run, dist } => crate::release_reuse::restore(context, run, &dist),
        SealValidation { directory } => crate::release_reuse::seal(context, &directory),
        ThreadSanitizer => {
            let plan =
                crate::thread_sanitizer_selection::execution_plan(|key| std::env::var(key).ok())?;
            let controls = match plan {
                Some(plan) => crate::ci_tests::sanitizer_controls(context, &plan)?,
                None => crate::thread_sanitizer_selection::CONTROLS.to_vec(),
            };
            crate::thread_sanitizer::run_selected(context, &controls)
        }
        Run(args) => run_planned(context, &args),
        Test {
            package,
            shard,
            part,
            target,
        } => test_planned_selection(
            context,
            (package.as_deref(), part.unwrap_or(0)),
            shard,
            target,
        ),
        Gate {
            checks,
            tests,
            has_tests,
        } => crate::ci_tests::require_success(&checks, &tests, has_tests),
        Publish { dist } => publish_planned(context, &dist),
        Plan => emit_plan(context),
        WarmPlan => {
            let github_output = std::env::var_os("GITHUB_OUTPUT").map(PathBuf::from);
            write_outputs(
                &crate::cache_warm::warm_plan_outputs(),
                github_output.as_deref(),
            )
        }
        Warm { package, target } => {
            crate::cache_warm::warm_package(context, &package, &BuildTarget::Triple(target))
        }
    }
}

fn test_planned_selection(
    context: &BuildContext<'_>,
    (package, part): (Option<&str>, usize),
    shard: Option<usize>,
    target: String,
) -> BuildResult<()> {
    let plan = current_plan()?;
    let target = BuildTarget::Triple(target);
    match (package, shard) {
        (Some(package), None) => {
            crate::ci_tests::test_package(context, &plan, package, part, &target)
        }
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
    let plan = crate::plan::pin_comparison(context, current_plan()?)?;
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

fn run_planned(context: &BuildContext<'_>, args: &CiRunArgs) -> BuildResult<()> {
    let plan = current_plan()?;
    let Some(stage) = args.stage else {
        return dispatch_plan(context, plan, &args.target, args.checks_only);
    };
    let CiPlan::PullRequest { base } = plan else {
        return Err(BuildError::Usage(format!(
            "check stage {stage:?} with plan {plan:?}; expected a pull request plan"
        )));
    };
    let target = BuildTarget::Triple(args.target.clone());
    crate::pr::validate_pr_stages(context, &base, &target, &[stage], false)
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
mod tests;
