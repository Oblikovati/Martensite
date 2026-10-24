//! A release records mixed package versions without republishing unchanged crates.

use std::{fs, path::PathBuf};

use crate::archive::write_checksum;
use crate::context::BUILD_TOOL_PACKAGE;
use crate::error::{BuildResult, io_error};
use crate::release::ReleaseRequest;
use crate::release_scope::ReleaseScope;
use crate::workspace::CargoWorkspace;

/// Records shipped versions and test scope beside a checksum of the manifest.
/// Example: `let assets = write_package_manifest(&request, &workspace, &scope)?;`.
pub(crate) fn write_package_manifest(
    request: &ReleaseRequest,
    workspace: &CargoWorkspace,
    scope: &ReleaseScope,
) -> BuildResult<[PathBuf; 2]> {
    let manifest = package_manifest(request, workspace, scope);
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
) -> serde_json::Value {
    let packages: Vec<_> = workspace
        .packages
        .iter()
        .filter(|p| p.name != BUILD_TOOL_PACKAGE)
        .map(|package| {
            serde_json::json!({"name": package.name, "version": package.version.to_string(),
            "shipped": scope.packages.contains(&package.name)})
        })
        .collect();
    serde_json::json!({"release": request.version.to_string(),
        "baseline": scope.baseline, "packages": packages, "tested_packages": scope.tests})
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::tests::sample_metadata;
    use semver::Version;

    #[test]
    fn manifest_distinguishes_test_impact_from_package_publication() {
        let dir = tempfile::tempdir().unwrap();
        let workspace = CargoWorkspace::parse(&sample_metadata()).unwrap();
        let scope = ReleaseScope {
            baseline: Some("v0.2.9".into()),
            tests: workspace.names(),
            packages: vec!["martensite-math".into()],
        };
        let request = ReleaseRequest {
            version: Version::new(0, 3, 1),
            triple: "host".into(),
            source_packages: true,
            dist_dir: dir.path().into(),
        };
        let paths = write_package_manifest(&request, &workspace, &scope).unwrap();
        let result: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&paths[0]).unwrap()).unwrap();
        assert_eq!(result["packages"][0]["shipped"], false);
        assert_eq!(result["packages"][1]["shipped"], true);
        assert_eq!(result["tested_packages"].as_array().unwrap().len(), 2);
        assert!(paths[1].exists());
    }
}
