// SPDX-License-Identifier: GPL-2.0-only
//! Plan one source-ordered chart cover by splitting exact Bernstein controls.
use super::*;
use crate::predicates::roots::certificate_arithmetic::{
    bernstein_curve::{bisect, capture_planar},
    circular_constants,
};
mod types;
pub use types::*;
#[cfg(test)]
mod tests;

const VERSION: ConicalChartDomainVersion = ConicalChartDomainVersion::RecenteredV4;

type PlanResult<T> = Result<T, ConicalChartPlanFailure>;
type Controls = [Vec<Interval>; 2];

/// Plan a complete local chart cover for one original positive-X Bézier span.
/// A chart's certified physical width must be less than pi. The returned intervals
/// partition the original parameter domain and have one bounded planning ledger.
/// Example: `let plan = plan_conical_subspan_charts(query, || false)?;`.
pub fn plan_conical_subspan_charts(
    query: ConicalChartPlanQuery<'_>,
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<ConicalChartPlan, ConicalChartPlanError> {
    let spans = [query.authored];
    let source = chart_query(query, &spans);
    admit(query, &source).map_err(|cause| ConicalChartPlanError {
        cause,
        operations: 0,
    })?;
    let mut arithmetic = Arithmetic::new(sector_api::policy(query.options), &mut cancelled);
    execute(query, source, &mut arithmetic).map_err(|cause| ConicalChartPlanError {
        cause,
        operations: arithmetic.operations(),
    })
}

fn chart_query<'a>(
    query: ConicalChartPlanQuery<'a>,
    spans: &'a [&'a [[f64; 2]]],
) -> ConicalChartDomainQuery<'a> {
    ConicalChartDomainQuery {
        spans,
        semi_angle: query.semi_angle,
        chart_origin: query.chart_origin,
        fit_center: query.chart_origin,
        developed_turn: 0,
        physical_turn: 0,
        options: query.options,
    }
}

fn admit(query: ConicalChartPlanQuery<'_>, source: &ConicalChartDomainQuery<'_>) -> PlanResult<()> {
    if query.max_level > 63 || !(1..=VERSION.span_limit()).contains(&query.max_charts) {
        return Err(ConicalChartPlanFailure::Limits {
            max_level: query.max_level,
            max_charts: query.max_charts,
        });
    }
    Ok(bounds::admit(source, VERSION)?)
}

struct Node {
    index: u64,
    level: u8,
    controls: Controls,
}
impl Node {
    fn split(&self, arithmetic: &mut Arithmetic<'_>) -> PlanResult<[Self; 2]> {
        let [a, b] = bisect(&self.controls[0], arithmetic)?;
        let [c, d] = bisect(&self.controls[1], arithmetic)?;
        let level = self.level + 1;
        let index = self.index * 2;
        Ok([
            Self {
                index,
                level,
                controls: [a, c],
            },
            Self {
                index: index + 1,
                level,
                controls: [b, d],
            },
        ])
    }
}

struct PlanContext<'a> {
    query: ConicalChartPlanQuery<'a>,
    source: ConicalChartDomainQuery<'a>,
    sector: sector::SectorEvidence,
    trig: [Interval; 2],
    limit: Interval,
}

fn execute(
    query: ConicalChartPlanQuery<'_>,
    source: ConicalChartDomainQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> PlanResult<ConicalChartPlan> {
    let (wedge, margin) = sector::common_wedge(source.spans, arithmetic)
        .map_err(ConicalChartDomainFailure::Sector)?;
    bounds::wedge(wedge)?;
    let trig = bounds::support_trigonometry(source.semi_angle, arithmetic)?;
    let limit = circular_constants::pi(arithmetic)?;
    let controls = capture_planar(query.authored, arithmetic)?;
    let sector = bounds::captured_sector(&source, margin, arithmetic);
    let context = PlanContext {
        query,
        source,
        sector,
        trig,
        limit,
    };
    cover(
        &context,
        Node {
            index: 0,
            level: 0,
            controls,
        },
        arithmetic,
    )
}

fn cover(
    context: &PlanContext<'_>,
    root: Node,
    arithmetic: &mut Arithmetic<'_>,
) -> PlanResult<ConicalChartPlan> {
    let mut pending = vec![root];
    let mut charts = Vec::new();
    let mut charged = 0;
    while let Some(node) = pending.pop() {
        let (physical, minimum_x) = physical_range(context, &node, arithmetic)?;
        if &physical.upper - &physical.lower >= context.limit.lower {
            append_children(context, &node, &mut pending, charts.len(), arithmetic)?;
            continue;
        }
        charts.push(seal(
            context, &node, physical, minimum_x, charged, arithmetic,
        )?);
        charged = arithmetic.operations();
    }
    Ok(ConicalChartPlan {
        charts,
        operations: charged,
    })
}

fn physical_range(
    context: &PlanContext<'_>,
    node: &Node,
    arithmetic: &mut Arithmetic<'_>,
) -> PlanResult<(Interval, Interval)> {
    let [x, y] = &node.controls;
    let (developed, minimum_x) = restricted::developed_range(x, y, arithmetic)?;
    Ok((arithmetic.divide(&developed, &context.trig[1])?, minimum_x))
}

fn append_children(
    context: &PlanContext<'_>,
    node: &Node,
    pending: &mut Vec<Node>,
    completed: usize,
    arithmetic: &mut Arithmetic<'_>,
) -> PlanResult<()> {
    if node.level >= context.query.max_level {
        return Err(ConicalChartPlanFailure::Level {
            index: node.index,
            level: node.level,
        });
    }
    let minimum = pending.len() + completed + 2;
    if minimum > context.query.max_charts {
        return Err(ConicalChartPlanFailure::Capacity {
            minimum,
            limit: context.query.max_charts,
        });
    }
    let [left, right] = node.split(arithmetic)?;
    pending.extend([right, left]);
    Ok(())
}

fn seal(
    context: &PlanContext<'_>,
    node: &Node,
    physical: Interval,
    minimum_x: Interval,
    charged: u32,
    arithmetic: &mut Arithmetic<'_>,
) -> PlanResult<ConicalChartDomainEvidence> {
    let center = fit_center(&physical, context.source.chart_origin, arithmetic)?;
    let source = ConicalChartDomainQuery {
        fit_center: center,
        ..context.source
    };
    let offset = bounds::fit_offset(&source, VERSION, arithmetic)?;
    bounds::check_chart(&physical, offset.as_ref(), arithmetic)?;
    arithmetic.check_cancelled()?;
    let [cosine, sine] = context.trig.clone();
    let mut sector = context.sector.clone();
    // Shared setup and split work is charged once to the first chart that uses it.
    sector.operations = arithmetic.operations() - charged;
    let scope = restriction_key(ConicalOriginalSpan {
        poles: context.query.authored,
        index: node.index,
        level: node.level,
    });
    Ok(bounds::evidence(
        &source,
        VERSION,
        scope,
        sector,
        (physical, sine, cosine, minimum_x),
        offset,
    ))
}

fn fit_center(physical: &Interval, datum: f64, arithmetic: &Arithmetic<'_>) -> PlanResult<f64> {
    let lower = arithmetic.upper_f64(physical.lower.clone())?;
    let upper = arithmetic.upper_f64(physical.upper.clone())?;
    let center = datum + lower.midpoint(upper);
    if !center.is_finite() {
        return Err(ConicalChartDomainFailure::Chart {
            centers: [datum, center],
            turns: [0, 0],
        }
        .into());
    }
    // Center selection uses binary64. The stored center is checked on the fixed grid.
    Ok(center)
}
