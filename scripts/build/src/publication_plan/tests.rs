use super::*;
use crate::{
    context::tests::context_at,
    fake_runner::RecordingRunner,
    published_history::fixtures::{PublicationApiFixture, package},
};

#[test]
fn unchanged_unpublished_version_is_selected_once_then_reused_exactly() {
    let directory = tempfile::tempdir().unwrap();
    let workspace = unchanged_workspace();
    let scope = unchanged_scope();
    let mut fixture = unshipped_fixture();
    let plan = load_plan(&fixture, directory.path(), &workspace, &scope);
    assert_eq!(plan.shipped_names(), ["martensite-math"]);
    assert_eq!(
        plan.packages["martensite-math"].reason,
        PublicationReason::FirstPublication
    );
    assert!(plan.require_sealed().is_err());
    assert!(scope.tests.is_empty());
    let next = fixture.release("v0.4.0");
    fixture.archive(
        next,
        "martensite-math",
        "0.3.5",
        "martensite-foundation='0.4'",
    );
    verify_reuse(&fixture, directory.path(), &workspace, &scope);
}

fn verify_reuse(
    fixture: &PublicationApiFixture,
    root: &Path,
    workspace: &CargoWorkspace,
    scope: &ReleaseScope,
) {
    let plan = load_plan(fixture, root, workspace, scope);
    assert!(plan.shipped_names().is_empty());
    assert_eq!(
        plan.packages["martensite-math"].reason,
        PublicationReason::HistoricalReuse
    );
    assert_eq!(
        plan.packages["martensite-math"].version,
        semver::Version::new(0, 3, 5)
    );
    plan.require_sealed().unwrap();
}

fn unchanged_workspace() -> CargoWorkspace {
    CargoWorkspace {
        target_directory: std::env::temp_dir().join("publication-fixture-target"),
        packages: vec![package("martensite-math", "0.3.5")],
    }
}

fn unchanged_scope() -> ReleaseScope {
    ReleaseScope {
        baseline: Some("v0.3.6".into()),
        tests: Vec::new(),
        packages: Vec::new(),
        history_sha256: None,
    }
}

fn load_plan(
    fixture: &PublicationApiFixture,
    root: &Path,
    workspace: &CargoWorkspace,
    scope: &ReleaseScope,
) -> PublicationPlan {
    let runner = fixture.seed(
        RecordingRunner::default()
            .with_stdout("git rev-parse --show-toplevel", &root.to_string_lossy()),
    );
    let context = context_at(&runner, root);
    let index = ReleaseIndex::load(&context).unwrap();
    PublicationPlan::load(&context, workspace, scope, &index).unwrap()
}

fn unshipped_fixture() -> PublicationApiFixture {
    let mut fixture = PublicationApiFixture::default();
    let release = fixture.release("v0.3.6");
    fixture.catalog(
        release,
        serde_json::json!([{"name":"martensite-math","version":"0.3.5","shipped":false}]),
    );
    fixture
}

#[test]
fn restored_package_completes_publication_graph_with_exact_bytes_and_anchor() {
    let directory = tempfile::tempdir().unwrap();
    let plan = restored_plan();
    let request = restored_request(directory.path());
    let graph = crate::archive_graph::inspect(&request, &unchanged_workspace(), &plan).unwrap();
    let expected = plan.packages["martensite-math"].archive_sha256.as_deref();
    assert_eq!(
        Some(graph.archives["martensite-math"].sha256.as_str()),
        expected
    );
    let origin = &plan.history_receipt().archives[0];
    assert_eq!(origin.restoration.as_ref().unwrap().release_tag, "v0.5.0");
}

#[test]
fn restored_package_graph_refuses_corrupted_local_companion() {
    let directory = tempfile::tempdir().unwrap();
    let plan = restored_plan();
    let request = restored_request(directory.path());
    let path = plan
        .archive_path("martensite-math", directory.path())
        .unwrap();
    std::fs::write(
        path.with_extension("crate.sha256"),
        "corrupted local companion",
    )
    .unwrap();
    assert!(crate::archive_graph::inspect(&request, &unchanged_workspace(), &plan).is_err());
}

fn restored_plan() -> PublicationPlan {
    let history = crate::published_history::fixtures::restored_history();
    let package = package("martensite-math", "0.3.5");
    let selected = select(&package, false, &history).unwrap();
    PublicationPlan {
        packages: [(package.name, selected)].into(),
        history,
    }
}

fn restored_request(directory: &Path) -> crate::release::ReleaseRequest {
    crate::release::ReleaseRequest {
        version: semver::Version::new(0, 6, 0),
        triple: "x86_64-unknown-linux-gnu".into(),
        source_packages: true,
        dist_dir: directory.into(),
    }
}
