// SPDX-License-Identifier: GPL-2.0-only
use crate::fixture::{lid, line, pid, seed};
use martensite_tksketch3d::{
    CurveId, CurveRole, Edit, EditFailure, EntityId, MinimumLength, PointRole, Snapshot,
    split_plane::{
        SegmentWorkPlane, SplitLineEdit, SplitLineFailure, SplitLineIds, SplitLineOutcome,
        UnchangedLine,
    },
};

fn source(points: [[f64; 3]; 2], minimum: f64) -> Snapshot {
    Snapshot::empty(MinimumLength::new(minimum).unwrap())
        .apply(
            &[
                seed(1, points[0], PointRole::Defining),
                seed(2, points[1], PointRole::Defining),
                line(3, 1, 2),
            ],
            || false,
        )
        .unwrap()
}

fn ids() -> SplitLineIds {
    SplitLineIds {
        cut: pid(4),
        fragments: [lid(6), lid(5)],
    }
}

fn plane(origin: [f64; 3], normal: [f64; 3]) -> SegmentWorkPlane {
    SegmentWorkPlane::new(origin, normal).unwrap()
}

fn yz() -> SegmentWorkPlane {
    plane([0.; 3], [1., 0., 0.])
}

fn completed(outcome: SplitLineOutcome) -> Box<SplitLineEdit> {
    let SplitLineOutcome::Split(result) = outcome else {
        panic!("expected split")
    };
    result
}

#[test]
fn owned_crossing_creates_two_ordered_halves_and_keeps_source_roles() {
    for role in [CurveRole::Model, CurveRole::Construction] {
        let source = source([[-2., 0., 0.], [2., 0., 0.]], 1e-9)
            .apply(
                &[Edit::SetCurveRole {
                    curve: CurveId::Line(lid(3)),
                    role,
                }],
                || false,
            )
            .unwrap();
        let result = completed(
            source
                .split_line_at_plane(lid(3), yz(), ids(), 0., || false)
                .unwrap(),
        );
        assert_eq!(
            result.fragments.map(|fragment| fragment.line.id),
            ids().fragments
        );
        assert_eq!(
            result.fragments.map(|fragment| fragment.source),
            [lid(3); 2]
        );
        assert_eq!(
            result.fragments.map(|fragment| fragment.line.endpoints),
            [[pid(1), pid(4)], [pid(4), pid(2)]]
        );
        assert_eq!(
            result.fragments.map(|fragment| fragment.source_parameters),
            [[[0.; 2], [0.5; 2]], [[0.5; 2], [1.; 2]],]
        );
        assert_eq!(result.cut.position, [0.; 3]);
        assert_eq!(result.cut.error_upper, 0.);
        assert_eq!(result.requested_error, 0.);
        assert_eq!(result.point, ids().cut);
        assert!(source.line(lid(3)).is_some());
        assert!(source.point(ids().cut).is_none());
        assert!(result.snapshot.line(lid(3)).is_none());
        for fragment in result.fragments {
            assert_eq!(
                result
                    .snapshot
                    .line(fragment.line.id)
                    .unwrap()
                    .length_estimate,
                2.
            );
            assert_eq!(
                result.snapshot.curve_role(CurveId::Line(fragment.line.id)),
                Some(role)
            );
        }
    }
}

#[test]
fn disjoint_touch_and_overlap_preserve_the_source_even_with_unusable_ids() {
    let input = source([[-2., 0., 0.], [2., 0., 0.]], 1e-9);
    let bad = SplitLineIds {
        cut: pid(1),
        fragments: [lid(3); 2],
    };
    for (x, expected) in [
        (3., UnchangedLine::Disjoint),
        (-2., UnchangedLine::Endpoint(pid(1))),
        (2., UnchangedLine::Endpoint(pid(2))),
    ] {
        assert_eq!(
            input.split_line_at_plane(lid(3), plane([x, 0., 0.], [1., 0., 0.]), bad, 0., || false),
            Ok(SplitLineOutcome::Unchanged(expected))
        );
    }
    assert_eq!(
        input.split_line_at_plane(lid(3), plane([0.; 3], [0., 0., 1.]), bad, 0., || false),
        Err(SplitLineFailure::Overlap(lid(3)))
    );
    assert_eq!(
        input.owned_ids().collect::<Vec<_>>(),
        [
            EntityId::Point(pid(1)),
            EntityId::Point(pid(2)),
            EntityId::Line(lid(3))
        ]
    );
}

#[test]
fn rounding_budget_refuses_without_spending_fragment_ids() {
    let input = source([[0.; 3], [1., 2., 0.]], 1e-9);
    let cut = plane([1., 0., 0.], [1.; 3]);
    assert!(matches!(
        input.split_line_at_plane(lid(3), cut, ids(), 0., || false),
        Err(SplitLineFailure::PositionError { upper_bound, limit: 0. }) if upper_bound > 0.
    ));
    let split = |limit| {
        completed(
            input
                .split_line_at_plane(lid(3), cut, ids(), limit, || false)
                .unwrap(),
        )
    };
    let accepted = split(1e-16);
    assert_eq!(
        accepted.fragments[0].source_parameters[1],
        accepted.cut.parameter
    );
    assert_eq!(
        accepted.fragments[1].source_parameters[0],
        accepted.cut.parameter
    );
    let equal = split(accepted.cut.error_upper);
    assert_eq!(accepted.snapshot, equal.snapshot);
    assert_eq!(accepted.cut, equal.cut);
}

