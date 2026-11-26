// SPDX-License-Identifier: GPL-2.0-only
//! Actual placed native full-period wall, closed rims, and scoped lifted solve publication.
use super::{BOUND, FaceError, STAMP, WallPlace as CylinderWallPlace, actual_face, native};
use crate::{
    constraints::{length, policy, rid},
    fixture::{pid, point, seed},
};
use martensite_tkbrep::{
    brep::{
        BRepTool,
        cylindrical_split::{
            CylinderPointError, CylindricalSplitDomain, capture_cylindrical_split_domain,
        },
        face_input_capture::FaceCaptureLimits,
        planar_attachment::{AttachmentSource, FaceAttachmentError},
    },
    topods::{self, Face, Solid},
};
use martensite_tkg3d::geom::GeomCylindricalSurface;
use martensite_tkmath::{
    gp::{Transform3, Vector3},
    toploc::Location,
};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeCylinder;
use martensite_tksketch3d::{
    PointRole, Snapshot,
    constraints::{
        ConstrainedSketch, CylindricalAttachedSketch, CylindricalFaceBinding,
        CylindricalFaceDragUpdate, CylindricalFaceFailure as Failure, CylindricalFaceRelation,
        RelationEdit, RelationSpec,
    },
};
use std::{f64::consts::TAU, num::NonZeroU64};

fn key(value: u64) -> NonZeroU64 {
    NonZeroU64::new(value).unwrap()
}

fn captured(body: &Solid, face: &Face, source: AttachmentSource) -> CylindricalSplitDomain {
    capture_cylindrical_split_domain(
        body,
        face,
        source,
        FaceCaptureLimits::bounded(),
        BOUND,
        || false,
    )
    .unwrap()
}

fn binding(height: f64, source: AttachmentSource, salt: u64) -> CylindricalFaceBinding {
    let (body, face) = native(BRepPrimAPIMakeCylinder::new(1., height));
    CylindricalFaceBinding::new(captured(&body, &face, source), key(salt))
}

fn world(angle: f64, axial: f64) -> [f64; 3] {
    let (sine, cosine) = martensite_cmath::sincos(angle);
    [cosine, sine, axial]
}

fn sketch(xyz: [f64; 3], coupled: bool) -> ConstrainedSketch {
    let mut points = vec![
        seed(1, xyz, PointRole::SketchMarker),
        seed(2, [0., 0., 4.], PointRole::Defining),
    ];
    let mut relations = vec![RelationEdit::Insert(
        rid(20),
        RelationSpec::FixPoint(pid(2)),
    )];
    if coupled {
        points.push(seed(3, xyz, PointRole::SketchMarker));
        relations.push(RelationEdit::Insert(
            rid(30),
            RelationSpec::Coincident([pid(1), pid(3)]),
        ));
    }
    let geometry = Snapshot::empty(length(1e-3))
        .apply(&points, || false)
        .unwrap();
    ConstrainedSketch::new(geometry, policy())
        .unwrap()
        .apply(&relations, || false)
        .unwrap()
}

fn attach(
    base: &ConstrainedSketch,
    binding: CylindricalFaceBinding,
    near: f64,
) -> CylindricalAttachedSketch {
    base.attach_point_on_cylinder(rid(50), pid(1), binding, near, || false)
        .unwrap()
}

fn xyz(sketch: &CylindricalAttachedSketch) -> [f64; 3] {
    *sketch
        .geometry()
        .point(pid(1))
        .unwrap()
        .position
        .coordinates()
}

fn assert_xyz_close(actual: [f64; 3], expected: [f64; 3]) {
    for (x, y) in actual.into_iter().zip(expected) {
        assert!((x - y).abs() < 1e-8, "actual {x}, expected {y}");
    }
}

