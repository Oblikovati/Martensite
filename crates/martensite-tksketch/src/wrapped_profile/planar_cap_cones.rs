//! Bind straight-homotopy cones to decoded angular cap controls in their own units.
use super::*;
#[path = "planar_cap_closure.rs"]
mod closure;
#[derive(Default)]
pub(in super::super) struct CapConeEvidence {
    pub pieces: Vec<SpanCone>,
    pub joins: Vec<SpanCone>,
}
struct CapSources<'a> {
    poles: BTreeMap<CurveSpan, Vec<[f64; 2]>>,
    proofs: ProofIndex<'a>,
}
pub(super) fn certify(
    request: &Request,
    stored: &[storage::StoredProof],
    cancelled: &mut CancelPoll<'_>,
) -> Result<CapConeEvidence, ConeFailure> {
    let sources = CapSources::new(request, stored)?;
    let mut evidence = CapConeEvidence::default();
    for contour in &request.loops {
        for (i, piece) in contour.iter().enumerate() {
            let next = contour[(i + 1) % contour.len()].span;
            evidence
                .pieces
                .push(sources.certify(&[piece.span], cancelled)?);
            evidence
                .joins
                .push(sources.certify(&[piece.span, next], cancelled)?);
        }
    }
    Ok(evidence)
}
pub(super) fn bind(
    request: &Request,
    stored: &[storage::StoredProof],
    evidence: &CapConeEvidence,
) -> Result<(), ConeFailure> {
    let count = request.loops.iter().map(Vec::len).sum();
    let pieces = binding::index(&evidence.pieces, count, 1)?;
    let joins = binding::index(&evidence.joins, count, 2)?;
    let sources = CapSources::new(request, stored)?;
    for contour in &request.loops {
        for (i, piece) in contour.iter().enumerate() {
            let next = contour[(i + 1) % contour.len()].span;
            sources.bind(&[piece.span], &pieces)?;
            sources.bind(&[piece.span, next], &joins)?;
        }
    }
    Ok(())
}
impl<'a> CapSources<'a> {
    fn new(request: &Request, stored: &'a [storage::StoredProof]) -> Result<Self, ConeFailure> {
        closure::certify(request)?;
        let poles = request
            .loops
            .iter()
            .flatten()
            .map(|piece| {
                let controls = cap_coordinates::angular_controls(&piece.poles, request.radius);
                (piece.span, controls)
            })
            .collect();
        Ok(Self {
            poles,
            proofs: proof_index(stored)?,
        })
    }
    fn source(&self, span: CurveSpan) -> Result<CylindricalConeSource<'_>, ConeFailure> {
        let authored = self.poles.get(&span).ok_or_else(|| membership(span, 0))?;
        let proof = self.proofs.get(&span).ok_or_else(|| membership(span, 0))?;
        let interval = CylindricalProfileInterval::new(0, 0)
            .map_err(|cause| ConeFailure::Interval { span, cause })?;
        Ok(CylindricalConeSource {
            authored,
            interval,
            support_radius: 1.,
            angular_origin: 0.,
            certificate: &proof.cap_profile,
        })
    }
    fn collection(
        &self,
        spans: &[CurveSpan],
    ) -> Result<Vec<CylindricalConeSource<'_>>, ConeFailure> {
        spans.iter().map(|span| self.source(*span)).collect()
    }
    fn certify(
        &self,
        spans: &[CurveSpan],
        cancelled: &mut CancelPoll<'_>,
    ) -> Result<SpanCone, ConeFailure> {
        certify_sources(spans, &self.collection(spans)?, cancelled)
    }
    fn bind(
        &self,
        spans: &[CurveSpan],
        indexed: &BTreeMap<&[CurveSpan], &CertifiedCylindricalCone>,
    ) -> Result<(), ConeFailure> {
        let certificate = indexed.get(spans).ok_or_else(|| cap_binding(spans))?;
        if !certificate.matches_sources(&self.collection(spans)?) {
            return Err(cap_binding(spans));
        }
        Ok(())
    }
}
fn cap_binding(spans: &[CurveSpan]) -> ConeFailure {
    ConeFailure::Binding {
        spans: spans.to_vec(),
        field: "decoded cap profile source binding",
        actual: 0,
        expected: 1,
    }
}

#[cfg(test)]
#[path = "planar_cap_cones_tests.rs"]
mod tests;
