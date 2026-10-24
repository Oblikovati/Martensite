//! Named history fixtures share the production publication boundary, not a second fetcher.
use super::*;
use crate::{
    archive::{BundleEntry, write_checksum, write_tar_gz},
    context::{BuildContext, tests::context_at},
    fake_runner::RecordingRunner,
    publication_plan::PublicationPlan,
    published_history::{ReleaseIndex, fixtures::PublicationApiFixture},
    release_scope::ReleaseScope,
};

pub(super) struct HistoricalAssetFixture {
    pub directory: tempfile::TempDir,
}

impl HistoricalAssetFixture {
    pub fn populated() -> Self {
        let fixture = Self {
            directory: tempfile::tempdir().unwrap(),
        };
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
        let plan = self.plan(true)?;
        self.inspect_plan(&plan)
    }

    pub fn inspect_plan(&self, plan: &PublicationPlan) -> BuildResult<ArchiveGraph> {
        super::inspect(&self.request(), &self.workspace(), plan)
    }

    pub fn plan(&self, published: bool) -> BuildResult<PublicationPlan> {
        let root = self.directory.path();
        let runner = self.history(published).seed(
            RecordingRunner::default()
                .with_stdout("git rev-parse --show-toplevel", &root.to_string_lossy()),
        );
        let context: BuildContext<'_> = context_at(&runner, root);
        let index = ReleaseIndex::load(&context)?;
        let mut plan = PublicationPlan::load(&context, &self.workspace(), &self.scope(), &index)?;
        plan.seal_outputs(&root.join("dist"))?;
        Ok(plan)
    }

    fn history(&self, published: bool) -> PublicationApiFixture {
        let mut fixture = PublicationApiFixture::default();
        if !published {
            return fixture;
        }
        let release = fixture.release("v0.3.6");
        for name in [
            "martensite-tkbrep-0.3.6.crate",
            "martensite-tkbrep-0.3.6.crate.sha256",
        ] {
            let bytes = std::fs::read(self.directory.path().join("published").join(name)).unwrap();
            fixture.asset(release, name, bytes);
        }
        fixture
    }

    pub fn archive(&self, location: &str, name: &str, version: &str, text: &str) {
        let source = self.directory.path().join("Cargo.toml");
        std::fs::write(&source, text).unwrap();
        let directory = self.directory.path().join(location);
        std::fs::create_dir_all(&directory).unwrap();
        let path = directory.join(format!("{name}-{version}.crate"));
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

    fn scope(&self) -> ReleaseScope {
        ReleaseScope {
            baseline: Some("v0.3.6".into()),
            history_sha256: None,
            tests: Vec::new(),
            packages: vec!["martensite-tkernel".into()],
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
