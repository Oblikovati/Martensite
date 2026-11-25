// SPDX-License-Identifier: GPL-2.0-only
use crate::fixture::{baseline, connect, lid, line, pid, point, seed};
use martensite_tkmath::{
    gp::{Axis2, Direction3, Point3},
    predicates::ReflectionComparisonError,
};
use martensite_tksketch3d::{
    CurveId, CurveRole, Edit, EditFailure, EntityId, FrozenPointSource, FrozenSourceRevision,
    LineId, MinimumLength, MirrorDerivation, MirrorFailure, MirrorResult, PointId, PointRole,
    SketchPoint, Snapshot,
};
use std::{num::NonZeroU64, process::Command};

fn plane(coordinates: [f64; 3], normal: Direction3) -> Axis2 {
    let horizontal = if normal.x().abs() > 0.9 {
        Direction3::Y
    } else {
        Direction3::X
    };
    Axis2::with_x_direction(point(coordinates), normal, horizontal).unwrap()
}

fn marker(xyz: [f64; 3]) -> Snapshot {
    Snapshot::empty(MinimumLength::new(0.01).unwrap())
        .apply(&[seed(1, xyz, PointRole::SketchMarker)], || false)
        .unwrap()
}

fn one(source: &Snapshot, axis: Axis2, limit: f64) -> Result<MirrorResult, MirrorFailure> {
    source.mirror_copies(axis, &[(pid(1), pid(2))], &[], limit, || false)
}

fn copied_point(result: &MirrorResult) -> Point3 {
    result.snapshot.point(pid(2)).unwrap().position
}

#[test]
fn cardinal_planes_reflect_closed_form_coordinates_exactly() {
    let source = marker([3., -4., 5.]);
    for (normal, expected) in [
        (Direction3::Z, [3., -4., -5.]),
        (Direction3::Y, [3., 4., 5.]),
        (Direction3::X, [-3., -4., 5.]),
    ] {
        let result = one(&source, plane([0.; 3], normal), 0.).unwrap();
        assert_eq!(copied_point(&result), point(expected));
        assert_eq!(result.position_error_bound, 0.);
    }
    let shifted = one(&source, plane([1., 7., 9.], Direction3::X), 0.).unwrap();
    assert_eq!(copied_point(&shifted), point([-1., -4., 5.]));
}

#[test]
fn oblique_planes_use_the_same_geometry_for_both_normal_senses() {
    let source = marker([3., 4., 5.]);
    let expected = [3., -4., -3.];
    let mut positions = Vec::new();
    for sign in [1., -1.] {
        let normal = Direction3::new(0., sign, sign).unwrap();
        let result = one(&source, plane([7., 1., 0.], normal), 0.).unwrap();
        let reflected = copied_point(&result);
        assert_eq!(reflected, point(expected));
        positions.push(reflected.coordinates().map(f64::to_bits));
    }
    assert_eq!(positions[0], positions[1]);
}

#[test]
fn exact_stationary_point_preserves_bits_with_overflowing_tangent_offset() {
    let source_point = [-f64::MAX, -0., 0.];
    let source = marker(source_point);
    let mirrored = one(&source, plane([f64::MAX, 0., 0.], Direction3::Z), 0.).unwrap();
    assert_eq!(
        copied_point(&mirrored).coordinates().map(f64::to_bits),
        source_point.map(f64::to_bits)
    );
}

#[test]
fn large_coordinate_reflection_respects_the_exact_inclusive_error_limit() {
    let large = 9_007_199_254_740_992.;
    let source = marker([large, 1., 2.]);
    let axis = plane([-0.5, 0., 0.], Direction3::X);
    assert!(matches!(
        one(&source, axis, 1_f64.next_down()),
        Err(MirrorFailure::PositionError { point, limit })
            if point == pid(1) && limit == 1_f64.next_down()
    ));
    let accepted = one(&source, axis, 1.).unwrap();
    assert_eq!(copied_point(&accepted), point([-large, 1., 2.]));
    assert_eq!(accepted.position_error_bound, 1.);
}

