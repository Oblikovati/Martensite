//! Compose source-bound O2 derivative bounds with decoded shared incidence.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedCylindricalCone, CylindricalConeError, CylindricalConeQuery, CylindricalConeSource,
    CylindricalProfileFailure, CylindricalProfileInterval, certify_cylindrical_cone,
};
use std::collections::BTreeMap;
#[cfg(test)]
#[path = "planar_cones_tests.rs"]
mod tests;

type ProofIndex<'a> = BTreeMap<CurveSpan, &'a storage::StoredProof>;
type AuthoredIndex<'a> = BTreeMap<SurfaceCurveId, &'a [[f64; 2]]>;

pub(super) struct ConeEvidence {
    pub pieces: Vec<SpanCone>,
    pub joins: Vec<SpanCone>,
}
pub(super) struct SpanCone {
    pub spans: Vec<CurveSpan>,
    pub certificate: CertifiedCylindricalCone,
}
#[derive(Debug)]
pub(super) enum ConeFailure {
    Authored(Box<storage::StorageFailure>),
    Membership {
        span: CurveSpan,
        count: usize,
    },
    Interval {
        span: CurveSpan,
        cause: CylindricalProfileFailure,
    },
    Certificate {
        spans: Vec<CurveSpan>,
        cause: CylindricalConeError,
    },
}
impl std::fmt::Display for ConeFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authored(cause) => write!(f, "cone authored source: {cause}"),
            Self::Membership { span, count } => write!(
                f,
                "cone span={span:?}, membership={count}; expected one source and one retained O2 proof"
            ),
            Self::Interval { span, cause } => write!(f, "cone span={span:?}: {cause}"),
            Self::Certificate { spans, cause } => write!(f, "cone spans={spans:?}: {cause}"),
        }
    }
}

pub(super) fn certify(
    original: &Request,
    prepared: &Request,
    stored: &[storage::StoredProof],
) -> Result<ConeEvidence, ConeFailure> {
    let authored = storage::authored_index(original)
        .map_err(|cause| ConeFailure::Authored(Box::new(cause)))?;
    let proofs = proof_index(stored)?;
    let mut evidence = ConeEvidence {
        pieces: Vec::new(),
        joins: Vec::new(),
    };
    for contour in &prepared.loops {
        contour_cones(contour, original.radius, &authored, &proofs, &mut evidence)?;
    }
    Ok(evidence)
}

fn contour_cones(
    contour: &[AuthoredPiece],
    radius: f64,
    authored: &AuthoredIndex<'_>,
    proofs: &ProofIndex<'_>,
    evidence: &mut ConeEvidence,
) -> Result<(), ConeFailure> {
    for (i, piece) in contour.iter().enumerate() {
        let next = contour[(i + 1) % contour.len()].span;
        let adjacent = [piece.span, next];
        evidence
            .pieces
            .push(prove(&[piece.span], radius, authored, proofs)?);
        evidence
            .joins
            .push(prove(&adjacent, radius, authored, proofs)?);
    }
    Ok(())
}

fn proof_index(stored: &[storage::StoredProof]) -> Result<ProofIndex<'_>, ConeFailure> {
    let mut result = BTreeMap::new();
    for proof in stored {
        if result.insert(proof.span, proof).is_some() {
            return Err(membership(proof.span, 2));
        }
    }
    Ok(result)
}

fn prove(
    spans: &[CurveSpan],
    radius: f64,
    authored: &AuthoredIndex<'_>,
    proofs: &ProofIndex<'_>,
) -> Result<SpanCone, ConeFailure> {
    let sources = spans
        .iter()
        .map(|span| source(*span, radius, authored, proofs))
        .collect::<Result<Vec<_>, _>>()?;
    certify_sources(spans, &sources)
}

fn certify_sources(
    spans: &[CurveSpan],
    sources: &[CylindricalConeSource<'_>],
) -> Result<SpanCone, ConeFailure> {
    let query = CylindricalConeQuery {
        sources,
        options: Default::default(),
    };
    let certificate =
        certify_cylindrical_cone(query, || false).map_err(|cause| ConeFailure::Certificate {
            spans: spans.to_vec(),
            cause,
        })?;
    Ok(SpanCone {
        spans: spans.to_vec(),
        certificate,
    })
}

fn source<'a>(
    span: CurveSpan,
    radius: f64,
    authored: &AuthoredIndex<'a>,
    proofs: &ProofIndex<'a>,
) -> Result<CylindricalConeSource<'a>, ConeFailure> {
    let missing = || membership(span, 0);
    let authored = authored.get(&span.curve).copied().ok_or_else(missing)?;
    let proof = proofs.get(&span).copied().ok_or_else(missing)?;
    let interval = CylindricalProfileInterval::new(u64::from(span.index), span.level)
        .map_err(|cause| ConeFailure::Interval { span, cause })?;
    Ok(CylindricalConeSource {
        authored,
        interval,
        support_radius: radius,
        angular_origin: 0.,
        certificate: &proof.profile,
    })
}

fn membership(span: CurveSpan, count: usize) -> ConeFailure {
    ConeFailure::Membership { span, count }
}
