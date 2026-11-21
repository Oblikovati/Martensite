//! Cargo invocations shared by the PR and release pipelines.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{BuildError, BuildResult};
use crate::process::{CommandRunner, CommandSpec};
mod script_budget_profiles;
mod test_build;
mod test_parts;
pub use test_parts::TestPart;

/// Which packages a cargo command acts on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageSelection {
    /// Every workspace member.
    Workspace,
    /// Every workspace member except the named ones.
    WorkspaceExcept(Vec<String>),
    /// Only the named packages.
    Packages(Vec<String>),
}

impl PackageSelection {
    fn includes_package(&self, package: &str) -> bool {
        match self {
            Self::Workspace => true,
            Self::WorkspaceExcept(names) => !names.iter().any(|name| name == package),
            Self::Packages(names) => names.iter().any(|name| name == package),
        }
    }

    fn cargo_args(&self) -> Vec<String> {
        match self {
            Self::Workspace => vec!["--workspace".into()],
            Self::WorkspaceExcept(names) => flagged("--workspace", "--exclude", names),
            Self::Packages(names) => names
                .iter()
                .flat_map(|name| ["--package".into(), name.clone()])
                .collect(),
        }
    }
}

fn flagged(head: &str, flag: &str, names: &[String]) -> Vec<String> {
    let pairs = names
        .iter()
        .flat_map(|name| [flag.to_owned(), name.clone()]);
    std::iter::once(head.to_owned()).chain(pairs).collect()
}

/// The compilation target; `Host` lets cargo use the toolchain's host triple.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildTarget {
    /// The machine running the build.
    Host,
    /// An explicit target triple such as `x86_64-unknown-linux-gnu`.
    Triple(String),
}

impl BuildTarget {
    /// The explicit triple, or `None` for the host.
    pub fn triple(&self) -> Option<&str> {
        match self {
            Self::Host => None,
            Self::Triple(triple) => Some(triple),
        }
    }

    fn cargo_args(&self) -> Vec<String> {
        match self {
            Self::Host => Vec::new(),
            Self::Triple(triple) => vec!["--target".into(), triple.clone()],
        }
    }
}

/// A compiled file a release ships: an executable or a linkable library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShippedArtifact {
    /// Where cargo wrote the file.
    pub path: PathBuf,
    /// True for executables, false for libraries.
    pub executable: bool,
}

/// Target kinds whose outputs are useful outside cargo; `lib`/`rlib`
/// outputs are only consumable through the source packages.
const SHIPPED_LIBRARY_KINDS: [&str; 3] = ["cdylib", "staticlib", "dylib"];

#[derive(Deserialize)]
struct CompilerMessage {
    reason: String,
    #[serde(default)]
    target: Option<MessageTarget>,
    #[serde(default)]
    filenames: Vec<PathBuf>,
    #[serde(default)]
    executable: Option<PathBuf>,
}

#[derive(Deserialize)]
struct MessageTarget {
    kind: Vec<String>,
}

/// Runs cargo subcommands in the workspace root.
pub struct CargoTasks<'a> {
    runner: &'a dyn CommandRunner,
    root: &'a Path,
}

impl<'a> CargoTasks<'a> {
    /// Binds cargo calls to a runner and the workspace root.
    ///
    /// ```ignore
    /// CargoTasks::new(&ProcessRunner, root).test(&PackageSelection::Workspace, &BuildTarget::Host)?;
    /// ```
    pub fn new(runner: &'a dyn CommandRunner, root: &'a Path) -> Self {
        Self { runner, root }
    }

    /// `cargo fmt --check` over these packages, one at a time: `cargo fmt --all` hands every
    /// target of the workspace to one rustfmt command, past the Windows command line limit.
    /// Reports every unformatted package, not only the first.
    pub fn format_check_packages(&self, names: &[String]) -> BuildResult<()> {
        let outcomes: Vec<_> = names
            .iter()
            .map(|name| {
                self.runner
                    .run(&self.cargo(&["fmt", "--package", name, "--check"]))
            })
            .collect();
        outcomes.into_iter().collect()
    }

    /// Clippy over all targets with every warning promoted to an error.
    pub fn clippy(&self, selection: &PackageSelection, target: &BuildTarget) -> BuildResult<()> {
        let spec = self.selected("clippy", selection, target).args([
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ]);
        self.runner.run(&spec)
    }

