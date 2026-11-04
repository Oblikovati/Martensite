use std::sync::atomic::{AtomicU64, Ordering};
use super::*;
use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
};

struct TemporaryWitnessRoot(PathBuf);
impl TemporaryWitnessRoot {
    fn new() -> Self {
        // Parallel tests can read the same clock value, so the counter keeps the paths distinct.
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let sequence = NEXT.fetch_add(1, Ordering::Relaxed);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("martensite-witness-{}-{nonce}-{sequence}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn path(&self) -> &Path {
        &self.0
    }
}
impl Drop for TemporaryWitnessRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn test_pieces(marker: &[u8]) -> Vec<WitnessPiece> {
    let mut pieces = PIECES
        .iter()
        .map(|name| WitnessPiece {
            name,
            bytes: marker.to_vec(),
        })
        .collect::<Vec<_>>();
    pieces
        .iter_mut()
        .find(|piece| piece.name == "complete.txt")
        .unwrap()
        .bytes = b"complete\n".to_vec();
    pieces
}

fn publish_test(root: &Path, ordinal: usize, image: &[u8], marker: &[u8]) -> io::Result<()> {
    let image_path = root.join("same.png");
    fs::write(&image_path, image)?;
    publish(
        &root.join("same.png.witness"),
        "same.png",
        ordinal,
        &image_path,
        &test_pieces(marker),
    )
}

#[test]
fn repeated_same_image_path_keeps_generations_and_points_to_latest_bytes() {
    let temp = TemporaryWitnessRoot::new();
    publish_test(temp.path(), 2, b"png-first", b"first").unwrap();
    publish_test(temp.path(), 7, b"png-final", b"second").unwrap();
    assert_final_occurrence(temp.path());
}

fn assert_final_occurrence(root: &Path) {
    let witness = root.join("same.png.witness");
    let index = read_index(&witness);
    assert_eq!(index["latest_ordinal"], 7);
    assert_eq!(index["latest_snapshot"], "occurrences/dump-000007");
    assert_eq!(index["occurrences"].as_array().unwrap().len(), 2);
    assert_eq!(
        index["occurrences"][1]["image_sha256"],
        digest(b"png-final")
    );
    assert_snapshot_bytes(&witness);
}

fn read_index(witness: &Path) -> Value {
    serde_json::from_slice(&fs::read(witness.join("latest.json")).unwrap()).unwrap()
}

fn assert_snapshot_bytes(witness: &Path) {
    assert_eq!(fs::read(witness.join("capture.rgba")).unwrap(), b"second");
    assert_eq!(
        fs::read(witness.join("occurrences/dump-000002/capture.rgba")).unwrap(),
        b"first"
    );
    assert_eq!(
        fs::read(witness.join("occurrences/dump-000007/capture.rgba")).unwrap(),
        b"second"
    );
}

#[test]
fn tampered_prior_generation_refuses_to_publish_a_new_latest_snapshot() {
    let temp = TemporaryWitnessRoot::new();
    publish_test(temp.path(), 1, b"png-one", b"first").unwrap();
    let witness = temp.path().join("same.png.witness");
    fs::write(
        witness.join("occurrences/dump-000001/capture.rgba"),
        b"foreign",
    )
    .unwrap();
    let error = publish_test(temp.path(), 3, b"png-three", b"third").unwrap_err();
    assert!(error.to_string().contains("immutable occurrence hash"));
    let index: Value =
        serde_json::from_slice(&fs::read(witness.join("latest.json")).unwrap()).unwrap();
    assert_eq!(index["latest_ordinal"], 1);
}

#[test]
fn incomplete_or_foreign_outputs_are_refused_without_replacing_latest() {
    let temp = TemporaryWitnessRoot::new();
    let witness = temp.path().join("same.png.witness");
    fs::create_dir(&witness).unwrap();
    fs::write(witness.join("foreign.txt"), b"unexpected").unwrap();
    let error = publish_test(temp.path(), 1, b"png", b"frame").unwrap_err();
    assert!(error.to_string().contains("fresh witness directory"));
    assert!(!witness.join("latest.json").exists());
}

#[test]
fn foreign_index_and_manifest_fields_refuse_later_publication() {
    assert_foreign_index_refused();
    assert_foreign_manifest_refused();
}

fn assert_foreign_index_refused() {
    let temp = TemporaryWitnessRoot::new();
    publish_test(temp.path(), 1, b"png-one", b"first").unwrap();
    let path = temp.path().join("same.png.witness/latest.json");
    let mut index: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    index["foreign"] = Value::Bool(true);
    fs::write(path, serde_json::to_vec(&index).unwrap()).unwrap();
    assert!(
        publish_test(temp.path(), 2, b"png-two", b"second")
            .unwrap_err()
            .to_string()
            .contains("owned witness sequence index")
    );
}

fn assert_foreign_manifest_refused() {
    let temp = TemporaryWitnessRoot::new();
    publish_test(temp.path(), 1, b"png-one", b"first").unwrap();
    let path = temp
        .path()
        .join("same.png.witness/occurrences/dump-000001/occurrence.json");
    let mut manifest: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    manifest["foreign"] = Value::Bool(true);
    fs::write(path, serde_json::to_vec(&manifest).unwrap()).unwrap();
    assert!(
        publish_test(temp.path(), 2, b"png-two", b"second")
            .unwrap_err()
            .to_string()
            .contains("matching complete occurrence manifest")
    );
}

#[cfg(unix)]
#[test]
fn symlink_witness_root_is_rejected_without_writing_through_it() {
    use std::os::unix::fs::symlink;
    let temp = TemporaryWitnessRoot::new();
    let outside = temp.path().join("outside");
    fs::create_dir(&outside).unwrap();
    symlink(&outside, temp.path().join("same.png.witness")).unwrap();
    let error = publish_test(temp.path(), 1, b"png", b"frame").unwrap_err();
    assert!(error.to_string().contains("real directory, not symlink"));
    assert_eq!(fs::read_dir(outside).unwrap().count(), 0);
}

#[cfg(unix)]
#[test]
fn symlinked_output_ancestor_is_rejected_without_writing_through_it() {
    use std::os::unix::fs::symlink;
    let temp = TemporaryWitnessRoot::new();
    let outside = temp.path().join("outside");
    fs::create_dir(&outside).unwrap();
    fs::write(outside.join("same.png"), b"png").unwrap();
    symlink(&outside, temp.path().join("alias")).unwrap();
    let image = temp.path().join("alias/same.png");
    let witness = temp.path().join("alias/same.png.witness");
    let error = publish(&witness, "same.png", 1, &image, &test_pieces(b"frame")).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("real directory ancestry without symlinks")
    );
    assert!(!outside.join("same.png.witness").exists());
}

#[test]
fn preexisting_partial_generation_blocks_later_publication() {
    let temp = TemporaryWitnessRoot::new();
    let witness = temp.path().join("same.png.witness");
    fs::create_dir_all(witness.join("occurrences/.pending-000001")).unwrap();
    fs::write(
        witness.join("occurrences/.pending-000001/capture.rgba"),
        b"partial",
    )
    .unwrap();
    let error = publish_test(temp.path(), 1, b"png", b"frame").unwrap_err();
    assert!(error.to_string().contains("fresh witness directory"));
    assert!(!witness.join("latest.json").exists());
}
