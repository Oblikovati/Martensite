// SPDX-License-Identifier: GPL-2.0-only
//! Original spherical BRep band, independent seam oracle and scoped solver adoption.
use crate::{
    constraints::{length, policy, rid},
    fixture::{pid, point, seed},
};
use martensite_tkbrep::{
    brep::{
        face_input_capture::FaceCaptureLimits,
        spherical_attachment::{
            AttachmentSource, SphericalBandPlace as Place, SphericalFaceDomain, SphericalFaceError,
            capture_spherical_band,
        },
    },
    topexp::Explorer,
    topods::{self, Face, Solid},
};
use martensite_tkg3d::top_abs::ShapeEnum;
use martensite_tkmath::{
    gp::{Transform3, Vector3},
    predicates::roots::metric_bounds::spherical_band::SphericalBandError,
    toploc::Location,
};
use martensite_tkprim::{brep_prim::BRepPrimOneAxis, brep_prim_api::BRepPrimAPIMakeSphere};
use martensite_tksketch3d::{
    PointRole, Snapshot,
    constraints::{
        ConstrainedSketch, RelationEdit, RelationSpec, SphericalAttachedSketch,
        SphericalFaceBinding, SphericalFaceDragUpdate, SphericalFaceFailure as Failure,
        SphericalFaceRelation,
    },
};
use martensite_tktopalgo::brep_builder_api::BRepBuilderAPIMakeShape;
use std::{f64::consts::TAU, num::NonZeroU64};

const SOURCE: AttachmentSource = AttachmentSource {
    document: [91; 16],
    body: NonZeroU64::new(3).unwrap(),
    face: NonZeroU64::new(5).unwrap(),
    incarnation: 3,
    revision: 18,
};
const TRACE: f64 = 1e-9;

// 55-digit Taylor evaluation of the separate closed form, rounded for authored binary64 targets.
const POINTS: [[f64; 3]; 3] = [
    [
        9.877_582_561_890_373,
        -4.479_425_538_604_203,
        16.494_807_918_509_046,
    ],
    [9.937_824_843_421_29, -4., 16.494_807_918_509_046],
    [
        9.877_582_561_890_373,
        -3.520_574_461_395_797,
        16.494_807_918_509_046,
    ],
];
const EQUATOR: [f64; 3] = [10., -4., 16.];

fn key(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).unwrap()
}

