//! Cargo invocation tests.
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
    let runner = script_budget_profile_tests::workspace_runner(&[
        "martensite-process-policy",
        "martensite-boolean-cases",
    ]);
    let tasks = CargoTasks::new(&runner, Path::new("/repo"));
    let target = BuildTarget::Host;
    tasks.clippy(&PackageSelection::Workspace, &target).unwrap();
    tasks
        .build_all_targets(&PackageSelection::Workspace, &target)
        .unwrap();
    tasks.test(&PackageSelection::Workspace, &target).unwrap();
    tasks.package_sources(&PackageSelection::Workspace).unwrap();
    let commands = runner.commands();
    assert_eq!(commands.len(), 6 + usize::from(cfg!(windows)));
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
