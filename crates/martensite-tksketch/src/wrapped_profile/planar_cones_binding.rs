//! Bind local homotopy certificates to the actual retained O2 evidence, not its census.
use super::*;
type ConeIndex<'a> = BTreeMap<&'a [CurveSpan], &'a CertifiedCylindricalCone>;
pub(in super::super) fn bind(
    original: &Request,
    prepared: &Request,
    stored: &[storage::StoredProof],
    evidence: &ConeEvidence,
) -> Result<(), ConeFailure> {
    let count = prepared.loops.iter().map(Vec::len).sum();
    count_sources(count, stored.len())?;
    let pieces = index(&evidence.pieces, count, 1)?;
    let joins = index(&evidence.joins, count, 2)?;
    let authored = storage::authored_index(original)
        .map_err(|cause| ConeFailure::Authored(Box::new(cause)))?;
    let proofs = proof_index(stored)?;
    let sources = ConeSources::new(original.radius, &authored, &proofs);
    for contour in &prepared.loops {
        bind_contour(contour, &pieces, &joins, &sources)?;
    }
    Ok(())
}
fn count_sources(expected: usize, actual: usize) -> Result<(), ConeFailure> {
    if expected == 0 {
        return Err(failure(&[], "nonempty prepared spans", 0, 1));
    }
    if actual != expected {
        return Err(failure(&[], "retained O2 proofs", actual, expected));
    }
    Ok(())
}
fn index(
    reports: &[SpanCone],
    expected: usize,
    arity: usize,
) -> Result<ConeIndex<'_>, ConeFailure> {
    if reports.len() != expected {
        return Err(failure(&[], "cone reports", reports.len(), expected));
    }
    let mut result = BTreeMap::new();
    for report in reports {
        insert_cone(&mut result, report, arity)?;
    }
    Ok(result)
}
fn insert_cone<'a>(
    indexed: &mut ConeIndex<'a>,
    report: &'a SpanCone,
    arity: usize,
) -> Result<(), ConeFailure> {
    let spans = report.spans.as_slice();
    if spans.len() != arity {
        return Err(failure(spans, "source arity", spans.len(), arity));
    }
    if indexed.insert(spans, &report.certificate).is_some() {
        return Err(failure(spans, "cone multiplicity", 2, 1));
    }
    Ok(())
}

fn bind_contour(
    contour: &[AuthoredPiece],
    pieces: &ConeIndex<'_>,
    joins: &ConeIndex<'_>,
    sources: &ConeSources<'_, '_>,
) -> Result<(), ConeFailure> {
    for (i, piece) in contour.iter().enumerate() {
        let next = contour[(i + 1) % contour.len()].span;
        require(&[piece.span], pieces, sources)?;
        require(&[piece.span, next], joins, sources)?;
    }
    Ok(())
}
fn require(
    spans: &[CurveSpan],
    indexed: &ConeIndex<'_>,
    context: &ConeSources<'_, '_>,
) -> Result<(), ConeFailure> {
    let certificate = indexed
        .get(spans)
        .ok_or_else(|| failure(spans, "cone membership", 0, 1))?;
    let sources = spans
        .iter()
        .map(|span| source(*span, context.radius, context.authored, context.proofs))
        .collect::<Result<Vec<_>, _>>()?;
    if !certificate.matches_sources(&sources) {
        return Err(failure(spans, "retained O2 source binding", 0, 1));
    }
    Ok(())
}
fn failure(
    spans: &[CurveSpan],
    field: &'static str,
    actual: usize,
    expected: usize,
) -> ConeFailure {
    ConeFailure::Binding {
        spans: spans.to_vec(),
        field,
        actual,
        expected,
    }
}
#[cfg(test)]
#[path = "planar_cones_binding_tests.rs"]
mod tests;
