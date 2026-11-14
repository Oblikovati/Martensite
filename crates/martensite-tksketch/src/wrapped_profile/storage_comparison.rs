//! Recapture both cap supports and every span before composing completed measures.
use super::*;
use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedCylindricalMeasureComparison, CertifiedCylindricalNormalMeasure,
    CylindricalMeasureComparisonQuery, CylindricalMeasureLimit, CylindricalMeasureQuery,
    compare_cylindrical_measures,
};

pub(in super::super) fn compare_flux(
    prepared: &Request,
    bindings: &[(BindingKey, Shape)],
    proofs: &[StoredProof],
    completed: (CertifiedCylindricalFlux, CertifiedCylindricalNormalMeasure),
    normal_source: CylindricalMeasureQuery<'_>,
) -> Result<CertifiedCylindricalMeasureComparison, StorageFailure> {
    let parts = flux::capture_parts(prepared, bindings, proofs)?;
    let spans: Vec<_> = parts.iter().map(flux::FluxSpan::borrowed).collect();
    let query = CylindricalMeasureComparisonQuery {
        flux_source: flux::flux_query(&spans, prepared),
        normal_source,
        limit: CylindricalMeasureLimit::Report,
        options: Default::default(),
    };
    compare_cylindrical_measures(completed.0, completed.1, query, || false)
        .map_err(|cause| comparison_failure(prepared.region, cause))
}
fn comparison_failure(
    region: SurfaceRegionId,
    cause: martensite_tkmath::predicates::roots::metric_bounds::CylindricalMeasureComparisonError,
) -> StorageFailure {
    StorageFailure {
        key: flux::region_key(region),
        cause: StorageCause::Comparison(Box::new(cause)),
    }
}