#[test]
fn near_endpoint_exact_hit_survives_and_collapsed_nominal_fragment_refuses() {
    let tiny = f64::from_bits(1);
    let input = source([[0.; 3], [2., 0., 0.]], tiny);
    let cut = plane([tiny, 0., 0.], [1., 0., 0.]);
    let result = completed(
        input
            .split_line_at_plane(lid(3), cut, ids(), 0., || false)
            .unwrap(),
    );
    assert_eq!(result.cut.parameter, [0., tiny]);
    assert_eq!(result.cut.position, [tiny, 0., 0.]);

    let large = 2_f64.powi(53);
    let input = source([[large, 0., 0.], [large + 2., tiny, 0.]], 1e-9);
    let cut = plane([large, 0.5, 0.], [1., 1., 0.]);
    assert!(matches!(
        input.split_line_at_plane(lid(3), cut, ids(), 1., || false),
        Err(SplitLineFailure::Edit(EditFailure::CoincidentEndpoints(_)))
    ));
    assert!(input.line(lid(3)).is_some());
}

#[test]
fn fragment_below_the_sketch_minimum_refuses_the_whole_batch() {
    let input = source([[-2., 0., 0.], [2., 0., 0.]], 1.);
    let near = plane([-1.5, 0., 0.], [1., 0., 0.]);
    assert!(matches!(
        input.split_line_at_plane(lid(3), near, ids(), 0., || false),
        Err(SplitLineFailure::Edit(EditFailure::BelowMinimumLength { line, .. }))
            if line == ids().fragments[0]
    ));
    assert!(input.line(lid(3)).is_some());
    assert!(input.line(ids().fragments[1]).is_none());
}

#[test]
fn reservations_and_cancellation_are_atomic_through_the_last_poll() {
    let input = source([[-2., 0., 0.], [2., 0., 0.]], 0.1);
    let bad = SplitLineIds {
        fragments: [lid(3), lid(6)],
        ..ids()
    };
    assert!(matches!(
        input.split_line_at_plane(lid(3), yz(), bad, 0., || false),
        Err(SplitLineFailure::Edit(
            EditFailure::DuplicateIdentity { .. }
        ))
    ));
    let mut polls = 0;
    let result = input
        .split_line_at_plane(lid(3), yz(), ids(), 0., || {
            polls += 1;
            false
        })
        .unwrap();
    assert!(matches!(result, SplitLineOutcome::Split(_)));
    for stop in 1..=polls {
        let mut seen = 0;
        let cancelled = input.split_line_at_plane(lid(3), yz(), ids(), 0., || {
            seen += 1;
            seen == stop
        });
        assert_eq!(
            cancelled,
            Err(SplitLineFailure::Cancelled),
            "cancel poll {stop}"
        );
    }
    assert_eq!(input.owned_ids().count(), 3);
    assert_eq!(
        result,
        input
            .split_line_at_plane(lid(3), yz(), ids(), 0., || false)
            .unwrap()
    );
}

#[test]
fn bad_error_limit_and_missing_source_are_typed() {
    let input = source([[-2., 0., 0.], [2., 0., 0.]], 1e-9);
    assert_eq!(
        input.split_line_at_plane(lid(3), yz(), ids(), -1., || false),
        Err(SplitLineFailure::InvalidLimit(-1.))
    );
    assert_eq!(
        input.split_line_at_plane(lid(9), yz(), ids(), 0., || false),
        Err(SplitLineFailure::MissingLine(lid(9)))
    );
}

#[test]
fn a_large_finite_line_can_split_even_when_both_plane_products_overflow() {
    let far = f64::from_bits(1523_u64 << 52);
    let source = source([[-far, 0., 0.], [far, 0., 0.]], 1.);
    let face = plane([0.; 3], [f64::MAX, 0., 0.]);
    let result = completed(
        source
            .split_line_at_plane(lid(3), face, ids(), 0., || false)
            .unwrap(),
    );
    assert_eq!(result.cut.parameter, [0.5; 2]);
    assert_eq!(result.cut.position, [0.; 3]);
    assert_eq!(
        result.fragments.map(|fragment| result
            .snapshot
            .line(fragment.line.id)
            .unwrap()
            .length_estimate),
        [far; 2]
    );
}

#[test]
fn independent_processes_replay_the_same_plane_cut_and_directed_fragments() {
    let first = replay_receipt();
    let second = replay_receipt();
    assert_eq!(first, second);
}

fn replay_receipt() -> String {
    let result = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "split_plane::emit_split_replay",
            "--nocapture",
            "--test-threads=1",
        ])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let stdout = String::from_utf8(result.stdout).unwrap();
    stdout
        .split_once("SPLIT_PLANE=")
        .unwrap()
        .1
        .lines()
        .next()
        .unwrap()
        .to_owned()
}

#[test]
fn emit_split_replay() {
    let source = source([[0.; 3], [1., 2., 0.]], 1e-9);
    let cut = plane([1., 0., 0.], [1.; 3]);
    let result = completed(
        source
            .split_line_at_plane(lid(3), cut, ids(), 1e-16, || false)
            .unwrap(),
    );
    let position = result.cut.position.map(f64::to_bits);
    let parameters = result.fragments.map(|fragment| {
        fragment
            .source_parameters
            .map(|bound| bound.map(f64::to_bits))
    });
    let fragments = result
        .fragments
        .map(|fragment| (fragment.source, fragment.line.id, fragment.line.endpoints));
    let error = result.cut.error_upper.to_bits();
    println!("SPLIT_PLANE={position:?} {parameters:?} {fragments:?} {error}");
}
