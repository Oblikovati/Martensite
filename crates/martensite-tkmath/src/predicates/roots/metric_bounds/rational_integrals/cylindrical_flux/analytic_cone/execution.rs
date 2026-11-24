// SPDX-License-Identifier: GPL-2.0-only
use super::*;

pub(super) fn integrate(
    query: AnalyticConicalFluxQuery<'_>,
    densities: &[caps::Density; 2],
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<(publish::BoundsSum, Vec<CylindricalFluxWork>)> {
    let engine = query.options.engine(query.options.max_total_subintervals);
    let share = publish::width_share(engine, query.spans.len(), &Ratio::from_finite(1.));
    let mut integration = Integration::new(query.options, &share);
    for (index, span) in query.spans.iter().copied().enumerate() {
        poll(cancelled).map_err(|cause| error::integration(Some(index), cause))?;
        integration.side(index, span, cancelled)?;
        caps::accumulate(&mut integration.sum, index, span, densities, cancelled)?;
    }
    Ok((integration.sum, integration.work))
}

struct Integration<'a> {
    options: CylindricalFluxOptions,
    share: &'a ExactRatio,
    remaining: u32,
    sum: publish::BoundsSum,
    work: Vec<CylindricalFluxWork>,
}
impl<'a> Integration<'a> {
    fn new(options: CylindricalFluxOptions, share: &'a ExactRatio) -> Self {
        Self {
            options,
            share,
            remaining: options.max_total_subintervals,
            sum: publish::BoundsSum::zero(),
            work: Vec::new(),
        }
    }
    fn side(
        &mut self,
        index: usize,
        span: AnalyticConicalFluxSpan<'_>,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Checked<()> {
        let options = self.options.engine(self.remaining);
        let side = super::super::side::integrate_boundaries(
            span.boundaries,
            options,
            self.share,
            cancelled,
        )
        .map_err(|cause| error::integration(Some(index), cause))?;
        self.record(options, side, span.side_reversed);
        Ok(())
    }
    fn record(
        &mut self,
        options: RationalIntegralOptions,
        side: curve::CurveIntegral,
        reversed: bool,
    ) {
        self.work.push(CylindricalFluxWork {
            effective_limit: options.max_subintervals,
            examined_subintervals: side.subintervals as u32,
        });
        self.remaining -= side.subintervals as u32;
        self.sum.accumulate(
            publish::reversed_bounds(side.bounds, reversed),
            side.subintervals,
        );
    }
}
