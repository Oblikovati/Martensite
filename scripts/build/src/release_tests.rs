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
pub(crate) struct ReleaseFixture {
    dir: tempfile::TempDir,
}

impl ReleaseFixture {
    pub(crate) fn new() -> Self {
        let fixture = Self {
            dir: tempfile::tempdir().unwrap(),
        };
        fixture.write_documents();
        for name in ["martensite-kernel", "martensite-math"] {
            fixture.write_package_version(name, "0.3.0");
            fixture.write_crate(name, &["Cargo.toml", SOURCE_LICENSE]);
        }
        fixture
    }

    fn write_documents(&self) {
        let root = self.root();
        fs::write(root.join("CHANGELOG.md"), CHANGELOG).unwrap();
        fs::write(root.join("LICENSE-APACHE"), "Apache-2.0").unwrap();
        fs::write(root.join("README.md"), "Martensite").unwrap();
        fs::write(root.join("martensite-cli"), "binary").unwrap();
        fs::create_dir_all(root.join("target/package")).unwrap();
        fs::write(root.join("Cargo.toml"), "[workspace]\nmembers=[]").unwrap();
    }

    fn write_package_version(&self, name: &str, version: &str) {
        let directory = self.root().join("crates").join(name);
        fs::create_dir_all(&directory).unwrap();
        fs::write(
            directory.join("Cargo.toml"),
            format!("[package]\nname='{name}'\nversion='{version}'"),
        )
        .unwrap();
    }

    /// Writes the `.crate` that `cargo package` would leave for `name`,
    /// holding `files` under its `<name>-0.3.0/` top directory.
    fn write_crate(&self, name: &str, files: &[&str]) {
        self.write_versioned_crate(name, "0.3.0", files);
    }

    fn write_versioned_crate(&self, name: &str, version: &str, files: &[&str]) {
        let sources = self.root().join("crate-sources");
        fs::create_dir_all(&sources).unwrap();
        let entries: Vec<BundleEntry> = files
            .iter()
            .map(|file| {
                fs::write(sources.join(file), file).unwrap();
                BundleEntry {
                    source: sources.join(file),
                    archive_path: (*file).to_owned(),
                    executable: false,
                }
            })
            .collect();
        let crate_file = self
            .root()
            .join(format!("target/package/{name}-{version}.crate"));
        write_tar_gz(&crate_file, &format!("{name}-{version}"), &entries).unwrap();
    }

    pub(crate) fn root(&self) -> &Path {
        self.dir.path()
    }

    pub(crate) fn runner(&self, tags: &str, version: &str) -> RecordingRunner {
        let root = self.root().display().to_string();
        let binary = serde_json::json!({"reason": "compiler-artifact", "target": {"kind": ["bin"]},
            "filenames": [self.root().join("martensite-cli")], "executable": self.root().join("martensite-cli")});
        baseline_runner()
            .with_stdout("git rev-parse --show-toplevel", &root)
            .with_stdout(METADATA, &metadata_at(&root, version))
            .with_stdout("git tag --list v*", tags)
            .with_stdout(&release_build_command(), &binary.to_string())
    }

