//! Cover each authored interval with restricted charts before fitting its rational edges.
use super::*;
use crate::surface_sketch::wrapped_profile::storage::MAX_CERTIFIED_SPANS;

pub(in super::super) fn plan(
    prepared: &mut Prepared,
    source: &AuthoredPiece,
    endpoints: &mut endpoints::EndpointPool,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
    fitted: &mut Vec<AuthoredPiece>,
) -> ConeChecked<()> {
    for chart in certify(prepared, source, work, cancelled)?.into_charts() {
        fitted.extend(super::plan(
            prepared,
            source,
            Some(chart),
            endpoints,
            work,
            cancelled,
        )?);
    }
    Ok(())
}

fn certify(
    prepared: &Prepared,
    source: &AuthoredPiece,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<ConicalChartPlan> {
    let query = query(prepared, source, work);
    let result = plan_conical_subspan_charts(query, cancelled);
    let operations = result
        .as_ref()
        .map_or_else(|cause| cause.operations, ConicalChartPlan::operations);
    let charged = work.charge(prepared.source.region, Some(source.span), operations);
    let plan = result.map_err(|cause| {
        failure(
            Some(prepared.source.region),
            Some(source.span),
            ConeCause::ChartPlan(Box::new(cause)),
        )
    })?;
    charged?;
    Ok(plan)
}

fn query<'a>(
    prepared: &Prepared,
    source: &'a AuthoredPiece,
    work: &Work,
) -> ConicalChartPlanQuery<'a> {
    ConicalChartPlanQuery {
        authored: &source.poles,
        semi_angle: prepared.semi_angle,
        chart_origin: prepared.chart_origin,
        max_level: prepared.source.max_fit_level,
        max_charts: MAX_CERTIFIED_SPANS as usize - prepared.local_charts.len(),
        options: work.remaining(),
    }
}