#[test]
fn actual_native_seam_crosses_both_ways_with_one_continuous_gesture() {
    fn shared<T: Send + Sync>() {}
    shared::<CylindricalAttachedSketch>();
    shared::<CylindricalFaceBinding>();
    shared::<CylindricalFaceDragUpdate>();
    shared::<CylindricalFaceRelation>();
    shared::<Failure>();
    let binding = binding(2., STAMP, 4);
    let source = binding.revision();
    for sign in [1., -1.] {
        let original = world(-0.24 * sign, 1.);
        let base = sketch(original, false);
        let seed = attach(&base, binding.clone(), TAU - 0.24 * sign);
        let rank = seed.inspect_rank().unwrap();
        assert_eq!((rank.rank, rank.degrees_of_freedom), (6, 2));
        let mut accepted = seed.clone();
        for angle in [-0.13, 0., 0.13, 0.26] {
            let prior = accepted.relation().uv[0];
            let target = world(angle * sign, 1.25);
            let output = accepted
                .drag_preferred(pid(1), point(target), 1e-8, || source, || false)
                .unwrap();
            let active = output.sketch.relation().uv[0];
            assert!(sign * (active - prior) > 0.);
            assert!((active - (TAU + angle * sign)).abs() < 1e-8);
            assert_eq!(output.attachment.world_place, CylinderWallPlace::Interior);
            assert_eq!(output.attachment.uv_place, CylinderWallPlace::Interior);
            assert!(output.attachment.distance_bound <= policy().length_tolerance.value());
            assert_eq!(output.rank, rank);
            assert_xyz_close(xyz(&output.sketch), target);
            accepted = output.sketch;
        }
        assert!(sign * (accepted.relation().uv[0] - TAU) > 0.);
        assert_eq!(xyz(&seed), original);
    }
}

#[test]
fn closed_rims_refuse_adjacent_outside_and_allow_recovery_on_the_same_lift() {
    let input = binding(2., STAMP, 4);
    let source = input.revision();
    let base = sketch([1., 0., 1.], false);
    let initial = attach(&base, input, TAU);
    let upper = initial
        .drag_preferred(pid(1), point([1., 0., 2.]), 1e-8, || source, || false)
        .unwrap();
    assert_eq!(upper.attachment.world_place, CylinderWallPlace::LastRim);
    assert_eq!(upper.attachment.uv_place, CylinderWallPlace::LastRim);
    assert_eq!(xyz(&upper.sketch), [1., 0., 2.]);
    let retained = upper.sketch.relation();
    let beyond = CylindricalFaceRelation {
        uv: [retained.uv[0], 2.0_f64.next_up()],
        ..retained
    };
    let detached = upper.sketch.detach(|| false).unwrap();
    let recaptured = binding(2., STAMP, 4);
    assert!(matches!(
        detached.restore_point_on_cylinder(beyond, recaptured, || false),
        Err(Failure::Trim(CylinderWallPlace::Above))
    ));
    for z in [2.0_f64.next_up(), 3.] {
        assert!(matches!(
            upper
                .sketch
                .drag_preferred(pid(1), point([1., 0., z]), 1e-8, || source, || false),
            Err(Failure::Trim(CylinderWallPlace::Above))
        ));
    }
    let lower = upper
        .sketch
        .drag_preferred(pid(1), point([1., 0., 0.]), 1e-8, || source, || false)
        .unwrap();
    assert_eq!(lower.attachment.world_place, CylinderWallPlace::FirstRim);
    assert_eq!(lower.attachment.uv_place, CylinderWallPlace::FirstRim);
    assert!(matches!(
        lower
            .sketch
            .drag_preferred(pid(1), point([1., 0., -0.2]), 1e-8, || source, || false),
        Err(Failure::Trim(CylinderWallPlace::Below))
    ));
    assert_eq!(xyz(&upper.sketch), [1., 0., 2.]);
    assert_eq!(xyz(&initial), [1., 0., 1.]);
}

