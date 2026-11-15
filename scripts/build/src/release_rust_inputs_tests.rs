use super::super::{
    publication_changes,
    tests::{actual_vulkan_manifest, changed, fixture},
};
use super::*;
use crate::{git::ChangedFile, package_manifest::PackageManifest};

fn module_fixture(files: &[(&str, &str)]) -> (tempfile::TempDir, PackageManifest) {
    let (root, manifest) = fixture("#[path=\"../tests/support/root.rs\"] mod entry;");
    for (path, source) in files {
        let path = root.path().join("crates/a").join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, source).unwrap();
    }
    (root, manifest)
}
fn retains(root: &Path, manifest: &PackageManifest, input: &str) -> bool {
    let changes = [changed(input)];
    !publication_changes(root, manifest, "crates/a", &changes).is_empty()
}
#[test]
fn literal_module_graph_excludes_unrelated_qa_and_keeps_transitive_inputs() {
    let (root, manifest) = module_fixture(&[
        ("tests/support/root.rs", "mod child;"),
        (
            "tests/support/child.rs",
            "#[path=\"../outside/input.rs\"] mod input;",
        ),
        (
            "tests/outside/input.rs",
            "const INPUT: &[u8] = include_bytes!(\"data.bin\");",
        ),
        ("tests/outside/data.bin", "owned production input"),
    ]);
    assert!(!retains(root.path(), &manifest, "tests/unrelated.rs"));
    assert_transitive_inputs_retained(root.path(), &manifest);
}
fn assert_transitive_inputs_retained(root: &Path, manifest: &PackageManifest) {
    for input in [
        "tests/support/root.rs",
        "tests/support/child.rs",
        "tests/outside/data.bin",
    ] {
        assert!(retains(root, manifest, input), "{input}");
    }
}
#[test]
fn implicit_module_children_resolve_under_their_own_module_directory() {
    let (root, manifest) = module_fixture(&[
        ("tests/support/root.rs", "mod child;"),
        ("tests/support/child.rs", "mod grandchild;"),
        (
            "tests/support/child/grandchild.rs",
            "#[path=\"../../outside/input.rs\"] mod input;",
        ),
        ("tests/outside/input.rs", "pub fn witness() {}"),
    ]);
    assert!(!retains(root.path(), &manifest, "tests/unrelated.rs"));
    assert!(retains(root.path(), &manifest, "tests/outside/input.rs"));
}
#[test]
fn repeated_file_visitation_keeps_distinct_module_contexts() {
    let (root, manifest) = module_fixture(&[
        (
            "tests/support/root.rs",
            "mod reused; #[path=\"reused.rs\"] mod alias;",
        ),
        ("tests/support/reused.rs", "mod child;"),
        ("tests/support/reused/child.rs", "pub fn witness() {}"),
        (
            "tests/support/child.rs",
            "#[path=\"../outside/input.rs\"] mod input;",
        ),
        ("tests/outside/input.rs", "pub fn witness() {}"),
    ]);
    assert!(!retains(root.path(), &manifest, "tests/unrelated.rs"));
    assert!(retains(root.path(), &manifest, "tests/outside/input.rs"));
}
#[test]
fn unsupported_qa_module_graphs_leave_publication_impact_unknown() {
    for source in [
        "include!(concat!(\"../\", env!(\"SOURCE\")));",
        "#[cfg_attr(windows, path=\"other.rs\")] mod child;",
        "mod child { #[path=\"other.rs\"] mod nested; }",
        "macro_rules! modules { () => { mod child; }; }",
        "#[path=\"../TESTS/source.rs\"] mod child;",
        "mod missing;",
    ] {
        let (root, manifest) = module_fixture(&[("tests/support/root.rs", source)]);
        assert!(
            retains(root.path(), &manifest, "tests/unrelated.rs"),
            "{source}"
        );
    }
}
#[test]
fn ambiguous_implicit_module_candidates_fail_closed() {
    let (root, manifest) = module_fixture(&[
        ("tests/support/root.rs", "mod child;"),
        ("tests/support/child.rs", "pub fn first() {}"),
        ("tests/support/child/mod.rs", "pub fn second() {}"),
    ]);
    assert!(retains(root.path(), &manifest, "tests/unrelated.rs"));
}
#[test]
fn deleting_a_referenced_module_retains_uncertain_inputs() {
    let (root, manifest) = module_fixture(&[("tests/support/root.rs", "pub fn witness() {}")]);
    fs::remove_file(root.path().join("crates/a/tests/support/root.rs")).unwrap();
    let mut input = changed("tests/support/root.rs");
    input.deleted = true;
    let changes = [input];
    let selected = publication_changes(root.path(), &manifest, "crates/a", &changes);
    assert_eq!(selected.as_ref(), &changes);
    assert!(retains(root.path(), &manifest, "tests/unrelated.rs"));
}
#[cfg(unix)]
#[test]
fn linked_qa_module_files_and_directories_fail_closed() {
    use std::os::unix::fs::symlink;
    for directory_link in [false, true] {
        let (root, manifest) = module_fixture(&[("tests/support/root.rs", "pub fn witness() {}")]);
        let package = root.path().join("crates/a");
        if directory_link {
            fs::rename(package.join("tests/support"), package.join("tests/real")).unwrap();
            symlink("real", package.join("tests/support")).unwrap();
        } else {
            fs::rename(
                package.join("tests/support/root.rs"),
                package.join("tests/support/real.rs"),
            )
            .unwrap();
            symlink("real.rs", package.join("tests/support/root.rs")).unwrap();
        }
        assert!(retains(root.path(), &manifest, "tests/unrelated.rs"));
    }
}
#[test]
fn actual_vulkan_production_inputs_are_disjoint_from_release_qa_edits() {
    let (root, manifest) = actual_vulkan_manifest();
    let changes: Vec<_> = include_str!(
        "../../../docs/features/release-source-scope/references/485-vulkan-qa-paths.txt"
    )
    .lines()
    .map(|path| ChangedFile {
        path: path.into(),
        deleted: false,
        untracked: false,
    })
    .collect();
    let selected = publication_changes(&root, &manifest, "crates/martensite-vulkan", &changes);
    assert!(
        selected.is_empty(),
        "unexpected publication inputs: {selected:?}"
    );
}
