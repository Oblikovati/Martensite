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
    cancelled: &mut CancelPoll<'_>,
) -> Result<CertifiedCylindricalMeasureComparison, StorageFailure> {
    let parts = flux::capture_parts(prepared, bindings, proofs)?;
    let spans: Vec<_> = parts.iter().map(flux::FluxSpan::borrowed).collect();
    let query = CylindricalMeasureComparisonQuery {
        flux_source: flux::flux_query(&spans, prepared),
        normal_source,
        limit: CylindricalMeasureLimit::Report,
        options: Default::default(),
    };
    compare_cylindrical_measures(completed.0, completed.1, query, cancelled)
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

#[cfg(test)]
mod cancellation_controls {
    use super::super::{cancellation_tests::encoded, tests::frozen};
    use super::*;
    use martensite_tkmath::predicates::roots::metric_bounds::{
        BoundedArithmeticFailure, CylindricalMeasureComparisonFailure,
    };
    #[test]
    fn cancelled_comparison_refuses_completed_input_measures_without_geometry_mutation() {
        let (original, prepared, bindings) = frozen();
        let stored = super::super::certify(&original, &prepared, &bindings).unwrap();
        let completed = completed_measures(&original, &prepared, &bindings, &stored);
        let (_, sources) = planar_cones::source_collection(&original, &prepared, &stored).unwrap();
        let query = ideal_query(&sources, original.depths);
        let before = encoded(&prepared, &bindings);
        let error = compare_flux(&prepared, &bindings, &stored, completed, query, &mut || {
            true
        })
        .err()
        .unwrap();
        assert_cancelled_comparison(error);
        assert_eq!(encoded(&prepared, &bindings), before);
    }
    fn completed_measures(
        original: &Request,
        prepared: &Request,
        bindings: &[(BindingKey, Shape)],
        stored: &[StoredProof],
    ) -> (CertifiedCylindricalFlux, CertifiedCylindricalNormalMeasure) {
        let flux = super::super::certify_flux(original, prepared, bindings, stored).unwrap();
        let normal = normal_measure::certify(original, prepared, stored).unwrap();
        (flux, normal.certificate)
    }
    fn ideal_query<'a>(
        sources: &'a [martensite_tkmath::predicates::roots::metric_bounds::CylindricalProfileSource<'a>],
        depths: [f64; 2],
    ) -> CylindricalMeasureQuery<'a> {
        CylindricalMeasureQuery {
            sources,
            depths,
            limit: CylindricalMeasureLimit::Report,
            options: Default::default(),
        }
    }
    fn assert_cancelled_comparison(error: StorageFailure) {
        let StorageCause::Comparison(cause) = error.cause else {
            panic!("comparison")
        };
        assert!(
            matches!(cause.cause, CylindricalMeasureComparisonFailure::Arithmetic(cause) if *cause == BoundedArithmeticFailure::Cancelled)
        );
        assert_eq!(cause.flux_spans, 19);
        assert_eq!(cause.normal_sources, 19);
    }
}
