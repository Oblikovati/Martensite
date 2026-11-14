//! Local and CI test front doors must use the same script-budget compilation profile.
use super::*;
use crate::fake_runner::RecordingRunner;

const CI_SCRIPT_PACKAGES: [&str; 11] = [
    "martensite-tkbo",
    "martensite-boolean-cases",
    "martensite-tkbool",
    "martensite-tkfillet",
    "martensite-tkfeat",
    "martensite-tkoffset",
    "martensite-tkdestl",
    "martensite-tkmeshvs",
    "martensite-tkdestep",
    "martensite-tkdeiges",
    "martensite-tkdeobj",
];

fn specs(selection: PackageSelection) -> Vec<CommandSpec> {
    let runner = RecordingRunner::default();
    CargoTasks::new(&runner, Path::new("/repo"))
        .test(&selection, &BuildTarget::Host)
        .unwrap();
    runner.command_specs()
}
fn package_spec(name: &str) -> CommandSpec {
    CommandSpec::new("cargo", Path::new("/repo")).args([
        "test",
        "--locked",
        "--package",
        name,
        "--no-fail-fast",
    ])
}
fn optimized(spec: CommandSpec) -> CommandSpec {
    spec.environment("CARGO_PROFILE_DEV_OPT_LEVEL", Some("1"))
        .environment("CARGO_PROFILE_TEST_OPT_LEVEL", Some("1"))
}
#[test]
fn boolean_tests_bind_the_ci_optimization_profile() {
    let found = specs(PackageSelection::Packages(vec![
        "martensite-boolean-cases".into(),
    ]));
    assert_eq!(found, [optimized(package_spec("martensite-boolean-cases"))]);
}
#[test]
fn all_existing_ci_script_packages_keep_light_optimization() {
    for name in CI_SCRIPT_PACKAGES {
        assert_eq!(
            specs(PackageSelection::Packages(vec![name.into()])),
            [optimized(package_spec(name))]
        );
    }
}
#[test]
fn unrelated_package_keeps_its_original_ambient_profile() {
    assert_eq!(
        specs(PackageSelection::Packages(vec!["martensite-tkmath".into()])),
        [package_spec("martensite-tkmath")]
    );
}
#[test]
fn mixed_selection_runs_each_package_once_without_losing_case_runner_features() {
    let found = specs(PackageSelection::Packages(vec![
        "martensite-boolean-cases".into(),
        "martensite-process-policy".into(),
    ]));
    let ordinary = package_spec("martensite-process-policy")
        .args(["--features", "martensite-process-policy/case-runner"]);
    assert_eq!(
        found,
        [
            ordinary,
            optimized(package_spec("martensite-boolean-cases"))
        ]
    );
}
#[test]
fn workspace_exclusions_apply_to_both_profile_groups() {
    let excluded = "martensite-boolean-cases".to_owned();
    let runner = workspace_runner(&[
        "martensite-boolean-cases",
        "martensite-tkmath",
        "martensite-tkbool",
        "martensite-process-policy",
    ]);
    let groups = script_budget_profiles::groups(
        &PackageSelection::WorkspaceExcept(vec![excluded.clone()]),
        &runner,
        Path::new("/repo"),
    )
    .unwrap();
    assert_eq!(groups.len(), 2);
    assert_membership_count(&groups, &excluded, 0);
    assert_single_membership(&groups);
}
fn assert_single_membership(groups: &[script_budget_profiles::TestGroup]) {
    for name in [
        "martensite-tkmath",
        "martensite-tkbool",
        "martensite-process-policy",
    ] {
        assert_membership_count(groups, name, 1);
    }
}

fn assert_membership_count(
    groups: &[script_budget_profiles::TestGroup],
    name: &str,
    expected: usize,
) {
    assert_eq!(
        groups
            .iter()
            .filter(|g| g.selection.includes_package(name))
            .count(),
        expected
    );
}

#[test]
fn a_failing_group_does_not_skip_the_other_profile_group() {
    let runner = RecordingRunner::default().with_exit_code(
        "cargo test --locked --package martensite-tkmath --no-fail-fast",
        1,
    );
    let selection = PackageSelection::Packages(vec![
        "martensite-tkmath".into(),
        "martensite-boolean-cases".into(),
    ]);
    assert!(
        CargoTasks::new(&runner, Path::new("/repo"))
            .test(&selection, &BuildTarget::Host)
            .is_err()
    );
    assert_eq!(runner.commands().len(), 2);
}
#[test]
fn workflow_uses_driver_package_policy_and_preserves_release_wide_optimization() {
    let workflow =
        include_str!("../../../../.github/workflows/martensite.yml").replace("\r\n", "\n");
    let package_job = workflow
        .split("  tests:\n")
        .nth(1)
        .unwrap()
        .split("  release-tests:\n")
        .next()
        .unwrap();
    assert!(!package_job.contains("CARGO_PROFILE_DEV_OPT_LEVEL"));
    let release_job = workflow
        .split("  release-tests:\n")
        .nth(1)
        .unwrap()
        .split("  thread-sanitizer:\n")
        .next()
        .unwrap();
    assert!(release_job.contains("CARGO_PROFILE_DEV_OPT_LEVEL: '1'"));
}

pub(super) fn workspace_runner(names: &[&str]) -> RecordingRunner {
    let packages: Vec<_> = names
        .iter()
        .map(|name| {
            serde_json::json!({
                "name": name, "version": "0.1.0", "dependencies": [],
                "manifest_path": format!("/repo/crates/{name}/Cargo.toml"),
            })
        })
        .collect();
    let metadata = serde_json::json!({
        "workspace_root": "/repo", "target_directory": "/repo/target", "packages": packages,
    });
    RecordingRunner::default().with_stdout(
        "cargo metadata --format-version 1 --no-deps --locked",
        &metadata.to_string(),
    )
}
#[test]
fn a_reduced_workspace_never_selects_absent_script_packages() {
    let runner = workspace_runner(&["martensite-tkmath"]);
    CargoTasks::new(&runner, Path::new("/repo"))
        .test(&PackageSelection::Workspace, &BuildTarget::Host)
        .unwrap();
    assert_eq!(
        runner.command_specs().last().unwrap(),
        &package_spec("martensite-tkmath")
    );
    assert_eq!(runner.commands().len(), 2);
}
