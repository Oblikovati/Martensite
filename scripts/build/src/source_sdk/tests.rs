use super::*;
use crate::{
    context::BuildContext,
    git::GitRepo,
    process::{CommandRunner, CommandSpec, ProcessRunner},
    report::{LogFlavor, StepReporter},
};

struct SdkFixture {
    directory: tempfile::TempDir,
}

impl SdkFixture {
    fn new() -> Self {
        let fixture = Self {
            directory: tempfile::tempdir().unwrap(),
        };
        fixture.write_root();
        fixture.write_packages();
        fixture.commit();
        fixture
    }

    fn root(&self) -> PathBuf {
        self.directory.path().join("repo")
    }

    fn write_root(&self) {
        let channel = self.installed_channel();
        self.write(
            "Cargo.toml",
            "[workspace]\nresolver='3'\nmembers=['crates/*']\n",
        );
        self.write(
            "rust-toolchain.toml",
            &format!("[toolchain]\nchannel='{channel}'\n"),
        );
        self.write(".cargo/config.toml", "[alias]\nsdk-marker='metadata'\n");
        self.write(".gitignore", "target/\n");
        self.write("LICENSE-GPL", "fixture license\n");
        ProcessRunner
            .run(&CommandSpec::new("git", &self.root()).args(["init", "--quiet"]))
            .unwrap();
    }

    fn installed_channel(&self) -> String {
        let active = ProcessRunner
            .capture(
                &CommandSpec::new("rustup", self.directory.path())
                    .args(["show", "active-toolchain"]),
            )
            .unwrap();
        active.split_whitespace().next().unwrap().into()
    }

    fn write(&self, name: &str, text: &str) {
        files::write(&self.root(), Path::new(name), text.as_bytes()).unwrap();
    }

    fn write_packages(&self) {
        self.package(
            "tkernel",
            "0.4.0",
            "semver='=1.0.28'\n",
            "pub mod standard { pub type Failure = std::io::Error; }",
        );
        self.package(
            "tkbrep",
            "0.3.6",
            "",
            "pub mod topods { #[derive(Default)] pub struct Shape; }",
        );
        self.package("tkgeombase", "0.3.5", "", GEOMETRY);
        self.package("tktopalgo", "0.4.0", TOPOLOGY_DEPS, TOPOLOGY);
        self.write_binary_member();
        ProcessRunner
            .run(&CommandSpec::new("cargo", &self.root()).args(["generate-lockfile", "--offline"]))
            .unwrap();
    }

    fn write_binary_member(&self) {
        self.write(
            "crates/build-fixture/Cargo.toml",
            "[package]\nname='sdk-build-fixture'\nversion='0.1.0'\nedition='2024'\npublish=false",
        );
        self.write("crates/build-fixture/src/main.rs", "fn main() {}");
    }

    fn package(&self, name: &str, version: &str, dependencies: &str, source: &str) {
        self.write(&format!("crates/martensite-{name}/Cargo.toml"), &format!("[package]\nname='martensite-{name}'\nversion='{version}'\nedition='2024'\n[dependencies]\n{dependencies}"));
        self.write(&format!("crates/martensite-{name}/src/lib.rs"), source);
    }

    fn commit(&self) {
        ProcessRunner
            .run(&CommandSpec::new("git", &self.root()).args(["add", "."]))
            .unwrap();
        ProcessRunner
            .run(&CommandSpec::new("git", &self.root()).args([
                "-c",
                "user.name=SdkFixture",
                "-c",
                "user.email=sdk@example.invalid",
                "commit",
                "--quiet",
                "--no-gpg-sign",
                "-m",
                "fixture",
            ]))
            .unwrap();
    }

    fn context(&self) -> BuildContext<'static> {
        BuildContext {
            runner: &ProcessRunner,
            git: GitRepo::discover(&ProcessRunner, &self.root()).unwrap(),
            reporter: StepReporter::new(LogFlavor::Terminal),
        }
    }

    fn request(&self, name: &str) -> ReleaseRequest {
        let dist_dir = self.directory.path().join(name);
        std::fs::create_dir(&dist_dir).unwrap();
        ReleaseRequest {
            version: semver::Version::new(0, 4, 0),
            triple: "fixture".into(),
            source_packages: true,
            dist_dir,
        }
    }
}

const GEOMETRY: &str = "pub mod gprop { #[derive(Default)] pub struct GProps; pub struct Point; impl Point {pub fn x(&self)->f64{0.0}} impl GProps {pub fn new()->Self{Self} pub fn mass(&self)->f64{0.0} pub fn centre_of_mass(&self)->Point{Point}} }";
const TOPOLOGY_DEPS: &str = "martensite-tkernel={path='../martensite-tkernel',version='0.4.0'}\nmartensite-tkbrep={path='../martensite-tkbrep',version='0.3.6'}\nmartensite-tkgeombase={path='../martensite-tkgeombase',version='0.3.5'}\n";
const TOPOLOGY: &str = "pub mod brep_gprop {pub struct BRepGProp; impl BRepGProp {pub fn surface_properties(_: &martensite_tkbrep::topods::Shape, _: &mut martensite_tkgeombase::gprop::GProps, _: bool, _: bool)->Result<(),martensite_tkernel::standard::Failure>{Ok(())}}}";

