use super::*;
use crate::workspace::WorkspacePackage;

fn names(list: &str) -> BTreeSet<String> {
    list.split_whitespace().map(str::to_owned).collect()
}

fn fixture() -> (CargoWorkspace, Vec<String>) {
    let order: Vec<String> = ["foundation", "renderer", "mesh", "topology"]
        .map(str::to_owned)
        .into();
    let mut packages = order
        .iter()
        .chain(SCOPE_EXEMPTIONS.map(str::to_owned).iter())
        .map(|name| crate::published_history::fixtures::package(name, "1.0.0"))
        .collect::<Vec<_>>();
    add_edge(&mut packages, "renderer", "foundation", true);
    add_edge(&mut packages, "renderer", "mesh", false);
    add_edge(&mut packages, "topology", "mesh", true);
    let workspace = CargoWorkspace {
        packages,
        target_directory: "target".into(),
    };
    (workspace, order)
}

fn add_edge(packages: &mut [WorkspacePackage], owner: &str, name: &str, is_library: bool) {
    let package = packages
        .iter_mut()
        .find(|package| package.name == owner)
        .unwrap();
    package.path_dependencies.push(name.into());
    if is_library {
        package.library_path_dependencies.push(name.into());
    }
}

fn file(path: &str) -> ChangedFile {
    ChangedFile {
        path: path.to_owned(),
        deleted: false,
        untracked: false,
    }
}

#[test]
fn first_step_selects_only_its_crate_even_when_all_consumers_are_affected() {
    let (workspace, order) = fixture();
    let selected = select(
        &workspace,
        &order,
        0,
        "1\n",
        &names("foundation renderer mesh topology"),
    )
    .unwrap();
    assert_eq!(selected.next, "foundation");
    assert_eq!(selected.tests, names("foundation"));
    assert!(selected.libraries.is_empty());
    assert!(selected.deferred.is_empty());
}

#[test]
fn pending_dev_dependency_blocks_test_targets_and_eligibility_replays_them() {
    let (workspace, order) = fixture();
    let stage = select(&workspace, &order, 1, "2", &names("renderer")).unwrap();
    assert_eq!(stage.libraries, names("renderer"));
    assert_eq!(stage.deferred, names("renderer"));
    assert!(stage.tests.is_empty());

    let next = select(&workspace, &order, 2, "3", &names("mesh topology")).unwrap();
    assert_eq!(next.tests, names("mesh renderer"));
    assert!(next.deferred.is_empty());
}

#[test]
fn vetted_paths_admit_only_the_next_crate_and_its_records() {
    let package = file("crates/martensite-tkernel/src/standard.rs");
    let cursor = file(CURSOR);
    let baseline = file("docs/features/safe-toolkits/unsafe-source-baseline.json");
    let lock = file("Cargo.lock");
    assert!(
        authorize_paths(
            &[package.clone(), cursor.clone(), baseline, lock],
            "martensite-tkernel",
            "crates/martensite-tkernel"
        )
        .is_ok()
    );
    assert!(authorize_paths(&[cursor], "martensite-tkernel", "crates/martensite-tkernel").is_err());
    for refused in [
        ".github/workflows/martensite.yml",
        ".github/actions/setup-target/action.yml",
        ".cargo/config.toml",
        "Cargo.toml",
        "crates/martensite-tkmath/src/lib.rs",
        "scripts/build/src/object_model_pr.rs",
        "scripts/build/src/object_model_pr/levels.txt",
        "docs/features/safe-toolkits/compiler-baselines/x86_64-unknown-linux-gnu.json",
    ] {
        assert!(
            authorize_paths(
                &[package.clone(), file(refused)],
                "martensite-tkernel",
                "crates/martensite-tkernel"
            )
            .is_err(),
            "{refused}"
        );
    }
}

#[test]
fn out_of_order_cursor_or_production_edges_fail_closed() {
    let (mut workspace, order) = fixture();
    let changed = names("foundation");
    for cursor in ["oops", "0", "2", "5"] {
        assert!(select(&workspace, &order, 0, cursor, &changed).is_err());
    }
    assert!(select(&workspace, &order, order.len(), "5", &changed).is_err());
    assert!(select(&workspace, &order[..3], 0, "1", &changed).is_err());
    add_edge(&mut workspace.packages, "foundation", "renderer", true);
    assert!(select(&workspace, &order, 0, "1", &changed).is_err());
}

