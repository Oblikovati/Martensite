use super::*;
use crate::{context::tests::context_at, fake_runner::RecordingRunner};
use fixtures::{PublicationApiFixture, pages_command};

fn runner(directory: &std::path::Path) -> RecordingRunner {
    RecordingRunner::default().with_stdout(
        "git rev-parse --show-toplevel",
        &directory.to_string_lossy(),
    )
}

#[test]
fn failed_git_tags_are_not_publication_and_release_pins_detect_changes() {
    let directory = tempfile::tempdir().unwrap();
    let mut fixture = PublicationApiFixture::default();
    fixture.release("v0.3.6");
    let runner =
        fixture.seed(runner(directory.path()).with_stdout("git tag --list v*", "v0.3.5\nv0.3.6"));
    let index = ReleaseIndex::load(&context_at(&runner, directory.path())).unwrap();
    assert_eq!(index.final_tags(), ["v0.3.6"]);
    index
        .require_pin(Some(&index.fingerprint().unwrap()))
        .unwrap();
    assert!(index.require_pin(Some(&"0".repeat(64))).is_err());
    assert!(
        runner
            .commands()
            .iter()
            .all(|command| !command.contains("release view"))
    );
}

#[test]
fn authentication_and_malformed_history_fail_instead_of_appearing_empty() {
    let directory = tempfile::tempdir().unwrap();
    let command = pages_command(RELEASES);
    let denied = runner(directory.path()).with_exit_code(&command, 1);
    assert!(ReleaseIndex::load(&context_at(&denied, directory.path())).is_err());
    let malformed = runner(directory.path()).with_stdout(&command, "[]");
    assert!(ReleaseIndex::load(&context_at(&malformed, directory.path())).is_err());
}

#[test]
fn every_release_page_is_read_and_drafts_do_not_establish_publication() {
    let directory = tempfile::tempdir().unwrap();
    let mut fixture = PublicationApiFixture::default();
    for patch in 0..101 {
        fixture.release(&format!("v0.1.{patch}"));
    }
    fixture.releases[0].draft = true;
    let runner = fixture.seed(runner(directory.path()));
    let index = ReleaseIndex::load(&context_at(&runner, directory.path())).unwrap();
    assert_eq!(index.releases.len(), 100);
    assert_eq!(index.releases.last().unwrap().tag_name, "v0.1.100");
}

#[test]
fn exact_archive_checksums_and_catalog_claims_are_required() {
    let directory = tempfile::tempdir().unwrap();
    let mut fixture = PublicationApiFixture::default();
    let release = fixture.release("v0.3.6");
    fixture.catalog(
        release,
        serde_json::json!([{"name":"martensite-math","version":"0.3.5","shipped":true}]),
    );
    let runner = fixture.seed(runner(directory.path()));
    let context = context_at(&runner, directory.path());
    let index = ReleaseIndex::load(&context).unwrap();
    let error = AssetHistory::load(&context, &index).err().unwrap();
    assert!(error.to_string().contains("shipment flag"));
}

#[test]
fn repeated_identities_require_identical_archive_bytes_including_prereleases() {
    for changed in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let mut fixture = PublicationApiFixture::default();
        let first = fixture.release("v0.3.6-rc.1");
        let second = fixture.release("v0.3.6");
        fixture.archive(first, "martensite-math", "0.3.5", "");
        fixture.archive(
            second,
            "martensite-math",
            "0.3.5",
            if changed { "extra='1'" } else { "" },
        );
        let runner = fixture.seed(runner(directory.path()));
        let result = load_history(&runner, directory.path());
        assert_eq!(result.is_err(), changed);
        if let Ok(history) = result {
            assert_eq!(history.archives.values().next().unwrap().origins.len(), 2);
        }
    }
}

