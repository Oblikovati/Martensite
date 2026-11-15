//! Command-line parsing and CI dispatch tests.
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
fn thread_sanitizer_selects_the_fixed_verification_pipeline() {
    let cli = Cli::try_parse_from(["cargo xtask", "ci", "thread-sanitizer"]).unwrap();
    assert!(matches!(
        cli.command,
        BuildCommand::Ci(CiCommand::ThreadSanitizer)
    ));
}

#[test]
fn ci_run_arguments_are_owned_send_sync() {
    let args = CiRunArgs {
        target: "x86_64-unknown-linux-gnu".into(),
        checks_only: true,
        stage: None,
    };
    let args = std::sync::Arc::new(args);
    let shared = args.clone();
    let worker = std::thread::spawn(move || (shared.target.clone(), shared.checks_only));
    assert_eq!(worker.join().unwrap(), (args.target.clone(), true));
}

#[test]
fn ci_run_preserves_existing_target_and_checks_only_flags() {
    let cli = Cli::try_parse_from([
        "cargo xtask",
        "ci",
        "run",
        "--target",
        "x86_64-unknown-linux-gnu",
        "--checks-only",
    ])
    .unwrap();
    assert!(matches!(cli.command, BuildCommand::Ci(CiCommand::Run(args))
        if args.target == "x86_64-unknown-linux-gnu" && args.checks_only));
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
fn ci_run_stage_is_a_checks_only_selection() {
    let prefix = [
        "cargo xtask",
        "ci",
        "run",
        "--target",
        "x86_64-unknown-linux-gnu",
    ];
    let staged = ["--checks-only", "--stage", "clippy"];
    let cli = Cli::try_parse_from(prefix.into_iter().chain(staged)).unwrap();
    assert!(matches!(cli.command, BuildCommand::Ci(CiCommand::Run(args))
        if args.stage == Some(crate::pr::CheckStage::Clippy)));
    let without_checks_only = ["--stage", "build"];
    assert!(Cli::try_parse_from(prefix.into_iter().chain(without_checks_only)).is_err());
    assert!(
        Cli::try_parse_from(prefix.into_iter().chain(["--checks-only", "--stage", "x"])).is_err()
    );
}

#[test]
fn ci_test_part_belongs_to_a_package_job() {
    let prefix = [
        "cargo xtask",
        "ci",
        "test",
        "--target",
        "x86_64-unknown-linux-gnu",
    ];
    let package_part = ["--package", "martensite-vulkan", "--part", "2"];
    let cli = Cli::try_parse_from(prefix.into_iter().chain(package_part)).unwrap();
    assert!(matches!(
        cli.command,
        BuildCommand::Ci(CiCommand::Test { part: Some(2), .. })
    ));
    let shard_part = ["--shard", "0", "--part", "1"];
    assert!(Cli::try_parse_from(prefix.into_iter().chain(shard_part)).is_err());
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
