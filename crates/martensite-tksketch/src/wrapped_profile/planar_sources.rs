//! Owned source coefficients prevent a planar identity from certifying later geometry edits.
use super::*;
#[derive(Debug)]
pub(in crate::surface_sketch::wrapped_profile) struct SourceSnapshot(
    BTreeMap<SurfaceCurveId, SourceCurve>,
);
#[derive(Debug)]
struct SourceCurve {
    span: CurveSpan,
    start: CornerSource,
    end: CornerSource,
    poles: Vec<[u64; 2]>,
}
impl SourceSnapshot {
    pub fn capture(request: &DevelopedRegionRequest) -> Self {
        Self(
            request
                .loops
                .iter()
                .flatten()
                .map(|piece| (piece.span.curve, SourceCurve::capture(piece)))
                .collect(),
        )
    }
    pub fn matches(&self, request: &DevelopedRegionRequest) -> bool {
        self.0.len() == request.loops.iter().map(Vec::len).sum::<usize>()
            && request
                .loops
                .iter()
                .flatten()
                .all(|p| self.0.get(&p.span.curve).is_some_and(|old| old.matches(p)))
    }
}
impl SourceCurve {
    fn capture(piece: &AuthoredPiece) -> Self {
        Self {
            span: piece.span,
            start: piece.start,
            end: piece.end,
            poles: piece.poles.iter().map(|p| p.map(f64::to_bits)).collect(),
        }
    }
    fn matches(&self, piece: &AuthoredPiece) -> bool {
        self.span == piece.span
            && self.start == piece.start
            && self.end == piece.end
            && self
                .poles
                .iter()
                .copied()
                .eq(piece.poles.iter().map(|p| p.map(f64::to_bits)))
    }
}
