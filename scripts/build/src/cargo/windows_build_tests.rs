use super::*;
use crate::fake_runner::RecordingRunner;

#[test]
fn driver_partition_preserves_every_other_package() {
    let driver = "martensite-build".to_owned();
    let other = "martensite-tkernel".to_owned();
    let cases = [
        (
            PackageSelection::Workspace,
            PackageSelection::WorkspaceExcept(vec![driver.clone()]),
        ),
        (
            PackageSelection::WorkspaceExcept(vec![other.clone()]),
            PackageSelection::WorkspaceExcept(vec![other.clone(), driver.clone()]),
        ),
        (
            PackageSelection::Packages(vec![driver.clone(), other.clone()]),
            PackageSelection::Packages(vec![other.clone()]),
        ),
    ];
    for (selection, expected) in cases {
        assert!(includes_build_driver(&selection));
        assert_eq!(without_build_driver(&selection), expected);
    }
    assert!(!includes_build_driver(&PackageSelection::WorkspaceExcept(
        vec![driver]
    )));
    assert!(!includes_build_driver(&PackageSelection::Packages(vec![
        other
    ])));
}

#[test]
fn isolated_driver_build_keeps_all_target_coverage() {
    let runner = RecordingRunner::default();
    CargoTasks::new(&runner, Path::new("/r"))
        .build_driver_separately(&PackageSelection::Workspace, &BuildTarget::Host)
        .unwrap();
    assert_eq!(
        runner.commands(),
        [
            "cargo build --locked --workspace --exclude martensite-build --all-targets",
            "cargo build --locked --package martensite-build --all-targets --target-dir target/build-driver-validation",
        ]
    );
}

#[test]
fn driver_only_build_never_selects_an_empty_package_set() {
    let runner = RecordingRunner::default();
    let selection = PackageSelection::Packages(vec!["martensite-build".into()]);
    CargoTasks::new(&runner, Path::new("/r"))
        .build_driver_separately(&selection, &BuildTarget::Host)
        .unwrap();
    assert_eq!(
        runner.commands(),
        [
            "cargo build --locked --package martensite-build --all-targets --target-dir target/build-driver-validation",
        ]
    );
}

#[test]
fn ordinary_and_cross_target_builds_keep_the_callers_target_directory() {
    let runner = RecordingRunner::default();
    let tasks = CargoTasks::new(&runner, Path::new("/r"));
    tasks
        .build_all_targets(
            &PackageSelection::Packages(vec!["martensite-tkernel".into()]),
            &BuildTarget::Host,
        )
        .unwrap();
    tasks
        .build_all_targets(
            &PackageSelection::Workspace,
            &BuildTarget::Triple("aarch64-apple-darwin".into()),
        )
        .unwrap();
    assert_eq!(
        runner.commands(),
        [
            "cargo build --locked --package martensite-tkernel --all-targets",
            "cargo build --locked --workspace --target aarch64-apple-darwin --all-targets",
        ]
    );
}
