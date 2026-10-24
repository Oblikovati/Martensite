use super::*;
use crate::{
    archive::{BundleEntry, write_checksum, write_tar_gz},
    fake_runner::RecordingRunner,
    process::{CommandOutput, CommandRunner, CommandSpec},
};

pub(super) struct HistoricalAssetFixture {
    pub directory: tempfile::TempDir,
    pub runner: HistoricalAssetRunner,
}

impl HistoricalAssetFixture {
    pub fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let runner = HistoricalAssetRunner {
            source: directory.path().join("published"),
            recording: recorded(directory.path()),
        };
        std::fs::create_dir(&runner.source).unwrap();
        std::fs::create_dir(directory.path().join("dist")).unwrap();
        Self { directory, runner }
    }

    pub fn populated() -> Self {
        let fixture = Self::new();
        fixture.archive(
            "dist",
            "martensite-tkernel",
            "0.4.0",
            "[package]\nname='martensite-tkernel'\nversion='0.4.0'",
        );
        fixture.archive(
            "published",
            "martensite-tkbrep",
            "0.3.6",
            &super::tests::ArchiveResolverFixture::published_requirement(),
        );
        fixture
    }

    pub fn regenerate_sibling(&self) {
        self.archive("dist", "martensite-tkbrep", "0.3.6", "[package]\nname='martensite-tkbrep'\nversion='0.3.6'\n[dependencies]\nmartensite-tkernel='0.4'");
    }

    pub fn inspect(&self) -> BuildResult<ArchiveGraph> {
        let scope = ReleaseScope {
            baseline: Some("v0.3.6".into()),
            tests: Vec::new(),
            packages: vec!["martensite-tkernel".into()],
        };
        super::inspect(&self.context(), &self.request(), &self.workspace(), &scope)
    }

    pub fn archive(&self, location: &str, name: &str, version: &str, text: &str) {
        let source = self.directory.path().join("Cargo.toml");
        std::fs::write(&source, text).unwrap();
        let path = self
            .directory
            .path()
            .join(location)
            .join(format!("{name}-{version}.crate"));
        let entry = BundleEntry {
            source,
            archive_path: "Cargo.toml".into(),
            executable: false,
        };
        write_tar_gz(&path, &format!("{name}-{version}"), &[entry]).unwrap();
        write_checksum(&path).unwrap();
    }

    pub fn workspace(&self) -> CargoWorkspace {
        CargoWorkspace {
            packages: [
                ("martensite-tkernel", "0.4.0"),
                ("martensite-tkbrep", "0.3.6"),
            ]
            .into_iter()
            .map(|(name, version)| crate::workspace::WorkspacePackage {
                name: name.into(),
                version: semver::Version::parse(version).unwrap(),
                dir: format!("crates/{name}"),
                path_dependencies: Vec::new(),
            })
            .collect(),
        }
    }

    pub fn context(&self) -> BuildContext<'_> {
        BuildContext {
            runner: &self.runner,
            git: crate::git::GitRepo::discover(&self.runner, self.directory.path()).unwrap(),
            reporter: crate::report::StepReporter::new(crate::report::LogFlavor::Terminal),
        }
    }

    pub fn request(&self) -> ReleaseRequest {
        ReleaseRequest {
            version: semver::Version::new(0, 4, 0),
            triple: "fixture".into(),
            source_packages: true,
            dist_dir: self.directory.path().join("dist"),
        }
    }
}

fn recorded(root: &Path) -> RecordingRunner {
    RecordingRunner::default()
        .with_stdout("git rev-parse --show-toplevel", &root.to_string_lossy())
        .with_stdout("git tag --list v*", "v0.3.6\n")
        .with_stdout(
            "gh release view v0.3.6 --json assets",
            "{\"assets\":[{\"name\":\"martensite-tkbrep-0.3.6.crate\"}]}",
        )
}

pub(super) struct HistoricalAssetRunner {
    source: PathBuf,
    recording: RecordingRunner,
}

impl CommandRunner for HistoricalAssetRunner {
    fn run(&self, spec: &CommandSpec) -> BuildResult<()> {
        let args = spec.argument_values();
        if args.first().is_some_and(|arg| arg == "release")
            && args.get(1).is_some_and(|arg| arg == "download")
        {
            self.copy_assets(args)?;
        }
        self.recording.run(spec)
    }
    fn capture(&self, spec: &CommandSpec) -> BuildResult<String> {
        self.recording.capture(spec)
    }
    fn exit_code(&self, spec: &CommandSpec) -> BuildResult<i32> {
        self.recording.exit_code(spec)
    }
    fn capture_output(&self, spec: &CommandSpec) -> BuildResult<CommandOutput> {
        self.recording.capture_output(spec)
    }
}

impl HistoricalAssetRunner {
    fn copy_assets(&self, args: &[String]) -> BuildResult<()> {
        let destination = Path::new(args.last().unwrap());
        for name in [
            "martensite-tkbrep-0.3.6.crate",
            "martensite-tkbrep-0.3.6.crate.sha256",
        ] {
            let source = self.source.join(name);
            std::fs::copy(&source, destination.join(name))
                .map_err(|cause| io_error(&source, cause))?;
        }
        Ok(())
    }
}
