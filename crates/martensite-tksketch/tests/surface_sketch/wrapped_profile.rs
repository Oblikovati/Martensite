//! Consumer-boundary checks use independent closed-form cylindrical measures.
use martensite_tkbo::profile_extrusion::{
    LoopUse, ProfileCurve, ProfileCurveId, ProfileLoop, ProfilePlane, ProfileVertex,
    ProfileVertexId, SketchProfile,
};
use martensite_tksketch::surface_sketch::*;

fn solved_rectangle() -> SketchProfile {
    SketchProfile {
        plane: ProfilePlane::world_xy(),
        vertices: [[0., 0.], [2., 0.], [2., 3.], [0., 3.]]
            .into_iter()
            .enumerate()
            .map(|(i, point)| ProfileVertex {
                id: ProfileVertexId(i as u64 + 1),
                point,
            })
            .collect(),
        curves: (0..4)
            .map(|i| ProfileCurve::Line {
                id: ProfileCurveId(i + 1),
                start: ProfileVertexId(i + 1),
                end: ProfileVertexId((i + 1) % 4 + 1),
            })
            .collect(),
    }
}
fn request() -> WrappedNormalRequest {
    WrappedNormalRequest {
        profile: solved_rectangle(),
        selections: vec![selection()],
        support: super::refusals::rectangle().support,
        depths: [-1., 2.],
        options: WrappedNormalOptions {
            spatial_error: 1e-5,
            maximum_fit_level: 8,
            volume: WrappedVolumePolicy::Refuse(1e-4),
        },
    }
}
fn selection() -> WrappedRegionSelection {
    WrappedRegionSelection {
        region: SurfaceRegionId(71),
        outer: ProfileLoop {
            uses: (1..=4)
                .map(|i| LoopUse {
                    curve: ProfileCurveId(i),
                    reversed: false,
                })
                .collect(),
        },
        holes: vec![],
    }
}

#[test]
fn consumer_receives_closed_form_measure_and_honest_validation_coverage() {
    let result = normal_extrude_profile(&request()).unwrap();
    let report = &result.reports()[&SurfaceRegionId(71)];
    // Area 6 times radial Jacobian integral (12² - 9²)/(2 * 10).
    let expected = 18.9;
    assert!(report.measures.authored_volume.lower <= expected);
    assert!(expected <= report.measures.authored_volume.upper);
    assert!(report.validation.is_valid());
    assert!(!report.validation.is_exact());
    assert!(report.coverage.faces >= 6);
    assert!(report.coverage.cap_pcurves > 0);
    assert!(report.coverage.side_pcurves > 0);
    assert!(report.spatial.cap_trace_error.upper <= result.budget().trace_allowance);
    assert!(report.spatial.cap_angular_error.upper >= 0.);
    assert!(report.spatial.cap_height_error.upper >= 0.);
    assert!(report.spatial.cap_realization_error.upper <= result.options().spatial_error);
    assert!(report.spatial.cap_angular_error.upper <= result.budget().trace_allowance / 10.);
}
#[test]
fn source_mutation_and_independent_native_exports_preserve_publication() {
    let mut input = request();
    let result = normal_extrude_profile(&input).unwrap();
    let bytes = result.encoded_brep().to_vec();
    input.profile.vertices[0].point = [100., 100.];
    input.selections.clear();
    let first = result.export_native().unwrap();
    let second = result.export_native().unwrap();
    assert!(!first.bodies.is_partner(&second.bodies));
    for (key, shape) in &first.entities {
        assert!(!shape.is_partner(&second.entities[key]));
    }
    assert_eq!(result.encoded_brep(), bytes);
    assert_eq!(result.reports().len(), 1);
}
#[test]
fn consumer_cancellation_returns_typed_failure_without_geometry() {
    let failure = normal_extrude_profile_with_cancel(&request(), || true)
        .err()
        .unwrap();
    assert_eq!(failure.operation(), WrappedOperation::NormalExtrusion);
    assert_eq!(failure.class(), WrappedFailureClass::Cancelled);
    assert_eq!(failure.stage(), WrappedStage::Admission);
}

mod replay;

#[test]
fn final_handoff_cancellation_discards_geometry_and_preserves_inputs() {
    let input = request();
    let owned = input.clone();
    let (completed, completed_polls) = completed_with_poll_count(&input);
    let mut cancelled_polls = 0;
    let failure = normal_extrude_profile_with_cancel(&input, || {
        cancelled_polls += 1;
        cancelled_polls == completed_polls
    })
    .err()
    .unwrap();
    assert_eq!(failure.class(), WrappedFailureClass::Cancelled);
    assert_eq!(failure.stage(), WrappedStage::Publication);
    assert_eq!(input, owned);
    assert_eq!(
        normal_extrude_profile(&input).unwrap().encoded_brep(),
        completed.encoded_brep()
    );
}
fn completed_with_poll_count(input: &WrappedNormalRequest) -> (WrappedNormalResult, usize) {
    let mut polls = 0;
    let result = normal_extrude_profile_with_cancel(input, || {
        polls += 1;
        false
    })
    .unwrap();
    (result, polls)
}

#[test]
fn insufficient_volume_allowance_refuses_instead_of_weakening_limits() {
    let mut input = request();
    input.options.volume = WrappedVolumePolicy::Refuse(0.);
    let owned = input.clone();
    let failure = normal_extrude_profile(&input).err().unwrap();
    assert_eq!(failure.class(), WrappedFailureClass::Incomplete);
    assert_eq!(failure.stage(), WrappedStage::Construction);
    assert!(std::error::Error::source(&failure).is_some());
    assert_eq!(input, owned);
    input.options.volume = WrappedVolumePolicy::Report;
    let reported = normal_extrude_profile(&input).unwrap();
    assert!(reported.budget().volume.discrepancy.upper > 0.);
    assert_eq!(reported.options().volume, WrappedVolumePolicy::Report);
}
