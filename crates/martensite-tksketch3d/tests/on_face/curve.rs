// SPDX-License-Identifier: GPL-2.0-only
//! Independent square-trim and retained BRep checks for the first Curve On Face.
use super::{captured, key, native, proof};
use martensite_tkbrep::{
    brep::{
        BRepTool,
        planar_attachment::{
            FaceAttachmentCurveError, PlanarAttachmentLocation, PlanarCurvePath,
            PlanarCurveProofFailure,
        },
    },
    topods,
};
use martensite_tkg3d::geom::{GeomBezierCurve, GeomCurve};
use martensite_tkmath::{
    gp::{Axis1, Direction3, Point3, Transform3, Vector3},
    toploc::Location,
};
use martensite_tksketch3d::constraints::{
    PlanarFaceBinding, PlanarFaceCurve, PlanarFaceCurveFailure as Failure, PlanarFaceCurveRecipe,
    PlanarFaceRevision,
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

fn converted(hole: f64, offset: [f64; 3]) -> PlanarFaceBinding {
    let [x, y, z] = offset;
    let mut placement = Transform3::default();
    placement.set_translation(Vector3::new(x, y, z));
    placed(hole, placement)
}

fn placed(hole: f64, placement: Transform3) -> PlanarFaceBinding {
    let (solid, face) = native(hole);
    let placement = Location::new(placement);
    let moved_solid = topods::solid(&solid.moved(&placement, false).unwrap()).unwrap();
    let moved_face = topods::face(&face.moved(&placement, false).unwrap()).unwrap();
    captured(&moved_solid, &moved_face, 23, 9)
}

fn recipe(binding: &PlanarFaceBinding, uv: [[f64; 2]; 2]) -> PlanarFaceCurveRecipe {
    PlanarFaceCurveRecipe {
        source: binding.revision(),
        uv,
        distance_limit: 1e-6,
    }
}

fn replay(binding: &PlanarFaceBinding, uv: [[f64; 2]; 2]) -> Result<PlanarFaceCurve, Failure> {
    binding.replay_straight_curve(recipe(binding, uv), || binding.revision(), || false)
}

#[test]
fn located_offset_hole_accepts_boundary_travel_and_isolated_vertex_touch() {
    fn shared<T: Send + Sync>() {}
    shared::<PlanarFaceCurve>();
    shared::<PlanarFaceCurveRecipe>();
    shared::<Failure>();
    for (hole, shift) in [(0., [0.; 3]), (0.5, [256., -128., 32.])] {
        let binding = converted(hole, shift);
        for uv in [
            [[hole - 2., 1.], [hole + 2., 1.]], // travels along the hole edge in the middle
            [[hole, 2.], [hole + 2., 0.]],      // touches only the upper-right hole vertex
            [[2., 4.], [-2., 4.]],              // directed reversed along the outer boundary
            [[hole + 1., -0.5], [hole + 1., 0.5]], // entirely on the closed hole edge
        ] {
            let curve = replay(&binding, uv).unwrap();
            assert_eq!(curve.recipe(), recipe(&binding, uv));
            assert_eq!(curve.certificate().distance_bound, 0.);
            assert!(curve.tolerance() <= curve.recipe().distance_limit);
            let [x, y, z] = shift;
            let expected = uv.map(|[u, v]| [u + x, v + y, z]);
            verify_retained_pair(&curve, expected);
            let restored = binding
                .replay_straight_curve(curve.recipe(), || binding.revision(), || false)
                .unwrap();
            assert_eq!(restored.certificate(), curve.certificate());
            verify_retained_pair(&restored, expected);
        }
    }
}

fn verify_retained_pair(curve: &PlanarFaceCurve, expected: [[f64; 3]; 2]) {
    let native = curve.to_native().unwrap();
    assert!(
        BRepCheckAnalyzer::new(&native.edge, true, false, true)
            .unwrap()
            .is_valid()
            .unwrap()
    );
    let world = BRepTool::curve_with_location(&native.edge).unwrap();
    let uv = BRepTool::curve_on_face(&native.edge, &native.support).unwrap();
    assert!(uv.is_stored);
    assert_eq!(
        [world.first, world.last, uv.first, uv.last],
        [0., 1., 0., 1.]
    );
    let world = world.curve.downcast::<GeomBezierCurve>();
    let spatial = world.get().unwrap();
    assert_eq!(
        uv.pcurve.dynamic_type().unwrap().name(),
        "Geom2d_BezierCurve"
    );
    let planar = uv.pcurve.get().unwrap();
    assert_eq!(spatial.degree(), 1);
    let poles = |i| *spatial.pole(i).unwrap().coordinates();
    assert_eq!([poles(1), poles(2)], expected);
    let uv_at = |parameter| *planar.d0(parameter).unwrap().xy().coordinates();
    assert_eq!([uv_at(0.), uv_at(1.)], curve.recipe().uv);
    let [start, end] = curve.recipe().uv;
    assert_eq!(
        uv_at(0.5),
        std::array::from_fn(|i| (start[i] + end[i]) * 0.5)
    );
    assert_eq!(
        *spatial.d0(0.5).unwrap().coordinates(),
        std::array::from_fn(|i| (expected[0][i] + expected[1][i]) * 0.5)
    );
}

#[test]
fn individually_usable_endpoints_cannot_hide_a_hole_crossing() {
    for (hole, shift) in [(0., [0.; 3]), (0.5, [256., -128., 32.])] {
        let binding = converted(hole, shift);
        for signs in [[-2., 2.], [2., -2.]] {
            assert!(matches!(
                replay(&binding, signs.map(|x| [x + hole, 0.])),
                Err(Failure::Native(FaceAttachmentCurveError::Certificate(
                    PlanarCurveProofFailure::Trim {
                        path: PlanarCurvePath::WorldProjection,
                        location: PlanarAttachmentLocation::HoleInterior,
                    }
                )))
            ));
        }
        assert!(matches!(
            replay(&binding, [[-3., 3.], [5., 3.]]),
            Err(Failure::Native(FaceAttachmentCurveError::Certificate(
                PlanarCurveProofFailure::Trim {
                    location: PlanarAttachmentLocation::OutsideOuter,
                    ..
                }
            )))
        ));
    }
    let binding = proof(5, 0., 11);
    assert!(matches!(
        replay(&binding, [[-2., 0.], [-2., -0.]]),
        Err(Failure::Native(FaceAttachmentCurveError::Certificate(
            PlanarCurveProofFailure::CoincidentControls(_)
        )))
    ));
    // The line one ULP inside the hole remains a strict crossing.
    assert!(matches!(
        replay(
            &binding,
            [[-2., 1.0_f64.next_down()], [2., 1.0_f64.next_down()]]
        ),
        Err(Failure::Native(FaceAttachmentCurveError::Certificate(
            PlanarCurveProofFailure::Trim {
                location: PlanarAttachmentLocation::HoleInterior,
                ..
            }
        )))
    ));
}

#[test]
fn verifies_published_curve_source_and_native_ownership() {
    let binding = proof(5, 0., 11);
    let stored = recipe(&binding, [[-2., 2.], [2., 2.]]);
    let changed = PlanarFaceRevision {
        input_salt: key(12),
        ..stored.source
    };
    let mut checks = 0;
    assert!(matches!(
        binding.replay_straight_curve(stored, || {
            checks += 1;
            if checks == 1 { stored.source } else { changed }
        }, || false),
        Err(Failure::StaleSource { expected, actual }) if expected == stored.source && actual == changed
    ));
    assert_eq!(checks, 2);
    let mut polls = 0;
    assert!(matches!(
        binding.replay_straight_curve(
            stored,
            || stored.source,
            || {
                polls += 1;
                polls == 6
            }
        ),
        Err(Failure::Cancelled)
    ));
    assert!(matches!(
        binding.replay_straight_curve(
            PlanarFaceCurveRecipe {
                distance_limit: 0.,
                ..stored
            },
            || stored.source,
            || false
        ),
        Err(Failure::Native(FaceAttachmentCurveError::Tolerance {
            limit: 0.,
            ..
        }))
    ));
    let curve = binding
        .replay_straight_curve(stored, || stored.source, || false)
        .unwrap();
    let export = curve.to_native().unwrap();
    let world = BRepTool::curve_with_location(&export.edge).unwrap();
    let world = world.curve.downcast::<GeomBezierCurve>();
    world
        .get()
        .unwrap()
        .set_pole(1, Point3::new(64., 64., 64.))
        .unwrap();
    verify_retained_pair(&curve, [[-2., 2., 0.], [2., 2., 0.]]);
}

#[test]
fn independent_processes_replay_the_same_retained_curve_and_proof() {
    let receipt = || {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "on_face::curve::emit_straight_curve_receipt",
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
            .split_once("FACE_CURVE=")
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
fn emit_straight_curve_receipt() {
    let source = converted(0.5, [256., -128., 32.]);
    let curve = replay(&source, [[-2., 1.], [3., 1.]]).unwrap();
    let retained = curve.to_native().unwrap();
    let bytes = crate::wire::archive(&retained.edge);
    let degree = curve.certificate();
    let metric = [degree.distance_bound.to_bits(), curve.tolerance().to_bits()];
    println!(
        "FACE_CURVE={:?} {metric:016x?} {bytes:02x?}",
        curve.recipe()
    );
}

#[test]
fn rotated_original_support_has_positive_certified_rounding_discrepancy() {
    let mut transform = Transform3::default();
    let normal = Direction3::new(0., 0., 1.).unwrap();
    transform
        .set_rotation(Axis1::new(Point3::new(0., 0., 0.), normal), 0.25)
        .unwrap();
    transform.set_translation_part(Vector3::new(256., -128., 32.));
    let binding = placed(0.5, transform);
    let uv = [[-2.1, 2.3], [2.4, 2.7]];
    let curve = replay(&binding, uv).unwrap();
    let report = curve.certificate();
    assert!(report.distance_bound > 0.);
    assert!(report.distance_bound < 1e-12);
    assert_eq!(
        report
            .world_projection
            .map(|endpoint| endpoint.distance_bound),
        [0.; 2]
    );
    let detached = curve.to_native().unwrap();
    let (surface, location) = BRepTool::surface_with_location(&detached.support).unwrap();
    assert_eq!(
        location
            .transformation()
            .checked_unit_placement_rows()
            .unwrap(),
        transform.checked_unit_placement_rows().unwrap(),
    );
    let world = BRepTool::curve(&detached.edge).unwrap().0;
    let world_sample = world.get().unwrap().d0(0.5).unwrap();
    let midpoint: [f64; 2] = std::array::from_fn(|i| (uv[0][i] + uv[1][i]) * 0.5);
    let face_sample = surface.get().unwrap().d0(midpoint[0], midpoint[1]).unwrap();
    assert!(world_sample.distance(&face_sample.transformed(transform)) < 1e-12);
}
