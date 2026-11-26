// SPDX-License-Identifier: GPL-2.0-only
//! Literal fit stations, actual checked native paired curves and a closed prism with an open hole.
use super::{captured, key, native, proof};
use crate::fixture::{pid, point, seed};
use martensite_tkbrep::{
    brep::{
        BRepTool,
        planar_attachment::{
            FaceAttachmentCubicError, PlanarAttachmentLocation, PlanarCubicOptions,
            PlanarCubicProofFailure, PlanarCurvePath, PlanarMonotoneAxis,
        },
    },
    topods,
};
use martensite_tkg3d::geom::{GeomBezierCurve, GeomCurve};
use martensite_tkmath::{
    gp::{Axis1, Direction3, Point3, Transform3, Vector2, Vector3},
    toploc::Location,
};
use martensite_tksketch3d::{
    ControlSplineProofPolicy, Edit, EditFailure, FitSplineFailure, FitSplineId,
    FitSplineParameters, FitSplinePolicy, MinimumLength, PointRole, Snapshot,
    constraints::{
        PlanarFaceBinding, PlanarFaceRevision, PlanarFaceSpline, PlanarFaceSplineEdit,
        PlanarFaceSplineFailure as Failure, PlanarFaceSplineRecipe,
        PlanarFaceSplineStation as Station,
    },
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

const PARAMETERS: [f64; 4] = [0., 0.25, 0.75, 1.];

fn analytic(t: f64, minimum_y: f64) -> [f64; 2] {
    let offset = t - 0.5;
    [-3. + 6. * t, minimum_y + 6. * offset * offset]
}

fn recipe(binding: &PlanarFaceBinding, minimum_y: f64) -> PlanarFaceSplineRecipe {
    PlanarFaceSplineRecipe {
        source: binding.revision(),
        spline: FitSplineId::new(key(20)),
        stations: std::array::from_fn(|i| Station::Create {
            point: pid([2, 7, 5, 9][i]),
            uv: analytic(PARAMETERS[i], minimum_y),
        }),
        parameters: FitSplineParameters::new(PARAMETERS).unwrap(),
        fit_policy: FitSplinePolicy {
            max_distortion: 8.,
            max_inverse_gain: 32.,
            max_station_identity_error: 1e-12,
            max_station_miss: MinimumLength::new(1e-10).unwrap(),
            regularity: ControlSplineProofPolicy::new(1e-8, 128).unwrap(),
        },
        native_options: PlanarCubicOptions {
            distance_limit: 1e-6,
            max_subdivisions: 16,
        },
    }
}

fn empty() -> Snapshot {
    Snapshot::empty(MinimumLength::new(1e-3).unwrap())
}

fn create(binding: &PlanarFaceBinding, minimum_y: f64) -> PlanarFaceSplineEdit {
    binding
        .create_fitted_curve(
            &empty(),
            recipe(binding, minimum_y),
            || binding.revision(),
            || false,
        )
        .unwrap()
}

fn assert_near<const N: usize>(actual: [f64; N], expected: [f64; N]) {
    for (a, b) in actual.into_iter().zip(expected) {
        assert!((a - b).abs() <= 2e-12, "{actual:?} vs {expected:?}");
    }
}

#[test]
fn actual_native_cubic_clears_the_hole_even_when_its_control_hull_does_not() {
    let binding = proof(5, 0., 9);
    let result = create(&binding, 1.25);
    let spline = result
        .snapshot
        .fit_spline(result.curve.recipe().spline)
        .unwrap();
    assert_eq!(spline.recipe().endpoints(), [pid(2), pid(9)]);
    assert_eq!(spline.recipe().stations, [pid(2), pid(7), pid(5), pid(9)]);
    assert_eq!(
        *result
            .snapshot
            .point(pid(9))
            .unwrap()
            .position
            .coordinates(),
        [3., 2.75, 0.]
    );
    assert_eq!(result.curve.certificate().subdivisions, [1, 1]);
    assert_eq!(
        result.curve.certificate().monotone,
        PlanarMonotoneAxis::IncreasingU
    );
    assert_eq!(result.curve.certificate().distance_bound, 0.);
    let native = result.curve.to_native().unwrap();
    let world = BRepTool::curve_with_location(&native.edge).unwrap();
    let uv = BRepTool::curve_on_face(&native.edge, &native.support).unwrap();
    assert!(uv.is_stored);
    assert_eq!(
        [world.first, world.last, uv.first, uv.last],
        [0., 1., 0., 1.]
    );
    let spatial = world.curve.downcast::<GeomBezierCurve>();
    let spatial = spatial.get().unwrap();
    assert_eq!(spatial.degree(), 3);
    assert_eq!(
        uv.pcurve.dynamic_type().unwrap().name(),
        "Geom2d_BezierCurve"
    );
    for (index, point) in spline.poles().into_iter().enumerate() {
        assert_eq!(
            spatial.pole(i32::try_from(index + 1).unwrap()).unwrap(),
            point
        );
    }
    assert!(spatial.pole(2).unwrap().y() < 1.);
    for t in [0., 0.25, 0.5, 0.75, 1.] {
        let [x, y] = analytic(t, 1.25);
        assert_near(*spatial.d0(t).unwrap().coordinates(), [x, y, 0.]);
        assert_near(
            *uv.pcurve.get().unwrap().d0(t).unwrap().xy().coordinates(),
            [x, y],
        );
    }
    assert!(
        BRepCheckAnalyzer::new(&native.edge, true, false, true)
            .unwrap()
            .is_valid()
            .unwrap()
    );
}

#[test]
fn a_hole_crossing_between_four_valid_stations_refuses_the_entire_edit() {
    let binding = proof(5, 0., 9);
    let snapshot = empty();
    let recipe = recipe(&binding, 0.75);
    assert!(matches!(
        binding.create_fitted_curve(&snapshot, recipe, || binding.revision(), || false),
        Err(Failure::Native(FaceAttachmentCubicError::Proof(
            PlanarCubicProofFailure::Trim {
                path: PlanarCurvePath::WorldProjection,
                location: PlanarAttachmentLocation::HoleInterior,
                parameter: 0.5,
            }
        )))
    ));
    assert!(snapshot.fit_spline(recipe.spline).is_none());
    assert!(snapshot.point(pid(9)).is_none());
}

#[test]
fn exact_work_exhaustion_and_adjoining_duplicate_stations_are_typed() {
    let binding = proof(5, 0., 9);
    let source = empty();
    let mut input = recipe(&binding, 1.25);
    input.native_options.max_subdivisions = 0;
    assert!(matches!(
        binding.create_fitted_curve(&source, input, || input.source, || false),
        Err(Failure::Native(FaceAttachmentCubicError::Proof(
            PlanarCubicProofFailure::Incomplete {
                parameter: [0., 1.],
                subdivisions: 0,
                ..
            }
        )))
    ));
    input.stations[1] = Station::Create {
        point: pid(7),
        uv: analytic(0., 1.25),
    };
    assert!(matches!(
        binding.create_fitted_curve(&source, input, || input.source, || false),
        Err(Failure::Edit(EditFailure::FitSpline { source: FitSplineFailure::CoincidentStations([a,b]), .. }))
            if [a,b] == [pid(2),pid(7)]
    ));
    assert_eq!(source, empty());
}

#[test]
fn explicit_reuse_keeps_the_owned_point_and_replay_rejects_moved_geometry() {
    let binding = proof(5, 0., 9);
    let mut input = recipe(&binding, 1.25);
    let uv = analytic(0., 1.25);
    input.stations[0] = Station::Reuse { point: pid(2), uv };
    assert!(matches!(
        binding.create_fitted_curve(&empty(), input, || input.source, || false),
        Err(Failure::MissingPoint(id)) if id == pid(2)
    ));
    let source = empty()
        .apply(
            &[seed(2, [uv[0], uv[1], 0.], PointRole::SketchMarker)],
            || false,
        )
        .unwrap();
    let result = binding
        .create_fitted_curve(&source, input, || input.source, || false)
        .unwrap();
    assert_eq!(result.snapshot.point(pid(2)), source.point(pid(2)));
    let replay = binding
        .replay_fitted_curve(&result.snapshot, input, || input.source, || false)
        .unwrap();
    assert_eq!(result.curve.certificate(), replay.certificate());
    let moved = result
        .snapshot
        .apply(
            &[Edit::MovePoint {
                point: pid(5),
                position: point([1.5, 1.75, 0.]),
            }],
            || false,
        )
        .unwrap();
    assert!(matches!(
        binding.replay_fitted_curve(&moved, input, || input.source, || false),
        Err(Failure::PointChanged { point, .. }) if point == pid(5)
    ));
}

#[test]
fn both_native_exports_are_mutation_isolated_and_final_source_is_checked() {
    let binding = proof(5, 0., 9);
    let input = recipe(&binding, 1.25);
    let stale = PlanarFaceRevision {
        input_salt: key(10),
        ..input.source
    };
    let mut checks = 0;
    assert!(matches!(
        binding.create_fitted_curve(&empty(), input, || {
            checks += 1;
            if checks == 1 { input.source } else { stale }
        }, || false), Err(Failure::StaleSource { actual, .. }) if actual == stale
    ));
    let result = create(&binding, 1.25);
    let export = result.curve.to_native().unwrap();
    let untouched = result.curve.to_native().unwrap();
    let expected = crate::wire::archive(&untouched.edge);
    BRepTool::curve(&export.edge)
        .unwrap()
        .0
        .downcast::<GeomBezierCurve>()
        .get()
        .unwrap()
        .set_pole(2, Point3::new(32., 32., 32.))
        .unwrap();
    let pcurve = BRepTool::curve_on_face(&export.edge, &export.support).unwrap();
    pcurve
        .pcurve
        .get()
        .unwrap()
        .translate(Vector2::new(4., 0.))
        .unwrap();
    assert_eq!(crate::wire::archive(&untouched.edge), expected);
    assert_eq!(
        crate::wire::archive(&result.curve.to_native().unwrap().edge),
        expected
    );
}

#[test]
fn original_located_chart_reports_positive_pair_error_without_losing_source() {
    let (body, face) = native(0.);
    let mut transform = Transform3::default();
    transform
        .set_rotation(Axis1::new(Point3::default(), Direction3::Z), 0.25)
        .unwrap();
    transform.set_translation_part(Vector3::new(256., -128., 32.));
    let location = Location::new(transform);
    let body = topods::solid(&body.moved(&location, false).unwrap()).unwrap();
    let face = topods::face(&face.moved(&location, false).unwrap()).unwrap();
    let binding = captured(&body, &face, 7, 11);
    let result = create(&binding, 1.25);
    let error = result.curve.certificate().distance_bound;
    assert!(error > 0. && error < 1e-12);
    let curve = result.curve.to_native().unwrap();
    let (_, placement) = BRepTool::surface_with_location(&curve.support).unwrap();
    assert_eq!(
        placement.transformation().checked_unit_placement_rows(),
        transform.checked_unit_placement_rows()
    );
}

#[test]
fn cancellation_during_the_last_host_source_read_prevents_publication() {
    use std::cell::Cell;
    let binding = proof(5, 0., 9);
    let input = recipe(&binding, 1.25);
    let reads = Cell::new(0);
    let source = empty();
    assert!(matches!(
        binding.create_fitted_curve(
            &source,
            input,
            || {
                reads.set(reads.get() + 1);
                input.source
            },
            || reads.get() == 2
        ),
        Err(Failure::Cancelled)
    ));
    assert_eq!(reads.get(), 2);
    assert_eq!(source, empty());
}

#[test]
fn point_and_pcurve_exports_replay_across_workers_and_fresh_processes() {
    let binding = proof(5, 0., 9);
    let result = create(&binding, 1.25);
    let expected = crate::wire::archive(&result.curve.to_native().unwrap().edge);
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                for _ in 0..4 {
                    assert_eq!(
                        crate::wire::archive(&result.curve.to_native().unwrap().edge),
                        expected
                    );
                }
            });
        }
    });
    let receipt = || {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "on_face::spline::emit_native_fit_receipt",
                "--nocapture",
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        let text = String::from_utf8(output.stdout).unwrap();
        text.lines()
            .find(|line| line.starts_with("FACE_SPLINE="))
            .unwrap()
            .to_owned()
    };
    assert_eq!(receipt(), receipt());
}

#[test]
fn emit_native_fit_receipt() {
    fn shared<T: Send + Sync>() {}
    shared::<PlanarFaceSpline>();
    shared::<PlanarFaceSplineRecipe>();
    shared::<PlanarFaceSplineEdit>();
    shared::<Failure>();
    let binding = proof(5, 0., 9);
    let result = create(&binding, 1.25);
    let source = result.curve.recipe();
    let proof = result.curve.certificate();
    let bytes = crate::wire::archive(&result.curve.to_native().unwrap().edge);
    println!("FACE_SPLINE={source:?} {proof:?} {bytes:02x?}");
}
