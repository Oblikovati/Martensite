//! Real Cargo output locations remain authoritative across environment/config choices (#285).
use crate::{
    context::BuildContext,
    error::BuildResult,
    git::GitRepo,
    process::{CommandOutput, CommandRunner, CommandSpec, ProcessRunner},
    release::{ReleaseRequest, package_sources},
    report::{LogFlavor, StepReporter},
    workspace::CargoWorkspace,
};
use std::path::{Path, PathBuf};

struct PackageTargetFixture {
    directory: tempfile::TempDir,
    external: tempfile::TempDir,
}

impl PackageTargetFixture {
    fn new(config: Option<&str>) -> Self {
        let fixture = Self {
            directory: tempfile::tempdir().unwrap(),
            external: tempfile::tempdir().unwrap(),
        };
        let root = fixture.directory.path();
        std::fs::create_dir(root.join("src")).unwrap();
        std::fs::write(root.join("src/lib.rs"), "pub fn value() -> u8 { 7 }\n").unwrap();
        std::fs::write(root.join("Cargo.toml"), "[package]\nname='martensite-target-proof'\nversion='0.1.0'\nedition='2024'\nlicense-file='LICENSE-GPL'\n").unwrap();
        std::fs::write(root.join("LICENSE-GPL"), "Independent packaging fixture\n").unwrap();
        if let Some(path) = config {
            fixture.write_config(path);
        }
        fixture
    }

    fn write_config(&self, path: &str) {
        let directory = self.directory.path().join(".cargo");
        std::fs::create_dir(&directory).unwrap();
        std::fs::write(
            directory.join("config.toml"),
            format!("[build]\ntarget-dir={path:?}\n"),
        )
        .unwrap();
    }

    fn verify(&self, target: Option<&str>, expected: &Path) {
        let root = self.directory.path();
        let runner = self.runner(target);
        runner
            .run(&CommandSpec::new("cargo", root).arg("generate-lockfile"))
            .unwrap();
        let workspace = CargoWorkspace::load(&runner, root).unwrap();
        assert_eq!(workspace.target_directory, expected);
        let context = fixture_context(&runner, root);
        let request = self.request();
        std::fs::create_dir(&request.dist_dir).unwrap();
        let assets = package_sources(&context, &request, &workspace, &workspace.names()).unwrap();
        self.assert_shipped(&assets, expected);
    }

    fn runner(&self, target: Option<&str>) -> PackageTargetRunner {
        PackageTargetRunner {
            root: self.directory.path().into(),
            home: self.external.path().join("cargo-home"),
            target: target.map(str::to_owned),
        }
    }

    fn request(&self) -> ReleaseRequest {
        ReleaseRequest {
            version: semver::Version::new(0, 1, 0),
            triple: "fixture".into(),
            source_packages: true,
            dist_dir: self.external.path().join("dist"),
        }
    }

    fn assert_shipped(&self, assets: &[PathBuf], expected: &Path) {
        let packaged = expected.join("package/martensite-target-proof-0.1.0.crate");
        assert_eq!(
            std::fs::read(&assets[0]).unwrap(),
            std::fs::read(packaged).unwrap()
        );
        crate::archive::verify_checksum(&assets[0]).unwrap();
        if expected != self.directory.path().join("target") {
            assert!(!self.directory.path().join("target/package").exists());
        }
    }
}

struct PackageTargetRunner {
    root: PathBuf,
    home: PathBuf,
    target: Option<String>,
}

impl PackageTargetRunner {
    fn configured(&self, spec: &CommandSpec) -> CommandSpec {
        spec.clone()
            .arg("--offline")
            .environment("CARGO_BUILD_TARGET_DIR", None)
            .environment("CARGO_TARGET_DIR", self.target.as_deref())
            .environment("CARGO_HOME", self.home.to_str())
    }
}

impl CommandRunner for PackageTargetRunner {
    fn run(&self, spec: &CommandSpec) -> BuildResult<()> {
        ProcessRunner.run(&self.configured(spec))
    }
    fn capture(&self, spec: &CommandSpec) -> BuildResult<String> {
        if spec.display() == "git rev-parse --show-toplevel" {
            return Ok(self.root.to_string_lossy().into());
        }
        ProcessRunner.capture(&self.configured(spec))
    }
    fn exit_code(&self, spec: &CommandSpec) -> BuildResult<i32> {
        ProcessRunner.exit_code(&self.configured(spec))
    }
    fn capture_output(&self, spec: &CommandSpec) -> BuildResult<CommandOutput> {
        ProcessRunner.capture_output(&self.configured(spec))
    }
}

fn fixture_context<'a>(runner: &'a PackageTargetRunner, root: &Path) -> BuildContext<'a> {
    BuildContext {
        runner,
        git: GitRepo::discover(runner, root).unwrap(),
        reporter: StepReporter::new(LogFlavor::Terminal),
    }
}

#[test]
fn packaged_archive_uses_cargo_default_target_directory() {
    let fixture = PackageTargetFixture::new(None);
    fixture.verify(None, &fixture.directory.path().join("target"));
}

#[test]
fn packaged_archive_uses_cargo_config_relative_target_directory() {
    let fixture = PackageTargetFixture::new(Some("configured-output"));
    fixture.verify(None, &fixture.directory.path().join("configured-output"));
}

#[test]
fn packaged_archive_uses_cargo_environment_relative_target_directory() {
    let fixture = PackageTargetFixture::new(Some("unused-config"));
    fixture.verify(
        Some("relative-output"),
        &fixture.directory.path().join("relative-output"),
    );
}

#[test]
fn packaged_archive_uses_cargo_environment_absolute_target_directory() {
    let fixture = PackageTargetFixture::new(None);
    let expected = fixture.external.path().join("absolute-output");
    fixture.verify(expected.to_str(), &expected);
}
