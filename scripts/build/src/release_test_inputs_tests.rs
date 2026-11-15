use super::*;
use crate::release_input_layout::scannable_target_path;

fn fixture(source: &str) -> (tempfile::TempDir, PackageManifest) {
    let directory = tempfile::tempdir().unwrap();
    fs::create_dir_all(directory.path().join("crates/a/src")).unwrap();
    fs::write(directory.path().join("crates/a/src/lib.rs"), source).unwrap();
    let manifest =
        PackageManifest::parse("[package]\nname='a'\nversion='1.0.0'", &toml::Table::new())
            .unwrap()
            .unwrap();
    (directory, manifest)
}
fn changed(path: &str) -> ChangedFile {
    ChangedFile {
        path: format!("crates/a/{path}"),
        deleted: false,
        untracked: false,
    }
}
fn selected(source: &str, path: &str) -> bool {
    let (root, manifest) = fixture(source);
    !publication_changes(root.path(), &manifest, "crates/a", &[changed(path)]).is_empty()
}
#[test]
fn ordinary_tests_are_qa_but_other_package_paths_remain_publication_source() {
    assert!(!selected("pub fn value() {}", "tests/regression.rs"));
    for path in [
        "src/lib.rs",
        "build.rs",
        "examples/tool.rs",
        "assets/image.bin",
    ] {
        assert!(selected("pub fn value() {}", path));
    }
}
#[test]
fn production_fixture_and_dynamic_includes_fail_closed() {
    for source in [
        "const DATA: &[u8] = include_bytes!(\"../tests/data.bin\");",
        "const DATA: &str = include_str!(concat!(\"../tests/\", env!(\"FIXTURE\")));",
        "const ROOT: &str = \"tests\";",
    ] {
        assert!(selected(source, "tests/data.bin"));
    }
}
#[test]
fn transitive_production_module_can_own_a_test_fixture() {
    let (root, manifest) = fixture("#[path=\"../tests/runtime.rs\"] mod runtime;");
    let tests = root.path().join("crates/a/tests");
    fs::create_dir_all(&tests).unwrap();
    fs::write(
        tests.join("runtime.rs"),
        "const DATA: &[u8] = include_bytes!(\"data.bin\");",
    )
    .unwrap();
    let changed = [changed("tests/data.bin")];
    let files = publication_changes(root.path(), &manifest, "crates/a", &changed);
    assert_eq!(files.len(), 1);
}
#[test]
fn custom_build_script_and_target_keep_uncertain_test_paths_as_source() {
    let (root, mut manifest) = fixture("pub fn value() {}");
    fs::write(root.path().join("crates/a/build.rs"), "fn main() {}").unwrap();
    assert_eq!(
        publication_changes(root.path(), &manifest, "crates/a", &[changed("tests/data")]).len(),
        1
    );
    fs::remove_file(root.path().join("crates/a/build.rs")).unwrap();
    manifest.source.insert(
        "lib".into(),
        "path='tests/runtime.rs'"
            .parse::<toml::Table>()
            .unwrap()
            .into(),
    );
    assert_eq!(
        publication_changes(root.path(), &manifest, "crates/a", &[changed("tests/data")]).len(),
        1
    );
}

#[test]
fn malformed_source_and_nonliteral_paths_keep_test_changes_as_source() {
    assert!(selected("fn incomplete {", "tests/data.bin"));
    assert!(selected(
        "#[path=concat!(\"../tests/\", \"runtime.rs\")] mod runtime;",
        "tests/data.bin"
    ));
    assert!(!selected(
        "pub fn lookup(path: usize) { let _ = [path]; }",
        "tests/regression.rs"
    ));
}
#[test]
fn fixed_dynamic_extension_refuses_relevant_files_but_excludes_disjoint_qa() {
    let source = "const DATA: &[u8] = include_bytes!(concat!(env!(\"ROOT\"), \".pixels\"));";
    assert!(selected(source, "tests/data.pixels"));
    assert!(!selected(source, "tests/regression.rs"));
    assert!(selected(source, "tests/data.PIXELS"));
}

#[test]
fn literal_macro_bounds_cover_calls_inside_other_macro_bodies() {
    let source = r#"macro_rules! asset { ($name:literal) => {
        include_bytes!(concat!(env!("ROOT"), $name))
    }; }
    macro_rules! wrapper { () => { asset!("danger.rs") }; }
    const A: &[u8] = asset!("safe.spv");
    const B: &[u8] = wrapper!();"#;
    assert!(selected(source, "tests/danger.rs"));
}
#[test]
fn literal_macro_arguments_prove_only_disjoint_extensions() {
    let source = r#"macro_rules! asset { ($name:literal) => {
        include_bytes!(concat!(env!("ROOT"), $name))
    }; }
    const A: &[u8] = asset!("safe.spv");"#;
    assert!(!selected(source, "tests/regression.rs"));
    assert!(selected(source, "tests/data.spv"));
}

