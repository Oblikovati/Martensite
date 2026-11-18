//! Actual stored UV occurrences are counted, coefficient-bound and jointly certified.
use super::*;
use capture::SourceCapture;
pub(super) struct Trims {
    pub proofs: [CertifiedConicalTrim; 2],
    pub pcurves: [Vec<[f64; 2]>; 2],
    pub count: usize,
    pub side_count: usize,
    pub maximum_tolerance: f64,
}
struct CapturedCaps {
    pcurves: [Vec<[f64; 2]>; 2],
    maximum_tolerance: f64,
}
pub(super) fn certify(
    source: &SourceCapture<'_>,
    piece: &AuthoredPiece,
    curves: &[retained::OwnedCurve; 2],
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<Trims> {
    let fit = selected_fit(source, piece)?;
    let cap = CapCapture {
        source,
        piece,
        expected: &fit.uv,
    };
    let (side_count, side_tolerance) = side(source, piece)?;
    let caps = cap.captured()?;
    let proofs = cap.certify(curves, &caps.pcurves, work, cancelled)?;
    let count = proofs.len();
    Ok(Trims {
        proofs,
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
        work: &mut Work,
        cancelled: &mut CancelPoll<'_>,
    ) -> ConeChecked<[CertifiedConicalTrim; 2]> {
        let query = ConicalTrimPairQuery {
            curves: curves.each_ref().map(retained::OwnedCurve::borrowed),
            pcurves: pcurves.each_ref().map(Vec::as_slice),
            carriers: self.source.carriers,
            spatial_limits: [self.source.prepared.source.tolerance / 2.; 2],
            options: work.profile(),
        };
        let result = certify_conical_trim_pair(query, cancelled);
        let pair = self.charged(result, work)?;
        self.bind(&pair, query)?;
        Ok(pair.into_traces())
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
