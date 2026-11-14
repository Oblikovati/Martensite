//! Public error projection retains typed admission classes and authored owners (#448).

use super::*;
use crate::surface_sketch::{CylinderSupport, HistorySource, NormalSense};
use martensite_tkbo::profile_extrusion::*;
const OWNER: SurfaceRegionId = SurfaceRegionId(41);
fn profile() -> SketchProfile {
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
        profile: profile(),
        selections: vec![selection()],
        support: support(),
        depths: [-1., 2.],
        options: WrappedNormalOptions {
            spatial_error: 1e-5,
            maximum_fit_level: 8,
            volume: WrappedVolumePolicy::Report,
        },
    }
}
fn support() -> CylinderSupport {
    CylinderSupport {
        origin: [0.; 3],
        axis: [0., 0., 1.],
        radial: [1., 0., 0.],
        radius: 10.,
        chart_origin: 0.,
        normal: NormalSense::Outward,
    }
}
fn selection() -> WrappedRegionSelection {
    WrappedRegionSelection {
        region: OWNER,
        holes: vec![],
        outer: ProfileLoop {
            uses: (1..=4)
                .map(|curve| LoopUse {
                    curve: ProfileCurveId(curve),
                    reversed: false,
                })
                .collect(),
        },
    }
}
fn refused(input: &WrappedNormalRequest) -> WrappedNormalFailure {
    normal_extrude_profile(input).err().unwrap()
}
fn assert_public(
    error: &WrappedNormalFailure,
    class: WrappedFailureClass,
    entities: &[HistorySource],
) {
    assert_eq!(error.operation(), WrappedOperation::NormalExtrusion);
    assert_eq!(error.class(), class);
    assert_eq!(error.entities(), entities);
    assert!(std::error::Error::source(error).is_some());
}
#[test]
fn invalid_radius_retains_admission_class_and_region() {
    let mut input = request();
    input.support.radius = 0.;
    assert_public(
        &refused(&input),
        WrappedFailureClass::InvalidInput,
        &[HistorySource::Region(OWNER)],
    );
}
#[test]
fn out_of_range_fit_level_is_invalid_input_with_an_owner() {
    let mut input = request();
    input.options.maximum_fit_level = 9;
    assert_public(
        &refused(&input),
        WrappedFailureClass::InvalidInput,
        &[HistorySource::Region(OWNER)],
    );
}
#[test]
fn duplicate_region_owner_is_named_once() {
    let mut input = request();
    input.selections.push(selection());
    assert_public(
        &refused(&input),
        WrappedFailureClass::InvalidInput,
        &[HistorySource::Region(OWNER)],
    );
}
#[test]
fn duplicate_curve_preserves_both_regions_and_the_curve() {
    let mut input = request();
    let mut second = selection();
    second.region = SurfaceRegionId(42);
    input.selections.push(second);
    assert_public(
        &refused(&input),
        WrappedFailureClass::InvalidInput,
        &[
            HistorySource::Region(OWNER),
            HistorySource::Region(SurfaceRegionId(42)),
            HistorySource::Curve(SurfaceCurveId(1)),
        ],
    );
}
#[test]
fn region_budget_and_native_failures_keep_their_known_owner() {
    let source =
        family::FamilyFailure::Budget(Box::new(publication_budget::BudgetFailure::Source(OWNER)));
    assert_public(
        &family_failure(source),
        WrappedFailureClass::Incomplete,
        &[HistorySource::Region(OWNER)],
    );
    let native =
        family::FamilyFailure::Budget(Box::new(publication_budget::BudgetFailure::Native {
            region: OWNER,
            stage: martensite_tkbo::body_check::ValidationStage::Geometry,
        }));
    assert_public(
        &family_failure(native),
        WrappedFailureClass::Native,
        &[HistorySource::Region(OWNER)],
    );
}

fn family_failure(cause: family::FamilyFailure) -> WrappedNormalFailure {
    let input = Request {
        region: OWNER,
        loops: vec![],
        radius: 10.,
        depths: [-1., 2.],
        tolerance: 1e-5,
        max_fit_level: 8,
    };
    failure_projection::family(cause, &[input])
}

