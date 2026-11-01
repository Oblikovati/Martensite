//! Named API fake exercises real restored archive bytes through the complete loader.
use super::super::{AssetHistory, ReleaseIndex, fixtures::PublicationApiFixture};
use super::*;
use crate::{context::tests::context_at, fake_runner::RecordingRunner};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

struct RestorationApiFake {
    api: PublicationApiFixture,
    current: u64,
    historical: ArchiveOrigin,
    payload: Vec<u8>,
    manifest: Value,
}

impl RestorationApiFake {
    fn new() -> Self {
        let mut api = original_api();
        let old = api.releases[0].id;
        let history = load(&api).unwrap();
        let historical = history.receipt.archives[0].clone();
        let payload = api.bodies[&historical.asset_id].clone();
        let current = api.release("v0.5.0");
        let catalog = catalog(&historical, history.receipt);
        let manifest = json!({"schema":1,"catalog_sha256":format!("{:x}",Sha256::digest(&catalog)),
            "entries":[{"historical_origin":historical}]});
        api.releases.retain(|release| release.id != old);
        api.pair(current, "martensite-0.5.0-packages.json", catalog.clone());
        Self {
            api,
            current,
            historical,
            payload,
            manifest,
        }
    }

    fn evaluate(
        mut self,
        extra: Option<(&str, Vec<u8>)>,
    ) -> crate::error::BuildResult<AssetHistory> {
        let entries = self.entries(extra);
        self.api
            .pair(self.current, BUNDLE_NAME, bundle_bytes(&entries));
        load(&self.api)
    }
    fn entries(&self, extra: Option<(&str, Vec<u8>)>) -> Vec<(String, Vec<u8>)> {
        let mut entries = vec![
            (
                "restoration.json".into(),
                self.manifest.to_string().into_bytes(),
            ),
            (
                format!("archives/{}", self.historical.archive),
                self.payload.clone(),
            ),
        ];
        if let Some((name, bytes)) = extra {
            entries.push((name.into(), bytes));
        }
        entries
    }
}

fn original_api() -> PublicationApiFixture {
    let mut api = PublicationApiFixture::default();
    let old = api.release("v0.3.6");
    api.archive(old, "martensite-math", "0.3.5", "");
    api
}

fn catalog(origin: &ArchiveOrigin, receipt: super::super::HistoryReceipt) -> Vec<u8> {
    json!({"schema":2,"release":"0.5.0","publication_history":receipt,
        "packages":[{"name":"martensite-math","version":"0.3.5","shipped":false,
        "publication_reason":"historical_reuse","archive_sha256":origin.sha256,
        "origins":[origin]}]})
    .to_string()
    .into_bytes()
}

fn load(api: &PublicationApiFixture) -> crate::error::BuildResult<AssetHistory> {
    let directory = tempfile::tempdir().unwrap();
    let runner = api.seed(RecordingRunner::default());
    let context = context_at(&runner, directory.path());
    AssetHistory::load(&context, &ReleaseIndex::load(&context)?)
}

fn bundle_bytes(entries: &[(String, Vec<u8>)]) -> Vec<u8> {
    let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    let mut tar = tar::Builder::new(encoder);
    for (name, bytes) in entries {
        let mut header = tar::Header::new_gnu();
        header.set_size(bytes.len() as u64);
        header.set_mode(0o644);
        tar.append_data(&mut header, name, bytes.as_slice())
            .unwrap();
    }
    tar.into_inner().unwrap().finish().unwrap()
}

#[test]
fn restored_bytes_record_historical_and_live_anchor_separately() {
    let fake = RestorationApiFake::new();
    let historical = fake.historical.clone();
    let history = fake.evaluate(None).unwrap();
    let verified = &history.receipt.archives[0];
    assert_eq!(verified.release_id, historical.release_id);
    assert!(verified.matches_claim(&historical));
    assert_ne!(
        verified.restoration.as_ref().unwrap().release_id,
        historical.release_id
    );
    assert!(verified.restoration.as_ref().unwrap().asset_id > historical.asset_id);
}

#[test]
fn restoration_cannot_replace_wrong_payload_or_manifest_digest() {
    let mut payload = RestorationApiFake::new();
    payload.payload.push(1);
    assert!(payload.evaluate(None).is_err());
    let mut manifest = RestorationApiFake::new();
    manifest.manifest["entries"][0]["historical_origin"]["manifest_sha256"] = json!("0".repeat(64));
    assert!(manifest.evaluate(None).is_err());
}

#[test]
fn restoration_requires_exact_catalog_and_historical_membership() {
    let mut catalog = RestorationApiFake::new();
    catalog.manifest["catalog_sha256"] = json!("0".repeat(64));
    assert!(catalog.evaluate(None).is_err());
    let mut origin = RestorationApiFake::new();
    origin.manifest["entries"][0]["historical_origin"]["asset_id"] = json!(9999);
    assert!(origin.evaluate(None).is_err());
}

#[test]
fn restoration_refuses_extra_duplicate_and_foreign_members() {
    for name in [
        "unrequested",
        "restoration.json",
        "archives/martensite-other-0.3.5.crate",
    ] {
        assert!(
            RestorationApiFake::new()
                .evaluate(Some((name, vec![1])))
                .is_err()
        );
    }
}

#[test]
fn restoration_refuses_empty_schema_and_missing_origin() {
    for replacement in [json!([]), json!([{"historical_origin":{}}])] {
        let mut fake = RestorationApiFake::new();
        fake.manifest["entries"] = replacement;
        assert!(fake.evaluate(None).is_err());
    }
    let mut fake = RestorationApiFake::new();
    fake.manifest["schema"] = json!(2);
    assert!(fake.evaluate(None).is_err());
}

#[test]
fn old_origin_wire_omits_restoration_and_anchor_claims_are_exact() {
    let fake = RestorationApiFake::new();
    let historical = fake.historical.clone();
    assert!(
        serde_json::to_value(&historical)
            .unwrap()
            .get("restoration")
            .is_none()
    );
    let history = fake.evaluate(None).unwrap();
    let mut wrong = history.receipt.archives[0].clone();
    wrong.restoration.as_mut().unwrap().asset_id += 1;
    assert!(!history.receipt.archives[0].matches_claim(&wrong));
}

pub(crate) fn verified_restored_history() -> AssetHistory {
    // Staging regressions share the actual named API fake rather than inventing another loader.
    RestorationApiFake::new().evaluate(None).unwrap()
}
