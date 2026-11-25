// SPDX-License-Identifier: GPL-2.0-only
use super::super::super::conical_sum::{ConicalSum, term_engine};
use super::*;

pub(super) fn integrate(
    query: AnalyticConicalFluxQuery<'_>,
    densities: &[caps::Density; 2],
    work: &mut error::ExecutionWork,
    cancelled: &mut (impl FnMut() -> bool + Send),
) -> Checked<(ConicalSum, Vec<CylindricalFluxWork>)> {
    let engine = term_engine(query.options, query.options.max_total_subintervals);
    let share = publish::width_share(engine, query.spans.len(), &Ratio::from_finite(1.));
    let mut integration = Integration::new(query, &share);
    for (index, span) in query.spans.iter().copied().enumerate() {
        poll(cancelled).map_err(|cause| error::integration(Some(index), cause))?;
        integration.side(index, span, work, cancelled)?;
        caps::accumulate(
            &mut integration.sum,
            index,
            span,
            densities,
            &mut work.operations,
            cancelled,
        )?;
    }
    Ok((integration.sum, integration.work))
}

struct Integration<'a> {
    options: CylindricalFluxOptions,
    share: &'a ExactRatio,
    remaining: u32,
    sum: ConicalSum,
    work: Vec<CylindricalFluxWork>,
}
impl<'a> Integration<'a> {
    fn new(query: AnalyticConicalFluxQuery<'_>, share: &'a ExactRatio) -> Self {
        let options = query.options;
        Self {
            options,
            share,
            remaining: options.max_total_subintervals,
            sum: ConicalSum::new(options.behavior_version == 2, query.arithmetic),
            work: Vec::new(),
        }
    }
    fn side(
        &mut self,
        index: usize,
        span: AnalyticConicalFluxSpan<'_>,
        work: &mut error::ExecutionWork,
        cancelled: &mut (impl FnMut() -> bool + Send),
    ) -> Checked<()> {
        let options = term_engine(self.options, self.remaining);
        let (side, cells) = super::super::side::integrate_boundaries_accounted(
            span.boundaries,
            options,
            self.share,
            cancelled,
        );
        work.cells += cells;
        let side = side.map_err(|cause| error::integration(Some(index), cause))?;
        self.record(
            index,
            side,
            span.side_reversed,
            options.max_subintervals,
            work,
            cancelled,
        )
    }
    fn record(
        &mut self,
        index: usize,
        side: curve::CurveIntegral,
        reversed: bool,
        limit: u32,
        work: &mut error::ExecutionWork,
        cancelled: &mut (impl FnMut() -> bool + Send),
    ) -> Checked<()> {
        self.work.push(CylindricalFluxWork {
            effective_limit: limit,
            examined_subintervals: side.subintervals as u32,
        });
        self.remaining -= side.subintervals as u32;
        self.sum
            .accumulate(
                publish::reversed_bounds(side.bounds, reversed),
                side.subintervals,
                &mut work.operations,
                cancelled,
            )
            .map_err(|cause| AnalyticConicalFluxFailure::Arithmetic {
                span: Some(index),
                depth: None,
                cause: Box::new(cause),
            })
    }
}
