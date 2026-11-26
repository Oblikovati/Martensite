// SPDX-License-Identifier: GPL-2.0-only
//! Original three literal stations on the actual selected block top and the E40 holed control.
use super::*;
use crate::fixture::{lid, line, pid, point, seed};
use martensite_tkbrep::brep::planar_attachment::{
    FaceAttachmentCubicError, FaceAttachmentCurveError, PlanarCubicOptions,
    PlanarCubicProofFailure, PlanarCurvePath, PlanarMonotoneAxis, capture_planar_split_domain,
};
use martensite_tkg3d::geom::{GeomBezierCurve, GeomCurve};
use martensite_tkprim::brep_prim_api::BRepPrimAPIMakeSweep;
use martensite_tksketch3d::{
    ControlSplineProofPolicy, CurveId, CurveRole, Edit, EditFailure, EntityId, FitSplineFailure,
    FitSplineId, FitSplineParameters, FitSplinePolicy, FitSplineQuantity, MinimumLength,
    SketchFitSpline,
    constraints::{
        ConstraintFailure, ContinuityFailure, CurvatureTolerance, LineFitCubicJoin,
        PlanarFaceQuadraticSplineEdit, PlanarFaceQuadraticSplineRecipe,
        PlanarFaceSplineFailure as Failure, PlanarFaceSplineStation as Station,
    },
};
use martensite_tktopalgo::brep_check::BRepCheckAnalyzer;

const PARAMETERS: [f64; 3] = [0., 0.5, 1.];
const STATIONS: [u64; 3] = [2, 7, 11];

pub(super) fn source_recipe(
    binding: &PlanarFaceBinding,
    samples: [[f64; 2]; 3],
) -> PlanarFaceQuadraticSplineRecipe {
    PlanarFaceQuadraticSplineRecipe {
        source: binding.revision(),
        spline: FitSplineId::new(key(20)),
        stations: std::array::from_fn(|i| Station::Create {
            point: pid(STATIONS[i]),
            uv: samples[i],
        }),
        fit_policy: FitSplinePolicy {
            max_distortion: 8.,
            max_inverse_gain: 3.,
            max_station_identity_error: 0.,
            max_station_miss: MinimumLength::new(1e-10).unwrap(),
            regularity: ControlSplineProofPolicy::new(1e-8, 128).unwrap(),
        },
        native_options: PlanarCubicOptions {
            distance_limit: 1e-6,
            max_subdivisions: 16,
        },
    }
}

fn top() -> PlanarFaceBinding {
    let builder =
        BRepBuilderAPIMakeFace::from_plane_wire(Plane3::default(), &square(0., 4., false), true)
            .unwrap();
    let mut prism = BRepPrimAPIMakePrism::new(
        &builder.face().unwrap(),
        Vector3::new(0., 0., 2.),
        false,
        true,
    )
    .unwrap();
    let top = prism.last_shape().unwrap();
    let body = topods::solid(prism.shape().unwrap()).unwrap();
    let top = Explorer::new(&body, ShapeEnum::Face, ShapeEnum::Shape)
        .unwrap()
        .find(|face| face.is_same(&top))
        .unwrap();
    let top = topods::face(&top).unwrap();
    let domain = capture_planar_split_domain(
        &body,
        &top,
        AttachmentSource {
            document: [2; 16],
            body: key(4),
            face: key(6),
            incarnation: 3,
            revision: 11,
        },
        FaceCaptureLimits::bounded(),
        || false,
    )
    .unwrap();
    PlanarFaceBinding::new(domain, key(19))
}

fn empty() -> Snapshot {
    Snapshot::empty(MinimumLength::new(1e-3).unwrap())
}

fn lifted(t: f64, minimum: f64) -> [f64; 2] {
    let dt = t - 0.5;
    [-3. + 6. * t, minimum + 6. * dt * dt]
}

fn create(
    binding: &PlanarFaceBinding,
    recipe: PlanarFaceQuadraticSplineRecipe,
) -> PlanarFaceQuadraticSplineEdit {
    binding
        .create_three_station_curve(&empty(), recipe, || recipe.source, || false)
        .unwrap()
}

