use super::{
    stock::{Faces, OriginalBoundary, StockFrustumReceipt, StockRefusal},
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
            topods::face(bound).map_err(|_| refused())
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
            .map_err(|_| refused())?;
        let [band, base, lid] = self.images;
        Ok(BoundStock {
            band,
            ends: [base, lid],
            spatial_upper: self.proof.spatial_upper(),
            boundary,
        })
    }
}

pub(super) fn stock_refusal(attempt: Attempt, cause: StockRefusal) -> CylinderFeatureFailure {
    attempt.fail(
        ASSEMBLY,
        CylinderFeatureCause::Native {
            query: "direct stock frustum certificate",
            cause: Box::new(Failure::new(&cause.to_string())),
        },
    )
}