#[test]
fn coherent_mixed_versions_build_and_consume_offline_with_identical_archive_bytes() {
    let fixture = SdkFixture::new();
    let first = fixture.request("first");
    let second = fixture.request("second");
    let a = create(&fixture.context(), &first).unwrap();
    let b = create(&fixture.context(), &second).unwrap();
    assert_eq!(a.sha256, b.sha256);
    assert_eq!(a.manifest_sha256, b.manifest_sha256);
    assert_eq!(
        std::fs::read(first.dist_dir.join(a.archive)).unwrap(),
        std::fs::read(second.dist_dir.join(b.archive)).unwrap()
    );
}

#[test]
fn dirty_sources_fail_before_any_acceptance_artifact() {
    let fixture = SdkFixture::new();
    fixture.write("untracked-secret", "not a release input");
    let request = fixture.request("dirty");
    let error = create(&fixture.context(), &request).unwrap_err();
    assert!(error.to_string().contains("dirty checkout"));
    assert_eq!(std::fs::read_dir(request.dist_dir).unwrap().count(), 0);
}

#[test]
fn unsafe_relative_paths_are_rejected() {
    for path in ["../secret", "/absolute", "C:\\escape", "", "a/../../b"] {
        assert!(files::relative(Path::new(path)).is_err(), "{path}");
    }
    assert!(files::relative(Path::new("crates/example/src/lib.rs")).is_ok());
}

#[test]
fn changed_payloads_cannot_pass_extraction_integrity() {
    let directory = tempfile::tempdir().unwrap();
    let stage = directory.path().join("stage");
    files::write(&stage, Path::new("fixture"), b"reviewed").unwrap();
    files::write(&stage, Path::new("Cargo.toml"), b"[workspace]").unwrap();
    files::write(&stage, Path::new("Cargo.lock"), b"version=4").unwrap();
    let manifest = seal_fixture(&stage);
    files::write(&stage, Path::new("fixture"), b"changed").unwrap();
    let archive = directory.path().join("tampered.tar.gz");
    archive::write_tar_gz(
        &archive,
        "martensite-0.4.0-source-workspace",
        &files::entries(&stage).unwrap(),
    )
    .unwrap();
    assert!(
        integrity::extract_verified(&archive, &directory.path().join("extract"), &manifest)
            .is_err()
    );
}

fn seal_fixture(stage: &Path) -> integrity::Manifest {
    let snapshot = snapshot::Snapshot {
        revision: "a".repeat(40),
        gitlinks: Default::default(),
        executable_paths: Vec::new(),
        committed_sha256: Default::default(),
    };
    integrity::write_manifest(stage, &snapshot, &semver::Version::new(0, 4, 0)).unwrap()
}

#[cfg(unix)]
#[test]
fn symlinked_inputs_are_rejected_without_traversing_them() {
    let directory = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink("/etc/passwd", directory.path().join("secret")).unwrap();
    assert!(
        files::entries(directory.path())
            .unwrap_err()
            .to_string()
            .contains("regular file")
    );
}

#[test]
fn a_failed_external_measurement_never_gets_an_acceptance_checksum() {
    let fixture = SdkFixture::new();
    let failing = TOPOLOGY.replace(
        "Ok(())",
        "Err(std::io::Error::other(\"measurement refused\"))",
    );
    fixture.write("crates/martensite-tktopalgo/src/lib.rs", &failing);
    fixture.commit();
    let request = fixture.request("failed-consumer");
    let error = create(&fixture.context(), &request).unwrap_err();
    assert!(error.to_string().contains("cargo test --locked --offline"));
    assert!(
        !request
            .dist_dir
            .join("martensite-0.4.0-source-workspace.tar.gz.sha256")
            .exists()
    );
}

#[test]
fn the_pinned_dataset_is_recorded_without_expanding_its_files() {
    let fixture = SdkFixture::new();
    let pin = "c58ee236484bce7b385392454f976d001f8443c4";
    let index = format!("160000,{pin},external/martensite-test-data");
    std::fs::create_dir_all(fixture.root().join("external/martensite-test-data")).unwrap();
    ProcessRunner
        .run(&CommandSpec::new("git", &fixture.root()).args([
            "update-index",
            "--add",
            "--cacheinfo",
            &index,
        ]))
        .unwrap();
    fixture.commit();
    let destination = fixture.directory.path().join("snapshot");
    let snapshot = snapshot::stage(&fixture.context(), &destination).unwrap();
    assert_eq!(snapshot.gitlinks["external/martensite-test-data"], pin);
    assert!(!destination.join("external/martensite-test-data").exists());
}

#[cfg(unix)]
#[test]
fn committed_symlinks_fail_before_any_target_is_read() {
    let fixture = SdkFixture::new();
    std::os::unix::fs::symlink("/etc/passwd", fixture.root().join("outside")).unwrap();
    fixture.commit();
    let destination = fixture.directory.path().join("snapshot");
    let error = snapshot::stage(&fixture.context(), &destination).unwrap_err();
    assert!(error.to_string().contains("mode 120000"));
    assert!(!destination.join("outside").exists());
}
