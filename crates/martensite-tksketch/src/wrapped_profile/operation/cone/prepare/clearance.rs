//! Original nonadjacent boundaries are compared over the exact chosen dyadic intervals.
use super::*;
pub(super) fn certify(
    originals: &[DevelopedRegionRequest],
    prepared: &[Prepared],
    work: &mut Work,
    cancelled: &mut CancelPoll<'_>,
) -> ConeChecked<CertifiedCylindricalClearance> {
    let mut sources = Vec::new();
    for input in prepared {
        for piece in input.source.loops.iter().flatten() {
            sources.push(source(originals, piece)?);
        }
    }
    let excluded = adjacent(prepared);
    let tolerance = originals.first().map_or(0., |r| r.tolerance);
    let query = query(&sources, &excluded, tolerance, work.remaining());
    let proof = certify_cylindrical_clearance(query, cancelled)
        .map_err(|cause| failure(None, None, ConeCause::Clearance(Box::new(cause))))?;
    let region = selection::first_region(originals)?;
    work.charge(region, None, proof.operations())?;
    Ok(proof)
}
fn query<'a>(
    sources: &'a [CylindricalClearanceSource<'a>],
    excluded: &'a [[usize; 2]],
    tolerance: f64,
    options: ConeClearanceOptions,
) -> CylindricalClearanceQuery<'a> {
    CylindricalClearanceQuery {
        sources,
        support_radius: 1.,
        angular_origin: 0.,
        domain: CylindricalClearanceDomain::Within { excluded },
        requested_clearance: 4. * tolerance,
        options: CylindricalClearanceOptions {
            fractional_bits: options.fractional_bits,
            max_operations: options.max_operations,
            ..Default::default()
        },
    }
}

fn source<'a>(
    originals: &'a [DevelopedRegionRequest],
    piece: &AuthoredPiece,
) -> ConeChecked<CylindricalClearanceSource<'a>> {
    let authored = originals
        .iter()
        .flat_map(|r| r.loops.iter().flatten())
        .find(|p| p.span.curve == piece.span.curve)
        .ok_or_else(|| source_failure(piece.span, "authored-source", 0., "one original owner"))?;
    let interval = CylindricalProfileInterval::new(u64::from(piece.span.index), piece.span.level)
        .map_err(|_| {
        source_failure(
            piece.span,
            "source-interval",
            f64::from(piece.span.level),
            "valid dyadic interval",
        )
    })?;
    Ok(CylindricalClearanceSource {
        authored: &authored.poles,
        interval,
    })
}
fn source_failure(
    span: CurveSpan,
    field: &'static str,
    actual: f64,
    expected: &'static str,
) -> ConeFailure {
    failure(
        None,
        Some(span),
        ConeCause::Scalar {
            field,
            actual,
            expected,
        },
    )
}

fn adjacent(prepared: &[Prepared]) -> Vec<[usize; 2]> {
    let mut result = std::collections::BTreeSet::new();
    let mut offset = 0;
    for contour in prepared.iter().flat_map(|r| &r.source.loops) {
        for i in 0..contour.len() {
            let mut pair = [offset + i, offset + (i + 1) % contour.len()];
            pair.sort();
            result.insert(pair);
        }
        offset += contour.len();
    }
    result.into_iter().collect()
}