#[test]
fn aliased_and_exported_macro_arguments_are_not_closed_to_workspace_calls() {
    let definition = r#"macro_rules! asset { ($name:literal) => {
        include_bytes!(concat!(env!("ROOT"), $name))
    }; }
    const A: &[u8] = asset!("safe.spv");"#;
    let alias = format!(
        "{definition} pub(crate) use asset as alias; const B: &[u8] = alias!(\"danger.rs\");"
    );
    assert!(selected(&alias, "tests/danger.rs"));
    assert!(selected(
        &format!("#[macro_export] {definition}"),
        "tests/danger.rs"
    ));
    assert!(selected(
        &format!("#[r#macro_export] {definition}"),
        "tests/danger.rs"
    ));
}
#[test]
fn nested_inline_module_path_uncertainty_keeps_test_source_selected() {
    let source = r#"mod outer { #[path="../../tests/runtime.rs"] mod runtime; }"#;
    assert!(selected(source, "tests/runtime.rs"));
}

#[test]
fn raw_macro_spellings_and_builtin_includes_keep_production_inputs() {
    let source = r#"macro_rules! asset { ($name:literal) => {
        include_bytes!(concat!(env!("ROOT"), $name))
    }; }
    const A: &[u8] = asset!("safe.spv");
    const B: &[u8] = r#asset!("danger.rs");"#;
    assert!(selected(source, "tests/danger.rs"));
    assert!(selected(
        "const A: &[u8] = r#include_bytes!(concat!(env!(\"ROOT\"), name));",
        "tests/data.rs"
    ));
}

#[test]
fn higher_order_macro_calls_do_not_supply_closed_literal_bounds() {
    let source = r#"macro_rules! asset { ($name:literal) => {
        include_bytes!(concat!(env!("ROOT"), $name))
    }; }
    macro_rules! invoke { ($m:ident) => { $m!("danger.rs") }; }
    const A: &[u8] = asset!("safe.spv");
    const B: &[u8] = invoke!(asset);"#;
    assert!(selected(source, "tests/danger.rs"));
}

#[test]
fn conditional_inline_paths_fail_closed_at_every_attribute_depth() {
    for attribute in [
        r##"#[cfg_attr(unix, path="../../tests/runtime.rs")]"##,
        r##"#[cfg_attr(unix, cfg_attr(feature="x", r#path="../../tests/runtime.rs"))]"##,
        r##"#![cfg_attr(unix, path="../../tests/runtime.rs")]"##,
    ] {
        let source = format!("mod outer {{ {attribute} mod runtime; }}");
        assert!(selected(&source, "tests/runtime.rs"), "{attribute}");
    }
}

#[test]
fn nonstandard_rust_source_suffixes_keep_transitive_inputs_as_source() {
    for source in [
        r##"#[path="../tests/runtime.inc"] mod runtime;"##,
        r##"include!("../tests/runtime.inc");"##,
    ] {
        let (root, manifest) = fixture(source);
        let tests = root.path().join("crates/a/tests");
        fs::create_dir_all(&tests).unwrap();
        let runtime_source = "const A: &[u8] = include_bytes!(\"data.bin\");";
        fs::write(tests.join("runtime.inc"), runtime_source).unwrap();
        fs::write(tests.join("data.bin"), [1, 2, 3]).unwrap();
        let changed = [changed("tests/data.bin")];
        assert_eq!(
            publication_changes(root.path(), &manifest, "crates/a", &changed).len(),
            1
        );
    }
}
#[test]
fn dynamic_or_external_rust_source_remains_uncertain() {
    assert!(selected(
        r##"include!(concat!(env!("ROOT"), ".rs"));"##,
        "tests/data.bin"
    ));
    assert!(selected(
        r##"#[path="../../shared/runtime.rs"] mod runtime;"##,
        "tests/data.bin"
    ));
}

#[test]
fn only_portable_scanned_production_target_paths_supply_a_proof() {
    for path in [
        "src/../tests/runtime.rs",
        "src/runtime.input",
        "src/runtime.RS",
        "src/runtime.rs:hidden.rs",
        "src\\runtime.rs",
        "../src/runtime.rs",
        "/src/runtime.rs",
    ] {
        assert!(!scannable_target_path(path), "{path}");
    }
    for path in ["src/lib.rs", "src/bin/runtime.rs"] {
        assert!(scannable_target_path(path), "{path}");
    }
}
