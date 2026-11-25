// SPDX-License-Identifier: GPL-2.0-only
use super::{coords, insertion, join, pid, point, policy, source};
use martensite_tksketch3d::{
    PointId,
    constraints::{ConstrainedSketch, CurvatureTolerance, RelationSpec},
};
use std::{hint::black_box, time::Instant};

#[test]
#[ignore = "release receipt for matched prepared point moves"]
fn line_cubic_warm_p95() {
    let shape = source(
        [-1., 0., 0.],
        [[1., 0., 0.], [2., 0., 0.], [3., 1., 2.]],
        false,
    );
    let free = ConstrainedSketch::new(shape, policy()).unwrap();
    let tangent = free
        .apply(&insertion(RelationSpec::Tangent(join().into())), || false)
        .unwrap();
    let cap = CurvatureTolerance::new(1e-7).unwrap();
    let g2 = free
        .apply(&insertion(RelationSpec::G2(join().into(), cap)), || false)
        .unwrap();
    matched_moves([&free, &tangent, &g2], pid(3), 0., |sample| {
        [1.1 + f64::from(sample % 31) / 1000., 0., 0.]
    });
}

pub(super) fn matched_moves(
    cases: [&ConstrainedSketch; 3],
    selected: PointId,
    error: f64,
    target: impl Fn(i32) -> [f64; 3],
) {
    let mut samples: [Vec<u128>; 3] = std::array::from_fn(|_| Vec::with_capacity(2048));
    for sample in 0..2112 {
        let target = point(target(sample));
        for (phase, sketch) in cases.into_iter().enumerate() {
            let start = Instant::now();
            let result = black_box(sketch.drag(selected, target, || false).unwrap());
            let elapsed = start.elapsed().as_nanos();
            let accepted = point(coords(result.sketch.geometry(), selected));
            assert!(accepted.distance(&target) <= error);
            assert_eq!(result.rank.equations, phase * 2);
            if sample >= 64 {
                samples[phase].push(elapsed);
            }
        }
    }
    for (name, mut phase) in ["free", "tangent", "g2"].into_iter().zip(samples) {
        phase.sort_unstable();
        let sample_count = phase.len();
        let p50 = phase[(sample_count * 50).div_ceil(100) - 1];
        let p95 = phase[(sample_count * 95).div_ceil(100) - 1];
        println!(
            "{{\"case\":\"{name}\",\"samples\":{sample_count},\"p50_ns\":{p50},\"p95_ns\":{p95}}}",
        );
    }
}
