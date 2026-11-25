use super::{
    stock::{Faces, OriginalBoundary, StockFrustumReceipt},
    *,
};

pub(super) struct InstalledStock {
    pub(super) proof: StockFrustumReceipt,
    pub(super) band: Face,
    pub(super) base: Face,
    pub(super) lid: Face,
}

pub(super) struct FrozenStock {
    proof: StockFrustumReceipt,
    images: [CylinderBodyRef; 3],
}

impl InstalledStock {
    pub(super) fn bind(self, attempt: Attempt, index: &body::Index) -> FeatureChecked<FrozenStock> {
        let reference = |face: &Face| {
            index.reference(face, attempt)?.ok_or_else(|| {
                attempt.fail(
                    CylinderFeatureStage::Encoding,
                    CylinderFeatureCause::ArchiveMismatch,
                )
            })
        };
        let images = [
            reference(&self.band)?,
            reference(&self.base)?,
            reference(&self.lid)?,
        ];
        Ok(FrozenStock {
            proof: self.proof,
            images,
        })
    }
}

pub(super) struct BoundStock {
    pub(super) band: CylinderBodyRef,
    pub(super) ends: [CylinderBodyRef; 2],
    pub(super) spatial_upper: f64,
    pub(super) boundary: OriginalBoundary,
}

impl FrozenStock {
    pub(super) fn rebind(
        &self,
        attempt: Attempt,
        decoded: &DetachedCylinderFeature,
    ) -> FeatureChecked<BoundStock> {
        let refused = || {
            attempt.fail(
                CylinderFeatureStage::Encoding,
                CylinderFeatureCause::ArchiveMismatch,
            )
        };
        let face = |key: CylinderBodyRef| -> FeatureChecked<Face> {
            let bound = decoded.entities.get(&key).ok_or_else(refused)?;
            attempt.native(
                CylinderFeatureStage::Encoding,
                "decoded stock face",
                topods::face(bound),
            )
        };
        let [band, base, lid] = self.images;
        let (band, base, lid) = (face(band)?, face(base)?, face(lid)?);
        let boundary = self
            .proof
            .rebind(
                &decoded.body,
                Faces {
                    band: &band,
                    base: &base,
                    lid: &lid,
                },
            )
            .map_err(|cause| stock_refusal(attempt, CylinderFeatureStage::Validation, cause))?;
        let [band, base, lid] = self.images;
        Ok(BoundStock {
            band,
            ends: [base, lid],
            spatial_upper: self.proof.spatial_upper(),
            boundary,
        })
    }
}

pub(super) fn stock_refusal(
    attempt: Attempt,
    stage: CylinderFeatureStage,
    cause: ConicalStockFailure,
) -> CylinderFeatureFailure {
    attempt.fail(stage, CylinderFeatureCause::ConicalStock(Box::new(cause)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const ATTEMPT: Attempt = Attempt {
        attachment: FaceAttachmentId(3),
        revision: SupportRevision(5),
        kind: CylinderFeatureKind::Pocket,
    };

    #[test]
    fn source_stock_refusal_retains_the_computed_numeric_limits() {
        let cause = ConicalStockFailure::SpatialLimit {
            upper: 2e-7,
            limit: 1e-7,
            collar_guard: 1e-8,
        };
        let failure = stock_refusal(ATTEMPT, CylinderFeatureStage::Admission, cause);
        assert_eq!(failure.stage, CylinderFeatureStage::Admission);
        let CylinderFeatureCause::ConicalStock(cause) = *failure.cause else {
            panic!("expected typed stock refusal");
        };
        assert!(matches!(
            *cause,
            ConicalStockFailure::SpatialLimit {
                upper: 2e-7,
                limit: 1e-7,
                collar_guard: 1e-8,
            }
        ));
    }

    #[test]
    fn decoded_stock_refusal_preserves_the_original_native_failure() {
        let native = Failure::with_type("Standard_OutOfRange", "rim index 7", "stock curve");
        let failure = stock_refusal(
            ATTEMPT,
            CylinderFeatureStage::Validation,
            native.clone().into(),
        );
        assert_eq!(failure.stage, CylinderFeatureStage::Validation);
        let CylinderFeatureCause::ConicalStock(cause) = *failure.cause else {
            panic!("expected typed stock refusal");
        };
        assert!(matches!(*cause, ConicalStockFailure::Native(actual) if actual == native));
    }
}