#[test]
fn moving_a_coincident_marker_uses_the_actual_surface_hard_rows() {
    let binding = binding(2., STAMP, 4);
    let source = binding.revision();
    let base = sketch(world(-0.13, 1.), true);
    let initial = attach(&base, binding, TAU - 0.13);
    let target = world(0.18, 1.15).map(|coordinate| coordinate * 1.12);
    let output = initial
        .drag_preferred(pid(3), point(target), 1e-10, || source, || false)
        .unwrap();
    assert_eq!((output.rank.rank, output.rank.degrees_of_freedom), (9, 2));
    assert!(output.sketch.relation().uv[0] > TAU);
    let bound = output.attachment.distance_bound;
    assert!(bound <= policy().length_tolerance.value());
    let attached = xyz(&output.sketch);
    let coupled = *output
        .sketch
        .geometry()
        .point(pid(3))
        .unwrap()
        .position
        .coordinates();
    assert_xyz_close(coupled, attached);
    assert_xyz_close(attached, world(0.18, target[2]));
}

#[test]
fn restore_rebind_and_late_input_rechecks_preserve_authority_and_sheet() {
    let original = binding(2., STAMP, 4);
    let old_source = original.revision();
    let base = sketch(world(-0.16, 1.), false);
    let initial = attach(&base, original.clone(), TAU - 0.16);
    let target = point(world(0.19, 1.1));
    let crossed = initial
        .drag_preferred(pid(1), target, 1e-8, || old_source, || false)
        .unwrap()
        .sketch;
    assert!(crossed.relation().uv[0] > TAU);
    let record = crossed.relation();
    let detached = crossed.detach(|| false).unwrap();
    let reopened = detached
        .restore_point_on_cylinder(record, original.clone(), || false)
        .unwrap();
    assert_eq!(reopened.relation(), record);
    assert_eq!(reopened.geometry(), crossed.geometry());
    let forged = CylindricalFaceRelation {
        uv: [record.uv[0] + 0.5, record.uv[1]],
        ..record
    };
    assert!(matches!(
        detached.restore_point_on_cylinder(forged, original, || false),
        Err(Failure::Attachment(cause)) if matches!(*cause, FaceError::Point(CylinderPointError::Distance { .. }))
    ));

    let revised = AttachmentSource {
        revision: STAMP.revision + 1,
        ..STAMP
    };
    let newer = binding(3., revised, 5);
    let next_source = newer.revision();
    let mut checks = 0;
    assert!(matches!(
        initial.drag_preferred(pid(1), target, 1e-8, || {
            checks += 1;
            if checks == 1 { old_source } else { next_source }
        }, || false),
        Err(Failure::StaleSource { expected, current }) if expected == old_source && current == next_source
    ));
    assert_eq!(checks, 2);
    let rebound = crossed.rebind(newer, || false).unwrap();
    assert_eq!(rebound.geometry(), crossed.geometry());
    assert_eq!(rebound.relation().source, next_source);
    assert!(rebound.relation().uv[0] > TAU);
    let different = AttachmentSource {
        document: [88; 16],
        ..revised
    };
    assert!(matches!(
        crossed.rebind(binding(3., different, 6), || false),
        Err(Failure::RebindSource { .. })
    ));
    let short = binding(0.5, revised, 7);
    assert!(matches!(
        crossed.rebind(short, || false),
        Err(Failure::Trim(CylinderWallPlace::Above))
    ));
}

#[test]
fn exact_antipodal_and_late_cancellation_never_publish() {
    let binding = binding(2., STAMP, 4);
    let source = binding.revision();
    let initial = attach(&sketch([1., 0., 1.], false), binding, 0.);
    assert!(matches!(
        initial.drag_preferred(pid(1), point([-1., 0., 1.]), 1e-8, || source, || false),
        Err(Failure::Attachment(cause)) if matches!(*cause, FaceError::Point(CylinderPointError::AmbiguousLift { .. }))
    ));
    let target = point(world(0.15, 1.4));
    let mut completed = 0;
    assert!(
        initial
            .drag_preferred(
                pid(1),
                target,
                1e-8,
                || source,
                || {
                    completed += 1;
                    false
                }
            )
            .is_ok()
    );
    let mut cancelled = 0;
    assert!(matches!(
        initial.drag_preferred(
            pid(1),
            target,
            1e-8,
            || source,
            || {
                cancelled += 1;
                cancelled == completed
            }
        ),
        Err(Failure::Cancelled)
    ));
    assert_eq!(xyz(&initial), [1., 0., 1.]);
    assert_eq!(initial.relation().uv, [0., 1.]);
}

