// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use std::time::Instant;

#[derive(Clone, Copy, Debug)]
enum Profile {
    Rail,
    Circle,
}

#[derive(Clone, Copy, Debug)]
enum Pointer {
    ExactOnModel,
    PreferredOnModel,
    PreferredOffModel,
}

#[test]
#[ignore = "release pointer latency; run single-threaded on a pinned core"]
fn measured_preferred_rail_and_circle_cursor_routes() {
    for coordinates in [15, 120, 240] {
        for profile in [Profile::Rail, Profile::Circle] {
            let (sketch, active) = corpus(profile, coordinates);
            for pointer in [
                Pointer::ExactOnModel,
                Pointer::PreferredOnModel,
                Pointer::PreferredOffModel,
            ] {
                measure(profile, pointer, coordinates, &sketch, active);
            }
        }
    }
}

fn measure(
    profile: Profile,
    mode: Pointer,
    coordinates: usize,
    initial: &ConstrainedSketch,
    active: u64,
) {
    let mut state = initial.clone();
    let mut samples = Vec::new();
    for step in 1..=120 {
        let (cursor, expected) = target(profile, mode, step);
        let start = Instant::now();
        let update = match mode {
            Pointer::ExactOnModel => state.drag(pid(active), point(cursor), || false).unwrap(),
            _ => {
                state
                    .drag_preferred(pid(active), point(cursor), TANGENT, || false)
                    .unwrap()
                    .update
            }
        };
        samples.push((start.elapsed().as_nanos(), update.residual_work_bound));
        let actual = where_point(&update.sketch, active);
        for (actual, expected) in actual.into_iter().zip(expected) {
            near(actual, expected, 3e-7);
        }
        state = update.sketch;
    }
    samples.sort_unstable();
    let p95 = samples[(95 * samples.len()).div_ceil(100) - 1].0;
    let max = samples.last().unwrap().0;
    let work = samples.iter().map(|sample| sample.1).max().unwrap();
    println!(
        "{{\"profile\":\"{profile:?}\",\"mode\":\"{mode:?}\",\"coordinates\":{coordinates},\"samples\":{},\"p95_ns\":{p95},\"max_ns\":{max},\"work\":{work}}}",
        samples.len()
    );
}

fn target(profile: Profile, mode: Pointer, step: i32) -> ([f64; 3], [f64; 3]) {
    let off = matches!(mode, Pointer::PreferredOffModel);
    match profile {
        Profile::Rail => {
            let x = f64::from(step) * 0.1;
            let expected = [x, 0., 0.];
            let target = if off {
                [x, 1. + x * 0.2, 0.25]
            } else {
                expected
            };
            (target, expected)
        }
        Profile::Circle => {
            let theta = f64::from(step) * std::f64::consts::TAU / 60.;
            let (s, c) = martensite_cmath::sincos(theta);
            let expected = [5. * c, 5. * s, 0.];
            let radius = if step % 2 == 0 { 3. } else { 7. };
            let target = if off {
                [radius * c, radius * s, 0.75]
            } else {
                expected
            };
            (target, expected)
        }
    }
}

fn corpus(profile: Profile, coordinates: usize) -> (ConstrainedSketch, u64) {
    let points = coordinates as u64 / 3;
    let per = match profile {
        Profile::Rail => 3,
        Profile::Circle => 2,
    };
    let mut edits = Vec::new();
    let mut specs = Vec::new();
    let groups = points / per;
    for i in 0..groups {
        let first = i * per + 1;
        let line_id = 1000 + i;
        let x = 20. * i as f64;
        append_group(profile, &mut edits, &mut specs, first, line_id, x);
    }
    for i in groups * per + 1..=points {
        edits.push(seed(i, [0., 0., 0.], PointRole::SketchMarker));
    }
    let data = Snapshot::empty(length(0.001))
        .apply(&edits, || false)
        .unwrap();
    let adds = specs
        .into_iter()
        .enumerate()
        .map(|(i, spec)| RelationEdit::Insert(rid(i as u64 + 1), spec))
        .collect::<Vec<_>>();
    let sketch = ConstrainedSketch::new(data, policy())
        .unwrap()
        .apply(&adds, || false)
        .unwrap();
    (sketch, per)
}

fn append_group(
    profile: Profile,
    edits: &mut Vec<Edit>,
    specs: &mut Vec<RelationSpec>,
    first: u64,
    line_id: u64,
    x: f64,
) {
    let span = if matches!(profile, Profile::Rail) {
        4.
    } else {
        5.
    };
    edits.push(seed(first, [x, 0., 0.], PointRole::Defining));
    edits.push(seed(first + 1, [x + span, 0., 0.], PointRole::Defining));
    edits.push(line(line_id, first, first + 1));
    match profile {
        Profile::Rail => {
            edits.push(seed(first + 2, [x + 0.5, 0., 0.], PointRole::SketchMarker));
            specs.push(RelationSpec::FixLine(lid(line_id)));
            specs.push(RelationSpec::PointOnLine(pid(first + 2), lid(line_id)));
        }
        Profile::Circle => {
            specs.push(RelationSpec::FixPoint(pid(first)));
            specs.push(RelationSpec::Length(lid(line_id), length(5.)));
            specs.push(RelationSpec::ParallelToPlane(lid(line_id), Direction3::Z));
        }
    }
}
