// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use std::process::Command;

#[test]
fn immutable_trim_and_extend_match_concurrently_and_in_separate_processes() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<(LineEditOutcome, LineEditFailure)>();
    let trim_source = fixture(&TRIM);
    let extend_source = fixture(&EXTEND);
    let before = (trim_source.clone(), extend_source.clone());
    let original = replay(&trim_source, &extend_source);
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for _ in 0..16 {
                    assert_eq!(replay(&trim_source, &extend_source), original);
                }
            });
        }
    });
    assert_eq!((trim_source, extend_source), before);
    assert_eq!(process(), process());
}

fn process() -> String {
    let result = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "line_edit::replay::emit_line_edit_replay",
            "--nocapture",
            "--test-threads=1",
        ])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let stdout = String::from_utf8(result.stdout).unwrap();
    stdout
        .split_once("LINE_EDIT_REPLAY=")
        .unwrap()
        .1
        .lines()
        .next()
        .unwrap()
        .to_owned()
}

#[test]
fn emit_line_edit_replay() {
    let result = replay(&fixture(&TRIM), &fixture(&EXTEND));
    println!("LINE_EDIT_REPLAY={result}");
}

fn replay(trim_source: &Snapshot, extend_source: &Snapshot) -> String {
    let trimmed = edited(trim_source.trim_line(trim(0.25), || false).unwrap());
    let extended = edited(
        extend_source
            .extend_line(extension(&[lid(6)], LineEndpoint::End), || false)
            .unwrap(),
    );
    format!("{} | {}", encode(&trimmed), encode(&extended))
}

fn encode(edit: &LineEdit) -> String {
    let position = edit.contact.position.map(f64::to_bits);
    let parameters = edit
        .contact
        .parameters
        .map(|bounds| bounds.map(f64::to_bits));
    format!("{:?} {position:016x?} {parameters:016x?}", edit.replacement)
}
