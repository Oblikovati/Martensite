//! Conditional ideal volume follows retained O2 sources, not the native volume estimate.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedCylindricalFlux, CertifiedCylindricalNormalMeasure, CylindricalMeasureError,
    CylindricalMeasureLimit, CylindricalMeasureQuery, CylindricalProfileSource,
    certify_cylindrical_normal_measure,
};

pub(super) struct NormalMeasureEvidence {
    pub spans: Vec<CurveSpan>,
    pub certificate: CertifiedCylindricalNormalMeasure,
}
#[derive(Debug)]
pub(super) enum NormalMeasureFailure {
    Sources(Box<planar_cones::ConeFailure>),
    Storage(Box<storage::StorageFailure>),
    SpanSequence {
        actual: Vec<CurveSpan>,
        expected: Vec<CurveSpan>,
    },
    Certificate(Box<CylindricalMeasureError>),
}
impl std::fmt::Display for NormalMeasureFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Storage(cause) => write!(f, "normal measure recapture: {cause}"),
            Self::SpanSequence { actual, expected } => {
                write!(f, "normal measure spans={actual:?}; expected {expected:?}")
            }
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
    certify_with_cancel(original, prepared, stored, || false)
}
pub(super) fn certify_with_cancel(
    original: &Request,
    prepared: &Request,
    stored: &[storage::StoredProof],
    cancelled: impl FnMut() -> bool + Send,
) -> Result<NormalMeasureEvidence, NormalMeasureFailure> {
    let (spans, sources) = planar_cones::source_collection(original, prepared, stored)
        .map_err(|cause| NormalMeasureFailure::Sources(Box::new(cause)))?;
    let query = source_query(&sources, original.depths);
    let certificate = certify_cylindrical_normal_measure(query, cancelled)
        .map_err(|cause| NormalMeasureFailure::Certificate(Box::new(cause)))?;
    Ok(NormalMeasureEvidence { spans, certificate })
}

pub(super) fn compare(
    original: &Request,
    prepared: &Request,
    bindings: &[(BindingKey, Shape)],
    stored: &[storage::StoredProof],
    (flux, normal): (CertifiedCylindricalFlux, NormalMeasureEvidence),
) -> Result<pipeline::MeasureEvidence, NormalMeasureFailure> {
    compare_with_cancel(original, prepared, bindings, stored, (flux, normal), || {
        false
    })
}
pub(super) fn compare_with_cancel(
    original: &Request,
    prepared: &Request,
    bindings: &[(BindingKey, Shape)],
    stored: &[storage::StoredProof],
    (flux, normal): (CertifiedCylindricalFlux, NormalMeasureEvidence),
    mut cancelled: impl FnMut() -> bool + Send,
) -> Result<pipeline::MeasureEvidence, NormalMeasureFailure> {
    let (spans, sources) = planar_cones::source_collection(original, prepared, stored)
        .map_err(|cause| NormalMeasureFailure::Sources(Box::new(cause)))?;
    span_sequence(&normal.spans, &spans)?;
    let query = source_query(&sources, original.depths);
    let completed = (flux, normal.certificate);
    let comparison =
        storage::compare_flux(prepared, bindings, stored, completed, query, &mut cancelled)
            .map_err(|cause| NormalMeasureFailure::Storage(Box::new(cause)))?;
    Ok(pipeline::MeasureEvidence { comparison, spans })
}
fn source_query<'a>(
    sources: &'a [CylindricalProfileSource<'a>],
    depths: [f64; 2],
) -> CylindricalMeasureQuery<'a> {
    CylindricalMeasureQuery {
        sources,
        depths,
        limit: CylindricalMeasureLimit::Report,
        options: Default::default(),
    }
}

fn span_sequence(actual: &[CurveSpan], expected: &[CurveSpan]) -> Result<(), NormalMeasureFailure> {
    if actual != expected {
        return Err(NormalMeasureFailure::SpanSequence {
            actual: actual.to_vec(),
            expected: expected.to_vec(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::planar_cones::tests::proofs;
    use super::*;
    #[test]
    fn ideal_volume_arithmetic_cancellation_preserves_depths_and_source_count() {
        let (original, prepared, stored) = proofs();
        let mut polls = 0;
        let error = certify_with_cancel(&original, &prepared, &stored, || {
            polls += 1;
            polls >= 8
        })
        .err()
        .unwrap();
        assert_cancelled_measure(error, original.depths);
        assert_eq!(polls, 8);
    }
    fn assert_cancelled_measure(error: NormalMeasureFailure, depths: [f64; 2]) {
        use martensite_tkmath::predicates::roots::metric_bounds::{
            BoundedArithmeticFailure, CylindricalMeasureFailure,
        };
        let NormalMeasureFailure::Certificate(cause) = error else {
            panic!("normal measure")
        };
        assert!(
            matches!(cause.cause, CylindricalMeasureFailure::Arithmetic(cause) if *cause == BoundedArithmeticFailure::Cancelled)
        );
        assert_eq!(cause.requested_depths, depths);
        assert_eq!(cause.source_count, 19);
    }
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
        let query = source_query(&sources, original.depths);
        assert!(evidence.certificate.matches_sources(query));
    }
}

#[cfg(test)]
#[path = "normal_measure_comparison_tests.rs"]
mod comparison_tests;
