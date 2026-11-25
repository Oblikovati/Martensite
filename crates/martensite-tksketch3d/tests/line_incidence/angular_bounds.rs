// SPDX-License-Identifier: GPL-2.0-only
use super::*;

#[test]
fn colinear_admits_exact_parallel_lines_under_minimum_normal_or_subnormal_guards() {
    for limit in [f64::from_bits(1), f64::MIN_POSITIVE] {
        for (sign, direction) in [
            (1., ParallelDirection::Same),
            (-1., ParallelDirection::Opposite),
        ] {
            let exact = [[0.; 3], [1., 0., 0.], [0.; 3], [sign, 0., 0.]];
            let sketch = restore(exact, direction, limit).unwrap();
            assert_rank(&sketch, (4, 8, 4));
            let mut near_miss = exact;
            near_miss[3][1] = 2. * limit;
            assert_infeasible(restore(near_miss, direction, limit));
        }
    }
}

#[test]
fn colinear_accepts_the_exact_closed_angular_threshold_on_both_branches() {
    // |(1,1,0) × (1,2,1)|² = 3 and the product of squared lengths is 12.
    for (sign, direction) in [
        (1., ParallelDirection::Same),
        (-1., ParallelDirection::Opposite),
    ] {
        let endpoints = [[0.; 3], [1., 1., 0.], [0.; 3], [sign, 2. * sign, sign]];
        restore(endpoints, direction, 0.5).unwrap();
        restore(endpoints, direction, 0.5_f64.next_up()).unwrap();
        assert_infeasible(restore(endpoints, direction, 0.5_f64.next_down()));
    }
}

#[test]
fn colinear_cancelling_dot_products_still_report_the_correct_source_branch() {
    let n = 4_503_599_627_370_496.;
    let endpoints = [[0.; 3], [n, n + 1., 0.], [0.; 3], [n, -(n - 1.), 0.]];
    assert_infeasible(restore(endpoints, ParallelDirection::Same, 0.5));
    assert!(matches!(
        restore(endpoints, ParallelDirection::Opposite, 0.5),
        Err(ConstraintFailure::ColinearSeedBranch(id, _, ParallelDirection::Opposite)) if id == rid(1)
    ));
}

fn assert_infeasible(result: Result<ConstrainedSketch, ConstraintFailure>) {
    assert!(
        matches!(result, Err(ConstraintFailure::Infeasible(id, Relation::Colinear(..))) if id == rid(1)),
        "expected independent metric refusal, received {:?}",
        result.err()
    );
}

fn restore(
    endpoints: [[f64; 3]; 4],
    direction: ParallelDirection,
    sine: f64,
) -> Result<ConstrainedSketch, ConstraintFailure> {
    let mut policy = policy();
    policy.parallel_sine_tolerance = sine;
    ConstrainedSketch::restore(
        geometry(&endpoints, &[(1, 2), (3, 4)]),
        &[(rid(1), Relation::Colinear([lid(10), lid(11)], direction))],
        policy,
        || false,
    )
}
