//! Named publication I/O fake with actual archive, catalog and checksum bytes.
use super::{PublishedRelease, inventory::Asset};
use crate::{
    archive::{BundleEntry, write_tar_gz},
    fake_runner::RecordingRunner,
};
use semver::Version;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Default)]
pub(crate) struct PublicationApiFixture {
    pub releases: Vec<PublishedRelease>,
    pub assets: BTreeMap<u64, Vec<Asset>>,
    pub bodies: BTreeMap<u64, Vec<u8>>,
    next_asset: u64,
}

impl PublicationApiFixture {
    pub fn release(&mut self, tag: &str) -> u64 {
        let id = self.releases.len() as u64 + 1;
        let prerelease = !crate::version::parse_release_tag(tag)
            .unwrap()
            .pre
            .is_empty();
        self.releases.push(PublishedRelease {
            id,
            tag_name: tag.into(),
            draft: false,
            prerelease,
            published_at: Some("2026-10-24T00:00:00Z".into()),
        });
        self.assets.insert(id, Vec::new());
        id
    }

    pub fn archive(&mut self, release: u64, name: &str, version: &str, requirements: &str) {
        let filename = format!("{name}-{version}.crate");
        let bytes = archive_bytes(name, version, requirements);
        self.pair(release, &filename, bytes);
    }

    pub fn catalog(&mut self, release: u64, packages: serde_json::Value) {
        let tag = self
            .releases
            .iter()
            .find(|item| item.id == release)
            .unwrap()
            .tag_name
            .trim_start_matches('v');
        let filename = format!("martensite-{tag}-packages.json");
        let bytes = serde_json::json!({"release":tag,"packages":packages})
            .to_string()
            .into_bytes();
        self.pair(release, &filename, bytes);
    }

    pub fn pair(&mut self, release: u64, filename: &str, bytes: Vec<u8>) {
        let checksum = format!("{:x}  {filename}\n", Sha256::digest(&bytes));
        self.asset(release, filename, bytes);
        self.asset(
            release,
            &format!("{filename}.sha256"),
            checksum.into_bytes(),
        );
    }

    pub fn asset(&mut self, release: u64, name: &str, bytes: Vec<u8>) -> u64 {
        self.next_asset += 1;
        let id = self.next_asset;
        let asset = Asset {
            id,
            name: name.into(),
            size: bytes.len() as u64,
            state: "uploaded".into(),
            digest: Some(format!("sha256:{:x}", Sha256::digest(&bytes))),
            updated_at: "2026-10-24T00:00:00Z".into(),
        };
        self.assets.get_mut(&release).unwrap().push(asset);
        self.bodies.insert(id, bytes);
        id
    }

    pub fn seed(&self, mut runner: RecordingRunner) -> RecordingRunner {
        runner = runner.with_stdout(&pages_command(super::RELEASES), &pages(&self.releases));
        for release in &self.releases {
            let endpoint = format!(
                "repos/{{owner}}/{{repo}}/releases/{}/assets?per_page=100",
                release.id
            );
            runner =
                runner.with_stdout(&pages_command(&endpoint), &pages(&self.assets[&release.id]));
        }
        for (id, bytes) in &self.bodies {
            runner = runner.with_bytes(&asset_command(*id), bytes.clone());
        }
        runner
    }
}

pub(crate) fn seed_releases(runner: RecordingRunner, tags: &str) -> RecordingRunner {
    let mut fixture = PublicationApiFixture::default();
    for tag in tags
        .lines()
        .filter(|tag| crate::version::parse_release_tag(tag).is_ok())
    {
        fixture.release(tag);
    }
    fixture.seed(runner)
}

pub(crate) fn pages_command(endpoint: &str) -> String {
    format!("gh api --paginate --slurp {endpoint}")
}

pub(crate) fn asset_command(id: u64) -> String {
    format!(
        "gh api repos/{{owner}}/{{repo}}/releases/assets/{id} --header Accept: application/octet-stream"
    )
}

pub(crate) fn pages<T: serde::Serialize>(items: &[T]) -> String {
    if items.is_empty() {
        return "[[]]".into();
    }
    serde_json::to_string(&items.chunks(100).collect::<Vec<_>>()).unwrap()
}

pub(crate) fn archive_bytes(name: &str, version: &str, requirements: &str) -> Vec<u8> {
    let directory = tempfile::tempdir().unwrap();
    let manifest = directory.path().join("Cargo.toml");
    std::fs::write(
        &manifest,
        format!("[package]\nname='{name}'\nversion='{version}'\n[dependencies]\n{requirements}"),
    )
    .unwrap();
    let license = directory.path().join("LICENSE-GPL");
    std::fs::write(&license, "fixture license").unwrap();
    let entries = archive_entries(manifest, license);
    let archive = directory.path().join("input.crate");
    write_tar_gz(&archive, &format!("{name}-{version}"), &entries).unwrap();
    std::fs::read(archive).unwrap()
}

fn archive_entries(manifest: std::path::PathBuf, license: std::path::PathBuf) -> [BundleEntry; 2] {
    [
        BundleEntry {
            source: manifest,
            archive_path: "Cargo.toml".into(),
            executable: false,
        },
        BundleEntry {
            source: license,
            archive_path: "LICENSE-GPL".into(),
            executable: false,
        },
    ]
}

pub(crate) fn package(name: &str, version: &str) -> crate::workspace::WorkspacePackage {
    crate::workspace::WorkspacePackage {
        name: name.into(),
        version: Version::parse(version).unwrap(),
        dir: format!("crates/{name}"),
        path_dependencies: Vec::new(),
    }
}