#[test]
fn later_asset_pages_are_required_to_find_published_package_identities() {
    let directory = tempfile::tempdir().unwrap();
    let mut fixture = PublicationApiFixture::default();
    let release = fixture.release("v0.3.6");
    for number in 0..100 {
        fixture.asset(release, &format!("notice-{number}.txt"), b"notice".to_vec());
    }
    fixture.archive(release, "martensite-math", "0.3.5", "");
    let runner = fixture.seed(runner(directory.path()));
    let context = context_at(&runner, directory.path());
    let history = AssetHistory::load(&context, &ReleaseIndex::load(&context).unwrap()).unwrap();
    assert!(
        history
            .archives
            .contains_key(&("martensite-math".into(), semver::Version::new(0, 3, 5)))
    );
}

#[test]
fn orphan_payloads_and_checksum_companions_are_incomplete_publications() {
    for remove_payload in [true, false] {
        let directory = tempfile::tempdir().unwrap();
        let mut fixture = PublicationApiFixture::default();
        let release = fixture.release("v0.3.6");
        fixture.archive(release, "martensite-math", "0.3.5", "");
        fixture
            .assets
            .get_mut(&release)
            .unwrap()
            .retain(|asset| asset.name.ends_with(".sha256") == remove_payload);
        let runner = fixture.seed(runner(directory.path()));
        let error = load_history(&runner, directory.path()).err().unwrap();
        assert!(error.to_string().contains(if remove_payload {
            "matching crate payload"
        } else {
            "checksum companion"
        }));
    }
}

#[test]
fn corrupt_asset_bytes_and_http_failure_do_not_establish_absence() {
    for http_error in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let mut fixture = PublicationApiFixture::default();
        let release = fixture.release("v0.3.6");
        fixture.archive(release, "martensite-math", "0.3.5", "");
        let id = fixture.assets[&release][0].id;
        if !http_error {
            fixture.bodies.get_mut(&id).unwrap()[0] ^= 1;
        }
        let mut runner = fixture.seed(runner(directory.path()));
        if http_error {
            runner = runner.with_exit_code(&fixtures::asset_command(id), 1);
        }
        let error = load_history(&runner, directory.path()).err().unwrap();
        assert!(error.to_string().contains(if http_error {
            "successful asset retrieval"
        } else {
            "published asset size and SHA-256"
        }));
    }
}

#[test]
fn orphan_catalog_checksums_do_not_erase_publication_receipts() {
    let directory = tempfile::tempdir().unwrap();
    let mut fixture = PublicationApiFixture::default();
    let release = fixture.release("v0.3.6");
    fixture.catalog(release, serde_json::json!([]));
    fixture
        .assets
        .get_mut(&release)
        .unwrap()
        .retain(|asset| asset.name.ends_with(".sha256"));
    let runner = fixture.seed(runner(directory.path()));
    let context = context_at(&runner, directory.path());
    let error = AssetHistory::load(&context, &ReleaseIndex::load(&context).unwrap())
        .err()
        .unwrap();
    assert!(
        error
            .to_string()
            .contains("complete release catalog payload")
    );
}

#[test]
fn publication_evidence_has_structural_thread_bounds() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<fixtures::PublicationApiFixture>();
    send_sync::<super::catalogs::Row>();
    send_sync::<PublishedRelease>();
    send_sync::<ReleaseIndex>();
    send_sync::<ArchiveOrigin>();
    send_sync::<HistoryReceipt>();
    send_sync::<AssetHistory>();
    send_sync::<super::archives::PublishedArchive>();
    send_sync::<super::inventory::Asset>();
    send_sync::<crate::publication_plan::PublicationReason>();
    send_sync::<crate::publication_plan::PackagePublication>();
    send_sync::<crate::publication_plan::PublicationPlan>();
}

fn load_history(
    runner: &RecordingRunner,
    directory: &std::path::Path,
) -> crate::error::BuildResult<AssetHistory> {
    let context = context_at(runner, directory);
    let index = ReleaseIndex::load(&context)?;
    AssetHistory::load(&context, &index)
}
