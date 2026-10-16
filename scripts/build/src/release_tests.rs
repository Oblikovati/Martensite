//! Release pipeline tests against a temporary repository and scripted tools.

use super::*;
use crate::context::tests::context_at;
use crate::fake_runner::RecordingRunner;
use crate::workspace::tests::metadata_at;

const METADATA: &str = "cargo metadata --format-version 1 --no-deps --locked";
const TRIPLE: &str = "x86_64-unknown-linux-gnu";
const CHANGELOG: &str =
    "# Changelog\n\n## [0.3.0] - 2026-10-14\n\n### Added\n- kernel crates (#1)\n";

/// A repository with the bundled documents, one built binary and the
/// `.crate` files `cargo package` would have written.
struct ReleaseFixture {
    dir: tempfile::TempDir,
}

impl ReleaseFixture {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(root.join("CHANGELOG.md"), CHANGELOG).unwrap();
        fs::write(root.join("LICENSE"), "GPL-2.0-only").unwrap();
        fs::write(root.join("README.md"), "Martensite").unwrap();
        fs::write(root.join("martensite-cli"), "binary").unwrap();
        fs::create_dir_all(root.join("target/package")).unwrap();
        for name in ["martensite-kernel", "martensite-math"] {
            fs::write(
                root.join(format!("target/package/{name}-0.3.0.crate")),
                name,
            )
            .unwrap();
        }
        Self { dir }
    }

    fn root(&self) -> &Path {
        self.dir.path()
    }

    fn runner(&self, tags: &str, version: &str) -> RecordingRunner {
        let root = self.root().display().to_string();
        let binary = serde_json::json!({"reason": "compiler-artifact", "target": {"kind": ["bin"]},
            "filenames": [self.root().join("martensite-cli")], "executable": self.root().join("martensite-cli")});
        RecordingRunner::default()
            .with_stdout("git rev-parse --show-toplevel", &root)
            .with_stdout(METADATA, &metadata_at(&root, version))
            .with_stdout("git tag --list v*", tags)
            .with_stdout(&release_build_command(), &binary.to_string())
    }

    fn request(&self, source_packages: bool) -> ReleaseRequest {
        let version = Version::new(0, 3, 0);
        let dist_dir = default_dist_dir(self.root(), &version);
        ReleaseRequest {
            version,
            triple: TRIPLE.into(),
            source_packages,
            dist_dir,
        }
    }
}

fn release_build_command() -> String {
    format!(
        "cargo build --locked --workspace --exclude {BUILD_TOOL_PACKAGE} --target {TRIPLE} \
         --release --message-format json-render-diagnostics"
    )
}

fn asset_names(assets: &[PathBuf]) -> Vec<String> {
    assets
        .iter()
        .map(|asset| file_name(asset).unwrap().to_owned())
        .collect()
}

const PRIMARY_ASSETS: [&str; 6] = [
    "martensite-0.3.0-x86_64-unknown-linux-gnu.tar.gz",
    "martensite-0.3.0-x86_64-unknown-linux-gnu.tar.gz.sha256",
    "martensite-kernel-0.3.0.crate",
    "martensite-kernel-0.3.0.crate.sha256",
    "martensite-math-0.3.0.crate",
    "martensite-math-0.3.0.crate.sha256",
];

/// The cargo gates after `cargo metadata`, in the order a release runs them.
fn release_gates() -> [String; 5] {
    [
        "cargo fmt --all --check".to_owned(),
        format!("cargo clippy --locked --workspace --target {TRIPLE} --all-targets -- -D warnings"),
        format!("cargo test --locked --workspace --target {TRIPLE} --no-fail-fast"),
        release_build_command(),
        "cargo package --locked --package martensite-kernel --package martensite-math".to_owned(),
    ]
}

#[test]
fn primary_target_packages_archive_and_source_crates() {
    let fixture = ReleaseFixture::new();
    let runner = fixture.runner("v0.2.7\n", "0.3.0");
    let assets = run_release(&context_at(&runner, fixture.root()), &fixture.request(true)).unwrap();
    assert_eq!(asset_names(&assets), PRIMARY_ASSETS);
    assert!(assets.iter().all(|asset| asset.exists()));
}

#[test]
fn primary_target_runs_every_gate_in_order() {
    let fixture = ReleaseFixture::new();
    let runner = fixture.runner("v0.2.7\n", "0.3.0");
    run_release(&context_at(&runner, fixture.root()), &fixture.request(true)).unwrap();
    let commands = runner
        .commands()
        .into_iter()
        .filter(|command| command.starts_with("cargo "));
    assert_eq!(commands.skip(1).collect::<Vec<_>>(), release_gates());
}

#[test]
fn secondary_target_skips_source_packages() {
    let fixture = ReleaseFixture::new();
    let runner = fixture.runner("", "0.3.0");
    let assets = run_release(
        &context_at(&runner, fixture.root()),
        &fixture.request(false),
    )
    .unwrap();
    assert_eq!(assets.len(), 2);
    assert!(
        !runner
            .commands()
            .iter()
            .any(|c| c.starts_with("cargo package"))
    );
}

#[test]
fn version_mismatch_stops_before_any_build() {
    let fixture = ReleaseFixture::new();
    let runner = fixture.runner("", "0.2.9");
    let error =
        run_release(&context_at(&runner, fixture.root()), &fixture.request(true)).unwrap_err();
    assert!(
        error.to_string().contains("martensite-kernel 0.2.9"),
        "{error}"
    );
    assert!(!runner.commands().iter().any(|c| c.starts_with("cargo fmt")));
}

#[test]
fn too_small_bump_is_refused() {
    let fixture = ReleaseFixture::new();
    fs::write(
        fixture.root().join("CHANGELOG.md"),
        "## [0.2.8]\n\n### Removed\n- x\n",
    )
    .unwrap();
    let runner = fixture.runner("v0.2.7\n", "0.2.8");
    let request = ReleaseRequest {
        version: Version::new(0, 2, 8),
        ..fixture.request(true)
    };
    let error = run_release(&context_at(&runner, fixture.root()), &request).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("Patch bump but the changelog requires Minor"),
        "{error}"
    );
}

#[test]
fn release_candidate_tag_is_not_the_bump_baseline() {
    let fixture = ReleaseFixture::new();
    let runner = fixture.runner("v0.2.7\nv0.3.0-rc.1\n", "0.3.0");
    assert!(run_release(&context_at(&runner, fixture.root()), &fixture.request(true)).is_ok());
}

#[test]
fn library_only_target_ships_no_binary_archive() {
    let fixture = ReleaseFixture::new();
    let runner = fixture
        .runner("", "0.3.0")
        .with_stdout(&release_build_command(), "");
    let assets = run_release(
        &context_at(&runner, fixture.root()),
        &fixture.request(false),
    )
    .unwrap();
    assert!(assets.is_empty());
}
