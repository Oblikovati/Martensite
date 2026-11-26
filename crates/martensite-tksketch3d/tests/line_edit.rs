// SPDX-License-Identifier: GPL-2.0-only
//! Owned straight Trim/Extend controls from trim:33 and extend:32.
mod guards;
mod replay;

use crate::fixture::{lid, line, pid, point, seed};
use martensite_tksketch3d::{
    CurveId, CurveRole, Edit, LineId, MinimumLength, PointRole, Snapshot,
    line_edit::{
        ExtendLineRequest, LineEdit, LineEditFailure, LineEditOutcome, LineEndpoint,
        LineEndpointTouch, LineParameterLocation, LineReplacementIds, SupersededLineEndpoint,
        TrimLineRequest, TrimMiss, UnchangedLineEdit,
    },
};

const TRIM: [[[f64; 3]; 2]; 2] = [
    [[-50., 0., 30.], [50., 0., 30.]],
    [[0., -40., 30.], [0., 40., 30.]],
];
const EXTEND: [[[f64; 3]; 2]; 2] = [
    [[-50., -30., 50.], [0., -30., 50.]],
    [[20., -50., 50.], [20., -10., 50.]],
];
const X: [[f64; 3]; 2] = [[0.; 3], [1., 0., 0.]];

fn fixture(lines: &[[[f64; 3]; 2]]) -> Snapshot {
    let edits: Vec<_> = lines
        .iter()
        .enumerate()
        .flat_map(|(index, &[a, b])| {
            let first = 3 * index as u64 + 1;
            [
                seed(first, a, PointRole::Defining),
                seed(first + 1, b, PointRole::Defining),
                line(first + 2, first, first + 1),
            ]
        })
        .collect();
    Snapshot::empty(MinimumLength::new(0.01).unwrap())
        .apply(&edits, || false)
        .unwrap()
}

fn ids() -> LineReplacementIds {
    LineReplacementIds {
        line: lid(52),
        endpoint: pid(51),
    }
}

fn trim(pick: f64) -> TrimLineRequest {
    TrimLineRequest {
        source: lid(3),
        boundary: lid(6),
        pick,
        ids: ids(),
        max_position_error: 0.,
    }
}

fn extension(boundaries: &[LineId], endpoint: LineEndpoint) -> ExtendLineRequest<'_> {
    ExtendLineRequest {
        source: lid(3),
        endpoint,
        boundaries,
        max_boundary_checks: boundaries.len(),
        ids: ids(),
        max_position_error: 0.,
    }
}

fn edited<S>(outcome: LineEditOutcome<S>) -> LineEdit<S> {
    match outcome {
        LineEditOutcome::Edited(edit) => *edit,
        LineEditOutcome::Unchanged(reason) => panic!("expected line edit, got {reason:?}"),
    }
}

#[test]
fn captured_trim_keeps_the_right_branch_and_targeted_retirement() {
    let source = fixture(&TRIM)
        .apply(
            &[
                seed(60, [77., 1., 2.], PointRole::Defining),
                Edit::SetCurveRole {
                    curve: CurveId::Line(lid(3)),
                    role: CurveRole::Construction,
                },
            ],
            || false,
        )
        .unwrap();
    let before = source.clone();
    let applied = edited(source.trim_line(trim(0.25), || false).unwrap());
    let lineage = applied.replacement;
    assert_eq!(lineage.retained_source_parameters, [[0.5; 2], [1.; 2]]);
    assert_eq!(lineage.continuation_parameters, None);
    assert_eq!(lineage.retained_endpoint, pid(2));
    assert_eq!(
        lineage.superseded_endpoint,
        SupersededLineEndpoint::Retired(pid(1))
    );
    assert_eq!(lineage.added_endpoint, pid(51));
    assert_eq!(lineage.source, lid(3));
    assert_eq!(lineage.line.endpoints, [pid(51), pid(2)]);
    assert_trimmed_geometry(&source, applied);
    assert_eq!(source, before);
}