#[test]
fn unsupported_curve_keeps_its_typed_source_entity() {
    let mut input = request();
    input.profile.curves[0] = ProfileCurve::Circle {
        id: ProfileCurveId(1),
        center: [0., 0.],
        radius: 1.,
    };
    assert_public(
        &refused(&input),
        WrappedFailureClass::Unsupported,
        &[
            HistorySource::Region(OWNER),
            HistorySource::Curve(SurfaceCurveId(1)),
        ],
    );
}
#[test]
fn missing_vertex_keeps_curve_and_vertex_entities() {
    let mut input = request();
    input.profile.curves[0] = ProfileCurve::Line {
        id: ProfileCurveId(1),
        start: ProfileVertexId(99),
        end: ProfileVertexId(2),
    };
    assert_public(
        &refused(&input),
        WrappedFailureClass::InvalidInput,
        &[
            HistorySource::Region(OWNER),
            HistorySource::Curve(SurfaceCurveId(1)),
            HistorySource::Vertex(SurfaceVertexId(99)),
        ],
    );
}
#[test]
fn aggregate_budget_uses_known_selected_owners_without_inventing_an_owner() {
    let mut input = request();
    input.options.volume = WrappedVolumePolicy::Refuse(0.);
    assert_public(
        &refused(&input),
        WrappedFailureClass::Incomplete,
        &[HistorySource::Region(OWNER)],
    );
    let cause =
        family::FamilyFailure::Budget(Box::new(publication_budget::BudgetFailure::Policy {
            field: "volume",
            value: -1.,
        }));
    assert_public(
        &failure_projection::family(cause, &[]),
        WrappedFailureClass::InvalidInput,
        &[],
    );
}
#[test]
fn family_admission_retains_its_owned_cause_after_projection() {
    let mut input = request();
    input.support.radius = 0.;
    let error = refused(&input);
    assert!(
        matches!(*error.cause, error::Cause::Family(family::FamilyFailure::Selection(cause))
        if matches!(*cause, selection::SelectionFailure::Admission { region: OWNER, .. }))
    );
}
#[test]
fn family_cancellation_keeps_its_class_and_known_selection() {
    let mut polls = 0;
    let error = normal_extrude_profile_with_cancel(&request(), || {
        polls += 1;
        polls == 4
    })
    .err()
    .unwrap();
    assert_public(
        &error,
        WrappedFailureClass::Cancelled,
        &[HistorySource::Region(OWNER)],
    );
}
#[test]
fn selected_support_and_depth_admission_keep_all_known_regions() {
    let mut input = request();
    input.support.origin[0] = 1.;
    assert_public(
        &refused(&input),
        WrappedFailureClass::Unsupported,
        &[HistorySource::Region(OWNER)],
    );
    input.support = support();
    input.depths = [1., 0.];
    assert_public(
        &refused(&input),
        WrappedFailureClass::InvalidInput,
        &[HistorySource::Region(OWNER)],
    );
}
#[test]
fn nested_planar_decline_retains_its_public_unsupported_category_and_curve() {
    let selection = unsupported_planar();
    assert_public(
        &family_failure(family::FamilyFailure::Selection(Box::new(selection))),
        WrappedFailureClass::Unsupported,
        &[
            HistorySource::Region(OWNER),
            HistorySource::Curve(SurfaceCurveId(1)),
        ],
    );
}
fn unsupported_planar() -> selection::SelectionFailure {
    let native = OperationDecline {
        operation: OperationKind::BuildPlanarRegions,
        stage: OperationStage::Admission,
        cause: DeclineCause::UnsupportedCurve {
            curve: ProfileCurveId(1),
            kind: "ellipse",
        },
    };
    selection::SelectionFailure::Capture(Box::new(planar::PlanarFailure::Capture(Box::new(native))))
}

#[test]
fn history_clearance_and_coverage_keep_their_semantic_curve_keys() {
    let causes = structural_failures(projection_key());
    for cause in causes {
        assert_public(
            &family_failure(cause),
            WrappedFailureClass::Incomplete,
            &[
                HistorySource::Region(OWNER),
                HistorySource::Curve(SurfaceCurveId(1)),
            ],
        );
    }
}

fn projection_key() -> BindingKey {
    BindingKey {
        source: Source::Span(CurveSpan {
            curve: SurfaceCurveId(1),
            level: 0,
            index: 0,
        }),
        role: crate::surface_sketch::HistoryRole::SideFace,
    }
}
fn structural_failures(key: BindingKey) -> [family::FamilyFailure; 3] {
    [
        family::FamilyFailure::History(Box::new(history::HistoryFailure::DuplicateTarget(key))),
        family::FamilyFailure::Binding(Box::new(
            planar_clearance::ClearanceFailure::DuplicateSource(SurfaceCurveId(1)),
        )),
        coverage_failure(key),
    ]
}
fn coverage_failure(key: BindingKey) -> family::FamilyFailure {
    family::FamilyFailure::Coverage(Box::new(coverage::CoverageFailure {
        owner: coverage::CoverageOwner::Binding(key),
        cause: coverage::CoverageCause::RootBinding,
    }))
}

#[test]
fn shared_vertex_owner_is_preserved_when_curve_ids_are_distinct() {
    let input = shared_vertex_request();
    assert_public(
        &refused(&input),
        WrappedFailureClass::InvalidInput,
        &[
            HistorySource::Region(OWNER),
            HistorySource::Region(SurfaceRegionId(42)),
            HistorySource::Vertex(SurfaceVertexId(1)),
        ],
    );
}

fn shared_vertex_request() -> WrappedNormalRequest {
    let mut input = request();
    input
        .profile
        .curves
        .extend((0..4).map(|i| ProfileCurve::Line {
            id: ProfileCurveId(i + 5),
            start: ProfileVertexId(i + 1),
            end: ProfileVertexId((i + 1) % 4 + 1),
        }));
    let mut second = selection();
    second.region = SurfaceRegionId(42);
    for usage in &mut second.outer.uses {
        usage.curve.0 += 4;
    }
    input.selections.push(second);
    input
}
