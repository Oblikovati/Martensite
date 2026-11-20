//! Retain an authored incidence once while projecting it into the owning fit chart.
use super::*;
use cone_mapping::fit::{FitFailure, FitInput, IdealEndpoint, SourceIncidence};

#[derive(Default)]
pub(super) struct EndpointPool(BTreeMap<CornerSource, PooledEndpoint>);

#[derive(Clone, Copy, PartialEq, Eq)]
struct ProjectionKey {
    chart: [u64; 3],
    turns: [i32; 2],
}
impl ProjectionKey {
    fn at(input: &FitInput<'_>, end: usize) -> Self {
        Self {
            chart: [input.semi_angle, input.origin, input.center].map(f64::to_bits),
            turns: [input.developed_turns[end], input.physical_turns[end]],
        }
    }
}

struct PooledEndpoint {
    source: [u64; 2],
    incidence: SourceIncidence,
    key: ProjectionKey,
    projection: IdealEndpoint,
}
impl EndpointPool {
    pub(super) fn pair(
        &mut self,
        region: SurfaceRegionId,
        piece: &AuthoredPiece,
        input: &FitInput<'_>,
    ) -> ConeChecked<[IdealEndpoint; 2]> {
        Ok([
            self.endpoint(region, piece, input, 0)?,
            self.endpoint(region, piece, input, 1)?,
        ])
    }

    fn endpoint(
        &mut self,
        region: SurfaceRegionId,
        piece: &AuthoredPiece,
        input: &FitInput<'_>,
        end: usize,
    ) -> ConeChecked<IdealEndpoint> {
        let (corner, point) = incidence(piece, end);
        let key = canonical(corner);
        if let Some(previous) = self.0.get_mut(&key) {
            bind(region, piece.span, point, previous.source)?;
            return candidate(region, piece.span, previous.project(input, end));
        }
        let endpoint = candidate(
            region,
            piece.span,
            PooledEndpoint::capture(point, input, end),
        )?;
        let projection = endpoint.projection;
        self.0.insert(key, endpoint);
        Ok(projection)
    }
}

impl PooledEndpoint {
    fn capture(point: [f64; 2], input: &FitInput<'_>, end: usize) -> Result<Self, FitFailure> {
        let incidence = SourceIncidence::capture(input, end)?;
        Ok(Self {
            source: point.map(f64::to_bits),
            key: ProjectionKey::at(input, end),
            incidence,
            projection: incidence.project(input, end)?,
        })
    }

    fn project(&mut self, input: &FitInput<'_>, end: usize) -> Result<IdealEndpoint, FitFailure> {
        let key = ProjectionKey::at(input, end);
        if self.key != key {
            self.projection = self.incidence.project(input, end)?;
            self.key = key;
        }
        Ok(self.projection)
    }
}

fn candidate<T>(
    region: SurfaceRegionId,
    span: CurveSpan,
    result: Result<T, FitFailure>,
) -> ConeChecked<T> {
    result.map_err(|cause| failure(Some(region), Some(span), ConeCause::Candidate(cause)))
}

fn incidence(piece: &AuthoredPiece, endpoint: usize) -> (CornerSource, [f64; 2]) {
    if endpoint == 0 {
        return (piece.start, piece.poles[0]);
    }
    (piece.end, piece.poles[piece.poles.len() - 1])
}
pub(super) fn canonical(source: CornerSource) -> CornerSource {
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
    expected: [u64; 2],
) -> ConeChecked<()> {
    if expected == actual.map(f64::to_bits) {
        return Ok(());
    }
    Err(failure(
        Some(region),
        Some(span),
        ConeCause::Binding {
            field: "shared ideal incidence source",
            actual: actual.map(f64::to_bits).to_vec(),
            expected: expected.to_vec(),
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
        let expected = [4., 0.].map(f64::to_bits);
        let span = CurveSpan {
            curve: SurfaceCurveId(4),
            index: 0,
            level: 0,
        };
        assert!(bind(SurfaceRegionId(1), span, [4., 0.], expected).is_ok());
        assert!(bind(SurfaceRegionId(1), span, [5., 0.], expected).is_err());
    }
}
