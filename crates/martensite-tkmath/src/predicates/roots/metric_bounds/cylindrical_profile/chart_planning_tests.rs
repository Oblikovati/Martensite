// SPDX-License-Identifier: GPL-2.0-only
use super::*;

fn query(authored: &[[f64; 2]]) -> CylindricalFitQuery<'_> {
    CylindricalFitQuery {
        authored,
        interval: CylindricalProfileInterval::new(0, 0).unwrap(),
        support_radius: 1.,
        angular_origin: 0.,
        spatial_limit: 1e-5,
        max_level: 12,
        max_operations: 100_000,
    }
}

#[test]
fn long_curve_gets_complete_conditioned_charts_before_fitting() {
    let input = query(&[[0., 0.], [6., 1.]]);
    assert!(plan_cylindrical_fit(input, || false).is_err());
    let plan = plan_cylindrical_charts(input, || false).unwrap();
    assert_eq!(plan.charts().len(), 8);
    for (index, chart) in plan.charts().iter().enumerate() {
        assert_linear_chart(chart, index, input);
    }
    assert!(
        plan.operations()
            > plan
                .charts()
                .iter()
                .map(|c| c.fit_plan().operations())
                .sum::<u32>()
    );
}

#[test]
fn one_chart_keeps_the_existing_fit_geometry_and_work_receipt() {
    let input = query(&[[0., 0.], [0.5, 1.]]);
    let original = plan_cylindrical_fit(input, || false).unwrap();
    let charts = plan_cylindrical_charts(input, || false).unwrap();
    assert_eq!(charts.charts().len(), 1);
    let actual = charts.charts()[0].fit_plan();
    assert_eq!(
        actual.chart_center().to_bits(),
        original.chart_center().to_bits()
    );
    assert_eq!(actual.subdivision_level(), original.subdivision_level());
    assert_eq!(
        actual.ideal_developed_error_bound(),
        original.ideal_developed_error_bound()
    );
    assert_eq!(actual.operations(), original.operations());
}

#[test]
fn chart_intervals_remain_exact_inside_an_original_subinterval() {
    let mut input = query(&[[0., 0.], [64., 1.]]);
    input.interval = CylindricalProfileInterval::new(3, 3).unwrap();
    let plan = plan_cylindrical_charts(input, || false).unwrap();
    assert_eq!(plan.charts().len(), 8);
    for (index, chart) in plan.charts().iter().enumerate() {
        assert_eq!(
            chart.interval(),
            CylindricalProfileInterval::new(24 + index as u64, 6).unwrap()
        );
        assert_eq!(chart.fit_plan().chart_center(), 24.5 + index as f64);
    }
}

#[test]
fn insufficient_chart_depth_refuses_without_selecting_fewer_charts() {
    let mut input = query(&[[0., 0.], [6., 1.]]);
    input.max_level = 2;
    let error = plan_cylindrical_charts(input, || false).unwrap_err();
    let diagnostic = error.to_string();
    assert!(diagnostic.contains("maximum: 2"));
    assert!(diagnostic.contains("angular_variation_upper: 1.5"));
    assert!(diagnostic.contains("angular variation bound <= 1 radian"));
    assert!(matches!(
        error.cause,
        CylindricalProfileFailure::FitChartLevel { maximum: 2, .. }
    ));
}

#[test]
fn one_shared_allowance_refuses_the_entire_plan_on_exhaustion_or_cancellation() {
    let mut input = query(&[[0., 0.], [6., 1.]]);
    input.max_operations = 100;
    let error = plan_cylindrical_charts(input, || false).unwrap_err();
    assert!(matches!(
        error.cause,
        CylindricalProfileFailure::Arithmetic(_)
    ));
    let mut polls = 0;
    assert!(
        plan_cylindrical_charts(query(input.authored), || {
            polls += 1;
            polls > 100
        })
        .is_err()
    );
}

#[test]
fn larger_work_allowance_never_changes_selected_chart_geometry() {
    let mut input = query(&[[0., 0.], [6., 1.]]);
    let first = plan_cylindrical_charts(input, || false).unwrap();
    input.max_operations *= 2;
    let second = plan_cylindrical_charts(input, || false).unwrap();
    assert_eq!(signature(&first), signature(&second));
    fn send_sync<T: Send + Sync>() {}
    send_sync::<CylindricalChartPlan>();
}

fn signature(plan: &CylindricalChartPlan) -> Vec<(CylindricalProfileInterval, u8, u64)> {
    plan.charts()
        .iter()
        .map(|chart| {
            (
                chart.interval(),
                chart.fit_plan().subdivision_level(),
                chart.fit_plan().chart_center().to_bits(),
            )
        })
        .collect()
}

fn assert_linear_chart(chart: &CylindricalFitChart, index: usize, input: CylindricalFitQuery<'_>) {
    assert_eq!(
        chart.interval(),
        CylindricalProfileInterval::new(index as u64, 3).unwrap()
    );
    assert_eq!(chart.fit_plan().chart_center(), (index as f64 + 0.5) * 0.75);
    assert!(chart.fit_plan().ideal_developed_error_bound() <= input.spatial_limit / 2.);
    assert!(chart.fit_plan().subdivision_level() + 3 <= input.max_level);
}

#[test]
fn nonlinear_source_uses_original_polynomial_bounds_for_each_chart_center() {
    let input = query(&[[0., 0.], [0., 0.25], [0., 0.75], [6., 1.]]);
    let plan = plan_cylindrical_charts(input, || false).unwrap();
    assert_eq!(plan.charts().len(), 32);
    for (i, chart) in plan.charts().iter().enumerate() {
        let j = i + 1;
        let expected = (3 * (i * i * i + j * j * j)) as f64 / 32768.;
        assert_eq!(
            chart.interval(),
            CylindricalProfileInterval::new(i as u64, 5).unwrap()
        );
        assert_eq!(chart.fit_plan().chart_center(), expected);
        assert!(chart.fit_plan().ideal_developed_error_bound() <= input.spatial_limit / 2.);
    }
}

#[test]
fn invalid_total_level_is_refused_before_chart_count_shift() {
    let mut input = query(&[[0., 0.], [6., 1.]]);
    input.max_level = 64;
    assert!(matches!(
        plan_cylindrical_charts(input, || false).unwrap_err().cause,
        CylindricalProfileFailure::FitPolicy {
            maximum_level: 64,
            ..
        }
    ));
}
