// SPDX-License-Identifier: GPL-2.0-only
//! Bound original angular speed before choosing a complete dyadic chart cover.
use super::{algebra::*, *};

/// Plan local charts before fitting, keeping every source interval exact.
/// V1 first bounds chart variation by one radian, then applies the original Hermite
/// planner to each chart under one shared 256-bit work allowance. No proof retry
/// changes geometry. Example: `let plan = plan_cylindrical_charts(query, || false)?;`.
pub fn plan_cylindrical_charts(
    query: CylindricalFitQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<CylindricalChartPlan, CylindricalProfileError> {
    execute(query, &mut cancelled).map_err(|cause| planning::failure(query, cause))
}

fn execute(
    query: CylindricalFitQuery<'_>,
    cancelled: &mut (dyn FnMut() -> bool + Send),
) -> Checked<CylindricalChartPlan> {
    planning::admit(query)?;
    let mut arithmetic = planning::arithmetic(query, cancelled);
    let level = chart_level(query, &mut arithmetic)?;
    let mut charts = Vec::new();
    for index in 0..(1u64 << level) {
        let local = local_query(query, index, level)?;
        charts.push(fit_chart(local, &mut arithmetic)?);
    }
    Ok(CylindricalChartPlan {
        charts,
        operations: arithmetic.operations(),
    })
}

fn fit_chart(
    query: CylindricalFitQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<CylindricalFitChart> {
    let fit = planning::planned_geometry(query, arithmetic)?;
    Ok(CylindricalFitChart {
        interval: query.interval,
        fit,
    })
}

fn chart_level(query: CylindricalFitQuery<'_>, arithmetic: &mut Arithmetic<'_>) -> Checked<u8> {
    let source = source::developed(
        query.authored,
        query.interval,
        query.support_radius,
        query.angular_origin,
        arithmetic,
    )?;
    let variation = magnitude(&derivative(&source[0], arithmetic)?);
    choose_level(query, variation, arithmetic)
}

fn choose_level(
    query: CylindricalFitQuery<'_>,
    mut variation: Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<u8> {
    let one = arithmetic.scalar(1.)?;
    let mut level = 0;
    while variation.upper > one.lower {
        admit_chart_level(query, level, &variation, arithmetic)?;
        variation = arithmetic.fraction(&variation, 1, 2)?;
        level += 1;
    }
    Ok(level)
}

fn admit_chart_level(
    query: CylindricalFitQuery<'_>,
    level: u8,
    variation: &Interval,
    arithmetic: &Arithmetic<'_>,
) -> Checked<()> {
    if level == query.max_level {
        return Err(CylindricalProfileFailure::FitChartLevel {
            maximum: level,
            angular_variation_upper: arithmetic.upper_f64(variation.upper.clone())?,
        });
    }
    Ok(())
}

fn local_query(
    query: CylindricalFitQuery<'_>,
    index: u64,
    level: u8,
) -> Checked<CylindricalFitQuery<'_>> {
    let interval = CylindricalProfileInterval::new(
        (query.interval.index() << level) + index,
        query.interval.level() + level,
    )?;
    Ok(CylindricalFitQuery {
        interval,
        max_level: query.max_level - level,
        ..query
    })
}

#[cfg(test)]
#[path = "chart_planning_tests.rs"]
mod tests;
