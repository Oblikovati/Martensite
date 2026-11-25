// SPDX-License-Identifier: GPL-2.0-only
use super::{coords, insertion, join, pid, point, policy_15, sample};
use martensite_tksketch3d::constraints::{ConstrainedSketch, CurvatureTolerance, RelationSpec};
use std::{process::Command, sync::Barrier, thread};

const RECEIPT: &str = "{\"spatial_g2_replay\":";

fn prepared() -> ConstrainedSketch {
    let snapshot = sample(
        [[-1., 0., 0.], [-2., 1., 0.], [-3., 1., 1.]],
        [[2., 0., 0.], [4., 4., 0.], [6., 4., 2.]],
        [true, false],
    );
    let cap = CurvatureTolerance::new(1e-7).unwrap();
    ConstrainedSketch::new(snapshot, policy_15())
        .unwrap()
        .apply(&insertion(RelationSpec::G2(join(), cap)), || false)
        .unwrap()
}

fn receipt(source: &ConstrainedSketch) -> Vec<u64> {
    let mut current = source.clone();
    let mut bits = Vec::new();
    for target in [
        [-1.04, -0.02, 0.01],
        [-1.08, 0.02, 0.03],
        [-1.12, 0.04, -0.02],
    ] {
        let moved = current.drag(pid(1), point(target), || false).unwrap();
        bits.extend([moved.rank.rank as u64, moved.rank.degrees_of_freedom as u64]);
        bits.push(moved.solver_hard_error.to_bits());
        bits.push(moved.residual_work_bound as u64);
        for id in 1..=7 {
            bits.extend(coords(moved.sketch.geometry(), pid(id)).map(f64::to_bits));
        }
        current = moved.sketch;
    }
    bits
}

#[test]
fn emit_replay() {
    println!("{RECEIPT}{:?}}}", receipt(&prepared()));
}

#[test]
fn adopted_pair_replays_byte_for_byte_across_processes() {
    let replay = || {
        let output = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "continuity::cubic_pair::replay::emit_replay",
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
    };
    assert_eq!(replay(), replay());
}

#[test]
fn simultaneous_ticks_keep_their_accepted_geometry_and_reuse_immutable_sources() {
    let source = prepared();
    let snapshot = source.geometry().clone();
    let expected = receipt(&source);
    let barrier = Barrier::new(4);
    thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                barrier.wait();
                for _ in 0..16 {
                    assert_eq!(receipt(&source), expected);
                }
            });
        }
    });
    assert_eq!(source.geometry(), &snapshot);
}
