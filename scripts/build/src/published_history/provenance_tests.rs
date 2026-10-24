//! Each negative mutates one otherwise valid retained publication receipt.
use super::{fixtures::*, *};
use crate::{context::tests::context_at, fake_runner::RecordingRunner};
use serde_json::{Value, json};

struct ReuseFixture {
    api: PublicationApiFixture,
    original: u64,
    later: u64,
    catalog: Value,
}

impl ReuseFixture {
    fn new() -> Self {
        let mut api = PublicationApiFixture::default();
        let original = api.release("v0.3.6");
        api.archive(original, "martensite-math", "0.3.5", "");
        let receipt = verified_receipt(&api);
        let origin = &receipt.archives[0];
        let later = api.release("v0.4.0");
        let catalog = json!({"schema":2,"release":"0.4.0","publication_history":receipt,
            "packages":[{"name":"martensite-math","version":"0.3.5","shipped":false,
                "publication_reason":"historical_reuse","archive_sha256":origin.sha256,
                "origins":[origin]}]});
        Self {
            api,
            original,
            later,
            catalog,
        }
    }

    fn load(mut self) -> crate::error::BuildResult<AssetHistory> {
        self.api.pair(
            self.later,
            "martensite-0.4.0-packages.json",
            self.catalog.to_string().into_bytes(),
        );
        let directory = tempfile::tempdir().unwrap();
        let runner = self.api.seed(RecordingRunner::default());
        let context = context_at(&runner, directory.path());
        AssetHistory::load(&context, &ReleaseIndex::load(&context)?)
    }

    fn row(&mut self) -> &mut Value {
        &mut self.catalog["packages"][0]
    }
}

fn verified_receipt(api: &PublicationApiFixture) -> HistoryReceipt {
    let directory = tempfile::tempdir().unwrap();
    let runner = api.seed(RecordingRunner::default());
    let context = context_at(&runner, directory.path());
    AssetHistory::load(&context, &ReleaseIndex::load(&context).unwrap())
        .unwrap()
        .receipt
}

#[test]
fn schema_two_retained_reuse_accepts_verified_identity() {
    let history = ReuseFixture::new().load().unwrap();
    assert_eq!(history.archives.len(), 1);
    assert_eq!(history.receipt.archives[0].release_tag, "v0.3.6");
}

#[test]
fn schema_two_reuse_digest_cannot_disagree_with_retained_bytes() {
    let mut fixture = ReuseFixture::new();
    fixture.row()["archive_sha256"] = json!("0".repeat(64));
    assert!(
        fixture
            .load()
            .err()
            .unwrap()
            .to_string()
            .contains("row digest matching")
    );
}

#[test]
fn schema_two_reuse_requires_real_nonempty_origins() {
    for empty in [false, true] {
        let mut fixture = ReuseFixture::new();
        if empty {
            fixture.row()["origins"] = json!([]);
        } else {
            fixture.row()["origins"][0]["asset_id"] = json!(999);
        }
        let error = fixture.load().err().unwrap().to_string();
        assert!(error.contains(if empty {
            "recorded historical origin"
        } else {
            "verified publication origins"
        }));
    }
}

#[test]
fn schema_two_reuse_cannot_be_relabeled_first_publication() {
    let mut fixture = ReuseFixture::new();
    fixture.row()["publication_reason"] = json!("first_publication");
    assert!(
        fixture
            .load()
            .err()
            .unwrap()
            .to_string()
            .contains("shipped new identity")
    );
}

#[test]
fn disappearing_original_assets_do_not_become_first_publications() {
    let mut fixture = ReuseFixture::new();
    fixture
        .api
        .assets
        .get_mut(&fixture.original)
        .unwrap()
        .clear();
    let error = fixture.load().err().unwrap().to_string();
    assert!(error.contains("verified published archive for a claimed digest"));
}

#[test]
fn release_index_change_during_archive_verification_fails_closed() {
    let directory = tempfile::tempdir().unwrap();
    let mut api = PublicationApiFixture::default();
    api.release("v0.3.6");
    let original = pages(&api.releases);
    api.release("v0.4.0");
    let runner = api.seed(RecordingRunner::default()).with_stdout_sequence(
        &pages_command(RELEASES),
        vec![original, pages(&api.releases)],
    );
    let context = context_at(&runner, directory.path());
    let error = AssetHistory::load(&context, &ReleaseIndex::load(&context).unwrap())
        .err()
        .unwrap();
    assert!(
        error
            .to_string()
            .contains("stable complete publication snapshot")
    );
}

#[test]
fn asset_listing_change_during_archive_verification_fails_closed() {
    let directory = tempfile::tempdir().unwrap();
    let mut api = PublicationApiFixture::default();
    let release = api.release("v0.3.6");
    api.asset(release, "notice.txt", b"notice".to_vec());
    let command = pages_command(&format!(
        "repos/{{owner}}/{{repo}}/releases/{release}/assets?per_page=100"
    ));
    let runner = api
        .seed(RecordingRunner::default())
        .with_stdout_sequence(&command, vec!["[[]]".into(), pages(&api.assets[&release])]);
    let context = context_at(&runner, directory.path());
    let error = AssetHistory::load(&context, &ReleaseIndex::load(&context).unwrap())
        .err()
        .unwrap();
    assert!(
        error
            .to_string()
            .contains("stable complete publication snapshot")
    );
}