#[test]
fn workflow_limits_migration_to_its_pr_base_and_keeps_release_gates() {
    let workflow =
        include_str!("../../../../.github/workflows/martensite.yml").replace("\r\n", "\n");
    let migration = workflow
        .split("  object-model-migration:\n")
        .nth(1)
        .unwrap()
        .split("\n  plan:\n")
        .next()
        .unwrap();
    assert!(migration.contains(
        "github.event_name == 'pull_request' && github.base_ref == 'feature/rust-object-model'"
    ));
    assert!(migration.contains("git worktree add --detach"));
    assert!(migration.contains("persist-credentials: false"));
    assert!(workflow.contains("cargo xtask ci run --target"));
    assert!(workflow.contains("cargo xtask ci gate --checks"));
    assert!(workflow.contains("cargo xtask ci publish --dist"));
    assert_eq!(
        workflow
            .matches("(github.event_name != 'pull_request' || github.base_ref != 'feature/rust-object-model')")
            .count(),
        2
    );
}

#[test]
fn plan_keeps_all_issue_levels_and_the_candidate_sketch_toolkit() {
    assert_eq!(LEVELS.lines().count(), 15);
    let order = schedule();
    assert_eq!(order.len(), 45);
    assert_eq!(order.first().unwrap(), "martensite-tkernel");
    let offset = |name| {
        order
            .iter()
            .position(|candidate| candidate == name)
            .unwrap()
    };
    assert!(offset("martensite-tkbo") < offset("martensite-tksketch"));
    assert!(offset("martensite-tksketch") < offset("martensite-tkv3d"));
    assert!(!needs_renderer(&names("martensite-tkernel")));
    assert!(needs_renderer(&names("martensite-vulkan")));
}

#[test]
fn audited_package_without_objects_can_advance_using_its_verification_record() {
    let evidence = file("docs/features/rust-object-model/process-policy-verification.md");
    let cursor = file(CURSOR);
    assert!(
        authorize_paths(
            &[evidence.clone(), cursor.clone()],
            "martensite-process-policy",
            "crates/martensite-process-policy"
        )
        .is_ok()
    );
    assert!(
        authorize_paths(
            &[evidence, cursor],
            "martensite-tkernel",
            "crates/martensite-tkernel"
        )
        .is_err()
    );
}

#[test]
fn replaying_every_step_of_the_current_workspace_qualifies_each_suite() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = CargoWorkspace::load(&crate::process::ProcessRunner, &root).unwrap();
    let order = schedule();
    let mut qualified = BTreeSet::new();
    for (index, name) in order.iter().enumerate() {
        let changed = BTreeSet::from([name.clone()]);
        let step = select(
            &workspace,
            &order,
            index,
            &(index + 1).to_string(),
            &changed,
        )
        .unwrap();
        assert_eq!(&step.next, name);
        assert!(step.tests.is_disjoint(&step.deferred));
        assert!(step.tests.iter().all(|name| order[..=index].contains(name)));
        qualified.extend(step.tests);
    }
    assert_eq!(qualified, order.into_iter().collect());
}

#[test]
fn hosted_or_local_run_requires_the_immutable_base_as_an_ancestor() {
    let base = "82b13dff919c19e79a581251cc41accd2fd00337";
    let lookup = format!("git merge-base {base} HEAD");
    let runner = crate::fake_runner::RecordingRunner::default()
        .with_stdout("git rev-parse --show-toplevel", "/repo")
        .with_stdout(&lookup, &format!("{base}\n"));
    let context = crate::context::tests::context_at(&runner, std::path::Path::new("/repo"));
    assert!(verify_base(&context, base).is_ok());
    assert!(verify_base(&context, "origin/feature/rust-object-model").is_err());

    let runner = runner.with_stdout(&lookup, "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb");
    let context = crate::context::tests::context_at(&runner, std::path::Path::new("/repo"));
    assert!(verify_base(&context, base).is_err());
}
