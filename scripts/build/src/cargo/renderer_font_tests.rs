//! Recorded renderer fonts belong to the gate's child process, not the user's shell (#433).
use super::*;
use crate::fake_runner::RecordingRunner;

fn test_spec(root: &Path, selection: &PackageSelection, target: &BuildTarget) -> CommandSpec {
    let runner = RecordingRunner::default();
    CargoTasks::new(&runner, root)
        .test(selection, target)
        .unwrap();
    runner.command_specs().remove(0)
}
fn unbound_test_spec(
    root: &Path,
    selection: &PackageSelection,
    target: &BuildTarget,
) -> CommandSpec {
    let runner = RecordingRunner::default();
    let mut spec = CargoTasks::new(&runner, root)
        .selected("test", selection, target)
        .arg("--no-fail-fast");
    if tests_case_runner(selection) {
        spec = spec.args(["--features", "martensite-process-policy/case-runner"]);
    }
    spec
}
fn bound_test_spec(root: &Path, selection: &PackageSelection, target: &BuildTarget) -> CommandSpec {
    let config = root.join("scripts/install/renderer-fontconfig.conf");
    unbound_test_spec(root, selection, target)
        .environment_os("FONTCONFIG_FILE", Some(config.as_os_str()))
}
#[test]
fn linux_renderer_test_binds_the_recorded_font_configuration() {
    let root = Path::new("/repo");
    let selection = PackageSelection::Packages(vec!["martensite-vulkan".into()]);
    let target = BuildTarget::Triple("x86_64-unknown-linux-gnu".into());
    assert_eq!(
        test_spec(root, &selection, &target),
        bound_test_spec(root, &selection, &target)
    );
}
fn renderer_membership_cases() -> [(PackageSelection, bool); 5] {
    use PackageSelection::{Packages, Workspace, WorkspaceExcept};
    [
        (Workspace, true),
        (WorkspaceExcept(vec!["martensite-tkmath".into()]), true),
        (WorkspaceExcept(vec!["martensite-vulkan".into()]), false),
        (Packages(vec!["martensite-tkmath".into()]), false),
        (
            Packages(vec!["martensite-tkmath".into(), "martensite-vulkan".into()]),
            true,
        ),
    ]
}
fn assert_font_binding(selection: &PackageSelection, target: &BuildTarget, bound: bool) {
    let root = Path::new("/repo");
    let expected = if bound {
        bound_test_spec(root, selection, target)
    } else {
        unbound_test_spec(root, selection, target)
    };
    assert_eq!(test_spec(root, selection, target), expected);
}
#[test]
fn workspace_and_explicit_renderer_membership_control_font_binding() {
    let target = BuildTarget::Triple("aarch64-unknown-linux-gnu".into());
    for (selection, renderer) in renderer_membership_cases() {
        assert_font_binding(&selection, &target, renderer);
    }
}
#[test]
fn non_linux_targets_keep_their_existing_font_environment() {
    let selection = PackageSelection::Packages(vec!["martensite-vulkan".into()]);
    for triple in ["aarch64-apple-darwin", "x86_64-pc-windows-msvc"] {
        assert_font_binding(&selection, &BuildTarget::Triple(triple.into()), false);
    }
    assert_font_binding(&selection, &BuildTarget::Host, cfg!(target_os = "linux"));
}
fn expected_non_test_specs(
    cargo: &CargoTasks<'_>,
    selection: &PackageSelection,
) -> [CommandSpec; 2] {
    let target = BuildTarget::Host;
    [
        cargo
            .selected("build", selection, &target)
            .arg("--all-targets"),
        cargo.selected("clippy", selection, &target).args([
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ]),
    ]
}
#[test]
fn build_and_clippy_do_not_bind_renderer_font_inputs() {
    let runner = RecordingRunner::default();
    let cargo = CargoTasks::new(&runner, Path::new("/repo"));
    let selection = PackageSelection::Packages(vec!["martensite-vulkan".into()]);
    cargo
        .build_all_targets(&selection, &BuildTarget::Host)
        .unwrap();
    cargo.clippy(&selection, &BuildTarget::Host).unwrap();
    assert_eq!(
        runner.command_specs(),
        expected_non_test_specs(&cargo, &selection)
    );
}
#[cfg(unix)]
#[test]
fn renderer_configuration_keeps_non_utf8_workspace_path_bytes() {
    use std::os::unix::ffi::OsStringExt;
    let root = PathBuf::from(std::ffi::OsString::from_vec(b"/repo-\xff".to_vec()));
    let selection = PackageSelection::Packages(vec!["martensite-vulkan".into()]);
    let target = BuildTarget::Triple("x86_64-unknown-linux-gnu".into());
    assert_eq!(
        test_spec(&root, &selection, &target),
        bound_test_spec(&root, &selection, &target)
    );
}

const CI_RENDERER_SETUP: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../.github/actions/setup-renderer-tests/action.yml"
));
const RECORDED_FONT_CONFIG: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../install/renderer-fontconfig.conf"
));
#[test]
fn renderer_font_binding_matches_ci_recording_setup() {
    assert!(
        CI_RENDERER_SETUP
            .contains("FONTCONFIG_FILE=$GITHUB_WORKSPACE/scripts/install/renderer-fontconfig.conf")
    );
    assert!(RECORDED_FONT_CONFIG.contains("<dir>/usr/share/fonts</dir>"));
}
