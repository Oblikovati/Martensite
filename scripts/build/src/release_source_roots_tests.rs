use super::tests::{changed, fixture};
use super::*;
use std::fs;

fn write_marker(root: &Path) {
    let marker = root.join("crates/a/src/marker.txt");
    fs::write(marker, "owned production marker").unwrap();
}
#[test]
fn lexical_and_canonical_workspace_roots_prove_the_same_qa_scope() {
    let (root, manifest) = fixture("const MARKER: &str = include_str!(\"marker.txt\");");
    write_marker(root.path());
    let changes = [changed("tests/unrelated.rs")];
    for workspace in [
        root.path().to_path_buf(),
        root.path().canonicalize().unwrap(),
    ] {
        assert!(publication_changes(&workspace, &manifest, "crates/a", &changes).is_empty());
    }
}
#[test]
fn lexical_and_canonical_roots_retain_missing_referenced_inputs() {
    let (root, manifest) = fixture("const INPUT: &str = \"../tests/input.bin\";");
    let mut input = changed("tests/input.bin");
    input.deleted = true;
    let changes = [input];
    for workspace in [
        root.path().to_path_buf(),
        root.path().canonicalize().unwrap(),
    ] {
        assert_eq!(
            publication_changes(&workspace, &manifest, "crates/a", &changes).as_ref(),
            &changes
        );
    }
}
#[cfg(unix)]
struct WorkspaceAliasFixture {
    physical: tempfile::TempDir,
    _alias: tempfile::TempDir,
    workspace: PathBuf,
    manifest: PackageManifest,
}
#[cfg(unix)]
impl WorkspaceAliasFixture {
    fn with_source(source: &str) -> Self {
        let (physical, manifest) = fixture(source);
        let alias = tempfile::tempdir().unwrap();
        let workspace = alias.path().join("workspace");
        std::os::unix::fs::symlink(physical.path(), &workspace).unwrap();
        Self {
            physical,
            _alias: alias,
            workspace,
            manifest,
        }
    }
    fn publication(&self, input: ChangedFile) -> Vec<ChangedFile> {
        let changes = [input];
        publication_changes(&self.workspace, &self.manifest, "crates/a", &changes).into_owned()
    }
}
#[cfg(unix)]
#[test]
fn workspace_aliases_do_not_turn_owned_literal_inputs_into_unknown_source() {
    let fixture =
        WorkspaceAliasFixture::with_source("const MARKER: &str = include_str!(\"marker.txt\");");
    write_marker(fixture.physical.path());
    assert!(
        fixture
            .publication(changed("tests/unrelated.rs"))
            .is_empty()
    );
}
#[cfg(unix)]
#[test]
fn workspace_aliases_cannot_drop_referenced_missing_qa_inputs() {
    let fixture = WorkspaceAliasFixture::with_source("const INPUT: &str = \"../tests/input.bin\";");
    let mut input = changed("tests/input.bin");
    input.deleted = true;
    assert_eq!(fixture.publication(input.clone()), [input]);
}
#[test]
fn literal_qa_modules_work_from_native_and_canonical_roots() {
    let (root, manifest) = fixture("#[path=\"../tests/support/runtime.rs\"] mod runtime;");
    let support = root.path().join("crates/a/tests/support");
    fs::create_dir_all(&support).unwrap();
    fs::write(support.join("runtime.rs"), "pub fn witness() {}").unwrap();
    let changes = [changed("tests/unrelated.rs")];
    for workspace in [
        root.path().to_path_buf(),
        root.path().canonicalize().unwrap(),
    ] {
        assert!(publication_changes(&workspace, &manifest, "crates/a", &changes).is_empty());
    }
}
#[test]
fn canonical_source_roots_keep_manifest_literal_references() {
    let (root, manifest) = fixture("const MANIFEST: &str = \"../Cargo.toml\";");
    let package = root.path().join("crates/a");
    let changes = [changed("Cargo.toml")];
    for package in [package.clone(), package.canonicalize().unwrap()] {
        assert!(!manifest_is_unreferenced(&package, &manifest, &changes));
    }
}
#[test]
fn native_components_and_authored_spelling_have_distinct_checks() {
    let native = Path::new("tests").join("support").join("runtime.rs");
    assert!(crate::release_input_layout::input_components_supported(
        &native
    ));
    assert!(!crate::release_input_layout::portable_relative_path(
        r"tests\support\runtime.rs"
    ));
    for path in [
        "target/input.rs",
        "TARGET/input.rs",
        ".git/input.rs",
        "../outside.rs",
    ] {
        assert!(!crate::release_input_layout::input_components_supported(
            Path::new(path)
        ));
    }
}
