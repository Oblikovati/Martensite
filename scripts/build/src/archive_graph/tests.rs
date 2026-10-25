use super::*;
use crate::process::CommandRunner;
use semver::Version;

const PUBLISHED_MANIFEST: &str = include_str!("published-tkbrep-0.3.6.toml");

fn selected(kernel: &str) -> BTreeMap<String, Version> {
    [
        ("martensite-tkernel", kernel),
        ("martensite-cmath", "0.3.5"),
        ("martensite-tkmath", "0.3.5"),
        ("martensite-tkg2d", "0.3.5"),
        ("martensite-tkg3d", "0.3.5"),
        ("martensite-tkgeombase", "0.3.5"),
    ]
    .into_iter()
    .map(|(name, version)| (name.into(), Version::parse(version).unwrap()))
    .collect()
}

#[test]
fn exact_published_manifest_rejects_minor_incompatible_foundation_identity() {
    // Actual published archive hash: 9520ea9c392c573dd002deb15925dda1e78d382416c198bfdbb82d544cf01200 (#279).
    let mismatches =
        manifest::mismatches("martensite-tkbrep", PUBLISHED_MANIFEST, &selected("0.4.0")).unwrap();
    assert_eq!(mismatches.len(), 1);
    assert_eq!(mismatches[0].dependency, "martensite-tkernel");
    assert_eq!(mismatches[0].requirement, "0.3.6");
    assert_eq!(mismatches[0].selected_version, "0.4.0");
    assert!(
        manifest::mismatches("martensite-tkbrep", PUBLISHED_MANIFEST, &selected("0.3.7"))
            .unwrap()
            .is_empty()
    );
}

#[test]
fn aliased_optional_and_target_dependencies_preserve_their_contracts() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<ArchiveGraph>();
    send_sync::<ArchiveIdentity>();
    send_sync::<Mismatch>();
    send_sync::<fixture::HistoricalAssetFixture>();
    send_sync::<ArchiveResolverFixture>();
    let text = "[target.'cfg(windows)'.dev-dependencies.alias]\npackage='martensite-tkernel'\nversion='0.3'\noptional=true";
    let mismatches = manifest::mismatches("example", text, &selected("0.4.0")).unwrap();
    assert_eq!(mismatches.len(), 1);
    assert_eq!(mismatches[0].dependency, "martensite-tkernel");
}

#[test]
fn missing_or_malformed_internal_requirements_fail_closed() {
    for text in [
        "[dependencies]\nmartensite-tkernel={path='../kernel'}",
        "[dependencies]\nmartensite-tkernel='invalid'",
        "[dependencies]\nmartensite-unknown='0.4'",
    ] {
        assert!(manifest::mismatches("example", text, &selected("0.4.0")).is_err());
    }
}

#[test]
fn cargo_rejects_the_actual_published_requirement_with_a_private_empty_cache() {
    let fixture = ArchiveResolverFixture::new();
    let output = crate::process::ProcessRunner
        .capture_output(&fixture.command())
        .unwrap();
    assert_eq!(output.code, 101);
    let message = String::from_utf8(output.stderr).unwrap();
    assert!(message.contains("^0.3.6"), "{message}");
    assert!(message.contains("0.4.0"), "{message}");
}

#[test]
fn the_catalog_uses_historical_bytes_instead_of_a_repackaged_local_sibling() {
    let fixture = fixture::HistoricalAssetFixture::populated();
    fixture.regenerate_sibling();
    let graph = fixture.inspect().unwrap();
    assert_eq!(graph.compatibility, "incompatible");
    assert_eq!(graph.incompatible_requirements.len(), 1);
    assert_eq!(graph.archives["martensite-tkbrep"].release, "v0.3.6");
    let published = fixture
        .directory
        .path()
        .join("published/martensite-tkbrep-0.3.6.crate");
    assert_eq!(
        graph.archives["martensite-tkbrep"].sha256,
        archive::sha256_hex(&published).unwrap()
    );
}

#[test]
fn corrupt_historical_checksums_abort_the_archive_graph() {
    let fixture = fixture::HistoricalAssetFixture::populated();
    std::fs::write(
        fixture
            .directory
            .path()
            .join("published/martensite-tkbrep-0.3.6.crate.sha256"),
        "wrong",
    )
    .unwrap();
    assert!(fixture.inspect().is_err());
}

