//! Connect the exact contour census to the live decoded rail references.
use super::{pieces::WindowsInput, prefix::*, *};

pub(in super::super) struct Windows {
    pub(super) cone: prefix::Cone,
    pub(super) pieces: Vec<prefix::Piece>,
    pub(super) census: prefix::Census,
}

impl WindowsInput {
    pub(in super::super) fn census(self, attempt: Attempt) -> FeatureChecked<Windows> {
        let cone = prefix::Cone {
            radius: self.radius,
            sin: self.sin,
            slant: self.slant,
            chart_angle: [0., 3.],
        };
        let pieces = self
            .pieces
            .iter()
            .map(|shape| {
                let id = PieceId(Edge(ordinal(attempt, shape.edge)?), shape.span);
                let contour = Loop(Face(ordinal(attempt, shape.contour.0)?), shape.contour.1);
                let vertex = |v: CylinderBodyRef| ordinal(attempt, v).map(Vertex);
                let [a, b] = shape.endpoints;
                Ok(prefix::Piece {
                    id,
                    contour,
                    endpoints: [a.map(vertex).transpose()?, b.map(vertex).transpose()?],
                    uv: shape.uv,
                })
            })
            .collect::<FeatureChecked<Vec<_>>>()?;
        let census = prefix::census(CensusInput {
            cone,
            pieces: &pieces,
            endpoint_window_threshold: self.window_threshold,
            native_rim_lower: self.rim,
            native_periodic_lower: self.periodic,
        })
        .map_err(|cause| contact_failure(attempt, "stored support contact domain census", cause))?;
        Ok(Windows {
            cone,
            pieces,
            census,
        })
    }
}

pub(super) fn ordinal(attempt: Attempt, shape: CylinderBodyRef) -> FeatureChecked<u32> {
    u32::try_from(shape.ordinal).map_err(|_| refused(attempt, "bounded native contact identity"))
}

pub(super) fn contact_failure(
    attempt: Attempt,
    query: &'static str,
    cause: impl std::fmt::Debug,
) -> CylinderFeatureFailure {
    attempt.fail(
        CylinderFeatureStage::Validation,
        CylinderFeatureCause::Native {
            query,
            cause: Box::new(martensite_tkernel::standard::Failure::new(&format!(
                "{cause:?}"
            ))),
        },
    )
}