    fn mixed_runner(&self, kernel: &str, math: &str) -> RecordingRunner {
        self.write_package_version("martensite-kernel", kernel);
        self.write_package_version("martensite-math", math);
        let mut metadata: serde_json::Value =
            serde_json::from_str(&metadata_at(&self.root().display().to_string(), "0.3.0"))
                .unwrap();
        metadata["packages"][0]["version"] = kernel.into();
        metadata["packages"][1]["version"] = math.into();
        self.runner("v0.2.7\n", "0.3.0")
            .with_stdout(METADATA, &metadata.to_string())
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

fn baseline_runner() -> RecordingRunner {
    let runner = RecordingRunner::default()
        .with_stdout("git show v0.2.7:Cargo.toml", "[workspace]\nmembers=[]")
        .with_stdout(
            "git ls-tree -r --name-only -z v0.2.7",
            "crates/martensite-kernel/Cargo.toml\0crates/martensite-math/Cargo.toml\0",
        )
        .with_stdout(
            "git diff --name-status --no-renames -z v0.2.7",
            "M\0crates/martensite-kernel/src/lib.rs\0M\0crates/martensite-math/src/lib.rs\0",
        );
    let runner = previous_package(runner, "martensite-kernel");
    previous_package(runner, "martensite-math")
}

fn previous_package(runner: RecordingRunner, name: &str) -> RecordingRunner {
    runner.with_stdout(
        &format!("git show v0.2.7:crates/{name}/Cargo.toml"),
        &format!("[package]\nname='{name}'\nversion='0.2.7'"),
    )
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

const PRIMARY_ASSETS: [&str; 10] = [
    "martensite-0.3.0-x86_64-unknown-linux-gnu.tar.gz",
    "martensite-0.3.0-x86_64-unknown-linux-gnu.tar.gz.sha256",
    "martensite-kernel-0.3.0.crate",
    "martensite-kernel-0.3.0.crate.sha256",
    "martensite-math-0.3.0.crate",
    "martensite-math-0.3.0.crate.sha256",
    "martensite-0.3.0-source-workspace.tar.gz",
    "martensite-0.3.0-source-workspace.tar.gz.sha256",
    "martensite-0.3.0-packages.json",
    "martensite-0.3.0-packages.json.sha256",
];

/// The cargo gates after `cargo metadata`, in the order a release runs them.
fn release_gates() -> [String; 5] {
    [
        "cargo fmt --all --check".to_owned(),
        format!("cargo clippy --locked --workspace --target {TRIPLE} --all-targets -- -D warnings"),
        format!(
            "cargo test --locked --package martensite-kernel --package martensite-math --target {TRIPLE} --no-fail-fast"
        ),
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
fn delegated_release_tests_preserve_validation_build_and_packaging() {
    let fixture = ReleaseFixture::new();
    let runner = fixture.runner("v0.2.7\n", "0.3.0");
    let context = context_at(&runner, fixture.root());
    let assets = run_release_checks(&context, &fixture.request(true), false).unwrap();
    let commands: Vec<_> = runner
        .commands()
        .into_iter()
        .filter(|c| c.starts_with("cargo "))
        .skip(1)
        .collect();
    let expected: Vec<_> = release_gates()
        .into_iter()
        .filter(|c| !c.starts_with("cargo test "))
        .collect();
    assert_eq!(commands, expected);
    assert_eq!(asset_names(&assets), PRIMARY_ASSETS);
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

#[test]
fn binary_archive_carries_the_apache_license_only() {
    let fixture = ReleaseFixture::new();
    let runner = fixture.runner("", "0.3.0");
    let assets = run_release(
        &context_at(&runner, fixture.root()),
        &fixture.request(false),
    )
    .unwrap();
    let members = archive_member_paths(&assets[0]).unwrap();
    let top = "martensite-0.3.0-x86_64-unknown-linux-gnu";
    assert!(
        members.contains(&format!("{top}/LICENSE-APACHE")),
        "{members:?}"
    );
    assert!(
        !members
            .iter()
            .any(|member| member.ends_with(SOURCE_LICENSE)),
        "{members:?}"
    );
}

#[test]
fn missing_apache_license_fails_naming_the_file() {
    let fixture = ReleaseFixture::new();
    fs::remove_file(fixture.root().join("LICENSE-APACHE")).unwrap();
    let runner = fixture.runner("", "0.3.0");
    let error = run_release(
        &context_at(&runner, fixture.root()),
        &fixture.request(false),
    )
    .unwrap_err();
    assert!(error.to_string().contains("LICENSE-APACHE"), "{error}");
}

#[test]
fn source_package_without_the_gpl_license_is_refused() {
    let fixture = ReleaseFixture::new();
    fixture.write_crate("martensite-math", &["Cargo.toml"]);
    let runner = fixture.runner("", "0.3.0");
    let error =
        run_release(&context_at(&runner, fixture.root()), &fixture.request(true)).unwrap_err();
    let message = error.to_string();
    assert!(
        message.contains("martensite-math-0.3.0.crate has no martensite-math-0.3.0/LICENSE-GPL"),
        "{message}"
    );
}

#[test]
fn mixed_release_retains_dependency_version_and_ships_only_changed_source() {
    let fixture = ReleaseFixture::new();
    let runner = fixture.mixed_runner("0.3.0", "0.2.7").with_stdout(
        "git diff --name-status --no-renames -z v0.2.7",
        "M\0crates/martensite-kernel/src/lib.rs\0",
    );
    let assets = run_release(&context_at(&runner, fixture.root()), &fixture.request(true)).unwrap();
    assert!(
        !asset_names(&assets)
            .iter()
            .any(|name| name.starts_with("martensite-math-"))
    );
    assert!(runner.commands().iter().any(|command| command
        == &format!(
            "cargo test --locked --package martensite-kernel --target {TRIPLE} --no-fail-fast"
        )));
    assert!(runner.commands().iter().any(|command| command
        == "cargo package --locked --package martensite-kernel --package martensite-math"));
    assert!(asset_names(&assets).contains(&"martensite-kernel-0.3.0.crate".into()));
}

#[test]
fn archive_uses_package_version_independently_of_repository_version() {
    let fixture = ReleaseFixture::new();
    let runner = fixture.mixed_runner("0.2.8", "0.3.0");
    fixture.write_versioned_crate(
        "martensite-kernel",
        "0.2.8",
        &["Cargo.toml", SOURCE_LICENSE],
    );
    let assets = run_release(&context_at(&runner, fixture.root()), &fixture.request(true)).unwrap();
    assert!(asset_names(&assets).contains(&"martensite-kernel-0.2.8.crate".into()));
    assert!(!asset_names(&assets).contains(&"martensite-kernel-0.3.0.crate".into()));
}

pub(crate) struct FixtureSourceRelease;

impl SourceRelease for FixtureSourceRelease {
    fn prepare(
        &self,
        _context: &BuildContext<'_>,
        request: &ReleaseRequest,
        _workspace: &CargoWorkspace,
        _scope: &ReleaseScope,
    ) -> BuildResult<crate::release_sources::SourceArtifacts> {
        Ok(fixture_source(request))
    }
}

pub(crate) fn fixture_source(request: &ReleaseRequest) -> crate::release_sources::SourceArtifacts {
    let name = format!("martensite-{}-source-workspace.tar.gz", request.version);
    let path = request.dist_dir.join(&name);
    fs::write(&path, b"named source fixture").unwrap();
    write_checksum(&path).unwrap();
    crate::release_sources::SourceArtifacts {
        sdk: crate::source_sdk::SdkRecord {
            archive: name,
            sha256: crate::archive::sha256_hex(&path).unwrap(),
            revision: "a".repeat(40),
            manifest_sha256: "b".repeat(64),
            verification: "named-fixture".into(),
        },
        graph: crate::archive_graph::ArchiveGraph {
            compatibility: "requirements-compatible".into(),
            validation: "named-fixture".into(),
            archives: Default::default(),
            incompatible_requirements: Vec::new(),
        },
    }
}

fn run_release(context: &BuildContext<'_>, request: &ReleaseRequest) -> BuildResult<Vec<PathBuf>> {
    super::run_release_using(context, request, true, &FixtureSourceRelease)
}

fn run_release_checks(
    context: &BuildContext<'_>,
    request: &ReleaseRequest,
    run_tests: bool,
) -> BuildResult<Vec<PathBuf>> {
    super::run_release_using(context, request, run_tests, &FixtureSourceRelease)
}
