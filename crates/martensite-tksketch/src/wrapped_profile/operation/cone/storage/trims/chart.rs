//! The explicit chart bridge consumes the immediately preceding decoded boundary receipt.
use super::*;
pub(super) fn certify(
    cap: &CapCapture<'_, '_>,
    pair: ConicalTrimPairQuery<'_>,
    curves: &[retained::OwnedCurve; 2],
    proof: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<(CertifiedConicalTrimPair, ConicalDevelopedBoundaryEvidence)> {
    let source = cap.source;
    let boundary = boundary::query(source.original, source.prepared, cap.piece, curves)?;
    let result = selected_call(pair, boundary, proof, domain, source.prepared, cancelled);
    charged(cap, result, work)
}
fn charged(
    cap: &CapCapture<'_, '_>,
    result: Result<
        (CertifiedConicalTrimPair, ConicalDevelopedBoundaryEvidence),
        ConicalChartTrimPairError,
    >,
    work: &mut Work,
) -> ConeChecked<(CertifiedConicalTrimPair, ConicalDevelopedBoundaryEvidence)> {
    let operations = result
        .as_ref()
        .map_or_else(|cause| cause.operations, |(pair, _)| pair.operations());
    let charged = work.charge(cap.source.original.region, Some(cap.piece.span), operations);
    match (result, charged) {
        (Err(cause), _) => Err(failure(
            cap.source.original.region,
            Some(cap.piece.span),
            ConeCause::ChartTrimPair(Box::new(cause)),
        )),
        (Ok(_), Err(cause)) => Err(cause),
        (Ok(pair), Ok(())) => Ok(pair),
    }
}
#[cfg(test)]
mod tests;

fn selected_call(
    pair: ConicalTrimPairQuery<'_>,
    boundary: ConicalBoundaryQuery<'_>,
    proof: &ConicalBoundaryEvidence,
    domain: &ConicalChartDomainEvidence,
    prepared: &Prepared,
    cancelled: &mut CancelPoll<'_>,
) -> Result<(CertifiedConicalTrimPair, ConicalDevelopedBoundaryEvidence), ConicalChartTrimPairError>
{
    certify_conical_trim_pair_with_developed_boundary_and_refinement(
        pair,
        boundary,
        proof,
        domain,
        prepared.reference_slant,
        super::super::super::domain::refinement(prepared.executed_selection),
        cancelled,
    )
}
