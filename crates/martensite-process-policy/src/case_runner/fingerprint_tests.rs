use super::{context::fingerprint, fake_controls::OwnedArtifactRoot};
use std::{fs, time::Instant};

#[test]
fn executable_fingerprint_keeps_sha256_and_detects_same_length_changes() {
    let root = OwnedArtifactRoot::new();
    let executable = root.path().join("executable-fixture");
    fs::write(&executable, b"abc").unwrap();
    let original = fingerprint(&executable).unwrap();
    assert_eq!(
        original,
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    fs::write(&executable, b"abd").unwrap();
    assert_ne!(fingerprint(&executable).unwrap(), original);
    fs::remove_file(&executable).unwrap();
    assert!(fingerprint(&executable).is_err());
}

// Explicit measurement, not a machine-dependent timing gate (#228).
#[test]
#[ignore = "run explicitly with --ignored --nocapture to measure fingerprint overhead"]
fn measure_three_executable_fingerprints() {
    let executable = std::env::current_exe().unwrap();
    let bytes = fs::metadata(&executable).unwrap().len();
    let started = Instant::now();
    let first = fingerprint(&executable).unwrap();
    assert_eq!(fingerprint(&executable).unwrap(), first);
    assert_eq!(fingerprint(&executable).unwrap(), first);
    println!(
        "{{\"executable_bytes\":{bytes},\"fingerprints\":3,\"elapsed_seconds\":{}}}",
        started.elapsed().as_secs_f64()
    );
}
