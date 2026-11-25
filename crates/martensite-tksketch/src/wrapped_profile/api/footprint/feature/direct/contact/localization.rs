//! Convert same-owner proofs into the permitted positive-depth contact domain.
use super::{carrier::Separation, collar::*, *};

type Collars = BTreeMap<CylinderBodyRef, Receipt>;

impl SourceGraph {
    pub(in super::super) fn localize(
        &self,
        attempt: Attempt,
        separation: &Separation,
        stock: &stock_binding::BoundStock,
        sew: f64,
    ) -> FeatureChecked<Collars> {
        let frame = separation.stock;
        let [r1, r2, r3] = frame.axes;
        let [l1, l2, l3] = frame.location.matrix;
        let datum = StockSource {
            angle: frame.semi_angle,
            radius: frame.radius,
            native_scale: frame.location.scale,
            origin: frame.origin,
            basis: [r1, r2, r3]
                .concat()
                .try_into()
                .map_err(|_| refused(attempt, "stock static basis"))?,
            location: frame.location.translation,
            location_basis: [l1, l2, l3]
                .concat()
                .try_into()
                .map_err(|_| refused(attempt, "stock static location basis"))?,
            sense: if attempt.kind == CylinderFeatureKind::Boss {
                Sense::Boss
            } else {
                Sense::Pocket
            },
        };
        let support = self.owner_tolerances.values().copied().collect::<Vec<_>>();
        let policy = PolicySource {
            fuzzy: martensite_tkernel::precision::confusion(),
            sew,
            class_maxima: self.limits.class_maxima,
            support_face_tolerances: &support,
        };
        let issuer =
            Issuer::new(&datum, policy).map_err(|cause| localization_failure(attempt, cause))?;
        let mut result = Collars::new();
        for ((key, rail), evidence) in self.rails.iter().zip(separation.proof.sides()) {
            if evidence.id() != key.ordinal as u64 {
                return Err(refused(
                    attempt,
                    "bound side identity for the contact collar",
                ));
            }
            let guarded_sources_normal_upper = rail
                .first
                .endpoints
                .iter()
                .try_fold(
                    stock.spatial_upper.max(evidence.zero_normal_upper()),
                    |upper, &point| {
                        issuer
                            .stock_point_normal_upper(point)
                            .map(|value| upper.max(value))
                    },
                )
                .map_err(|cause| localization_failure(attempt, cause))?;
            let bound = issuer
                .issue(
                    &SideSource {
                        normalized_domain: [0., 1., 0., 1.],
                        affine: rail.first.location,
                        first: &rail.first.poles,
                        last: &rail.last.poles,
                        seam_uv: &rail.poles,
                        seam_knots: &rail.knots,
                        seam_tolerance: rail.first.tolerance,
                        side_tolerance: self.limits.entity(rail.side)?,
                    },
                    |_, _| {
                        Some(UpstreamEvidence {
                            first_layer_lower: evidence.signed_normal_lower()[0],
                            same_parameter_gap_upper: rail.gap_upper,
                            guarded_sources_normal_upper,
                        })
                    },
                )
                .map_err(|cause| localization_failure(attempt, cause))?;
            if bound.face().normalized_depth_upper > bound.all().normalized_depth_upper {
                return Err(refused(
                    attempt,
                    "face contact collar inside the complete pair collar",
                ));
            }
            result.insert(*key, bound);
        }
        if result.len() != self.rails.len() {
            return Err(refused(
                attempt,
                "complete native positive-depth contact collar",
            ));
        }
        Ok(result)
    }
}

fn localization_failure(attempt: Attempt, cause: collar::Refusal) -> CylinderFeatureFailure {
    attempt.fail(
        CylinderFeatureStage::Validation,
        CylinderFeatureCause::Native {
            query: "positive-depth contact collar",
            cause: Box::new(Failure::new(&format!("{cause:?}"))),
        },
    )
}
