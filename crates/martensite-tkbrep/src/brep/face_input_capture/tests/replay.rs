//! The owned publication payload includes every field in the complete typed snapshot.
use super::*;
use std::process::Command;

const REPLAY_SELECTOR: &str = "MARTENSITE_FACE_INPUT_REPLAY";
const REPLAY_MARKER: &str = "FACE_INPUT_REPLAY:";
const REPLAY_TEST: &str =
    "brep::face_input_capture::tests::replay::owned_input_is_identical_between_two_processes";

#[test]
fn owned_input_is_identical_between_two_processes() {
    if std::env::var_os(REPLAY_SELECTOR).is_some() {
        println!("{REPLAY_MARKER}{:?}", complete_snapshot());
        return;
    }
    assert_eq!(process_payload(), process_payload());
}
fn complete_snapshot() -> FaceInputSnapshot {
    let (face, fields) = face_fixture();
    let surface = fields.surface().cast::<GeomSurfaceOfLinearExtrusion>();
    representations::append_owner(&face, representations::seam_edge(&surface).cast());
    let mut transform = martensite_tkmath::gp::Transform3::default();
    transform.set_translation(martensite_tkmath::gp::Vector3::new(3., -2., 5.));
    fields.set_location(&Location::new(transform));
    captured(&face)
}
fn process_payload() -> String {
    // The canonical package harness bounds these real child processes.
    let output = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", REPLAY_TEST, "--nocapture"])
        .env(REPLAY_SELECTOR, "1")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "child status {:?}; expected success; stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let payload = stdout
        .split_once(REPLAY_MARKER)
        .expect("expected actual owned input payload")
        .1;
    payload.lines().next().unwrap().to_owned()
}
