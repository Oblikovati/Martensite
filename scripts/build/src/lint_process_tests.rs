//! #122: Cargo formatting does not discover every source included by a suite.
use super::*;
use crate::process::ProcessRunner;

struct IncludedSourceFixture {
    directory: tempfile::TempDir,
}

impl IncludedSourceFixture {
    fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        std::fs::create_dir(root.join("src")).unwrap();
        std::fs::write(
            root.join("Cargo.toml"),
            "[package]\nname = \"included-format-fixture\"\nversion = \"0.0.0\"\nedition = \"2024\"\n[workspace]\n",
        )
        .unwrap();
        std::fs::write(root.join("src/lib.rs"), "include!(\"included case.rs\");\n").unwrap();
        std::fs::write(root.join("src/included case.rs"), "pub fn included( ){ }\n").unwrap();
        Self { directory }
    }

    fn run(&self, program: &str, args: &[&str]) {
        ProcessRunner
            .run(&CommandSpec::new(program, self.directory.path()).args(args.iter().copied()))
            .unwrap();
    }
}

#[test]
fn direct_formatting_repairs_the_included_file_named_by_the_gate() {
    let fixture = IncludedSourceFixture::new();
    fixture.run("cargo", &["fmt", "--all"]);
    let changed = [ChangedFile {
        path: "src/included case.rs".into(),
        deleted: false,
        untracked: false,
    }];
    let check = || check_rust_format(&ProcessRunner, fixture.directory.path(), &changed);
    let error = check().unwrap_err().to_string();
    assert!(error.contains("rustfmt -- <file>"));
    assert!(error.contains("path quoted for your shell"));
    assert!(error.contains("\"src/included case.rs\""));
    fixture.run("rustfmt", &["--", "src/included case.rs"]);
    check().unwrap();
}
