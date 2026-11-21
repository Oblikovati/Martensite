//! Temporary gate for #592 while the Rust-native toolkit migration is in progress.
use std::collections::BTreeSet;

use crate::cargo::{BuildTarget, CargoTasks, PackageSelection};
use crate::context::BuildContext;
use crate::error::{BuildError, BuildResult};
use crate::git::ChangedFile;
use crate::targets::PRIMARY_TARGET;
use crate::thread_sanitizer_selection::Control;
use crate::workspace::CargoWorkspace;

mod scope;
use scope::{Selection, select};

const LEVELS: &str = include_str!("object_model_pr/levels.txt");
const CURSOR: &str = "docs/features/rust-object-model/migration-step";
const FEATURE_DOCS: &str = "docs/features/rust-object-model/";
const INVENTORY_DOCS: &str = "docs/features/safe-toolkits/";
// These packages consume migrated toolkits without declaring objects of their own.
const VERIFICATION_ONLY: [&str; 3] = [
    "martensite-process-policy",
    "martensite-test-support",
    "martensite-topology-test-support",
];
const SCOPE_EXEMPTIONS: [&str; 2] = ["martensite-build", "martensite-cmath"];

/// Selects the immutable PR comparison for the next toolkit migration.
#[derive(clap::Args)]
pub(crate) struct ObjectModelPrArgs {
    /// The exact feature-branch base commit of this step.
    #[arg(long)]
    base: String,
    /// Report selected suites without executing them.
    #[arg(long)]
    plan: bool,
}

pub(crate) fn run(context: &BuildContext<'_>, args: &ObjectModelPrArgs) -> BuildResult<()> {
    let (changed, selection) = resolve(context, &args.base)?;
    let controls = crate::thread_sanitizer_selection::select(&selection.tests, &changed)
        .into_iter()
        .filter(|control| selection.tests.contains(control.owner()))
        .collect::<Vec<_>>();
    report(context, &selection, &controls);
    if args.plan {
        return emit_conditions(&selection, &controls);
    }
    crate::pr::lint_changed_files(context, &args.base, &changed)?;
    qualification(context, &args.base, selection, &controls)
}

fn resolve(context: &BuildContext<'_>, base: &str) -> BuildResult<(Vec<ChangedFile>, Selection)> {
    verify_base(context, base)?;
    let order = schedule();
    let prior = parse_cursor(&context.git.file_at(base, CURSOR)?)?;
    let next = next_package(&order, prior)?;
    let changed = context.git.changed_files(base)?;
    let workspace = CargoWorkspace::load(context.runner, context.root())?;
    let cursor = candidate_progress(context, &workspace, next, &changed)?;
    let changed_packages =
        crate::release_impact::pull_request_packages(context, &workspace, base, &changed)?;
    let selection = select(&workspace, &order, prior, &cursor, &changed_packages)?;
    Ok((changed, selection))
}

fn candidate_progress(
    context: &BuildContext<'_>,
    workspace: &CargoWorkspace,
    next: &str,
    changed: &[ChangedFile],
) -> BuildResult<String> {
    let package = workspace
        .packages
        .iter()
        .find(|package| package.name == *next)
        .ok_or_else(|| {
            usage(format!(
                "migration package {next:?}; expected a workspace member"
            ))
        })?;
    authorize_paths(changed, next, &package.dir)?;
    let progress = context.root().join(CURSOR);
    std::fs::read_to_string(&progress).map_err(|error| crate::error::io_error(&progress, error))
}

fn verify_base(context: &BuildContext<'_>, base: &str) -> BuildResult<()> {
    if base.len() != 40 || !base.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(usage(format!(
            "migration base {base:?}; expected a full 40-digit commit SHA"
        )));
    }
    let common = context.git.merge_base(base)?;
    if common != base {
        return Err(usage(format!(
            "migration merge base {common:?}; expected the event base {base} to be an ancestor"
        )));
    }
    Ok(())
}

fn schedule() -> Vec<String> {
    LEVELS
        .split_whitespace()
        .map(|name| format!("martensite-{name}"))
        .collect()
}

