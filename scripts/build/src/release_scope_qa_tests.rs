use super::*;
use crate::{context::tests::context_at, fake_runner::RecordingRunner};
use std::path::Path;

fn package_text(name: &str) -> String {
    let base = format!("[package]\nname='{name}'\nversion='0.3.0'\nautotests=false\n");
    if name == "martensite-kernel" {
        return format!(
            "{base}[dependencies]\nmartensite-math={{path='../martensite-math',version='0.3.0'}}\n"
        );
    }
    base
}
fn write_member(root: &Path, name: &str, suffix: &str, contents: &str) {
    let path = root.join("crates").join(name).join(suffix);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, contents).unwrap();
}
fn prepare_member(root: &Path, name: &str, qa_target: bool) -> PackageManifest {
    let previous = package_text(name);
    let extra = if qa_target && name == "martensite-math" {
        "[[test]]\nname='existing'\npath='tests/existing.rs'\n"
    } else {
        "# Manifest whitespace only\n"
    };
    write_member(root, name, "Cargo.toml", &format!("{previous}{extra}"));
    write_member(root, name, "src/lib.rs", "pub fn value() {}\n");
    write_member(
        root,
        name,
        "tests/existing.rs",
        "#[test] fn existing() {}\n",
    );
    PackageManifest::parse(&previous, &Table::new())
        .unwrap()
        .unwrap()
}
struct ReleaseQaFixture {
    root: tempfile::TempDir,
    workspace: CargoWorkspace,
    previous: BTreeMap<String, PackageManifest>,
}
fn previous_packages(
    root: &Path,
    workspace: &CargoWorkspace,
    qa_target: bool,
) -> BTreeMap<String, PackageManifest> {
    workspace
        .packages
        .iter()
        .map(|package| {
            let manifest = prepare_member(root, &package.name, qa_target);
            (package.name.clone(), manifest)
        })
        .collect()
}
fn fixture(qa_target: bool) -> ReleaseQaFixture {
    let root = tempfile::tempdir().unwrap();
    fs::write(root.path().join("Cargo.toml"), "[workspace]\n").unwrap();
    let metadata =
        crate::workspace::tests::metadata_at(&root.path().display().to_string(), "0.3.0");
    let workspace = CargoWorkspace::parse(&metadata).unwrap();
    let previous = previous_packages(root.path(), &workspace, qa_target);
    ReleaseQaFixture {
        root,
        workspace,
        previous,
    }
}
fn runner(root: &Path) -> RecordingRunner {
    let mut runner = RecordingRunner::default()
        .with_stdout("git rev-parse --show-toplevel", &root.display().to_string())
        .with_stdout(
            "git diff --name-status --no-renames -z v0.3.0",
            "M\0crates/martensite-math/Cargo.toml\0",
        )
        .with_stdout("git show v0.3.0:Cargo.toml", "[workspace]\n")
        .with_stdout(
            "git ls-tree -r --name-only -z v0.3.0",
            "crates/martensite-kernel/Cargo.toml\0crates/martensite-math/Cargo.toml\0",
        );
    for name in ["martensite-kernel", "martensite-math"] {
        runner = runner.with_stdout(
            &format!("git show v0.3.0:crates/{name}/Cargo.toml"),
            &package_text(name),
        );
    }
    runner
}
fn selected_scope(qa_target: bool) -> ReleaseScope {
    let fixture = fixture(qa_target);
    let runner = runner(fixture.root.path());
    scope_since_release(
        &context_at(&runner, fixture.root.path()),
        &fixture.workspace,
        &fixture.previous,
        "v0.3.0".into(),
    )
    .unwrap()
}
#[test]
fn qa_only_manifest_change_tests_affected_packages_without_republishing() {
    let scope = selected_scope(true);
    assert!(scope.packages.is_empty());
    assert_eq!(scope.tests, ["martensite-kernel", "martensite-math"]);
}
#[test]
fn unchanged_manifest_structure_skips_unaffected_release_suites() {
    let scope = selected_scope(false);
    assert!(scope.packages.is_empty());
    assert!(scope.tests.is_empty());
}
