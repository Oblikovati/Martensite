// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::metric_bounds::rational_integrals::subinterval::ContributionBounds;
fn run(
    accounted: bool,
    limit: u32,
    stop: usize,
) -> (
    Result<ContributionBounds, RationalIntegralFailure>,
    u64,
    usize,
) {
    let boundaries = boundaries();
    let options = RationalIntegralOptions {
        max_subintervals: limit,
        ..Default::default()
    };
    let share = ExactRatio::from_finite(if limit == 1 { 0. } else { 1e-9 });
    let mut polls = 0;
    let mut cancel = || {
        polls += 1;
        polls == stop
    };
    if accounted {
        let (result, cells) =
            integrate_boundaries_accounted(boundaries, options, &share, &mut cancel);
        return (result.map(|v| v.bounds), cells, polls);
    }
    let result = integrate_boundaries(boundaries, options, &share, &mut cancel);
    let cells = result.as_ref().map_or(0, |v| v.subintervals);
    (result.map(|v| v.bounds), cells, polls)
}
fn signature(
    result: Result<ContributionBounds, RationalIntegralFailure>,
) -> Result<[ExactRatio; 2], RationalIntegralFailure> {
    result.map(|v| [v.lower, v.upper])
}
#[test]
fn accounted_side_preserves_original_value_polls_and_exact_rectangle_oracle() {
    let old = run(false, 4096, usize::MAX);
    let new = run(true, 4096, usize::MAX);
    assert_eq!(signature(old.0), signature(new.0.clone()));
    assert_eq!((old.1, old.2), (new.1, new.2));
    let exact = ExactRatio::new(1.into(), 3.into());
    let bounds = new.0.unwrap();
    assert!(bounds.lower <= exact && bounds.upper >= exact);
    for stop in [1, new.2 / 2, new.2] {
        let old = run(false, 4096, stop);
        let new = run(true, 4096, stop);
        assert_eq!(signature(old.0), signature(new.0));
        assert_eq!(old.2, new.2);
    }
}
#[test]
fn accounted_failed_side_keeps_actual_examined_cell() {
    let old = run(false, 1, usize::MAX);
    let new = run(true, 1, usize::MAX);
    assert_eq!(signature(old.0), signature(new.0));
    assert_eq!(old.2, new.2);
    assert_eq!(new.1, 1);
}

fn boundaries() -> [MetricBezier<'static, 3>; 2] {
    const A: [[f64; 3]; 2] = [[0., 0., 1.], [1., 0., 1.]];
    const B: [[f64; 3]; 2] = [[0., 1., 1.], [1., 1., 1.]];
    const W: [f64; 2] = [1., 2.];
    [
        MetricBezier {
            poles: &A,
            weights: Some(&W),
        },
        MetricBezier {
            poles: &B,
            weights: Some(&W),
        },
    ]
}