pub(super) struct ArchiveResolverFixture {
    directory: tempfile::TempDir,
}

impl ArchiveResolverFixture {
    fn new() -> Self {
        let fixture = Self {
            directory: tempfile::tempdir().unwrap(),
        };
        fixture.write("Cargo.toml", "[workspace]\nresolver='3'\nmembers=['foundation','brep']\n[patch.crates-io]\nmartensite-tkernel={path='foundation'}");
        fixture.write(
            "foundation/Cargo.toml",
            "[package]\nname='martensite-tkernel'\nversion='0.4.0'\nedition='2024'",
        );
        fixture.write("foundation/src/lib.rs", "pub fn foundation() {}");
        fixture.write("brep/Cargo.toml", &Self::published_requirement());
        fixture.write("brep/src/lib.rs", "pub fn boundary() {}");
        std::fs::create_dir(fixture.directory.path().join("empty-cargo-home")).unwrap();
        fixture
    }

    pub(super) fn published_requirement() -> String {
        let table = crate::package_manifest::parse_table(PUBLISHED_MANIFEST).unwrap();
        let requirement = table["dependencies"]["martensite-tkernel"]["version"]
            .as_str()
            .unwrap();
        format!(
            "[package]\nname='martensite-tkbrep'\nversion='0.3.6'\nedition='2024'\n[dependencies]\nmartensite-tkernel='{requirement}'"
        )
    }

    fn write(&self, name: &str, text: &str) {
        let path = self.directory.path().join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }

    fn command(&self) -> crate::process::CommandSpec {
        crate::process::CommandSpec::new("cargo", self.directory.path())
            .args(["metadata", "--offline", "--format-version", "1"])
            .environment(
                "CARGO_HOME",
                self.directory.path().join("empty-cargo-home").to_str(),
            )
            .environment("CARGO_NET_OFFLINE", Some("true"))
    }
}

#[test]
fn never_published_unchanged_identity_uses_the_verified_generated_archive() {
    let fixture = fixture::HistoricalAssetFixture::populated();
    fixture.regenerate_sibling();
    let plan = fixture.plan(false).unwrap();
    assert_eq!(
        plan.packages["martensite-tkbrep"].reason,
        crate::publication_plan::PublicationReason::FirstPublication
    );
    let graph = fixture.inspect_plan(&plan).unwrap();
    assert_eq!(graph.compatibility, "requirements-compatible");
    assert_eq!(graph.archives["martensite-tkbrep"].release, "v0.4.0");
    let generated = fixture
        .directory
        .path()
        .join("dist/martensite-tkbrep-0.3.6.crate");
    assert_eq!(
        graph.archives["martensite-tkbrep"].sha256,
        archive::sha256_hex(&generated).unwrap()
    );
}

#[test]
fn publication_plan_retains_downloaded_bytes_through_graph_inspection() {
    let fixture = fixture::HistoricalAssetFixture::populated();
    let plan = fixture.plan(true).unwrap();
    let path = plan
        .archive_path("martensite-tkbrep", &fixture.request().dist_dir)
        .unwrap();
    assert!(path.is_file());
    fixture.inspect_plan(&plan).unwrap();
    assert!(path.is_file());
    drop(plan);
    assert!(!path.exists());
}

#[test]
fn changing_historical_bytes_and_their_checksum_cannot_replace_the_sealed_identity() {
    let fixture = fixture::HistoricalAssetFixture::populated();
    let plan = fixture.plan(true).unwrap();
    let path = plan
        .archive_path("martensite-tkbrep", &fixture.request().dist_dir)
        .unwrap();
    let changed =
        crate::published_history::fixtures::archive_bytes("martensite-tkbrep", "0.3.6", "");
    std::fs::write(&path, changed).unwrap();
    archive::write_checksum(&path).unwrap();
    let error = fixture.inspect_plan(&plan).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("sealed publication archive digest"),
        "{error}"
    );
}

#[test]
fn archive_graph_requires_generated_publications_to_be_sealed() {
    let fixture = fixture::HistoricalAssetFixture::populated();
    let mut plan = fixture.plan(true).unwrap();
    plan.packages
        .get_mut("martensite-tkernel")
        .unwrap()
        .archive_sha256 = None;
    let error = fixture.inspect_plan(&plan).unwrap_err();
    assert!(
        error.to_string().contains("verified archive digest"),
        "{error}"
    );
}
