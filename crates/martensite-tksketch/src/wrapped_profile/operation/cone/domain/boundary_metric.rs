//! Retain one chart per planned interval and resolve its authority by authored owner.
use super::*;
use std::num::NonZeroU16;

pub(in super::super) type ChartSlot = NonZeroU16;
pub(in super::super) struct LocalChart {
    curve: SurfaceCurveId,
    evidence: ConicalChartDomainEvidence,
}

#[derive(Clone, Copy)]
pub(in super::super) struct BoundaryMetric<'a> {
    pub(in super::super) chart: Option<&'a ConicalChartDomainEvidence>,
    directed: bool,
}

impl Prepared {
    pub(in super::super) fn retain_local_chart(
        &mut self,
        span: CurveSpan,
        evidence: ConicalChartDomainEvidence,
    ) -> ConeChecked<ChartSlot> {
        let count = self.local_charts.len() + 1;
        let limit = crate::surface_sketch::wrapped_profile::storage::MAX_CERTIFIED_SPANS;
        let slot = u16::try_from(count).ok().and_then(NonZeroU16::new);
        let slot = slot
            .filter(|slot| u32::from(slot.get()) <= limit)
            .ok_or_else(|| {
                local_failure(
                    self,
                    span,
                    "local cone chart count",
                    count as u64,
                    u64::from(limit),
                )
            })?;
        if self.chart_domain.is_some() || directed(self) || evidence.source_restriction().is_none()
        {
            return Err(local_failure(
                self,
                span,
                "exclusive restricted local cone chart authority",
                0,
                1,
            ));
        }
        self.local_charts.push(LocalChart {
            curve: span.curve,
            evidence,
        });
        Ok(slot)
    }

    pub(in super::super) fn boundary_metric(
        &self,
        span: CurveSpan,
        slot: Option<ChartSlot>,
    ) -> ConeChecked<BoundaryMetric<'_>> {
        let chart = match slot {
            Some(slot) => Some(self.local_chart(span, slot)?),
            None if self.local_charts.is_empty() => self.chart_domain.as_deref(),
            None => {
                return Err(local_failure(
                    self,
                    span,
                    "local cone chart slot for fitted span",
                    0,
                    1,
                ));
            }
        };
        Ok(BoundaryMetric {
            chart,
            directed: directed(self),
        })
    }

    fn local_chart(
        &self,
        span: CurveSpan,
        slot: ChartSlot,
    ) -> ConeChecked<&ConicalChartDomainEvidence> {
        let index = usize::from(slot.get() - 1);
        let local = self.local_charts.get(index).ok_or_else(|| {
            local_failure(
                self,
                span,
                "existing local cone chart slot",
                u64::from(slot.get()),
                self.local_charts.len() as u64,
            )
        })?;
        if local.curve != span.curve {
            return Err(local_failure(
                self,
                span,
                "local cone chart authored owner",
                local.curve.0,
                span.curve.0,
            ));
        }
        let restriction = local.evidence.source_restriction();
        if !restriction
            .is_some_and(|ancestor| covers(ancestor, (u64::from(span.index), span.level)))
        {
            return Err(interval_failure(self, span, restriction));
        }
        Ok(&local.evidence)
    }
}

pub(in super::super) fn covers((index, level): (u64, u8), (child, child_level): (u64, u8)) -> bool {
    child_level
        .checked_sub(level)
        .is_some_and(|delta| child.checked_shr(u32::from(delta)) == Some(index))
}

fn interval_failure(
    prepared: &Prepared,
    span: CurveSpan,
    restriction: Option<(u64, u8)>,
) -> ConeFailure {
    prepare::failure(
        Some(prepared.source.region),
        Some(span),
        ConeCause::Binding {
            field: "fitted span within local cone chart interval",
            actual: vec![u64::from(span.index), u64::from(span.level)],
            expected: restriction
                .map_or_else(Vec::new, |(index, level)| vec![index, u64::from(level)]),
        },
    )
}

impl BoundaryMetric<'_> {
    pub(in super::super) fn certify(
        self,
        query: ConicalBoundaryQuery<'_>,
        options: ConeClearanceOptions,
        cancelled: &mut CancelPoll<'_>,
    ) -> Result<ConicalBoundaryEvidence, ConicalBoundaryError> {
        match self.chart {
            Some(domain) if self.directed => certify_conical_boundary_in_chart_with_refinement(
                query,
                domain,
                options,
                ConicalPrimitiveRefinement::HalfSplitV1,
                cancelled,
            ),
            Some(domain) => certify_conical_boundary_in_chart(query, domain, options, cancelled),
            None => certify_conical_boundary(query, options, cancelled),
        }
    }

    pub(in super::super) fn matches(
        self,
        proof: &ConicalBoundaryEvidence,
        query: &ConicalBoundaryQuery<'_>,
    ) -> bool {
        match self.chart {
            Some(domain) if self.directed => proof.matches_chart_with_refinement(
                query,
                domain,
                ConicalPrimitiveRefinement::HalfSplitV1,
            ),
            Some(domain) => proof.matches_chart(query, domain),
            None => proof.matches_legacy(query),
        }
    }
}

fn local_failure(
    prepared: &Prepared,
    span: CurveSpan,
    field: &'static str,
    actual: u64,
    expected: u64,
) -> ConeFailure {
    prepare::failure(
        Some(prepared.source.region),
        Some(span),
        ConeCause::Binding {
            field,
            actual: vec![actual],
            expected: vec![expected],
        },
    )
}
