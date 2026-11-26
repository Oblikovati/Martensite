// SPDX-License-Identifier: GPL-2.0-only
use super::{DIRECTION, POINTS, archive, recipe, result, snapshot};

#[test]
fn detached_prism_and_semantic_history_replay_across_processes() {
    let original = recorded_prism(false);
    assert_eq!(original, recorded_prism(false));
    assert_eq!(original, recorded_prism(true));
}

#[test]
fn concurrent_detached_reads_repeat_the_serial_export() {
    let prism = result(&recipe(&snapshot(POINTS)), DIRECTION);
    let canonical = archive(&prism.export_brep(|| false).unwrap().solid);
    std::thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| {
                for _ in 0..4 {
                    let detached = prism.export_brep(|| false).unwrap();
                    assert_eq!(detached.images.len(), 26);
                    assert_eq!(archive(&detached.solid), canonical);
                }
            });
        }
    });
}

fn recorded_prism(changed_legacy_precision: bool) -> String {
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .env(
            "MARTENSITE_TEST_CHANGE_LEGACY_PRECISION",
            if changed_legacy_precision { "1" } else { "0" },
        )
        .args([
            "--exact",
            "replay::emit_prism_archive_and_history",
            "--nocapture",
            "--test-threads=1",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let row = stdout.split_once("PRISM_ARCHIVE=").unwrap().1;
    row.lines().take(2).collect::<Vec<_>>().join("\n")
}

#[test]
fn emit_prism_archive_and_history() {
    use std::fmt::Write;
    if std::env::var("MARTENSITE_TEST_CHANGE_LEGACY_PRECISION").as_deref() == Ok("1") {
        martensite_tktopalgo::brep_lib::set_precision(0.5);
    }
    let prism = result(&recipe(&snapshot(POINTS)), DIRECTION);
    let bytes = archive(&prism.export_brep(|| false).unwrap().solid);
    let mut hex = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(hex, "{byte:02x}").unwrap();
    }
    println!("PRISM_ARCHIVE={hex}");
    println!("PRISM_HISTORY={:?}", prism.history().collect::<Vec<_>>());
}
