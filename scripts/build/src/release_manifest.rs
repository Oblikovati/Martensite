//! A release records mixed package versions without republishing unchanged crates.

use std::{fs, path::PathBuf};

use crate::archive::write_checksum;
use crate::context::BUILD_TOOL_PACKAGE;
use crate::error::{BuildResult, io_error};
use crate::publication_plan::{PublicationPlan, PublicationReason};
use crate::release::ReleaseRequest;
use crate::release_scope::ReleaseScope;
use crate::release_sources::SourceArtifacts;
use crate::workspace::CargoWorkspace;

/// Records shipped versions and test scope beside a checksum of the manifest.
/// Example: `let assets = write_package_manifest(&request, &workspace, &scope, &plan, &source)?;`.
pub(crate) fn write_package_manifest(
    request: &ReleaseRequest,
    workspace: &CargoWorkspace,
    scope: &ReleaseScope,
    plan: &PublicationPlan,
    source: &SourceArtifacts,
) -> BuildResult<[PathBuf; 2]> {
    plan.require_sealed()?;
    let manifest = package_manifest(request, workspace, scope, plan, source);
    let path = request
        .dist_dir
        .join(format!("martensite-{}-packages.json", request.version));
    fs::write(&path, format!("{manifest:#}\n")).map_err(|cause| io_error(&path, cause))?;
    let checksum = write_checksum(&path)?;
    Ok([path, checksum])
}

fn package_manifest(
    request: &ReleaseRequest,
    workspace: &CargoWorkspace,
    scope: &ReleaseScope,
    plan: &PublicationPlan,
    source: &SourceArtifacts,
) -> serde_json::Value {
    let packages = catalog_packages(workspace, plan);
    serde_json::json!({"schema": 2, "publication_history": plan.history_receipt(),
        "source_consumption": "source_workspace", "source_workspace": source.sdk,
        "individual_archives": source.graph,
        "release": request.version.to_string(),
        "baseline": scope.baseline, "packages": packages, "tested_packages": scope.tests})
}

fn catalog_packages(workspace: &CargoWorkspace, plan: &PublicationPlan) -> Vec<serde_json::Value> {
    workspace
        .packages
        .iter()
        .filter(|p| p.name != BUILD_TOOL_PACKAGE)
        .map(|package| {
            serde_json::json!({"name": package.name, "version": package.version.to_string(),
            "shipped": plan.packages[&package.name].reason != PublicationReason::HistoricalReuse,
            "publication_reason": plan.packages[&package.name].reason,
            "archive_sha256": plan.packages[&package.name].archive_sha256,
            "origins": plan.packages[&package.name].origins})
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::tests::sample_metadata;
    use crate::{
        context::tests::context_at,
        fake_runner::RecordingRunner,
        published_history::{ReleaseIndex, fixtures::PublicationApiFixture},
    };
    use semver::Version;

    #[test]
    fn manifest_distinguishes_test_impact_from_package_publication() {
        let dir = tempfile::tempdir().unwrap();
        let workspace = CargoWorkspace::parse(&sample_metadata()).unwrap();
        let scope = sample_scope(&workspace);
        let request = sample_request(dir.path());
        let plan = publication_fixture(dir.path(), &workspace, &scope);
        let source = crate::release::tests::fixture_source(&request);
        let paths = write_package_manifest(&request, &workspace, &scope, &plan, &source).unwrap();
        let result: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&paths[0]).unwrap()).unwrap();
        assert_eq!(result["packages"][0]["shipped"], false);
        assert_eq!(result["packages"][1]["shipped"], true);
        assert_eq!(result["tested_packages"].as_array().unwrap().len(), 2);
        assert_catalog_evidence(&result);
        assert!(paths[1].exists());
    }
    fn assert_catalog_evidence(catalog: &serde_json::Value) {
        assert_eq!(catalog["source_consumption"], "source_workspace");
        assert_eq!(catalog["source_workspace"]["verification"], "named-fixture");
        assert_eq!(
            catalog["individual_archives"]["validation"],
            "named-fixture"
        );
        assert_publication_evidence(catalog);
    }

    fn assert_publication_evidence(catalog: &serde_json::Value) {
        assert!(catalog["publication_history"].is_object());
        let historical = &catalog["packages"][0];
        assert_eq!(historical["publication_reason"], "historical_reuse");
        assert!(historical["archive_sha256"].is_string());
        assert!(!historical["origins"].as_array().unwrap().is_empty());
        assert_eq!(
            catalog["packages"][1]["publication_reason"],
            "changed_source"
        );
    }

    fn sample_scope(workspace: &CargoWorkspace) -> ReleaseScope {
        ReleaseScope {
            baseline: Some("v0.2.9".into()),
            history_sha256: None,
            tests: workspace.names(),
            packages: vec!["martensite-math".into()],
        }
    }

    fn sample_request(directory: &std::path::Path) -> ReleaseRequest {
        ReleaseRequest {
            version: Version::new(0, 3, 1),
            triple: "host".into(),
            source_packages: true,
            dist_dir: directory.into(),
        }
    }

    fn publication_fixture(
        root: &std::path::Path,
        workspace: &CargoWorkspace,
        scope: &ReleaseScope,
    ) -> PublicationPlan {
        let mut fixture = PublicationApiFixture::default();
        let release = fixture.release("v0.2.9");
        fixture.archive(release, "martensite-kernel", "0.3.0", "");
        let runner = fixture.seed(
            RecordingRunner::default()
                .with_stdout("git rev-parse --show-toplevel", &root.to_string_lossy()),
        );
        let context = context_at(&runner, root);
        let index = ReleaseIndex::load(&context).unwrap();
        let mut plan = PublicationPlan::load(&context, workspace, scope, &index).unwrap();
        write_generated_math(root);
        plan.seal_outputs(root).unwrap();
        plan
    }
    fn write_generated_math(root: &std::path::Path) {
        let archive = root.join("martensite-math-0.3.0.crate");
        fs::write(
            &archive,
            crate::published_history::fixtures::archive_bytes("martensite-math", "0.3.0", ""),
        )
        .unwrap();
        write_checksum(&archive).unwrap();
    }
}