fn assert_trimmed_geometry(source: &Snapshot, applied: LineEdit) {
    let sketch = applied.sketch;
    assert_eq!(applied.contact.parameters, [[0.5; 2]; 2]);
    assert_eq!(applied.contact.error_upper, 0.);
    assert_eq!(
        sketch.line(lid(52)).unwrap().points,
        [point([0., 0., 30.]), point(TRIM[0][1])]
    );
    assert_eq!(sketch.line(lid(6)), source.line(lid(6)));
    assert_eq!(sketch.point(pid(60)), source.point(pid(60)));
    assert!(sketch.point(pid(1)).is_none());
    assert!(sketch.line(lid(3)).is_none());
    assert_eq!(
        sketch.curve_role(CurveId::Line(lid(52))),
        Some(CurveRole::Construction)
    );
}

#[test]
fn authored_pick_on_the_other_side_preserves_directed_start() {
    let source = fixture(&TRIM);
    let applied = edited(source.trim_line(trim(0.75), || false).unwrap());
    assert_eq!(
        applied.replacement.retained_source_parameters,
        [[0.; 2], [0.5; 2]]
    );
    assert_eq!(applied.replacement.line.endpoints, [pid(1), pid(51)]);
    assert_eq!(
        applied.sketch.line(lid(52)).unwrap().points,
        [point(TRIM[0][0]), point([0., 0., 30.])]
    );
    assert!(matches!(
        source.trim_line(trim(0.5), || false),
        Err(LineEditFailure::PickAtCut { source, boundary }) if source == lid(3) && boundary == lid(6)
    ));
}

#[test]
fn captured_extend_records_source_and_continuation_separately() {
    let source = fixture(&EXTEND);
    let before = source.clone();
    let applied = edited(
        source
            .extend_line(extension(&[lid(6)], LineEndpoint::End), || false)
            .unwrap(),
    );
    assert_eq!(
        applied.contact.parameters,
        [[1.4, 1.4_f64.next_up()], [0.5; 2]]
    );
    assert_eq!(applied.contact.error_upper, 0.);
    assert_eq!(
        applied.replacement.retained_source_parameters,
        [[0.; 2], [1.; 2]]
    );
    assert_eq!(
        applied.replacement.continuation_parameters,
        Some([[1.; 2], [1.4, 1.4_f64.next_up()]])
    );
    assert_eq!(applied.replacement.retained_endpoint, pid(1));
    assert_eq!(
        applied.replacement.superseded_endpoint,
        SupersededLineEndpoint::Retired(pid(2))
    );
    assert_eq!(
        applied.sketch.line(lid(52)).unwrap().points,
        [point(EXTEND[0][0]), point([20., -30., 50.])]
    );
    assert_eq!(applied.sketch.line(lid(6)), source.line(lid(6)));
    assert_eq!(source, before);
}

#[test]
fn extend_start_terminates_at_literal_boundary_endpoint_without_sharing_its_id() {
    let start = [-2., -0., -0.];
    let source = fixture(&[X, [start, [-2., 2., 0.]], [[-3., -2., 0.], [-3., 2., 0.]]]);
    let applied = edited(
        source
            .extend_line(extension(&[lid(6), lid(9)], LineEndpoint::Start), || false)
            .unwrap(),
    );
    assert_eq!(applied.boundary_location, LineParameterLocation::Start);
    assert_eq!(
        applied.replacement.continuation_parameters,
        Some([[-2.; 2], [0.; 2]])
    );
    assert_eq!(applied.replacement.line.endpoints, [pid(51), pid(2)]);
    assert_eq!(
        applied
            .sketch
            .point(pid(51))
            .unwrap()
            .position
            .coordinates()
            .map(f64::to_bits),
        start.map(f64::to_bits)
    );
    assert_eq!(applied.sketch.point(pid(4)), source.point(pid(4)));
    assert_eq!(applied.sketch.line(lid(6)), source.line(lid(6)));
}

