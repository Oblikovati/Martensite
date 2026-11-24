//! A common-chart reference computes ideal incidences once by authored or dyadic identity.
use super::*;
use cone_mapping::fit::{FitInput, IdealEndpoint};
#[derive(Default)]
pub(super) struct EndpointPool(BTreeMap<CornerSource, PooledEndpoint>);
#[derive(Clone, Copy)]
struct PooledEndpoint {
    source: [u64; 2],
    ideal: IdealEndpoint,
}
impl EndpointPool {
    pub(super) fn pair(
        &mut self,
        prepared: &Prepared,
        piece: &AuthoredPiece,
        input: &FitInput<'_>,
    ) -> ConeChecked<[IdealEndpoint; 2]> {
        Ok([
            self.endpoint(prepared, piece, input, 0)?,
            self.endpoint(prepared, piece, input, 1)?,
        ])
    }
    fn endpoint(
        &mut self,
        prepared: &Prepared,
        piece: &AuthoredPiece,
        input: &FitInput<'_>,
        endpoint: usize,
    ) -> ConeChecked<IdealEndpoint> {
        let (corner, point) = incidence(piece, endpoint);
        let key = canonical(corner);
        if let Some(previous) = self.0.get(&key) {
            return bind(prepared.source.region, piece.span, point, previous);
        }
        let ideal = cone_mapping::fit::ideal_endpoint(input, endpoint).map_err(|cause| {
            failure(
                Some(prepared.source.region),
                Some(piece.span),
                ConeCause::Candidate(cause),
            )
        })?;
        self.0.insert(
            key,
            PooledEndpoint {
                source: point.map(f64::to_bits),
                ideal,
            },
        );
        Ok(ideal)
    }
}
fn incidence(piece: &AuthoredPiece, endpoint: usize) -> (CornerSource, [f64; 2]) {
    if endpoint == 0 {
        return (piece.start, piece.poles[0]);
    }
    (piece.end, piece.poles[piece.poles.len() - 1])
}
fn canonical(source: CornerSource) -> CornerSource {
    let CornerSource::Split {
        curve,
        numerator,
        level,
    } = source
    else {
        return source;
    };
    let shift = numerator.trailing_zeros().min(u32::from(level));
    CornerSource::Split {
        curve,
        numerator: numerator >> shift,
        level: level - shift as u8,
    }
}
fn bind(
    region: SurfaceRegionId,
    span: CurveSpan,
    actual: [f64; 2],
    previous: &PooledEndpoint,
) -> ConeChecked<IdealEndpoint> {
    if previous.source == actual.map(f64::to_bits) {
        return Ok(previous.ideal);
    }
    Err(failure(
        Some(region),
        Some(span),
        ConeCause::Binding {
            field: "shared ideal incidence source",
            actual: actual.map(f64::to_bits).to_vec(),
            expected: previous.source.to_vec(),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn dyadic(numerator: u32, level: u8) -> CornerSource {
        CornerSource::Split {
            curve: SurfaceCurveId(4),
            numerator,
            level,
        }
    }
    #[test]
    fn canonical_dyadic_incidence_uses_authored_parameter_identity() {
        assert_eq!(canonical(dyadic(2, 2)), dyadic(1, 1));
        assert_ne!(
            canonical(dyadic(1, 1)),
            CornerSource::Authored(SurfaceVertexId(4))
        );
    }
    #[test]
    fn pooled_identity_refuses_a_different_source_point() {
        let previous = PooledEndpoint {
            source: [4., 0.].map(f64::to_bits),
            ideal: IdealEndpoint {
                tangent: 0.,
                slant: 4.,
            },
        };
        let span = CurveSpan {
            curve: SurfaceCurveId(4),
            index: 0,
            level: 0,
        };
        assert!(bind(SurfaceRegionId(1), span, [4., 0.], &previous).is_ok());
        assert!(bind(SurfaceRegionId(1), span, [5., 0.], &previous).is_err());
    }
}
