use super::{CaseRunner, CaseSpec, FactorProfile, spec::CHILD_ENV};

fn inherited_live_spec(identity: &str) -> CaseSpec<'_> {
    let profile = FactorProfile::RequireUnset;
    CaseSpec::inherit_process_policy(identity, profile)
}

fn owned_live_root() -> std::path::PathBuf {
    let parent = std::env::temp_dir();
    parent.join("martensite-inherited-process-policy-controls")
}

#[test]
fn actual_fresh_child_preserves_untouched_application_policy_and_v2_receipt() {
    let parent = std::env::var_os(CHILD_ENV).is_none();
    let before = crate::reported_process_limit();
    let root = owned_live_root();
    let identity = "case_runner::inherited_live_controls::actual_fresh_child_preserves_untouched_application_policy_and_v2_receipt";
    let receipt = CaseRunner::new(&root, None)
        .run(inherited_live_spec(identity), || {
            assert_eq!(crate::reported_process_limit(), 0);
            println!("inherited application policy stayed untouched");
            Ok(())
        })
        .unwrap();
    if parent {
        assert_eq!(crate::reported_process_limit(), before);
        assert_inherited_live_records(&receipt.directory);
    }
}

fn assert_inherited_live_records(directory: &std::path::Path) {
    let request = std::fs::read_to_string(directory.join("request.txt")).unwrap();
    assert!(request.starts_with("MARTENSITE_EXACT_CASE_V2\n"));
    assert_eq!(
        std::fs::read_to_string(directory.join("entry.txt")).unwrap(),
        request
    );
    assert_eq!(
        std::fs::read_to_string(directory.join("complete.txt")).unwrap(),
        request
    );
    let receipt = std::fs::read_to_string(directory.join("receipt.json")).unwrap();
    assert!(receipt.contains("\"runner_policy_installed\":false"));
    assert!(receipt.contains("\"budget_seconds\":null"));
}

#[cfg(unix)]
fn inherited_unix_finite_policy() -> rustix::process::Rlimit {
    use rustix::process::{Resource, Rlimit, getrlimit, setrlimit};
    let baseline = getrlimit(Resource::Cpu);
    let requested = Rlimit {
        current: Some(baseline.maximum.unwrap_or(200).min(200)),
        maximum: baseline.maximum,
    };
    if std::env::var_os(CHILD_ENV).is_some() {
        setrlimit(Resource::Cpu, requested).unwrap();
    }
    requested
}

#[cfg(unix)]
#[test]
fn actual_child_inherits_finite_os_limit_without_clearing_or_installing_policy() {
    let expected = inherited_unix_finite_policy();
    let root = owned_live_root();
    let identity = "case_runner::inherited_live_controls::actual_child_inherits_finite_os_limit_without_clearing_or_installing_policy";
    CaseRunner::new(&root, None)
        .run(inherited_live_spec(identity), || {
            assert_eq!(
                rustix::process::getrlimit(rustix::process::Resource::Cpu),
                expected
            );
            assert_eq!(crate::reported_process_limit(), 0);
            Ok(())
        })
        .unwrap();
}

#[cfg(unix)]
#[test]
fn actual_prepared_child_preserves_current_os_policy_and_owned_preparation() {
    let expected = rustix::process::getrlimit(rustix::process::Resource::Cpu);
    let root = owned_live_root();
    let identity = "case_runner::inherited_live_controls::actual_prepared_child_preserves_current_os_policy_and_owned_preparation";
    CaseRunner::new(&root, None)
        .run_prepared(
            inherited_live_spec(identity),
            || Ok(String::from("owned before policy boundary")),
            |owned| {
                assert_eq!(owned, "owned before policy boundary");
                assert_eq!(
                    rustix::process::getrlimit(rustix::process::Resource::Cpu),
                    expected
                );
                assert_eq!(crate::reported_process_limit(), 0);
                Ok(())
            },
        )
        .unwrap();
}
