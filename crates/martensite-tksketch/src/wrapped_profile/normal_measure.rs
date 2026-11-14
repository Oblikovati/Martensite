//! Conditional ideal volume follows retained O2 sources, not the native volume estimate.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedCylindricalNormalMeasure, CylindricalMeasureError, CylindricalMeasureLimit,
    CylindricalMeasureQuery, certify_cylindrical_normal_measure,
};

pub(super) struct NormalMeasureEvidence {
    pub spans: Vec<CurveSpan>,
    pub certificate: CertifiedCylindricalNormalMeasure,
}
#[derive(Debug)]
pub(super) enum NormalMeasureFailure {
    Sources(Box<planar_cones::ConeFailure>),
    Certificate(Box<CylindricalMeasureError>),
}
impl std::fmt::Display for NormalMeasureFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sources(cause) => write!(f, "normal measure sources: {cause}"),
            Self::Certificate(cause) => write!(f, "{cause}"),
        }
    }
}
pub(super) fn certify(
    original: &Request,
    prepared: &Request,
    stored: &[storage::StoredProof],
) -> Result<NormalMeasureEvidence, NormalMeasureFailure> {
    let (spans, sources) = planar_cones::source_collection(original, prepared, stored)
        .map_err(|cause| NormalMeasureFailure::Sources(Box::new(cause)))?;
    let query = CylindricalMeasureQuery {
        sources: &sources,
        depths: original.depths,
        limit: CylindricalMeasureLimit::Report,
        options: Default::default(),
    };
    let certificate = certify_cylindrical_normal_measure(query, || false)
        .map_err(|cause| NormalMeasureFailure::Certificate(Box::new(cause)))?;
    Ok(NormalMeasureEvidence { spans, certificate })
}

#[cfg(test)]
mod tests {
    use super::super::planar_cones::tests::proofs;
    use super::*;
    #[test]
    fn missing_profile_proof_refuses_the_normal_measure() {
        let (original, prepared, mut stored) = proofs();
        stored.remove(0);
        assert!(matches!(
            certify(&original, &prepared, &stored),
            Err(NormalMeasureFailure::Sources(_))
        ));
    }
    #[test]
    fn relabelled_profile_proofs_do_not_cover_new_measure_sources() {
        let (original, prepared, mut stored) = proofs();
        let (a, b) = (stored[0].span, stored[1].span);
        stored[0].span = b;
        stored[1].span = a;
        assert!(matches!(
            certify(&original, &prepared, &stored),
            Err(NormalMeasureFailure::Certificate(_))
        ));
    }
    #[test]
    fn retained_measure_matches_all_prepared_sources() {
        let (original, prepared, stored) = proofs();
        let evidence = certify(&original, &prepared, &stored).unwrap();
        let (spans, sources) =
            planar_cones::source_collection(&original, &prepared, &stored).unwrap();
        assert_eq!(evidence.spans, spans);
        let query = CylindricalMeasureQuery {
            sources: &sources,
            depths: original.depths,
            limit: CylindricalMeasureLimit::Report,
            options: Default::default(),
        };
        assert!(evidence.certificate.matches_sources(query));
    }
}