#[test]
fn distinct_exact_forward_hits_that_round_alike_choose_the_nearer_boundary() {
    let next = 2_f64.next_up();
    let source = fixture(&[
        X,
        [[2., -1., 0.], [next, 3., 0.]],
        [[2., -1., 0.], [next, 7., 0.]],
    ]);
    for boundaries in [[lid(6), lid(9)], [lid(9), lid(6)]] {
        let mut request = extension(&boundaries, LineEndpoint::End);
        request.max_position_error = f64::EPSILON;
        let result = edited(source.extend_line(request, || false).unwrap());
        assert_eq!(result.boundary, lid(9));
        assert_eq!(result.contact.position, [2., 0., 0.]);
        assert_eq!(result.contact.parameters[0], [2., next]);
        assert_eq!(
            result.contact.error_upper,
            f64::from_bits((1023 - 54) << 52)
        );
    }
}

#[test]
fn exact_ties_refuse_but_a_farther_tie_does_not_veto_a_closer_hit() {
    let source = fixture(&[
        X,
        [[2., -1., 0.], [2., 1., 0.]],
        [[2., -2., 0.], [2., 3., 0.]],
        [[1.5, -1., 0.], [1.5, 1., 0.]],
    ]);
    let result = source.extend_line(extension(&[lid(9), lid(6)], LineEndpoint::End), || false);
    assert!(
        matches!(result, Err(LineEditFailure::AmbiguousExtension { source, boundaries })
        if source == lid(3) && boundaries == [lid(6), lid(9)])
    );
    let actual = edited(
        source
            .extend_line(
                extension(&[lid(9), lid(12), lid(6)], LineEndpoint::End),
                || false,
            )
            .unwrap(),
    );
    assert_eq!(actual.boundary, lid(12));
    assert_eq!(actual.contact.position, [1.5, 0., 0.]);
}

#[test]
fn exact_skew_parallel_and_outside_bounds_do_not_trim() {
    let tiny = f64::from_bits(1);
    for (boundary, reason) in [
        ([[0.5, -1., tiny], [0.5, 1., tiny]], TrimMiss::Skew),
        ([[0., 1., 0.], [1., 1., 0.]], TrimMiss::Parallel),
        ([[2., -1., 0.], [2., 1., 0.]], TrimMiss::AuthoredRanges),
        ([[0.5, 1., 0.], [0.5, 2., 0.]], TrimMiss::AuthoredRanges),
    ] {
        let source = fixture(&[X, boundary]);
        assert_eq!(
            source.trim_line(trim(0.25), || false).unwrap(),
            LineEditOutcome::Unchanged(UnchangedLineEdit::Miss(reason))
        );
    }
}

#[test]
fn transverse_and_collinear_trim_touches_are_classified() {
    use LineParameterLocation::{End, Interior, Start};
    for (boundary, at_source, at_boundary) in [
        ([[0.5, 0., 0.], [0.5, 1., 0.]], Interior, Start),
        ([[2., 0., 0.], [1., 0., 0.]], End, End),
        ([[0., -1., 0.], [0., 1., 0.]], Start, Interior),
    ] {
        let source = fixture(&[X, boundary]);
        assert_eq!(
            source.trim_line(trim(0.25), || false).unwrap(),
            LineEditOutcome::Unchanged(UnchangedLineEdit::Touch(LineEndpointTouch {
                boundary: lid(6),
                source_location: at_source,
                boundary_location: at_boundary,
            }))
        );
    }
}

#[test]
fn empty_trim_overlap_and_extend_no_forward_are_explicit() {
    let source = fixture(&[X, [[0.5, 0., 0.], [2., 0., 0.]]]);
    assert!(matches!(
        source.trim_line(trim(0.25), || false),
        Err(LineEditFailure::Overlap { .. })
    ));
    assert!(matches!(
        source.extend_line(extension(&[lid(6)], LineEndpoint::End), || false),
        Err(LineEditFailure::Overlap { .. })
    ));
    let source = fixture(&[X, [[-2., -1., 0.], [-2., 1., 0.]]]);
    for (boundaries, expected) in [
        (&[][..], UnchangedLineEdit::EmptyBoundaries),
        (&[lid(6)][..], UnchangedLineEdit::NoForwardHit),
    ] {
        assert_eq!(
            source
                .extend_line(extension(boundaries, LineEndpoint::End), || false)
                .unwrap(),
            LineEditOutcome::Unchanged(expected)
        );
    }
}

