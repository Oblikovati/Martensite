//! Cargo invocations shared by the PR and release pipelines.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::error::{BuildError, BuildResult};
use crate::process::{CommandRunner, CommandSpec};

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

    /// `cargo fmt --all --check` over the whole workspace.
    pub fn format_check_all(&self) -> BuildResult<()> {
        self.runner.run(&self.cargo(&["fmt", "--all", "--check"]))
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

    /// Runs unit, integration and doc tests; keeps going past the first failure.
    pub fn test(&self, selection: &PackageSelection, target: &BuildTarget) -> BuildResult<()> {
        let mut spec = self
            .selected("test", selection, target)
            .arg("--no-fail-fast");
        if tests_case_runner(selection) {
            spec = spec.args(["--features", "martensite-process-policy/case-runner"]);
        }
        self.runner.run(&spec)
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
    match selection {
        PackageSelection::Workspace => true,
        PackageSelection::WorkspaceExcept(names) => {
            !names.iter().any(|name| name == "martensite-build")
        }
        PackageSelection::Packages(names) => names.iter().any(|name| name == "martensite-build"),
    }
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
    match selection {
        PackageSelection::Workspace => true,
        PackageSelection::WorkspaceExcept(names) => {
            !names.iter().any(|name| name == "martensite-process-policy")
        }
        PackageSelection::Packages(names) => {
            names.iter().any(|name| name == "martensite-process-policy")
        }
    }
}

#[cfg(test)]
mod windows_build_tests;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isolated_process_policy_keeps_case_runner_coverage() {
        use crate::fake_runner::RecordingRunner;
        let runner = RecordingRunner::default();
        let cargo = CargoTasks::new(&runner, Path::new("/repo"));
        let policy = vec!["martensite-process-policy".to_owned()];
        cargo
            .test(
                &PackageSelection::Packages(policy.clone()),
                &BuildTarget::Host,
            )
            .unwrap();
        assert_eq!(
            runner.commands(),
            [
                "cargo test --locked --package martensite-process-policy --no-fail-fast --features martensite-process-policy/case-runner"
            ]
        );
        assert!(tests_case_runner(&PackageSelection::Workspace));
        assert!(!tests_case_runner(&PackageSelection::WorkspaceExcept(
            policy
        )));
        assert!(!tests_case_runner(&PackageSelection::Packages(vec![
            "martensite-tkmath".into()
        ])));
    }
    use crate::fake_runner::RecordingRunner;

    #[test]
    fn selections_render_as_cargo_flags() {
        let except = PackageSelection::WorkspaceExcept(vec!["tool".into()]);
        assert_eq!(except.cargo_args(), ["--workspace", "--exclude", "tool"]);
        let some = PackageSelection::Packages(vec!["a".into(), "b".into()]);
        assert_eq!(some.cargo_args(), ["--package", "a", "--package", "b"]);
    }

    #[test]
    fn gate_commands_exclude_host_math_and_optional_integration_features() {
        // `host-libm` (platform C library results) and `libm-attribution` must never reach
        // a PR or release gate. Only case-runner coverage is selected explicitly (#250).
        let runner = RecordingRunner::default();
        let tasks = CargoTasks::new(&runner, Path::new("/r"));
        let target = BuildTarget::Host;
        tasks.clippy(&PackageSelection::Workspace, &target).unwrap();
        tasks
            .build_all_targets(&PackageSelection::Workspace, &target)
            .unwrap();
        tasks.test(&PackageSelection::Workspace, &target).unwrap();
        tasks.package_sources(&PackageSelection::Workspace).unwrap();
        let commands = runner.commands();
        assert_eq!(commands.len(), 4 + usize::from(cfg!(windows)));
        for command in commands {
            let command = command.replace(" --features martensite-process-policy/case-runner", "");
            assert!(
                !command.contains("--features") && !command.contains("--all-features"),
                "{command}"
            );
        }
    }

    #[test]
    fn clippy_denies_warnings_for_an_explicit_target() {
        let runner = RecordingRunner::default();
        let target = BuildTarget::Triple("aarch64-apple-darwin".into());
        CargoTasks::new(&runner, Path::new("/r"))
            .clippy(&PackageSelection::Workspace, &target)
            .unwrap();
        assert_eq!(
            runner.commands(),
            [
                "cargo clippy --locked --workspace --target aarch64-apple-darwin --all-targets -- -D warnings"
            ]
        );
    }

    #[test]
    fn shipped_artifacts_keep_bins_and_native_libraries_only() {
        let stdout = [
            r#"{"reason":"compiler-artifact","target":{"kind":["bin"]},"filenames":["/t/tool"],"executable":"/t/tool"}"#,
            r#"{"reason":"compiler-artifact","target":{"kind":["lib"]},"filenames":["/t/libm.rlib"],"executable":null}"#,
            r#"{"reason":"compiler-artifact","target":{"kind":["cdylib","rlib"]},"filenames":["/t/libk.so","/t/libk.rlib"]}"#,
            r#"{"reason":"build-finished","success":true}"#,
        ]
        .join("\n");
        let artifacts = shipped_artifacts(&stdout).unwrap();
        let found: Vec<_> = artifacts
            .iter()
            .map(|a| (a.path.to_str().unwrap(), a.executable))
            .collect();
        assert_eq!(found, [("/t/libk.so", false), ("/t/tool", true)]);
    }

    #[test]
    fn malformed_cargo_message_names_the_line() {
        let error = shipped_artifacts("{not json").unwrap_err();
        assert!(error.to_string().contains("{not json"));
    }

    #[test]
    fn host_triple_is_read_from_rustc() {
        let output = "rustc 1.99.0\nhost: x86_64-unknown-linux-gnu\nrelease: 1.99.0\n";
        assert_eq!(
            parse_host_triple(output).unwrap(),
            "x86_64-unknown-linux-gnu"
        );
        assert!(parse_host_triple("rustc 1.99.0").is_err());
    }
}
