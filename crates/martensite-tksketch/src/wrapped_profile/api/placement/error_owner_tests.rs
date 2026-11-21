//! Existing budget causes already carry exact owners; retain those identities (#448).
use super::*;
use crate::surface_sketch::HistorySource;

fn mapped(cause: publication_budget::BudgetFailure) -> WrappedNormalFailure {
    fail(PlacementFailure::Budget(cause))
}
fn assert_context(failure: &WrappedNormalFailure, entities: &[HistorySource]) {
    assert_eq!(failure.stage(), WrappedStage::Placement);
    assert_eq!(failure.class(), WrappedFailureClass::Incomplete);
    assert_eq!(failure.entities(), entities);
}

#[test]
fn native_validation_budget_cause_retains_its_actual_region() {
    let region = SurfaceRegionId(41);
    let failure = mapped(publication_budget::BudgetFailure::Native {
        region,
        stage: martensite_tkbo::body_check::ValidationStage::Geometry,
    });
    assert_context(&failure, &[HistorySource::Region(region)]);
    assert!(
        matches!(failure.cause.as_ref(), super::super::super::error::Cause::Placement(cause)
        if matches!(cause.as_ref(), PlacementFailure::Budget(publication_budget::BudgetFailure::Native { region: actual, .. }) if *actual == region))
    );
}

#[test]
fn member_budget_limit_retains_region_and_curve() {
    let region = SurfaceRegionId(41);
    let curve = SurfaceCurveId(17);
    let failure = mapped(publication_budget::BudgetFailure::Limit {
        quantity: publication_budget::Quantity::Trace,
        actual: 2.,
        limit: 1.,
        region: Some(region),
        span: Some(CurveSpan {
            curve,
            level: 0,
            index: 0,
        }),
    });
    assert_context(
        &failure,
        &[HistorySource::Region(region), HistorySource::Curve(curve)],
    );
}

#[test]
fn global_budget_limit_keeps_no_region_or_curve() {
    let failure = mapped(publication_budget::BudgetFailure::Limit {
        quantity: publication_budget::Quantity::WorldSpatial,
        actual: 2.,
        limit: 1.,
        region: None,
        span: None,
    });
    assert_context(&failure, &[]);
}
