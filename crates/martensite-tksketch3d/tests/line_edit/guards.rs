// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use martensite_tksketch3d::{
    ControlSplineProofPolicy, EditFailure, EntityId, FitSplineId, FitSplineParameters,
    FitSplinePolicy, FrozenPointSource, FrozenSourceRevision, SketchFitSpline, SketchPoint,
    constraints::{ConstrainedSketch, ConstraintFailure, Relation, RelationId},
    line_edit::EndpointProtection,
};
use std::num::NonZeroU64;

#[test]
fn shared_and_frozen_endpoints_refuse_without_rewiring() {
    let shared = fixture(&TRIM)
        .apply(
            &[seed(7, [-70., 0., 0.], PointRole::Defining), line(8, 1, 7)],
            || false,
        )
        .unwrap();
    let result = shared.trim_line(trim(0.25), || false);
    assert!(
        matches!(result, Err(LineEditFailure::ProtectedEndpoint { point, protection: EndpointProtection::Shared(EntityId::Line(other)) })
        if point == pid(1) && other == lid(8))
    );
    assert_eq!(
        shared.line(lid(8)).unwrap().line.endpoints,
        [pid(1), pid(7)]
    );

    let stamp = FrozenPointSource {
        document: [3; 16],
        sketch: NonZeroU64::new(20).unwrap(),
        revision: FrozenSourceRevision::SessionGeneration { generation: 16 },
        point: pid(11),
    };
    let frozen = fixture(&[TRIM[1]])
        .apply(
            &[
                Edit::InsertFrozenPoint {
                    point: SketchPoint {
                        id: pid(4),
                        role: PointRole::Defining,
                        position: point(TRIM[0][0]),
                    },
                    source: stamp,
                },
                seed(5, TRIM[0][1], PointRole::Defining),
                line(6, 4, 5),
            ],
            || false,
        )
        .unwrap();
    let request = TrimLineRequest {
        source: lid(6),
        boundary: lid(3),
        ..trim(0.25)
    };
    assert!(matches!(frozen.trim_line(request, || false),
        Err(LineEditFailure::ProtectedEndpoint { point, protection: EndpointProtection::Frozen(actual) })
        if point == pid(4) && actual == stamp));
    assert_eq!(frozen.frozen_source(pid(4)), Some(stamp));
}

#[test]
fn a_fit_station_survives_line_edits_and_protects_its_endpoint() {
    let spline = SketchFitSpline {
        id: FitSplineId::new(NonZeroU64::new(70).unwrap()),
        stations: [1, 71, 72, 73].map(pid),
        parameters: FitSplineParameters::new([0., 0.25, 0.75, 1.]).unwrap(),
        policy: FitSplinePolicy {
            max_distortion: 2.,
            max_inverse_gain: 32.,
            max_station_identity_error: 1e-12,
            max_station_miss: MinimumLength::new(1e-10).unwrap(),
            regularity: ControlSplineProofPolicy::new(1e-8, 128).unwrap(),
        },
    };
    let source = fixture(&TRIM)
        .apply(
            &[
                seed(71, [-49., 0., 30.], PointRole::Defining),
                seed(72, [-47., 0., 30.], PointRole::Defining),
                seed(73, [-46., 0., 30.], PointRole::Defining),
                Edit::InsertFitSpline(spline),
            ],
            || false,
        )
        .unwrap();
    assert!(matches!(
        source.trim_line(trim(0.25), || false),
        Err(LineEditFailure::ProtectedEndpoint {
            point,
            protection: EndpointProtection::Shared(EntityId::FitSpline(other)),
        }) if point == pid(1) && other == spline.id
    ));
    let accepted = edited(source.trim_line(trim(0.75), || false).unwrap());
    assert_eq!(accepted.sketch.point(pid(1)), source.point(pid(1)));
    assert_eq!(
        accepted.sketch.fit_spline(spline.id),
        source.fit_spline(spline.id)
    );
}

#[test]
fn independent_markers_and_retained_shared_points_keep_their_original_values() {
    let source = fixture(&[TRIM[1]])
        .apply(
            &[
                seed(4, TRIM[0][0], PointRole::SketchMarker),
                seed(5, TRIM[0][1], PointRole::Defining),
                line(6, 4, 5),
                seed(7, [77., 5., 30.], PointRole::Defining),
                line(8, 5, 7),
            ],
            || false,
        )
        .unwrap();
    let request = TrimLineRequest {
        source: lid(6),
        boundary: lid(3),
        ..trim(0.25)
    };
    let applied = edited(source.trim_line(request, || false).unwrap());
    assert_eq!(
        applied.replacement.superseded_endpoint,
        SupersededLineEndpoint::RetainedMarker(pid(4))
    );
    assert_eq!(applied.sketch.point(pid(4)), source.point(pid(4)));
    assert_eq!(applied.sketch.point(pid(5)), source.point(pid(5)));
    assert_eq!(applied.replacement.line.endpoints, [pid(51), pid(5)]);
    assert_eq!(applied.sketch.line(lid(8)), source.line(lid(8)));
}

fn relation(id: u64) -> RelationId {
    RelationId::new(NonZeroU64::new(id).unwrap())
}

