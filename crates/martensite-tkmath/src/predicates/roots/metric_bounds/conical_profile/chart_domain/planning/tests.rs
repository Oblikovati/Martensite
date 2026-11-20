// SPDX-License-Identifier: GPL-2.0-only
use super::*;

const WIDE: [[f64; 2]; 3] = [[10., -5.5], [10., -2.], [30., 16.5]];

fn query() -> ConicalChartPlanQuery<'static> {
    ConicalChartPlanQuery {
        authored: &WIDE,
        semi_angle: 0.125,
        chart_origin: 0.25,
        max_level: 8,
        max_charts: 256,
        options: ConeClearanceOptions {
            fractional_bits: 128,
            max_operations: 100_000,
        },
    }
}

#[test]
fn chart_cover_partitions_the_original_and_shares_one_source_and_ledger() {
    let first = plan_conical_subspan_charts(query(), || false).unwrap();
    let second = plan_conical_subspan_charts(query(), || false).unwrap();
    assert!(first.charts.len() > 2);
    assert_eq!(
        first.operations(),
        first
            .charts
            .iter()
            .map(ConicalChartDomainEvidence::operations)
            .sum::<u32>()
    );
    assert_eq!(first.operations(), second.operations());
    assert_eq!(first.charts.len(), second.charts.len());
    let end = first
        .charts
        .iter()
        .zip(&second.charts)
        .fold(0, |start, (a, b)| {
            let end = check_chart(a, start, &first.charts[0]);
            assert_eq!(a.source_restriction(), b.source_restriction());
            assert_eq!(a.fit_center().to_bits(), b.fit_center().to_bits());
            assert_eq!(a.relative_physical_bounds(), b.relative_physical_bounds());
            end
        });
    assert_eq!(end, 1 << query().max_level);
}

fn check_chart(
    a: &ConicalChartDomainEvidence,
    start: u64,
    first: &ConicalChartDomainEvidence,
) -> u64 {
    let (index, level) = a.source_restriction().unwrap();
    let width = 1_u64 << (query().max_level - level);
    assert_eq!(index * width, start);
    assert!(std::sync::Arc::ptr_eq(
        &a.sector.source,
        &first.sector.source
    ));
    assert!(a.contains_original(&WIDE));
    let [lo, hi] = a
        .relative_physical_bounds()
        .map(|bound| bound.binary64_bounds().unwrap());
    assert!(hi[1] - lo[0] < std::f64::consts::PI);
    let center = a.fit_center() - a.chart_origin();
    assert!(hi[1] - center < std::f64::consts::FRAC_PI_2 + 1e-12);
    assert!(center - lo[0] < std::f64::consts::FRAC_PI_2 + 1e-12);
    start + width
}

#[test]
fn pending_siblings_count_towards_the_limit() {
    let mut input = query();
    input.max_charts = 2;
    let Err(error) = plan_conical_subspan_charts(input, || false) else {
        panic!("expected the pending sibling to exhaust capacity");
    };
    assert!(matches!(
        error.cause,
        ConicalChartPlanFailure::Capacity {
            minimum: 3,
            limit: 2
        }
    ));
    assert!((1..input.options.max_operations).contains(&error.operations));
}

#[test]
fn planner_preserves_depth_exhaustion_and_cancellation() {
    let mut input = query();
    input.max_level = 0;
    let Err(depth) = plan_conical_subspan_charts(input, || false) else {
        panic!("expected maximum chart depth to refuse");
    };
    assert!(matches!(
        depth.cause,
        ConicalChartPlanFailure::Level { index: 0, level: 0 }
    ));
    assert!(!depth.is_cancelled());
    let Err(error) = plan_conical_subspan_charts(query(), || true) else {
        panic!("expected cancellation to stop chart planning");
    };
    assert!(error.is_cancelled());
    let ConicalChartPlanFailure::Domain(cause) = error.cause else {
        panic!("expected domain arithmetic cancellation");
    };
    assert!(domain_error(cause, error.operations).is_cancelled());
}