#[test]
fn extend_refuses_a_competing_overlap_but_keeps_a_closer_isolated_hit() {
    for (overlap_start, expected_hit) in [(1.5, false), (2., false), (3., true)] {
        let source = fixture(&[
            X,
            [[2., -1., 0.], [2., 1., 0.]],
            [[overlap_start, 0., 0.], [4., 0., 0.]],
        ]);
        let boundaries = [lid(9), lid(6)];
        let request = extension(&boundaries, LineEndpoint::End);
        let outcome = source.extend_line(request, || false);
        if expected_hit {
            assert_eq!(edited(outcome.unwrap()).boundary, lid(6));
        } else {
            assert!(
                matches!(outcome, Err(LineEditFailure::Overlap { boundary, .. }) if boundary == lid(9))
            );
        }
    }
}

#[test]
fn extend_existing_end_touch_does_not_publish() {
    let source = fixture(&[X, [[1., -1., 0.], [1., 1., 0.]]]);
    let outcome = source
        .extend_line(extension(&[lid(6)], LineEndpoint::End), || false)
        .unwrap();
    assert_eq!(
        outcome,
        LineEditOutcome::Unchanged(UnchangedLineEdit::Touch(LineEndpointTouch {
            boundary: lid(6),
            source_location: LineParameterLocation::End,
            boundary_location: LineParameterLocation::Interior,
        }))
    );
}

#[test]
fn huge_midpoint_obeys_the_exact_position_cap_and_geometry_admission() {
    let large = 9_007_199_254_740_992.;
    let boundary = [[large, -1., 0.], [large + 2., 1., 0.]];
    let source = fixture(&[[[large - 2., 0., 0.], [large + 4., 0., 0.]], boundary]);
    let mut pick = trim(0.25);
    pick.max_position_error = 1_f64.next_down();
    assert!(matches!(
        source.trim_line(pick, || false),
        Err(LineEditFailure::PositionError {
            upper_bound: 1.,
            ..
        })
    ));
    pick.max_position_error = 1.;
    let applied = edited(source.trim_line(pick, || false).unwrap());
    assert_eq!(applied.contact.error_upper, 1.);
    assert_eq!(applied.contact.coordinates[0], [large, large + 2.]);
    assert_eq!(applied.contact.position, [large, 0., 0.]);
    let collapses = fixture(&[[[large, 0., 0.], [large + 2., 0., 0.]], boundary]);
    pick.pick = 0.75;
    assert!(matches!(
        collapses.trim_line(pick, || false),
        Err(LineEditFailure::Geometry(martensite_tksketch3d::EditFailure::CoincidentEndpoints(line)))
            if line.id == lid(52)
    ));
}

#[test]
fn exact_near_tip_edits_require_a_representably_changed_extent() {
    let tip = 1_f64;
    let inward = fixture(&[X, [[tip, -1., 0.], [tip.next_down(), 7., 0.]]]);
    let mut trim_request = trim(tip);
    trim_request.max_position_error = f64::EPSILON;
    assert!(matches!(
        inward.trim_line(trim_request, || false),
        Err(LineEditFailure::UnrepresentableExtent { source, endpoint, parameter })
            if (source, endpoint) == (lid(3), pid(2)) && parameter == [tip.next_down(), tip]
    ));
    let outward = fixture(&[X, [[tip, -1., 0.], [tip.next_up(), 7., 0.]]]);
    let boundaries = [lid(6)];
    let mut extend_request = extension(&boundaries, LineEndpoint::End);
    extend_request.max_position_error = f64::EPSILON;
    assert!(matches!(
        outward.extend_line(extend_request, || false),
        Err(LineEditFailure::UnrepresentableExtent { source, endpoint, parameter })
            if (source, endpoint) == (lid(3), pid(2)) && parameter == [tip, tip.next_up()]
    ));
    for original in [inward, outward] {
        assert_eq!(
            original.line(lid(3)).unwrap().line.endpoints,
            [pid(1), pid(2)]
        );
        assert!(original.point(pid(51)).is_none());
    }
}
