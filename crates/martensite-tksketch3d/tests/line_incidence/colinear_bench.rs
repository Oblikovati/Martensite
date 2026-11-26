// SPDX-License-Identifier: GPL-2.0-only
use super::*;

#[test]
#[ignore = "release pointer latency, run single-threaded on a pinned core"]
fn measured_native_colinear_chart_crossings() {
    for groups in [1, 10, 20] {
        let mut steady = Vec::new();
        let mut crossing = Vec::new();
        let initial = corpus(groups);
        for _ in 0..100 {
            let mut sketch = initial.clone();
            for (step, target) in path().into_iter().enumerate() {
                let start = Instant::now();
                let moved = sketch.drag(pid(2), point(target), || false).unwrap();
                let sample = (start.elapsed().as_nanos(), moved.residual_work_bound);
                if step == 1 {
                    crossing.push(sample);
                } else {
                    steady.push(sample);
                }
                assert_incident(moved.sketch.geometry(), lid(CORPUS_FIRST_LINE), pid(3));
                assert_incident(moved.sketch.geometry(), lid(CORPUS_FIRST_LINE), pid(4));
                sketch = moved.sketch;
            }
        }
        print_times("colinear_steady", 12 * groups, steady);
        print_times("colinear_crossing", 12 * groups, crossing);
    }
}

fn corpus(groups: usize) -> ConstrainedSketch {
    let mut points = Vec::new();
    let mut lines = Vec::new();
    let mut specs = Vec::new();
    for i in 0..groups as u64 {
        let x = 8. * i as f64;
        let first = 4 * i;
        let source = CORPUS_FIRST_LINE + 2 * i;
        points.extend([x, x + 2., x + 3., x + 5.].map(|x| [x, 0., 0.]));
        lines.extend([(first + 1, first + 2), (first + 3, first + 4)]);
        specs.push(RelationSpec::FixPoint(pid(first + 1)));
        specs.push(RelationSpec::Colinear(
            [lid(source), lid(source + 1)],
            ParallelDirection::Same,
        ));
    }
    author(geometry_from(&points, &lines, CORPUS_FIRST_LINE), &specs)
}