#[test]
fn invalid_plane_and_nonrepresentable_candidate_have_typed_refusals() {
    let source = marker([0.; 3]);
    let mut invalid = Axis2::default();
    let zero = Direction3::new(f64::MAX, 0., 0.).unwrap();
    assert!(invalid.set_direction(zero).is_err());
    let failure = one(&source, invalid, 0.).unwrap_err();
    assert!(
        matches!(failure, MirrorFailure::InvalidPlane(error) if error.exception_type() == "Standard_ConstructionError")
    );
    let axis = plane([f64::MAX, 0., 0.], Direction3::X);
    let failure = one(&source, axis, f64::MAX).unwrap_err();
    assert!(
        matches!(failure, MirrorFailure::MetricInput { point, source: ReflectionComparisonError::Unrepresentable { axis: 0 } }
        if point == pid(1))
    );
    for limit in [-1., f64::NAN, f64::INFINITY] {
        assert!(matches!(
            one(&source, Axis2::default(), limit),
            Err(MirrorFailure::InvalidErrorLimit(_))
        ));
    }
}

#[test]
fn equivalent_yz_origins_keep_a_tangent_half_unit() {
    assert_yz_origin_invariant(0.5, 9_007_199_254_740_992.);
}

#[test]
fn equivalent_yz_origins_keep_a_tangent_coordinate_at_maximum() {
    assert_yz_origin_invariant(-f64::MAX, f64::MAX);
}

fn assert_yz_origin_invariant(tangent: f64, shifted_origin: f64) {
    let source = marker([1., tangent, 0.]);
    for origin in [0., shifted_origin] {
        let axis = plane([0., origin, 0.], Direction3::X);
        let result = one(&source, axis, 0.).unwrap();
        let actual = copied_point(&result).coordinates().map(f64::to_bits);
        assert_eq!(actual, [-1., tangent, 0.].map(f64::to_bits));
    }
}

#[test]
fn nearest_storage_is_certified_on_both_sides_of_a_signed_midpoint() {
    let large: f64 = 9_007_199_254_740_992.;
    for (sense, normal) in [(1., Direction3::X), (-1., -Direction3::X)] {
        let source = marker([sense * large, 0.5, -0.]);
        for (offset, expected, limit) in [(0.5, large, 1.), (0.75, large.next_up(), 0.5)] {
            let axis = plane([-sense * offset, 0., 0.], normal);
            let expected = [-sense * expected, 0.5, -0.].map(f64::to_bits);
            let result = one(&source, axis, limit).unwrap();
            assert_eq!(
                copied_point(&result).coordinates().map(f64::to_bits),
                expected
            );
            assert!(matches!(
                one(&source, axis, limit.next_down()),
                Err(MirrorFailure::PositionError { point, .. }) if point == pid(1)
            ));
        }
    }
}

#[test]
fn geometry_admission_refuses_collapsed_copies_atomically() {
    let source = connect([0.; 3], [1., 0., 0.], 0.01).unwrap();
    let before = source.clone();
    let axis = plane([9_007_199_254_740_992., 0., 0.], Direction3::X);
    let outcome = source.mirror_copies(
        axis,
        &[(pid(1), pid(7)), (pid(2), pid(8))],
        &[(lid(3), lid(9))],
        1.,
        || false,
    );
    assert!(
        matches!(outcome, Err(MirrorFailure::Geometry(EditFailure::CoincidentEndpoints(line))) if line.id == lid(9))
    );
    assert_eq!(source, before);
}

const MAP_POINTS: [(u64, u64); 4] = [(4, 12), (2, 14), (1, 15), (3, 13)];
const MAP_LINES: [(u64, u64); 2] = [(6, 16), (5, 11)];

fn two_lines_and_markers() -> Snapshot {
    baseline()
        .apply(
            &[Edit::SetCurveRole {
                curve: CurveId::Line(lid(6)),
                role: CurveRole::Construction,
            }],
            || false,
        )
        .unwrap()
}

fn all_copies(
    source: &Snapshot,
    reversed: bool,
    cancelled: impl FnMut() -> bool,
) -> Result<MirrorResult, MirrorFailure> {
    let mut point_ids = MAP_POINTS.map(|(a, b)| (pid(a), pid(b)));
    let mut line_ids = MAP_LINES.map(|(a, b)| (lid(a), lid(b)));
    if reversed {
        point_ids.reverse();
        line_ids.reverse();
    }
    source.mirror_copies(
        plane([0.; 3], Direction3::X),
        &point_ids,
        &line_ids,
        0.,
        cancelled,
    )
}