#[test]
fn constrained_trim_refuses_retired_operands_and_keeps_the_prior_program() {
    let geometry = fixture(&TRIM);
    let policy = crate::constraints::policy();
    for (binding, operand) in [
        (
            Relation::FixLine(lid(3), TRIM[0].map(point)),
            EntityId::Line(lid(3)),
        ),
        (
            Relation::FixPoint(pid(1), point(TRIM[0][0])),
            EntityId::Point(pid(1)),
        ),
        (
            Relation::Perpendicular([lid(3), lid(6)]),
            EntityId::Line(lid(3)),
        ),
    ] {
        let relations = [(relation(9), binding)];
        let source =
            ConstrainedSketch::restore(geometry.clone(), &relations, policy, || false).unwrap();
        let rank = source.inspect_rank().unwrap();
        let outcome = source.trim_line(trim(0.25), || false);
        assert!(
            matches!(outcome, Err(LineEditFailure::Constraint(ConstraintFailure::MissingOperand(id, actual)))
            if id == relation(9) && actual == operand)
        );
        assert_eq!(source.geometry(), &geometry);
        assert_eq!(source.relations().collect::<Vec<_>>(), relations);
        assert_eq!(source.inspect_rank().unwrap(), rank);
    }
}

#[test]
fn constrained_extend_replays_unchanged_anchors_and_rank() {
    let geometry = fixture(&EXTEND);
    let anchors = [
        (relation(9), Relation::FixPoint(pid(1), point(EXTEND[0][0]))),
        (
            relation(10),
            Relation::FixLine(lid(6), EXTEND[1].map(point)),
        ),
    ];
    let source = ConstrainedSketch::restore(
        geometry.clone(),
        &anchors,
        crate::constraints::policy(),
        || false,
    )
    .unwrap();
    let rank = source.inspect_rank().unwrap();
    let boundaries = [lid(6)];
    let request = extension(&boundaries, LineEndpoint::End);
    let mut calls = 0;
    let accepted = edited(
        source
            .extend_line(request, || {
                calls += 1;
                false
            })
            .unwrap(),
    );
    assert_eq!(accepted.sketch.relations().collect::<Vec<_>>(), anchors);
    assert_eq!(accepted.sketch.inspect_rank().unwrap(), rank);
    assert_eq!(
        accepted.sketch.geometry().point(pid(1)).unwrap().position,
        point(EXTEND[0][0])
    );
    let mut observed = 0;
    assert!(matches!(
        source.extend_line(request, || {
            observed += 1;
            observed == calls
        }),
        Err(LineEditFailure::Cancelled)
    ));
    assert_eq!(source.geometry(), &geometry);
    assert_eq!(source.relations().collect::<Vec<_>>(), anchors);
}

#[test]
fn constrained_line_edit_keeps_rank_with_an_additional_free_endpoint() {
    let geometry = fixture(&[TRIM[1]])
        .apply(
            &[
                seed(4, TRIM[0][0], PointRole::SketchMarker),
                seed(5, TRIM[0][1], PointRole::Defining),
                line(6, 4, 5),
            ],
            || false,
        )
        .unwrap();
    let retained = [(
        relation(9),
        Relation::Length(lid(3), crate::constraints::length(80.)),
    )];
    let source = ConstrainedSketch::restore(
        geometry.clone(),
        &retained,
        crate::constraints::policy(),
        || false,
    )
    .unwrap();
    let request = TrimLineRequest {
        source: lid(6),
        boundary: lid(3),
        ..trim(0.25)
    };
    let edit = edited(source.trim_line(request, || false).unwrap());
    let before = source.inspect_rank().unwrap();
    let after = edit.sketch.inspect_rank().unwrap();
    assert_eq!((before.rank, before.equations), (1, 1));
    assert_eq!(
        (after.rank, after.equations),
        (before.rank, before.equations)
    );
    assert_eq!(after.degrees_of_freedom, before.degrees_of_freedom + 3);
    assert_eq!(edit.sketch.relations().collect::<Vec<_>>(), retained);
    assert_eq!(edit.sketch.geometry().point(pid(4)), geometry.point(pid(4)));
    assert!(edit.sketch.geometry().point(pid(51)).is_some());
    assert_eq!(source.geometry(), &geometry);
}

#[test]
fn work_cap_duplicates_and_fresh_identity_refusals_leave_geometry_unchanged() {
    let source = fixture(&EXTEND);
    let before = source.clone();
    let boundaries = [lid(6)];
    let mut request = extension(&boundaries, LineEndpoint::End);
    request.max_boundary_checks = 0;
    assert!(matches!(
        source.extend_line(request, || false),
        Err(LineEditFailure::BoundaryLimit {
            requested: 1,
            maximum: 0
        })
    ));
    let repeated = [lid(6), lid(6)];
    assert!(
        matches!(source.extend_line(extension(&repeated, LineEndpoint::End), || false),
        Err(LineEditFailure::RepeatedBoundary(id)) if id == lid(6))
    );
    request.max_boundary_checks = 1;
    request.ids.line = lid(3);
    assert!(matches!(
        source.extend_line(request, || false),
        Err(LineEditFailure::Geometry(
            EditFailure::DuplicateIdentity { .. }
        ))
    ));
    assert!(matches!(
        fixture(&TRIM).trim_line(trim(1.5), || false),
        Err(LineEditFailure::InvalidPick(1.5))
    ));
    assert!(matches!(
        source.extend_line(extension(&[lid(3)], LineEndpoint::End), || false),
        Err(LineEditFailure::SourceIsBoundary(id)) if id == lid(3)
    ));
    assert_eq!(source, before);
}

#[test]
fn cancellation_at_every_poll_keeps_the_original_snapshot() {
    let source = fixture(&TRIM);
    let before = source.clone();
    let mut observed = 0;
    let complete = source
        .trim_line(trim(0.25), || {
            observed += 1;
            false
        })
        .unwrap();
    for stop in 1..=observed {
        let mut polls = 0;
        let result = source.trim_line(trim(0.25), || {
            polls += 1;
            polls == stop
        });
        assert!(
            matches!(result, Err(LineEditFailure::Cancelled)),
            "unexpected result at poll {stop}: {result:?}"
        );
        assert_eq!(source, before);
    }
    assert_eq!(complete, source.trim_line(trim(0.25), || false).unwrap());
}