    /// Clippy and build a library whose dev dependencies cannot yet compile.
    pub(crate) fn check_library_targets(
        &self,
        selection: &PackageSelection,
        target: &BuildTarget,
    ) -> BuildResult<()> {
        self.runner.run(
            &self
                .selected("clippy", selection, target)
                .args(["--lib", "--", "-D", "warnings"]),
        )?;
        self.runner
            .run(&self.selected("build", selection, target).arg("--lib"))
    }

    /// Debug build of every target kind (lib, bins, tests, examples, benches).
    pub fn build_all_targets(
        &self,
        selection: &PackageSelection,
        target: &BuildTarget,
    ) -> BuildResult<()> {
        if cfg!(windows) && matches!(target, BuildTarget::Host) && includes_build_driver(selection)
        {
            return self.build_driver_separately(selection, target);
        }
        self.runner.run(
            &self
                .selected("build", selection, target)
                .arg("--all-targets"),
        )
    }

    fn build_driver_separately(
        &self,
        selection: &PackageSelection,
        target: &BuildTarget,
    ) -> BuildResult<()> {
        let ordinary = without_build_driver(selection);
        if !matches!(&ordinary, PackageSelection::Packages(names) if names.is_empty()) {
            self.runner.run(
                &self
                    .selected("build", &ordinary, target)
                    .arg("--all-targets"),
            )?;
        }
        // Windows locks the running xtask image. Rebuild it, including every
        // target kind, away from that image without changing the caller's target dir (#339).
        let driver = PackageSelection::Packages(vec!["martensite-build".into()]);
        self.runner
            .run(&self.selected("build", &driver, target).args([
                "--all-targets",
                "--target-dir",
                "target/build-driver-validation",
            ]))
    }

    /// Runs unit, integration and doc tests with CI's package budget profiles.
    /// A failed profile group does not skip later groups; other packages keep their ambient profile.
    /// Example: `tasks.test(&selection, &BuildTarget::Host)?;`.
    pub fn test(&self, selection: &PackageSelection, target: &BuildTarget) -> BuildResult<()> {
        let outcomes: Vec<_> = script_budget_profiles::groups(selection, self.runner, self.root)?
            .into_iter()
            .map(|group| self.test_group(&group.selection, target, group.optimization))
            .collect();
        outcomes.into_iter().collect()
    }
    fn test_group(
        &self,
        selection: &PackageSelection,
        target: &BuildTarget,
        optimization: script_budget_profiles::Optimization,
    ) -> BuildResult<()> {
        self.runner
            .run(&self.test_spec(selection, target, optimization))
    }

    fn test_spec(
        &self,
        selection: &PackageSelection,
        target: &BuildTarget,
        optimization: script_budget_profiles::Optimization,
    ) -> CommandSpec {
        let mut spec = self
            .selected("test", selection, target)
            .arg("--no-fail-fast");
        if tests_case_runner(selection) {
            spec = spec.args(["--features", "martensite-process-policy/case-runner"]);
        }
        renderer_test_environment(optimization.apply(spec), self.root, selection, target)
    }

    /// Release build that returns the shippable artifacts cargo reported.
    pub fn release_build(
        &self,
        selection: &PackageSelection,
        target: &BuildTarget,
    ) -> BuildResult<Vec<ShippedArtifact>> {
        let spec = self.selected("build", selection, target).args([
            "--release",
            "--message-format",
            "json-render-diagnostics",
        ]);
        shipped_artifacts(&self.runner.capture(&spec)?)
    }

    /// `cargo package` (with its isolated verification build) into `target/package`.
    pub fn package_sources(&self, selection: &PackageSelection) -> BuildResult<()> {
        let spec = self
            .cargo(&["package", "--locked"])
            .args(selection.cargo_args());
        self.runner.run(&spec)
    }

    /// The host triple reported by `rustc -vV`.
    pub fn host_triple(&self) -> BuildResult<String> {
        let spec = CommandSpec::new("rustc", self.root).arg("-vV");
        parse_host_triple(&self.runner.capture(&spec)?)
    }

