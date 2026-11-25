//! Resolve original curves without retaining a second copy of their control points.
use super::*;
pub(super) struct AuthoredPositions(BTreeMap<SurfaceCurveId, [usize; 2]>);

impl AuthoredPositions {
    pub(super) fn capture(source: &DevelopedRegionRequest) -> Self {
        let mut positions = BTreeMap::new();
        for (contour, pieces) in source.loops.iter().enumerate() {
            for (ordinal, piece) in pieces.iter().enumerate() {
                positions
                    .entry(piece.span.curve)
                    .or_insert([contour, ordinal]);
            }
        }
        Self(positions)
    }
    fn resolve<'a>(
        &self,
        original: &'a DevelopedRegionRequest,
        curve: SurfaceCurveId,
    ) -> Option<&'a AuthoredPiece> {
        let &[contour, ordinal] = self.0.get(&curve)?;
        let piece = original.loops.get(contour)?.get(ordinal)?;
        (piece.span.curve == curve).then_some(piece)
    }
}

impl Prepared {
    pub(super) fn authored_piece<'a>(
        &self,
        original: &'a DevelopedRegionRequest,
        curve: SurfaceCurveId,
    ) -> Option<&'a AuthoredPiece> {
        if self.source.region != original.region {
            return None;
        }
        self.authored_positions.resolve(original, curve)
    }
}