fn parse_cursor(text: &str) -> BuildResult<usize> {
    text.trim()
        .parse()
        .map_err(|_| usage(format!("migration cursor {text:?}; expected a crate index")))
}

fn next_package(order: &[String], prior: usize) -> BuildResult<&str> {
    order.get(prior).map(String::as_str).ok_or_else(|| {
        usage(format!(
            "migration cursor {prior}; expected an unfinished step below {}",
            order.len()
        ))
    })
}

fn authorize_paths(changed: &[ChangedFile], name: &str, package_dir: &str) -> BuildResult<()> {
    let package_dir = format!("{package_dir}/");
    for file in changed {
        if !in_step(&file.path, &package_dir) {
            return Err(usage(format!(
                "migration path {:?}; expected this step's package {package_dir} or migration records",
                file.path
            )));
        }
    }
    let changed_crate = changed
        .iter()
        .any(|file| file.path.starts_with(&package_dir));
    if !changed_crate && !has_verification_record(changed, name) {
        return Err(usage(format!(
            "change set has no files in {package_dir}; expected the scheduled crate migration"
        )));
    }
    Ok(())
}

fn has_verification_record(changed: &[ChangedFile], name: &str) -> bool {
    if !VERIFICATION_ONLY.contains(&name) {
        return false;
    }
    let name = name.trim_start_matches("martensite-");
    let path = format!("{FEATURE_DOCS}{name}-verification.md");
    changed
        .iter()
        .any(|file| file.path == path && !file.deleted)
}

fn in_step(path: &str, package_dir: &str) -> bool {
    path.starts_with(package_dir)
        || path.starts_with(FEATURE_DOCS)
        || path == format!("{INVENTORY_DOCS}unsafe-source-baseline.json")
        || matches!(path, "Cargo.lock" | "CHANGELOG.md" | "README.md")
}

fn report(context: &BuildContext<'_>, selection: &Selection, controls: &[Control]) {
    let reporter = &context.reporter;
    reporter.note(&format!("Migration step: {}", selection.next));
    reporter.note(&format!("Library-only checks: {:?}", selection.libraries));
    reporter.note(&format!("Full crate suites: {:?}", selection.tests));
    reporter.note(&format!("Deferred crate suites: {:?}", selection.deferred));
    reporter.note(&format!("ThreadSanitizer controls: {controls:?}"));
}

fn emit_conditions(selection: &Selection, controls: &[Control]) -> BuildResult<()> {
    let outputs = [
        ("has-sanitizer", (!controls.is_empty()).to_string()),
        ("has-renderer", needs_renderer(&selection.tests).to_string()),
    ];
    let output = std::env::var_os("GITHUB_OUTPUT").map(std::path::PathBuf::from);
    crate::plan::write_outputs(&outputs, output.as_deref())
}

fn needs_renderer(tests: &BTreeSet<String>) -> bool {
    [
        "vulkan", "tkvcaf", "tkxcaf", "tkfeat", "tkoffset", "tkdestl", "tkmeshvs", "tkdestep",
        "tkdeiges", "tkdeobj",
    ]
    .into_iter()
    .any(|name| tests.contains(&format!("martensite-{name}")))
}

fn qualification(
    context: &BuildContext<'_>,
    base: &str,
    selection: Selection,
    controls: &[Control],
) -> BuildResult<()> {
    let target = BuildTarget::Triple(PRIMARY_TARGET.triple.into());
    let cargo = CargoTasks::new(context.runner, context.root());
    crate::compiler_inventory::capture_migration(
        context,
        base,
        &selection.libraries,
        &selection.tests,
    )?;
    if !selection.libraries.is_empty() {
        let packages = PackageSelection::Packages(selection.libraries.into_iter().collect());
        cargo.check_library_targets(&packages, &target)?;
    }
    if !selection.tests.is_empty() {
        let packages = PackageSelection::Packages(selection.tests.into_iter().collect());
        cargo.clippy(&packages, &target)?;
        cargo.build_all_targets(&packages, &target)?;
        cargo.test(&packages, &target)?;
    }
    crate::thread_sanitizer::run_selected(context, controls)
}

fn usage(message: String) -> BuildError {
    BuildError::Usage(message)
}

#[cfg(test)]
mod tests;