#[test]
fn three_actual_top_face_clicks_create_one_degree_two_edge_and_three_owned_points() {
    let binding = top();
    let uv = PARAMETERS.map(|t| lifted(t, 1.25));
    let recipe = source_recipe(&binding, uv);
    let result = create(&binding, recipe);
    let fit = result.snapshot.quadratic_fit_spline(recipe.spline).unwrap();
    assert!(result.snapshot.fit_spline(recipe.spline).is_none());
    assert_eq!(
        result.snapshot.owned_ids().collect::<Vec<_>>(),
        [
            EntityId::Point(pid(2)),
            EntityId::Point(pid(7)),
            EntityId::Point(pid(11)),
            EntityId::FitSpline(recipe.spline),
        ]
    );
    assert_eq!(fit.recipe().stations, STATIONS.map(pid));
    assert_eq!(fit.recipe().endpoints(), [pid(2), pid(11)]);
    assert_eq!(fit.poles()[1], point([0., -0.25, 2.]));
    assert_eq!(fit.station_miss_upper(), [0.; 3]);
    assert_eq!(result.curve.uv_station_miss_upper(), [0.; 3]);
    assert_eq!(fit.operator_report().inverse_gain_upper, 3.);
    assert_eq!(result.curve.certificate().distance_bound, 0.);
    let native = result.curve.to_native().unwrap();
    let world = BRepTool::curve_with_location(&native.edge).unwrap();
    let uv = BRepTool::curve_on_face(&native.edge, &native.support).unwrap();
    assert!(uv.is_stored);
    assert_eq!(
        [world.first, world.last, uv.first, uv.last],
        [0., 1., 0., 1.]
    );
    let world = world.curve.downcast::<GeomBezierCurve>();
    assert_eq!(world.get().unwrap().degree(), 2);
    for t in [0., 0.25, 0.5, 0.75, 1.] {
        let [x, y] = lifted(t, 1.25);
        assert_eq!(fit.value(t).unwrap(), point([x, y, 2.]));
        assert_eq!(world.get().unwrap().d0(t).unwrap(), point([x, y, 2.]));
        assert_eq!(
            *uv.pcurve.get().unwrap().d0(t).unwrap().xy().coordinates(),
            [x, y]
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
fn actual_world_and_chart_midpoint_misses_retain_a_sub_ulp_residual() {
    let binding = proof(5, 0., 9);
    let raised = 2.0_f64.next_up();
    let miss = (raised - 2.) / 4.;
    let mut input = source_recipe(&binding, [[-3., 2.], [0., raised], [3., raised]]);
    input.fit_policy.max_station_miss = MinimumLength::new(miss).unwrap();
    let result = create(&binding, input);
    let fit = result.snapshot.quadratic_fit_spline(input.spline).unwrap();
    assert_eq!(fit.poles()[1].y(), raised.next_up());
    assert_eq!(fit.station_miss_upper(), [0., miss, 0.]);
    assert_eq!(result.curve.uv_station_miss_upper(), [0., miss, 0.]);

    input.fit_policy.max_station_miss = MinimumLength::new(miss.next_down()).unwrap();
    assert!(matches!(
        binding.create_three_station_curve(&empty(), input, || input.source, || false),
        Err(Failure::Edit(EditFailure::QuadraticFitSpline {
            source: FitSplineFailure::Limit { quantity: FitSplineQuantity::StationMiss(1), upper, .. }, ..
        })) if upper == miss
    ));
}

#[test]
fn exact_rim_tangency_reverses_material_and_authored_endpoint_identity() {
    let binding = proof(5, 0., 9);
    let mut recipe = source_recipe(&binding, PARAMETERS.map(|t| lifted(t, 1.)));
    let first = create(&binding, recipe);
    assert_eq!(first.curve.certificate().subdivisions, [1, 1]);
    assert_eq!(
        first.curve.certificate().monotone,
        PlanarMonotoneAxis::IncreasingU
    );
    recipe.stations.reverse();
    let reversed = create(&binding, recipe);
    assert_eq!(
        reversed.curve.certificate().monotone,
        PlanarMonotoneAxis::DecreasingU
    );
    let curve = reversed
        .snapshot
        .quadratic_fit_spline(recipe.spline)
        .unwrap();
    assert_eq!(curve.recipe().endpoints(), [pid(11), pid(2)]);
    assert_eq!(curve.value(0.5).unwrap(), point([0., 1., 0.]));
    let original = first.snapshot.quadratic_fit_spline(recipe.spline).unwrap();
    let [start, middle, end] = original.poles();
    assert_eq!(curve.poles(), [end, middle, start]);
}

#[test]
fn three_valid_samples_still_refuse_the_actual_off_center_hole_entry() {
    let binding = proof(5, 0., 9);
    let samples = PARAMETERS.map(|t| {
        let delta = t - 0.375;
        [-3. + 6. * t, 31. / 32. + 4. * delta * delta]
    });
    let recipe = source_recipe(&binding, samples);
    let source = empty();
    assert!(matches!(
        binding.create_three_station_curve(&source, recipe, || recipe.source, || false),
        Err(Failure::Native(FaceAttachmentCubicError::Proof(
            PlanarCubicProofFailure::Trim {
                path: PlanarCurvePath::WorldProjection,
                location: PlanarAttachmentLocation::HoleInterior,
                parameter: 0.375,
            }
        )))
    ));
    assert_eq!(source.owned_ids().count(), 0);
}

#[test]
fn zero_distance_zero_chord_duplicate_id_and_zero_work_are_independently_typed() {
    let binding = proof(5, 0., 9);
    let source = empty();
    let original = source_recipe(&binding, PARAMETERS.map(|t| lifted(t, 1.25)));
    let mut input = original;
    input.native_options.max_subdivisions = 0;
    let refuse = |input| {
        binding
            .create_three_station_curve(&source, input, || original.source, || false)
            .unwrap_err()
    };
    assert!(matches!(
        refuse(input),
        Failure::Native(FaceAttachmentCubicError::Proof(
            PlanarCubicProofFailure::Incomplete { .. }
        ))
    ));
    let mut input = original;
    input.native_options.distance_limit = 0.;
    assert!(matches!(
        refuse(input),
        Failure::Native(FaceAttachmentCubicError::Native(
            FaceAttachmentCurveError::Tolerance { limit: 0., .. }
        ))
    ));
    let mut input = original;
    input.stations[1] = Station::Create {
        point: pid(7),
        uv: lifted(0., 1.25),
    };
    assert!(matches!(refuse(input),
        Failure::Edit(EditFailure::QuadraticFitSpline { source: FitSplineFailure::CoincidentStations([a,b]), .. })
            if [a,b] == [pid(2),pid(7)]
    ));
    input.stations[1] = input.stations[0];
    assert!(matches!(
        refuse(input),
        Failure::Edit(EditFailure::DuplicateIdentity { .. })
    ));
    let mut input = original;
    input.fit_policy.max_inverse_gain = 3.0_f64.next_down();
    assert!(matches!(
        refuse(input),
        Failure::Edit(EditFailure::QuadraticFitSpline {
            source: FitSplineFailure::Limit {
                quantity: FitSplineQuantity::InverseGain,
                upper: 3.,
                ..
            },
            ..
        })
    ));
}

#[test]
fn legacy_continuity_refuses_quadratic_and_editing_keeps_the_same_ordinary_identity() {
    let binding = proof(5, 0., 9);
    let input = source_recipe(&binding, PARAMETERS.map(|t| lifted(t, 1.25)));
    let result = create(&binding, input);
    let mut curves = result
        .snapshot
        .apply(
            &[
                seed(30, [-4., 2.75, 0.], PointRole::Defining),
                line(31, 30, 2),
                Edit::SetCurveRole {
                    curve: CurveId::FitSpline(input.spline),
                    role: CurveRole::Construction,
                },
            ],
            || false,
        )
        .unwrap();
    assert_eq!(
        curves.curve_role(CurveId::FitSpline(input.spline)),
        Some(CurveRole::Construction)
    );
    let constrain = ConstrainedSketch::new(curves.clone(), policy()).unwrap();
    let join = LineFitCubicJoin {
        line: lid(31),
        fit: input.spline,
        endpoint: pid(2),
    };
    for relation in [
        RelationSpec::Tangent(join.into()),
        RelationSpec::G2(join.into(), CurvatureTolerance::new(1e-8).unwrap()),
    ] {
        assert!(
            matches!(constrain.apply(&[RelationEdit::Insert(rid(41), relation)], || false),
                Err(ConstraintFailure::Continuity(_, ContinuityFailure::QuadraticFitUnsupported(id))) if id == input.spline
            )
        );
    }
    curves = curves
        .apply(
            &[Edit::MovePoint {
                point: pid(7),
                position: point([0., 1.5, 0.]),
            }],
            || false,
        )
        .unwrap();
    let moved = curves.quadratic_fit_spline(input.spline).unwrap();
    assert_eq!(moved.value(0.5).unwrap(), point([0., 1.5, 0.]));
    assert!(
        matches!(binding.replay_three_station_curve(&curves, input, || input.source, || false),
            Err(Failure::PointChanged { point, .. }) if point == pid(7)
        )
    );
    let removed = curves
        .apply(&[Edit::RemovePoint(pid(7))], || false)
        .unwrap();
    assert!(removed.quadratic_fit_spline(input.spline).is_none());
    assert!(removed.point(pid(11)).is_none());
    assert!(removed.point(pid(2)).is_some());
}

#[test]
fn a_shared_fit_identity_cannot_replay_or_replace_a_four_station_owner() {
    let binding = proof(5, 0., 9);
    let input = source_recipe(&binding, PARAMETERS.map(|t| lifted(t, 1.25)));
    let mut legacy_policy = input.fit_policy;
    legacy_policy.max_inverse_gain = 32.;
    legacy_policy.max_station_identity_error = 1e-12;
    let fit = SketchFitSpline {
        id: input.spline,
        stations: [40, 41, 42, 43].map(pid),
        parameters: FitSplineParameters::new([0., 0.25, 0.75, 1.]).unwrap(),
        policy: legacy_policy,
    };
    let mut edits = vec![Edit::InsertFitSpline(fit)];
    for (id, t) in [40, 41, 42, 43].into_iter().zip([0., 0.25, 0.75, 1.]) {
        let [x, y] = lifted(t, 1.25);
        edits.push(seed(id, [x, y, 0.], PointRole::Defining));
    }
    let legacy = empty().apply(&edits, || false).unwrap();
    assert!(legacy.quadratic_fit_spline(input.spline).is_none());
    assert_eq!(legacy.fit_spline(input.spline).unwrap().recipe(), fit);
    assert!(
        matches!(binding.replay_three_station_curve(&legacy, input, || input.source, || false),
            Err(Failure::SplineRecipe(id)) if id == input.spline
        )
    );
    assert!(
        matches!(binding.create_three_station_curve(&legacy, input, || input.source, || false),
            Err(Failure::Edit(EditFailure::DuplicateIdentity {
                existing: EntityId::FitSpline(a), attempted: EntityId::FitSpline(b)
            })) if a == input.spline && b == input.spline
        )
    );
}

#[test]
fn a_reused_station_and_last_source_or_cancel_check_are_explicit() {
    use std::cell::Cell;
    let binding = proof(5, 0., 9);
    let mut input = source_recipe(&binding, PARAMETERS.map(|t| lifted(t, 1.25)));
    let uv = lifted(0., 1.25);
    input.stations[0] = Station::Reuse { point: pid(2), uv };
    assert!(
        matches!(binding.create_three_station_curve(&empty(), input, || input.source, || false),
            Err(Failure::MissingPoint(id)) if id == pid(2)
        )
    );
    let source = empty()
        .apply(
            &[seed(2, [uv[0], uv[1], 0.], PointRole::SketchMarker)],
            || false,
        )
        .unwrap();
    let reads = Cell::new(0);
    let result = binding.create_three_station_curve(
        &source,
        input,
        || {
            reads.set(reads.get() + 1);
            input.source
        },
        || reads.get() == 2,
    );
    assert!(matches!(result, Err(Failure::Cancelled)));
    let success = binding
        .create_three_station_curve(&source, input, || input.source, || false)
        .unwrap();
    assert_eq!(success.snapshot.point(pid(2)), source.point(pid(2)));
    let replayed = binding
        .replay_three_station_curve(&success.snapshot, input, || input.source, || false)
        .unwrap();
    assert_eq!(success.curve.certificate(), replayed.certificate());
    assert_eq!(
        crate::wire::archive(&success.curve.to_native().unwrap().edge),
        crate::wire::archive(&replayed.to_native().unwrap().edge)
    );
}

#[test]
fn emit_three_station_native_receipt() {
    let binding = proof(5, 0., 9);
    let input = source_recipe(&binding, PARAMETERS.map(|t| lifted(t, 1.25)));
    let result = create(&binding, input);
    let encoded = crate::wire::archive(&result.curve.to_native().unwrap().edge);
    println!(
        "THREE_STATION={:?} {:?} {encoded:02x?}",
        result.curve.recipe(),
        result.curve.certificate()
    );
}

#[test]
fn native_quadratic_exports_are_independent_across_threads_and_processes() {
    fn shared<T: Send + Sync>() {}
    shared::<PlanarFaceQuadraticSplineEdit>();
    let binding = proof(5, 0., 9);
    let result = create(
        &binding,
        source_recipe(&binding, PARAMETERS.map(|t| lifted(t, 1.25))),
    );
    let expected = crate::wire::archive(&result.curve.to_native().unwrap().edge);
    std::thread::scope(|scope| {
        for _ in 0..4 {
            scope.spawn(|| {
                assert_eq!(
                    crate::wire::archive(&result.curve.to_native().unwrap().edge),
                    expected
                );
            });
        }
    });
    let run = || {
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "on_face::three_station::emit_three_station_native_receipt",
                "--nocapture",
            ])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout)
            .unwrap()
            .lines()
            .find(|line| line.starts_with("THREE_STATION="))
            .unwrap()
            .to_owned()
    };
    assert_eq!(run(), run());
}

#[test]
fn three_distinct_collinear_stations_keep_their_quadratic_identity() {
    let binding = proof(5, 0., 9);
    let input = source_recipe(&binding, [[-3., 2.], [0., 2.], [3., 2.]]);
    let result = create(&binding, input);
    let fit = result.snapshot.quadratic_fit_spline(input.spline).unwrap();
    assert_eq!(fit.stations(), fit.poles());
    assert_eq!(fit.value(0.25).unwrap(), point([-1.5, 2., 0.]));
    let native = result.curve.to_native().unwrap();
    let spatial = BRepTool::curve(&native.edge)
        .unwrap()
        .0
        .downcast::<GeomBezierCurve>();
    assert_eq!(spatial.get().unwrap().degree(), 2);
}
