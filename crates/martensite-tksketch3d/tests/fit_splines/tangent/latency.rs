// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use std::{
    hash::{DefaultHasher, Hasher},
    time::Instant,
};

const WARMUP: usize = 88;
const SAMPLES: usize = 512;

#[test]
#[ignore = "opt-in release-mode latency witness"]
fn fit_tangent_curved_preferred_drag_latency() {
    let (geometry, join) = joined_geometry(PARAMETERS.map(stations), recipe(), 1, [-2., 0., 0.]);
    let mut sketch = ConstrainedSketch::new(geometry, tangent_policy())
        .unwrap()
        .apply(&[tangent_edit(join)], || false)
        .unwrap();
    let center = *sketch
        .geometry()
        .point(pid(2))
        .unwrap()
        .position
        .coordinates();
    let mut times = Vec::with_capacity(SAMPLES);
    let mut receipt = DefaultHasher::new();
    let mut work = 0;
    for step in 0..WARMUP + SAMPLES {
        let target = cursor(center, step);
        let start = Instant::now();
        let moved = sketch
            .drag_preferred(pid(2), target, 1e-8, || false)
            .unwrap();
        let elapsed = start.elapsed().as_nanos();
        if step >= WARMUP {
            times.push(elapsed);
        }
        assert!(moved.preference.scaled_cursor_error < 1e-7);
        assert_eq!(moved.update.rank.degrees_of_freedom, 13);
        work = work.max(moved.update.residual_work_bound);
        let curve = moved.update.sketch.geometry().fit_spline(join.fit).unwrap();
        for value in curve
            .stations()
            .iter()
            .flat_map(|point| point.coordinates())
        {
            receipt.write_u64(value.to_bits());
        }
        for value in fitted(&moved.update.sketch, join) {
            receipt.write_u64(value.to_bits());
        }
        receipt.write_usize(moved.update.residual_work_bound);
        sketch = moved.update.sketch;
    }
    times.sort_unstable();
    assert!(work <= tangent_policy().residual_work);
    println!(
        "accepted={} timed={SAMPLES} p95_ns={} max_ns={} scalar_work_max={work} receipt={:016x}",
        WARMUP + SAMPLES,
        times[(SAMPLES * 95).div_ceil(100) - 1],
        times[SAMPLES - 1],
        receipt.finish()
    );
}

fn cursor(center: [f64; 3], step: usize) -> martensite_tkmath::gp::Point3 {
    let (sine, cosine) = martensite_cmath::sincos(step as f64 * 0.02);
    point([
        center[0] + 0.03 * sine,
        center[1],
        center[2] + 0.03 * (1. - cosine),
    ])
}