#[test]
fn placed_source_capture_stays_frozen_when_actual_native_cylinder_changes() {
    let (body, original) = native(BRepPrimAPIMakeCylinder::new(1., 2.));
    let mut translation = Transform3::default();
    translation.set_translation(Vector3::new(8., -4., 16.));
    let placement = Location::new(translation);
    let placed = topods::solid(&body.moved(&placement, false).unwrap()).unwrap();
    let face = actual_face(&placed, &original);
    assert!(matches!(
        capture_cylindrical_split_domain(
            &placed,
            &original,
            STAMP,
            FaceCaptureLimits::bounded(),
            BOUND,
            || false
        ),
        Err(FaceError::Source(FaceAttachmentError::FaceOccurrence))
    ));
    let proof = captured(&placed, &face, STAMP);
    let site = [9., -4., 17.];
    let query = proof
        .point_qualify(STAMP, [0., 1.], site, 0., || false)
        .unwrap();
    assert_eq!(query.lifted, site.map(|v| [v; 2]));
    assert_eq!(query.distance_bound, 0.);
    let displaced = [9.375, -3.5, 17.];
    let admitted = proof
        .point_qualify(STAMP, [0., 1.], displaced, 0.625, || false)
        .unwrap();
    assert_eq!(admitted.distance_bound, 0.625);
    assert!(matches!(
        proof.point_qualify(STAMP, [0., 1.], displaced, 0.625_f64.next_down(), || false),
        Err(FaceError::Point(CylinderPointError::Distance {
            bound: Some(0.625),
            ..
        }))
    ));
    let wrong = AttachmentSource {
        face: key(1),
        ..STAMP
    };
    assert!(matches!(
        proof.point_qualify(wrong, [0., 1.], site, BOUND, || false),
        Err(FaceError::Source(FaceAttachmentError::StaleSource { .. }))
    ));
    let (surface, _) = BRepTool::surface_with_location(&face).unwrap();
    surface
        .downcast::<GeomCylindricalSurface>()
        .get()
        .unwrap()
        .set_radius(1.2)
        .unwrap();
    assert!(
        capture_cylindrical_split_domain(
            &placed,
            &face,
            STAMP,
            FaceCaptureLimits::bounded(),
            BOUND,
            || false
        )
        .is_err()
    );
    let binding = CylindricalFaceBinding::new(proof, key(4));
    let source = binding.revision();
    let native = attach(&sketch(site, false), binding, 0.);
    let target = [9., -4., 17.5];
    let output = native
        .drag_preferred(pid(1), point(target), 1e-8, || source, || false)
        .unwrap();
    assert_xyz_close(xyz(&output.sketch), target);
    assert!(output.sketch.trim_trace().upper_bound <= BOUND);
}

#[test]
fn independent_processes_replay_identical_lift_and_trim_bits() {
    let receipt = || {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "split_cylinder::point_on_face::emit_periodic_receipt",
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
            .split_once("PERIODIC_POINT=")
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
fn emit_periodic_receipt() {
    let input = binding(2., STAMP, 4);
    let source = input.revision();
    let base = sketch(world(-0.16, 1.), false);
    let initial = attach(&base, input, TAU - 0.16);
    let output = initial
        .drag_preferred(pid(1), point(world(0.19, 1.1)), 1e-8, || source, || false)
        .unwrap();
    let uv = output.sketch.relation().uv.map(f64::to_bits);
    let point = xyz(&output.sketch).map(f64::to_bits);
    let bound = output.attachment.distance_bound.to_bits();
    let trim = [output.attachment.world_place, output.attachment.uv_place];
    println!("PERIODIC_POINT={uv:016x?} {point:016x?} {bound:016x} {trim:?}");
}