    fn selected(
        &self,
        subcommand: &str,
        selection: &PackageSelection,
        target: &BuildTarget,
    ) -> CommandSpec {
        self.cargo(&[subcommand, "--locked"])
            .args(selection.cargo_args())
            .args(target.cargo_args())
    }

    fn cargo(&self, args: &[&str]) -> CommandSpec {
        CommandSpec::new("cargo", self.root).args(args.iter().copied())
    }
}

// Native image references were recorded with this system-font configuration (#433).
fn renderer_test_environment(
    spec: CommandSpec,
    root: &Path,
    selection: &PackageSelection,
    target: &BuildTarget,
) -> CommandSpec {
    let linux = target
        .triple()
        .map_or(cfg!(target_os = "linux"), |triple| triple.contains("linux"));
    if !linux || !selection.includes_package("martensite-vulkan") {
        return spec;
    }
    let config = root.join("scripts/install/renderer-fontconfig.conf");
    spec.environment_os("FONTCONFIG_FILE", Some(config.as_os_str()))
}

/// Extracts executables and linkable libraries from cargo's JSON messages.
fn shipped_artifacts(stdout: &str) -> BuildResult<Vec<ShippedArtifact>> {
    let mut artifacts = Vec::new();
    for line in stdout.lines().filter(|line| line.starts_with('{')) {
        let message: CompilerMessage =
            serde_json::from_str(line).map_err(|error| BuildError::UnexpectedOutput {
                origin: "cargo build --message-format json".into(),
                detail: format!("{error} in {line:?}; expected one JSON message per line"),
            })?;
        artifacts.extend(artifacts_of(message));
    }
    artifacts.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(artifacts)
}

fn artifacts_of(message: CompilerMessage) -> Vec<ShippedArtifact> {
    if message.reason != "compiler-artifact" {
        return Vec::new();
    }
    if let Some(path) = message.executable {
        return vec![ShippedArtifact {
            path,
            executable: true,
        }];
    }
    library_artifacts(message)
}

fn library_artifacts(message: CompilerMessage) -> Vec<ShippedArtifact> {
    let kinds = message.target.map(|target| target.kind).unwrap_or_default();
    if !kinds
        .iter()
        .any(|kind| SHIPPED_LIBRARY_KINDS.contains(&kind.as_str()))
    {
        return Vec::new();
    }
    let libraries = message
        .filenames
        .into_iter()
        .filter(|path| !is_rust_only(path));
    libraries
        .map(|path| ShippedArtifact {
            path,
            executable: false,
        })
        .collect()
}

/// `.rlib`/`.rmeta` sit beside cdylibs in the same message but need rustc.
fn is_rust_only(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("rlib" | "rmeta")
    )
}

fn parse_host_triple(version_output: &str) -> BuildResult<String> {
    version_output
        .lines()
        .find_map(|line| line.strip_prefix("host: "))
        .map(|triple| triple.trim().to_owned())
        .ok_or_else(|| BuildError::UnexpectedOutput {
            origin: "rustc -vV".into(),
            detail: format!("{version_output:?}; expected a `host: <triple>` line"),
        })
}

fn includes_build_driver(selection: &PackageSelection) -> bool {
    selection.includes_package("martensite-build")
}

fn without_build_driver(selection: &PackageSelection) -> PackageSelection {
    match selection {
        PackageSelection::Workspace => {
            PackageSelection::WorkspaceExcept(vec!["martensite-build".into()])
        }
        PackageSelection::WorkspaceExcept(names) => {
            let mut names = names.clone();
            names.push("martensite-build".into());
            PackageSelection::WorkspaceExcept(names)
        }
        PackageSelection::Packages(names) => PackageSelection::Packages(
            names
                .iter()
                .filter(|name| *name != "martensite-build")
                .cloned()
                .collect(),
        ),
    }
}

// Per-package jobs must retain the case-runner tests previously enabled by
// dataset toolkit dependencies in a combined Cargo invocation (#250).
fn tests_case_runner(selection: &PackageSelection) -> bool {
    selection.includes_package("martensite-process-policy")
}

#[cfg(test)]
mod windows_build_tests;

#[cfg(test)]
mod renderer_font_tests;
#[cfg(test)]
mod script_budget_profile_tests;

#[cfg(test)]
mod tests;
