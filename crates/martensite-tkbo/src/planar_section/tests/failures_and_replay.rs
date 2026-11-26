// SPDX-License-Identifier: GPL-2.0-only
use super::*;

#[test]
fn raw_source_tolerance_and_requested_spatial_allowance_remain_separate() {
    let face = setup();
    let edges = bound_edges(&face);
    let [vertex, _] = topology::directed_vertices(&edges[0].1).unwrap();
    let storage = vertex.tshape().downcast::<BRepTVertex>();
    storage.get().unwrap().set_tolerance(-2.);
    let failure = run(&face, &edges, x_zero(), &ProgressRange::default()).unwrap_err();
    assert!(matches!(
        failure.cause,
        PlanarSectionCause::SourceTolerance { actual: -2., .. }
    ));
}

struct CancelAfter {
    polls: AtomicUsize,
    limit: usize,
}

impl ProgressIndicatorHooks for CancelAfter {
    fn show(&self, _: &ProgressScope<'_>, _: f64, _: bool) {}
    fn user_break(&self) -> bool {
        self.polls.fetch_add(1, Ordering::Relaxed) >= self.limit
    }
}

#[test]
fn cancellation_and_typed_source_refusal_publish_no_partial_geometry() {
    let face = setup();
    let edges = bound_edges(&face);
    for limit in [0, 10, 50] {
        let indicator = ProgressIndicator::new(Box::new(CancelAfter {
            polls: AtomicUsize::new(0),
            limit,
        }));
        let cancelled = run(&face, &edges, x_zero(), &indicator.start()).unwrap_err();
        assert!(matches!(cancelled.cause, PlanarSectionCause::Cancelled));
    }
    assert_eq!(
        segment(&cut(&face, x_zero()).unwrap()).evidence.endpoints,
        EXPECTED
    );

    let counterfeit = Face::unchecked(edges[0].1.oriented(Orientation::Forward));
    let invalid = run(&counterfeit, &edges, x_zero(), &ProgressRange::default()).unwrap_err();
    assert!(matches!(
        invalid.cause,
        PlanarSectionCause::ShapeKind {
            actual: ShapeEnum::Edge,
            expected: ShapeEnum::Face
        }
    ));
}

#[test]
#[ignore = "explicit local Section latency and deterministic receipt"]
fn measured_block_section_replay() {
    use std::{hint::black_box, time::Instant};
    let face = setup();
    let edges = bound_edges(&face);
    let tool = x_zero();
    let progress = ProgressRange::default();
    let expected = run(&face, &edges, tool, &progress).unwrap();
    for _ in 0..12 {
        black_box(run(&face, &edges, tool, &progress).unwrap());
    }
    let mut samples_us = [0_u128; 100];
    for elapsed in &mut samples_us {
        let start = Instant::now();
        let result = black_box(run(&face, &edges, tool, &progress).unwrap());
        *elapsed = start.elapsed().as_micros();
        assert_eq!(result, expected);
    }
    samples_us.sort_unstable();
    println!(
        "{{\"section_samples\":100,\"p50_us\":{},\"p95_us\":{},\"max_us\":{}}}",
        samples_us[49], samples_us[94], samples_us[99]
    );
    println!("section_receipt={expected:?}");
}