#[test]
fn shared_endpoints_roles_and_directed_history_survive_fresh_mapping() {
    let source = two_lines_and_markers();
    let result = all_copies(&source, false, || false).unwrap();
    assert_eq!(result, all_copies(&source, true, || false).unwrap());
    let copied = &result.snapshot;
    assert_eq!(
        copied.line(lid(11)).unwrap().line.endpoints,
        [pid(15), pid(14)]
    );
    assert_eq!(
        copied.line(lid(16)).unwrap().line.endpoints,
        [pid(14), pid(13)]
    );
    assert_eq!(
        copied.point(pid(14)).unwrap().position,
        point([-4., 0., 0.])
    );
    assert_eq!(
        copied.curve_role(CurveId::Line(lid(11))),
        Some(CurveRole::Model)
    );
    assert_eq!(
        copied.curve_role(CurveId::Line(lid(16))),
        Some(CurveRole::Construction)
    );
    for (old, new) in MAP_POINTS {
        assert_eq!(
            copied.point(pid(new)).unwrap().role,
            source.point(pid(old)).unwrap().role
        );
    }
    assert_sources_and_history(&source, &result);
}

fn assert_sources_and_history(source: &Snapshot, result: &MirrorResult) {
    let retained = &result.snapshot;
    for (old, _) in MAP_POINTS {
        assert_eq!(retained.point(pid(old)), source.point(pid(old)));
    }
    for (old, _) in MAP_LINES {
        assert_eq!(retained.line(lid(old)), source.line(lid(old)));
    }
    let mut expected = Vec::new();
    expected.extend(
        [(1, 15), (2, 14), (3, 13), (4, 12)].map(|(a, b)| MirrorDerivation::Point {
            source: pid(a),
            copy: pid(b),
        }),
    );
    expected.extend([(5, 11), (6, 16)].map(|(a, b)| MirrorDerivation::Line {
        source: lid(a),
        copy: lid(b),
    }));
    assert_eq!(result.history, expected);
}

#[test]
fn history_is_ordered_by_raw_source_key_across_entity_kinds() {
    let source = Snapshot::empty(MinimumLength::new(0.01).unwrap())
        .apply(
            &[
                seed(3, [1., 0., 0.], PointRole::Defining),
                seed(5, [2., 0., 0.], PointRole::Defining),
                line(4, 5, 3),
            ],
            || false,
        )
        .unwrap();
    let result = source
        .mirror_copies(
            plane([0.; 3], Direction3::X),
            &[(pid(5), pid(11)), (pid(3), pid(12))],
            &[(lid(4), lid(13))],
            0.,
            || false,
        )
        .unwrap();
    assert_eq!(
        result.history,
        [
            MirrorDerivation::Point {
                source: pid(3),
                copy: pid(12)
            },
            MirrorDerivation::Line {
                source: lid(4),
                copy: lid(13)
            },
            MirrorDerivation::Point {
                source: pid(5),
                copy: pid(11)
            },
        ]
    );
    assert_eq!(
        result.snapshot.line(lid(13)).unwrap().line.endpoints,
        [pid(11), pid(12)]
    );
}

#[test]
fn frozen_source_stamp_survives_while_the_mirror_copy_is_authored() {
    let stamp = FrozenPointSource {
        document: [3; 16],
        sketch: NonZeroU64::new(15).unwrap(),
        revision: FrozenSourceRevision::SessionGeneration { generation: 22 },
        point: pid(77),
    };
    let frozen = SketchPoint {
        id: pid(1),
        role: PointRole::Defining,
        position: point([2., 0., 0.]),
    };
    let source = Snapshot::empty(MinimumLength::new(0.01).unwrap())
        .apply(
            &[
                Edit::InsertFrozenPoint {
                    point: frozen,
                    source: stamp,
                },
                seed(3, [2., 2., 0.], PointRole::Defining),
                line(4, 1, 3),
            ],
            || false,
        )
        .unwrap();
    let result = source
        .mirror_copies(
            plane([0.; 3], Direction3::X),
            &[(pid(1), pid(12)), (pid(3), pid(13))],
            &[(lid(4), lid(14))],
            0.,
            || false,
        )
        .unwrap();
    assert_eq!(result.snapshot.frozen_source(pid(1)), Some(stamp));
    assert_eq!(result.snapshot.point(pid(1)), Some(frozen));
    assert_eq!(result.snapshot.frozen_source(pid(12)), None);
    assert!(result.history.contains(&MirrorDerivation::Point {
        source: pid(1),
        copy: pid(12)
    }));
    assert!(
        result
            .snapshot
            .apply(
                &[Edit::MovePoint {
                    point: pid(12),
                    position: point([-2., 0., 1.])
                }],
                || false
            )
            .is_ok()
    );
}

