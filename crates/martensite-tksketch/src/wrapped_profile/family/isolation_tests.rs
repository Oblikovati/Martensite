//! Refusal never lends input ownership to a partial native result.
use super::tests::requests;
use super::*;
#[derive(Debug, PartialEq, Eq)]
struct InputSnapshot {
    region: SurfaceRegionId,
    context: [u64; 4],
    max_fit_level: u8,
    contours: Vec<Vec<PieceSnapshot>>,
}
#[derive(Debug, PartialEq, Eq)]
struct PieceSnapshot {
    span: CurveSpan,
    corners: [CornerSource; 2],
    chart: u64,
    poles: Vec<[u64; 2]>,
}
fn snapshot(requests: &[Request]) -> Vec<InputSnapshot> {
    requests
        .iter()
        .map(|r| InputSnapshot {
            region: r.region,
            context: [r.radius, r.depths[0], r.depths[1], r.tolerance].map(f64::to_bits),
            max_fit_level: r.max_fit_level,
            contours: r
                .loops
                .iter()
                .map(|c| c.iter().map(piece_snapshot).collect())
                .collect(),
        })
        .collect()
}
fn piece_snapshot(piece: &AuthoredPiece) -> PieceSnapshot {
    PieceSnapshot {
        span: piece.span,
        corners: [piece.start, piece.end],
        chart: piece.chart_center.to_bits(),
        poles: piece.poles.iter().map(|p| p.map(f64::to_bits)).collect(),
    }
}
fn targets() -> Vec<Checkpoint> {
    use FamilyStage::*;
    [
        (Selection, None),
        (Construction, Some(SurfaceRegionId(100))),
        (Encoding, None),
        (Coverage, None),
        (History, None),
        (Storage, Some(SurfaceRegionId(100))),
        (Binding, None),
        (MemberChecks, Some(SurfaceRegionId(100))),
        (Completion, None),
    ]
    .into_iter()
    .map(|(stage, region)| Checkpoint { stage, region })
    .collect()
}
fn assert_cancelled<T>(result: Checked<T>, expected: Checkpoint) {
    match result {
        Err(FamilyFailure::Cancelled(actual)) => assert_eq!(actual, expected),
        Err(FamilyFailure::Selection(cause)) => {
            assert_eq!(
                expected,
                Checkpoint {
                    stage: FamilyStage::Selection,
                    region: None
                }
            );
            assert!(
                matches!(cause.as_ref(), selection::SelectionFailure::Capture(cause)
                if matches!(cause.as_ref(), planar::PlanarFailure::WorkCapture(work) if work.is_cancelled()))
            );
        }
        Err(FamilyFailure::Member(cause)) => assert_member_cancelled(&cause, expected),
        Err(other) => panic!("expected cancellation at {expected:?}, got {other}"),
        Ok(_) => panic!("partial family escaped cancellation at {expected:?}"),
    }
}
fn assert_member_cancelled(cause: &Refusal, expected: Checkpoint) {
    use FamilyStage::{MemberChecks, Storage};
    use RefusalCause::{Cones, NormalMeasure, Storage as Stored};
    assert_eq!(Some(cause.region), expected.region);
    let expected_stage = match (expected.stage, &cause.cause) {
        (Storage, Stored(failure)) if failure.is_cancelled() => Stage::Storage,
        (MemberChecks, Cones(failure)) if failure.cancelled() => Stage::Cones,
        (MemberChecks, Stored(failure)) if failure.is_cancelled() => Stage::Measure,
        (MemberChecks, NormalMeasure(failure)) if failure.cancelled() => Stage::Measure,
        _ => panic!("expected typed cancellation at {expected:?}, got {cause:?}"),
    };
    assert_eq!(cause.stage, expected_stage);
}

