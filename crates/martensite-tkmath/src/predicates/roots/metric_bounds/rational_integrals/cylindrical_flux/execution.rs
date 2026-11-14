// SPDX-License-Identifier: GPL-2.0-only
use super::*;

pub(super) fn integrate(
    query: CylindricalFluxQuery<'_>,
    share: &ExactRatio,
    cancelled: &mut impl FnMut() -> bool,
) -> Checked<(publish::BoundsSum, Vec<CylindricalFluxWork>)> {
    let mut integration = Integration::new(query, share);
    for (index, span) in query.spans.iter().copied().enumerate() {
        integration.add(index, span, cancelled)?;
    }
    Ok((integration.sum, integration.work))
}

struct Integration<'a> {
    options: CylindricalFluxOptions,
    radii: [f64; 2],
    share: &'a ExactRatio,
    remaining: u32,
    sum: publish::BoundsSum,
    work: Vec<CylindricalFluxWork>,
}
impl<'a> Integration<'a> {
    fn new(query: CylindricalFluxQuery<'_>, share: &'a ExactRatio) -> Self {
        Self {
            options: query.options,
            radii: query.radii,
            share,
            remaining: query.options.max_total_subintervals,
            sum: publish::BoundsSum::zero(),
            work: Vec::new(),
        }
    }
    fn add(
        &mut self,
        index: usize,
        span: CylindricalFluxSpan<'_>,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Checked<()> {
        poll(cancelled).map_err(|cause| error::integration(Some(index), cause))?;
        self.add_side(index, span, cancelled)?;
        let cap = caps::integrate(span, self.radii, self.options, index, cancelled)?;
        self.sum.accumulate(exact_bounds(cap), 0);
        Ok(())
    }
    fn add_side(
        &mut self,
        index: usize,
        span: CylindricalFluxSpan<'_>,
        cancelled: &mut impl FnMut() -> bool,
    ) -> Checked<()> {
        let options = self.options.engine(self.remaining);
        let side = side::integrate(span, options, self.share, cancelled)
            .map_err(|cause| error::integration(Some(index), cause))?;
        self.work.push(CylindricalFluxWork {
            effective_limit: options.max_subintervals,
            examined_subintervals: side.subintervals as u32,
        });
        self.remaining -= side.subintervals as u32;
        self.sum.accumulate(
            publish::reversed_bounds(side.bounds, span.side_reversed),
            side.subintervals,
        );
        Ok(())
    }
}
fn exact_bounds(value: ExactRatio) -> super::super::subinterval::ContributionBounds {
    super::super::subinterval::ContributionBounds {
        lower: value.clone(),
        upper: value,
    }
}
