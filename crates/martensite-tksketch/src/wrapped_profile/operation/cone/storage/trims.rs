//! Actual stored UV occurrences are counted, coefficient-bound and jointly certified.
use super::*;
use capture::SourceCapture;
mod chart;
pub(super) struct Trims {
    pub proofs: [CertifiedConicalTrim; 2],
    pub developed_boundary: Option<ConicalDevelopedBoundaryEvidence>,
    pub pcurves: [Vec<[f64; 2]>; 2],
    pub count: usize,
    pub side_count: usize,
    pub maximum_tolerance: f64,
}

struct CapProofs {
    traces: [CertifiedConicalTrim; 2],
    developed_boundary: Option<ConicalDevelopedBoundaryEvidence>,
}
struct CapturedCaps {
    pcurves: [Vec<[f64; 2]>; 2],
    maximum_tolerance: f64,
}
pub(super) fn certify(
    source: &SourceCapture<'_>,
    piece: &AuthoredPiece,
    curves: &[retained::OwnedCurve; 2],
    boundary: &ConicalBoundaryEvidence,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Trims> {
    let fit = selected_fit(source, piece)?;
    let cap = CapCapture {
        source,
        piece,
        expected: &fit.uv,
        chart: source
            .prepared
            .boundary_metric(piece.span, fit.local_chart)?
            .chart,
    };
    let (side_count, side_tolerance) = side(source, piece)?;
    let caps = cap.captured()?;
    let proofs = cap.certify(curves, &caps.pcurves, boundary, work, cancelled)?;
    let count = proofs.traces.len();
    Ok(Trims {
        proofs: proofs.traces,
        developed_boundary: proofs.developed_boundary,
        pcurves: caps.pcurves,
        count,
        side_count,
        maximum_tolerance: side_tolerance.max(caps.maximum_tolerance),
    })
}
fn selected_fit<'a>(source: &SourceCapture<'a>, piece: &AuthoredPiece) -> ConeChecked<&'a SpanFit> {
    source.prepared.fits.get(&piece.span).ok_or_else(|| {
        mismatch(
            source.original.region,
            Some(piece.span),
            "trim fit membership",
            vec![0],
            vec![1],
        )
    })
}
fn side(source: &SourceCapture<'_>, piece: &AuthoredPiece) -> ConeChecked<(usize, f64)> {
    retained::trims::side_trims(&source.prepared.source, piece, source.bindings)
        .map_err(|cause| captured(source.original.region, cause))
}
struct CapCapture<'a, 'b> {
    source: &'a SourceCapture<'b>,
    piece: &'a AuthoredPiece,
    expected: &'a [[f64; 2]],
    chart: Option<&'b ConicalChartDomainEvidence>,
}
impl CapCapture<'_, '_> {
    fn captured(&self) -> ConeChecked<CapturedCaps> {
        // Complete representation admission precedes paired mathematics on this unshipped path.
        let (first, a) = self.retained(0)?;
        let (last, b) = self.retained(1)?;
        Ok(CapturedCaps {
            pcurves: [first, last],
            maximum_tolerance: a.max(b),
        })
    }
    fn certify(
        &self,
        curves: &[retained::OwnedCurve; 2],
        pcurves: &[Vec<[f64; 2]>; 2],
        boundary: &ConicalBoundaryEvidence,
        work: &mut Work,
        cancelled: &mut CancelPoll<'_>,
    ) -> ConeChecked<CapProofs> {
        let query = self.query(curves, pcurves, work.profile());
        let (pair, developed_boundary) =
            self.selected_pair(query, curves, boundary, work, cancelled)?;
        self.bind(&pair, query)?;
        Ok(CapProofs {
            traces: pair.into_traces(),
            developed_boundary,
        })
    }
    fn selected_pair(
        &self,
        query: ConicalTrimPairQuery<'_>,
        curves: &[retained::OwnedCurve; 2],
        boundary: &ConicalBoundaryEvidence,
        work: &mut Work,
        cancelled: &mut CancelPoll<'_>,
    ) -> ConeChecked<(
        CertifiedConicalTrimPair,
        Option<ConicalDevelopedBoundaryEvidence>,
    )> {
        if let Some(domain) = self.chart {
            let (pair, evidence) =
                chart::certify(self, query, curves, boundary, domain, work, cancelled)?;
            return Ok((pair, Some(evidence)));
        }
        let result = certify_conical_trim_pair(query, cancelled);
        self.charged(result, work).map(|pair| (pair, None))
    }
    fn query<'a>(
        &self,
        curves: &'a [retained::OwnedCurve; 2],
        pcurves: &'a [Vec<[f64; 2]>; 2],
        options: CylindricalProfileOptions,
    ) -> ConicalTrimPairQuery<'a> {
        ConicalTrimPairQuery {
            curves: curves.each_ref().map(retained::OwnedCurve::borrowed),
            pcurves: pcurves.each_ref().map(Vec::as_slice),
            carriers: self.source.carriers,
            spatial_limits: [self.source.prepared.source.tolerance / 2.; 2],
            options,
        }
    }
    fn charged(
        &self,
        result: Result<CertifiedConicalTrimPair, ConicalTrimPairError>,
        work: &mut Work,
    ) -> ConeChecked<CertifiedConicalTrimPair> {
        let operations = result.as_ref().map_or_else(
            |cause| cause.operations,
            CertifiedConicalTrimPair::operations,
        );
        let charged = work.charge(
            self.source.original.region,
            Some(self.piece.span),
            operations,
        );
        // A bounded leaf's original refusal outranks secondary ledger bookkeeping failures.
        match (result, charged) {
            (Err(cause), _) => Err(self.failure(cause)),
            (Ok(_), Err(cause)) => Err(cause),
            (Ok(pair), Ok(())) => Ok(pair),
        }
    }
    fn bind(
        &self,
        pair: &CertifiedConicalTrimPair,
        query: ConicalTrimPairQuery<'_>,
    ) -> ConeChecked<()> {
        if !pair.matches_source(query) {
            return Err(mismatch(
                self.source.original.region,
                Some(self.piece.span),
                "paired cap trim source binding",
                vec![0],
                vec![1],
            ));
        }
        Ok(())
    }
    fn failure(&self, cause: ConicalTrimPairError) -> ConeFailure {
        failure(
            self.source.original.region,
            Some(self.piece.span),
            ConeCause::TrimPair(Box::new(cause)),
        )
    }
    fn retained(&self, depth: usize) -> ConeChecked<(Vec<[f64; 2]>, f64)> {
        let region = self.source.original.region;
        let edge = retained::key(
            self.piece.span,
            [HistoryRole::FirstDepthEdge, HistoryRole::LastDepthEdge][depth],
        );
        let face = capture::region_key(
            region,
            [HistoryRole::FirstDepthFace, HistoryRole::LastDepthFace][depth],
        );
        let pcurve = retained::trims::pcurve(edge, face, self.expected, self.source.bindings)
            .map_err(|cause| captured(region, cause))?;
        let tolerance = retained::trims::topology_tolerances(
            edge,
            face,
            self.source.prepared.source.tolerance / 2.,
            self.source.bindings,
        )
        .map_err(|cause| captured(region, cause))?;
        Ok((pcurve, tolerance))
    }
}

#[cfg(test)]
mod tests;
