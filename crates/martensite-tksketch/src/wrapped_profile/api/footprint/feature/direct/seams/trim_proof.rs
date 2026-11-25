//! Receipt for a directed rounding enclosure under the private direct policy.
use super::*;

pub(super) struct TrimProof(ProofKind);

enum ProofKind {
    Filtered {
        source: Box<fast::Query>,
        bounds: [f64; 3],
    },
    Dyadic(CertifiedConicalTrim),
}

impl TrimProof {
    pub(super) fn dyadic(proof: CertifiedConicalTrim) -> Self {
        Self(ProofKind::Dyadic(proof))
    }

    pub(super) fn filtered(query: ConicalTrimQuery<'_>) -> Option<Self> {
        let source = capture(query)?;
        let bounds = fast::certify(&source)?;
        Some(Self(ProofKind::Filtered {
            source: Box::new(source),
            bounds,
        }))
    }

    pub(super) fn residual_bounds(&self) -> [f64; 3] {
        match &self.0 {
            ProofKind::Filtered { bounds, .. } => *bounds,
            ProofKind::Dyadic(proof) => proof.residual_bounds(),
        }
    }

    pub(super) fn matches_source(&self, query: ConicalTrimQuery<'_>) -> bool {
        match &self.0 {
            ProofKind::Filtered { source, .. } => {
                capture(query).is_some_and(|other| words(source).eq(words(&other)))
            }
            ProofKind::Dyadic(proof) => proof.matches_source(query),
        }
    }
}

fn capture(query: ConicalTrimQuery<'_>) -> Option<fast::Query> {
    let poles: &[[f64; 3]; 10] = query.curve.poles.try_into().ok()?;
    let weights: &[f64; 10] = query.curve.weights?.try_into().ok()?;
    let uv: &[[f64; 2]; 4] = query.pcurve.try_into().ok()?;
    if query.options != Default::default() {
        return None;
    }
    let curve = std::array::from_fn(|i| {
        let [x, y, z] = poles[i];
        [x, y, z, weights[i]]
    });
    Some(fast::Query {
        angle: query.carrier.semi_angle,
        radius: query.carrier.radius,
        origin: query.carrier.axial_origin,
        limit: query.spatial_limit,
        curve,
        uv: *uv,
    })
}

fn words(query: &fast::Query) -> impl Iterator<Item = u64> + '_ {
    [query.angle, query.radius, query.origin, query.limit]
        .into_iter()
        .chain(query.curve.iter().flatten().copied())
        .chain(query.uv.iter().flatten().copied())
        .map(f64::to_bits)
}