fn native(latitude: [f64; 2]) -> (Solid, Face) {
    let mut maker = BRepPrimAPIMakeSphere::with_latitudes(2., latitude[0], latitude[1]).unwrap();
    let mut body = topods::solid(maker.shape().unwrap()).unwrap();
    let surface = maker.sphere().lateral_face().unwrap();
    let mut translation = Transform3::default();
    translation.set_translation(Vector3::new(8., -4., 16.));
    body.set_location(&Location::new(translation), false)
        .unwrap();
    let occurrence = Explorer::new(&body, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .find(|face| face.is_partner(&surface))
        .unwrap();
    (body, topods::face(&occurrence).unwrap())
}

fn domain(latitude: [f64; 2], source: AttachmentSource) -> SphericalFaceDomain {
    let (body, face) = native(latitude);
    capture_spherical_band(
        &body,
        &face,
        source,
        FaceCaptureLimits::bounded(),
        TRACE,
        || false,
    )
    .unwrap()
}

fn binding(latitude: [f64; 2], source: AttachmentSource, salt: u64) -> SphericalFaceBinding {
    SphericalFaceBinding::new(domain(latitude, source), key(salt))
}

fn base(xyz: [f64; 3]) -> ConstrainedSketch {
    let geometry = Snapshot::empty(length(1e-3))
        .apply(
            &[
                seed(1, xyz, PointRole::SketchMarker),
                seed(2, [4., -4., 16.], PointRole::SketchMarker),
            ],
            || false,
        )
        .unwrap();
    ConstrainedSketch::new(geometry, policy())
        .unwrap()
        .apply(
            &[RelationEdit::Insert(
                rid(20),
                RelationSpec::FixPoint(pid(2)),
            )],
            || false,
        )
        .unwrap()
}

fn attach(xyz: [f64; 3], binding: SphericalFaceBinding, near: f64) -> SphericalAttachedSketch {
    base(xyz)
        .attach_point_on_sphere(rid(40), pid(1), binding, near, || false)
        .unwrap()
}

fn xyz(sketch: &SphericalAttachedSketch) -> [f64; 3] {
    *sketch
        .geometry()
        .point(pid(1))
        .unwrap()
        .position
        .coordinates()
}

fn close(actual: [f64; 3], expected: [f64; 3]) {
    for (a, b) in actual.into_iter().zip(expected) {
        assert!((a - b).abs() < 1e-8, "actual {a}, expected {b}");
    }
}

#[test]
fn stored_spherical_band_crosses_the_original_seam_in_both_directions() {
    fn shared<T: Send + Sync>() {}
    shared::<SphericalAttachedSketch>();
    shared::<SphericalFaceBinding>();
    shared::<SphericalFaceDragUpdate>();
    shared::<SphericalFaceRelation>();
    shared::<Failure>();
    let binding = binding([0., 0.5], SOURCE, 4);
    let source = binding.revision();
    for (order, sign) in [([0, 1, 2], 1.), ([2, 1, 0], -1.)] {
        let initial = attach(POINTS[order[0]], binding.clone(), TAU - sign * 0.25);
        let rank = initial.inspect_rank().unwrap();
        assert_eq!((rank.rank, rank.degrees_of_freedom), (6, 2));
        let mut accepted = initial.clone();
        for index in order.into_iter().skip(1) {
            let prior = accepted.relation().uv;
            let next = accepted
                .drag_preferred(pid(1), point(POINTS[index]), 1e-9, || source, || false)
                .unwrap();
            let uv = next.sketch.relation().uv;
            let expected = [-0.25, 0., 0.25][index] + TAU;
            assert!((uv[0] - expected).abs() < 1e-8);
            assert!((uv[1] - 0.25).abs() < 1e-8);
            assert!(sign * (uv[0] - prior[0]) > 0.);
            assert_eq!(next.attachment.world_place, Place::Interior);
            assert_eq!(next.attachment.uv_place, Place::Interior);
            assert!(next.attachment.distance_bound <= policy().length_tolerance.value());
            assert_eq!(next.rank, rank);
            close(xyz(&next.sketch), POINTS[index]);
            accepted = next.sketch;
        }
        assert_eq!(xyz(&initial), POINTS[order[0]]);
    }
}

#[test]
fn exact_equator_closes_and_the_next_world_or_uv_value_outside_refuses() {
    let bound = domain([0., 0.5], SOURCE);
    let chart = bound.point_chart(SOURCE, [0., 0.], || false).unwrap();
    assert_eq!(chart.world, EQUATOR);
    assert_eq!(chart.tangents, [[0., 2., 0.], [0., 0., 2.]]);
    let area = bound.relative_area_lower();
    let ideal_bound = 0.292_527_520_630_124_2_f64;
    assert!(area > 0.29 && area <= ideal_bound.next_up());
    let exact = bound
        .point_qualify(SOURCE, [0., 0.], EQUATOR, 0., || false)
        .unwrap();
    assert_eq!(exact.lifted, EQUATOR.map(|value| [value; 2]));
    assert_eq!(exact.distance_bound, 0.);
    assert_eq!(
        (exact.world_place, exact.uv_place),
        (Place::FirstRim, Place::FirstRim)
    );
    let displaced = [10.375, -3.5, 16.];
    assert_eq!(
        bound
            .point_qualify(SOURCE, [0., 0.], displaced, 0.625, || false)
            .unwrap()
            .distance_bound,
        0.625
    );
    assert!(matches!(
        bound.point_qualify(SOURCE, [0., 0.], displaced, 0.625_f64.next_down(), || false),
        Err(SphericalFaceError::Numeric(SphericalBandError::Distance {
            bound: Some(0.625),
            ..
        }))
    ));
    let binding = SphericalFaceBinding::new(bound, key(4));
    let source = binding.revision();
    let initial = attach(POINTS[1], binding.clone(), TAU);
    let rim = initial
        .drag_preferred(pid(1), point(EQUATOR), 1e-9, || source, || false)
        .unwrap();
    assert_eq!(rim.attachment.world_place, Place::FirstRim);
    assert_eq!(rim.attachment.uv_place, Place::FirstRim);
    assert_eq!(xyz(&rim.sketch), EQUATOR);
    let below = [EQUATOR[0], EQUATOR[1], EQUATOR[2].next_down()];
    assert!(matches!(
        rim.sketch
            .drag_preferred(pid(1), point(below), 1e-9, || source, || false),
        Err(Failure::Trim(Place::Below))
    ));
    let record = rim.sketch.relation();
    let forged = SphericalFaceRelation {
        uv: [record.uv[0], 0.0_f64.next_down()],
        ..record
    };
    assert!(matches!(
        rim.sketch
            .detach(|| false)
            .unwrap()
            .restore_point_on_sphere(forged, binding, || false),
        Err(Failure::Trim(Place::Below))
    ));
    let recovered = rim
        .sketch
        .drag_preferred(pid(1), point(POINTS[2]), 1e-9, || source, || false)
        .unwrap();
    assert!(recovered.sketch.relation().uv[0] > TAU);
    assert_eq!(recovered.attachment.world_place, Place::Interior);
    close(xyz(&recovered.sketch), POINTS[2]);
}

#[test]
fn pole_and_antipodal_targets_never_publish_a_false_two_direction_rank() {
    let binding = binding([0., 0.5], SOURCE, 4);
    let source = binding.revision();
    let seed = attach(EQUATOR, binding, 0.);
    let pole = point([8., -4., 18.]);
    assert!(
        matches!(seed.drag_preferred(pid(1), pole, 1e-9, || source, || false),
            Err(Failure::Attachment(cause)) if matches!(*cause, SphericalFaceError::Numeric(SphericalBandError::Pole))
        )
    );
    let opposite = point([6., -4., 16.]);
    assert!(
        matches!(seed.drag_preferred(pid(1), opposite, 1e-9, || source, || false),
            Err(Failure::Attachment(cause)) if matches!(*cause, SphericalFaceError::Numeric(SphericalBandError::Lift { .. }))
        )
    );
    let above = point([9.705_049_044_119_011, -4., 17.045_374_457_861_32_f64]);
    assert!(matches!(
        seed.drag_preferred(pid(1), above, 1e-9, || source, || false),
        Err(Failure::Trim(Place::Above))
    ));
    assert_eq!(xyz(&seed), EQUATOR);
}

#[test]
fn restored_and_rebound_spherical_motion_preserves_the_original_sheet_and_authority() {
    let old = binding([0., 0.5], SOURCE, 4);
    let expected = old.revision();
    let initial = attach(POINTS[0], old.clone(), TAU - 0.25);
    let target = point(POINTS[2]);
    let crossed = initial
        .drag_preferred(pid(1), target, 1e-9, || expected, || false)
        .unwrap()
        .sketch;
    let detached = crossed.detach(|| false).unwrap();
    let restored = detached
        .restore_point_on_sphere(crossed.relation(), old, || false)
        .unwrap();
    assert_eq!(restored.relation(), crossed.relation());
    assert_eq!(restored.geometry(), crossed.geometry());

    let stamp = AttachmentSource {
        revision: SOURCE.revision + 1,
        ..SOURCE
    };
    let new = binding([0., 0.75], stamp, 5);
    let current = new.revision();
    let mut calls = 0;
    assert!(matches!(initial.drag_preferred(pid(1), target, 1e-9, || {
        calls += 1;
        if calls == 1 { expected } else { current }
    }, || false), Err(Failure::StaleSource { expected: a, current: b }) if a == expected && b == current));
    assert_eq!(calls, 2);
    let rebound = crossed.rebind(new, || false).unwrap();
    assert_eq!(rebound.geometry(), crossed.geometry());
    assert_eq!(rebound.relation().source, current);
    assert!(rebound.relation().uv[0] > TAU);
    let foreign = binding(
        [0., 0.75],
        AttachmentSource {
            document: [7; 16],
            ..stamp
        },
        6,
    );
    assert!(matches!(
        rebound.rebind(foreign, || false),
        Err(Failure::RebindSource { .. })
    ));
    let short = binding(
        [0., 0.125],
        AttachmentSource {
            revision: stamp.revision + 1,
            ..stamp
        },
        7,
    );
    assert!(matches!(
        rebound.rebind(short, || false),
        Err(Failure::Trim(Place::Above))
    ));
}

#[test]
fn actual_spherical_publication_is_late_cancelled_before_adoption() {
    let binding = binding([0., 0.5], SOURCE, 4);
    let source = binding.revision();
    let initial = attach(POINTS[0], binding, TAU - 0.25);
    let target = point(POINTS[2]);
    let mut checkpoints = 0;
    assert!(
        initial
            .drag_preferred(
                pid(1),
                target,
                1e-9,
                || source,
                || {
                    checkpoints += 1;
                    false
                }
            )
            .is_ok()
    );
    let mut polls = 0;
    assert!(matches!(
        initial.drag_preferred(
            pid(1),
            target,
            1e-9,
            || source,
            || {
                polls += 1;
                polls == checkpoints
            }
        ),
        Err(Failure::Cancelled)
    ));
    assert_eq!(xyz(&initial), POINTS[0]);
}

#[test]
fn near_pole_band_must_clear_an_independent_physical_chart_guard() {
    let binding = binding([1.5, 1.53125], SOURCE, 4);
    let geometry = base([8.14, -4., 17.9975]).geometry().clone();
    let mut guarded = policy();
    guarded.solve.rank_tolerance = 0.1;
    let sketch = ConstrainedSketch::new(geometry, guarded).unwrap();
    assert!(matches!(
        sketch.attach_point_on_sphere(rid(42), pid(1), binding, 0., || false),
        Err(Failure::SingularChart { lower, limit: 0.1 }) if lower > 0. && lower < 0.1
    ));
}

#[test]
fn independent_processes_replay_the_same_spherical_lift_bits() {
    let receipt = || {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "spherical_on_face::emit_spherical_receipt",
                "--nocapture",
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
            .split_once("SPHERICAL_POINT=")
            .unwrap()
            .1
            .lines()
            .next()
            .unwrap()
            .to_owned()
    };
    assert_eq!(receipt(), receipt());
}

#[test]
fn emit_spherical_receipt() {
    let binding = binding([0., 0.5], SOURCE, 4);
    let source = binding.revision();
    let initial = attach(POINTS[0], binding, TAU - 0.25);
    let output = initial
        .drag_preferred(pid(1), point(POINTS[2]), 1e-9, || source, || false)
        .unwrap();
    let uv = output.sketch.relation().uv.map(f64::to_bits);
    let xyz = xyz(&output.sketch).map(f64::to_bits);
    let achieved = output.attachment.distance_bound.to_bits();
    let trace = output.sketch.trim_trace().map(f64::to_bits);
    println!("SPHERICAL_POINT={uv:016x?} {xyz:016x?} {achieved:016x} {trace:016x?}");
}