#[test]
fn each_family_boundary_can_cancel_without_changing_any_input_bits() {
    let inputs = requests();
    let before = snapshot(&inputs);
    for target in targets() {
        let mut visited = Vec::new();
        let result = construct_with_cancel(&inputs, |point| {
            visited.push(point);
            point == target
        });
        assert_cancelled(result, target);
        assert_eq!(visited.last(), Some(&target));
        assert_eq!(snapshot(&inputs), before);
    }
}
#[test]
fn cancellation_after_encoding_discards_the_completed_shape_table() {
    let inputs = requests();
    let before = snapshot(&inputs);
    let target = Checkpoint {
        stage: FamilyStage::Encoding,
        region: None,
    };
    let mut encoding_polls = 0;
    let result = construct_with_cancel(&inputs, |point| {
        if point == target {
            encoding_polls += 1;
        }
        point == target && encoding_polls == 2
    });
    assert_cancelled(result, target);
    assert_eq!(encoding_polls, 2);
    assert_eq!(snapshot(&inputs), before);
}
#[test]
fn nested_selection_poll_can_cancel_before_any_native_member_is_built() {
    let inputs = requests();
    let target = Checkpoint {
        stage: FamilyStage::Selection,
        region: None,
    };
    let mut polls = 0;
    let mut later_stage = false;
    let result = construct_with_cancel(&inputs, |point| {
        later_stage |= point != target;
        polls += 1;
        polls == 3
    });
    assert_cancelled(result, target);
    assert!(!later_stage);
    assert!(polls >= 3);
}
#[test]
fn later_source_mutation_does_not_change_frozen_geometry_or_retained_sources() {
    let mut inputs = requests();
    let before = snapshot(&inputs);
    let result = construct(&inputs).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(snapshot(&inputs), before);
    let prepared = snapshot(&result.selection.prepared.prepared);
    let bytes = result.bytes.clone();
    inputs[0].loops[0][0].poles[0][0] += 1.;
    inputs[0].radius += 1.;
    assert_eq!(snapshot(&result.selection.prepared.prepared), prepared);
    assert!(
        !result.selection.prepared.planar.members[&inputs[0].region].matches_request(&inputs[0])
    );
    let (root, bindings) = pipeline::decode(&result.bytes, &result.keys).unwrap();
    assert_eq!(
        crate::surface_sketch::encoding::encode(&root, &bindings).unwrap(),
        bytes
    );
}
#[test]
fn failed_later_member_storage_preserves_inputs_and_decoded_geometry() {
    let inputs = requests();
    let before = snapshot(&inputs);
    let selected = selection::certify(&inputs, Default::default()).unwrap();
    let prepared = &selected.prepared.prepared;
    let (bytes, _, root, bindings) = freeze::construct(prepared).unwrap();
    let (_, mut views) = coverage::certify_family(prepared, &root, &bindings).unwrap();
    views.last_mut().unwrap().bindings.pop();
    assert!(matches!(
        store(&inputs, prepared, views),
        Err(FamilyFailure::Member(_))
    ));
    assert_eq!(snapshot(&inputs), before);
    assert_eq!(
        crate::surface_sketch::encoding::encode(&root, &bindings).unwrap(),
        bytes
    );
}
#[test]
fn successful_replay_is_unchanged_after_refusal_and_late_cancellation() {
    let inputs = requests();
    let baseline = construct(&inputs).unwrap_or_else(|e| panic!("{e}"));
    check_refusal(&inputs);
    let target = Checkpoint {
        stage: FamilyStage::Completion,
        region: None,
    };
    assert_cancelled(construct_with_cancel(&inputs, |p| p == target), target);
    let replay = construct(&inputs).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(replay.bytes, baseline.bytes);
    assert_eq!(replay.keys, baseline.keys);
    assert_eq!(replay.history, baseline.history);
}

fn check_refusal(inputs: &[Request]) {
    let mut invalid = inputs.to_vec();
    invalid[0].max_fit_level = 0;
    let before = snapshot(&invalid);
    assert!(matches!(
        construct(&invalid),
        Err(FamilyFailure::Selection(_))
    ));
    assert_eq!(snapshot(&invalid), before);
}

#[test]
fn family_cancellation_reaches_retained_storage_arithmetic() {
    let inputs = requests();
    let before = snapshot(&inputs);
    let target = Checkpoint {
        stage: FamilyStage::Storage,
        region: Some(SurfaceRegionId(5)),
    };
    let mut polls = 0;
    let volume =
        martensite_tkmath::predicates::roots::metric_bounds::CylindricalMeasureLimit::Report;
    let result = construct_budgeted_with_cancel(&inputs, volume, |point| {
        if point == target {
            polls += 1;
        }
        point == target && polls >= 8
    });
    assert_cancelled(result, target);
    assert_eq!(polls, 8);
    assert_eq!(snapshot(&inputs), before);
}

#[test]
fn family_member_checks_can_cancel_inside_cones_or_after_all_cones_complete() {
    let inputs = requests();
    let before = snapshot(&inputs);
    for limit in [8, cone_poll_count() + 8] {
        cancel_member_proofs(&inputs, limit);
        assert_eq!(snapshot(&inputs), before);
    }
}
fn cone_poll_count() -> usize {
    let (original, prepared, stored) = planar_cones::tests::proofs();
    let mut polls = 0;
    planar_cones::certify_with_cancel(&original, &prepared, &stored, || {
        polls += 1;
        false
    })
    .unwrap();
    polls
}
fn cancel_member_proofs(inputs: &[Request], limit: usize) {
    let target = Checkpoint {
        stage: FamilyStage::MemberChecks,
        region: Some(SurfaceRegionId(5)),
    };
    let mut polls = 0;
    let result = construct_with_cancel(inputs, |point| {
        if point == target {
            polls += 1;
        }
        point == target && polls >= limit
    });
    assert_cancelled(result, target);
    assert_eq!(polls, limit);
}

#[test]
fn public_cylinder_keeps_typed_storage_and_member_cancellation_classes() {
    use crate::surface_sketch::wrapped_profile::api;
    let input = api::operation_tests::request();
    for phase in [FamilyStage::Storage, FamilyStage::MemberChecks] {
        let target = Checkpoint {
            stage: phase,
            region: Some(SurfaceRegionId(5)),
        };
        let stop = input.selections.len() + 2 + math_stop_count(&input, target);
        let mut polls = 0;
        let failure = api::normal_extrude_profile_with_cancel(&input, || {
            polls += 1;
            polls == stop
        })
        .err()
        .unwrap();
        assert_eq!(failure.class(), api::WrappedFailureClass::Cancelled);
        assert_eq!(failure.stage(), api::WrappedStage::Construction);
        assert_eq!(polls, stop);
    }
}
fn math_stop_count(input: &WrappedNormalRequest, target: Checkpoint) -> usize {
    let mut total = 0;
    let mut target_polls = 0;
    let result = construct_budgeted_with_cancel(&requests(), input.options.volume, |point| {
        total += 1;
        if point == target {
            target_polls += 1;
        }
        point == target && target_polls == 8
    });
    assert_cancelled(result, target);
    assert_eq!(target_polls, 8);
    total
}
