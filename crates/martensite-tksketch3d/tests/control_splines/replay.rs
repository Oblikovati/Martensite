// SPDX-License-Identifier: GPL-2.0-only
use super::{NONPLANAR, rows, sid, snapshot};
use martensite_tksketch3d::{CurveId, CurveRole, Edit};
use std::process::Command;

const RECEIPT: &str = "{\"control_spline_replay\":";

#[test]
fn independent_processes_replay_the_same_spline_coefficients_and_jets() {
    let first = replay_receipt();
    let second = replay_receipt();
    assert_eq!(first, second);
}

fn replay_receipt() -> String {
    let output = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "control_splines::replay::emit_control_spline_replay",
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
    stdout
        .split_once(RECEIPT)
        .unwrap()
        .1
        .lines()
        .next()
        .unwrap()
        .to_owned()
}

#[test]
fn emit_control_spline_replay() {
    let source = snapshot(NONPLANAR);
    let curve = CurveId::ControlSpline(sid(5));
    let role = CurveRole::Construction;
    let updated = source
        .apply(&[Edit::SetCurveRole { curve, role }], || false)
        .unwrap();
    let spline = updated.control_spline(sid(5)).unwrap();
    let proof = spline.regularity();
    let policy = spline.recipe().proof;
    let mut receipt = vec![
        proof.source().curve_id(),
        u64::from(updated.curve_role(curve) == Some(role)),
        policy.parameter_width().to_bits(),
        u64::from(policy.max_bisections()),
        u64::from(proof.bisections()),
    ];
    receipt.extend(spline.recipe().poles.map(|point| point.get().get()));
    receipt.extend(
        proof
            .source()
            .poles()
            .iter()
            .flatten()
            .map(|value| value.to_bits()),
    );
    for parameter in spline.range().into_iter().chain([0.25, 0.5, 0.75]) {
        receipt.extend(
            rows(spline.jet(parameter).unwrap())
                .into_iter()
                .flatten()
                .map(f64::to_bits),
        );
    }
    println!("{RECEIPT}{receipt:?}}}");
}