#[test]
fn incomplete_duplicate_and_nonfresh_mappings_are_refused() {
    let source = two_lines_and_markers();
    let axis = Axis2::default();
    let run = |points: &[(PointId, PointId)], lines: &[(LineId, LineId)]| {
        source.mirror_copies(axis, points, lines, 0., || false)
    };
    assert!(matches!(run(&[], &[]), Err(MirrorFailure::EmptySelection)));
    assert!(
        matches!(run(&[(pid(6), pid(10))], &[]), Err(MirrorFailure::MissingSource(EntityId::Point(p))) if p == pid(6))
    );
    assert!(
        matches!(run(&[(pid(1), pid(8)), (pid(1), pid(9))], &[]), Err(MirrorFailure::RepeatedSource(EntityId::Point(p))) if p == pid(1))
    );
    assert!(
        matches!(run(&[(pid(1), pid(6))], &[]), Err(MirrorFailure::DuplicateTarget { existing: EntityId::Line(a), attempted: EntityId::Point(b) }) if a == lid(6) && b == pid(6))
    );
    assert!(
        matches!(run(&[(pid(1), pid(9)), (pid(2), pid(10))], &[(lid(5), lid(9))]), Err(MirrorFailure::DuplicateTarget { existing: EntityId::Point(a), attempted: EntityId::Line(b) }) if a == pid(9) && b == lid(9))
    );
    assert!(
        matches!(run(&[(pid(1), pid(9))], &[(lid(5), lid(11))]), Err(MirrorFailure::UnmappedEndpoint { line, point }) if line == lid(5) && point == pid(2))
    );
    assert_eq!(source, two_lines_and_markers());
}

#[test]
fn every_cancellation_poll_leaves_the_source_available() {
    let source = two_lines_and_markers();
    let mut observed = 0;
    let complete = all_copies(&source, false, || {
        observed += 1;
        false
    })
    .unwrap();
    let before = source.clone();
    for stop in 1..=observed {
        let mut polls = 0;
        let result = all_copies(&source, true, || {
            polls += 1;
            polls == stop
        });
        assert!(
            matches!(result, Err(MirrorFailure::Cancelled)),
            "poll {stop} produced {result:?}"
        );
        assert_eq!(source, before);
    }
    assert_eq!(complete, all_copies(&source, true, || false).unwrap());
}

#[test]
fn concurrent_mirrors_match_serial_without_altering_the_source() {
    let source = two_lines_and_markers();
    let before = source.clone();
    let expected = all_copies(&source, false, || false).unwrap();
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for _ in 0..16 {
                    assert_eq!(all_copies(&source, true, || false).unwrap(), expected);
                }
            });
        }
    });
    assert_eq!(source, before);
}

#[test]
fn independent_processes_replay_the_same_mirror_bits() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<(MirrorResult, MirrorDerivation, MirrorFailure)>();
    assert_eq!(replay_output(), replay_output());
}

fn replay_output() -> String {
    let output = Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "mirror::emit_mirror_replay",
            "--nocapture",
            "--test-threads=1",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    stdout
        .split_once("MIRROR_REPLAY=")
        .unwrap()
        .1
        .lines()
        .next()
        .unwrap()
        .to_owned()
}

#[test]
fn emit_mirror_replay() {
    let source = two_lines_and_markers();
    let result = all_copies(&source, true, || false).unwrap();
    let coordinates = [12, 13, 14, 15].map(|id| {
        result
            .snapshot
            .point(pid(id))
            .unwrap()
            .position
            .coordinates()
            .map(f64::to_bits)
    });
    println!("MIRROR_REPLAY={:?} {coordinates:016x?}", result.history);
}
