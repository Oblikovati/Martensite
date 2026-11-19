//! Chart join refusals keep actual source ownership and real consumed work.
use super::*;
#[test]
fn chart_join_failure_retains_owner_and_charges_actual_work() {
    let region = SurfaceRegionId(3);
    let span = CurveSpan {
        curve: SurfaceCurveId(9),
        level: 5,
        index: 16,
    };
    let cause = ConicalChartJoinsError {
        index: Some(7),
        operations: 23,
        cause: Box::new(ConicalJoinsFailure::Boundary { index: 7 }),
    };
    let mut work = Work::new(ConeClearanceOptions::default());
    let failure = chart_failure(region, Some(span), cause, &mut work);
    assert_eq!(failure.region, Some(region));
    assert_eq!(failure.span, Some(span));
    assert_eq!(work.consumed, 23);
    assert!(matches!(failure.cause, ConeCause::ChartJoins(cause) if cause.index == Some(7)));
}
#[test]
fn chart_join_proof_failure_precedes_accounting_refusal() {
    let cause = ConicalChartJoinsError {
        index: Some(11),
        operations: 2,
        cause: Box::new(ConicalJoinsFailure::Boundary { index: 11 }),
    };
    let mut work = Work::new(ConeClearanceOptions {
        max_operations: 1,
        ..Default::default()
    });
    let failure = chart_failure(SurfaceRegionId(4), None, cause, &mut work);
    assert!(matches!(failure.cause, ConeCause::ChartJoins(cause) if cause.index == Some(11)));
    assert_eq!(work.consumed, 0);
}
