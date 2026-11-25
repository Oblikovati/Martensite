//! Every directed row end and native vertex consumes one final source incidence.
use super::*;
#[derive(Clone, Copy, Debug, PartialEq)]
struct FinalEndpoint {
    source: [u64; 2],
    nearest: [[u64; 3]; 2],
    points: [[f64; 3]; 2],
}
#[derive(Default)]
pub(in crate::surface_sketch::wrapped_profile) struct FinalEndpoints {
    entries: BTreeMap<CornerSource, FinalEndpoint>,
}
impl FinalEndpoints {
    pub(super) fn plan(
        prepared: &[Prepared],
        gamma: &ConicalDirectedGammaEvidence,
    ) -> ConeChecked<Self> {
        let mut table = Self::default();
        for member in prepared {
            for piece in member.source.loops.iter().flatten() {
                table.plan_piece(member, piece, gamma)?;
            }
        }
        Ok(table)
    }
    fn plan_piece(
        &mut self,
        member: &Prepared,
        piece: &AuthoredPiece,
        gamma: &ConicalDirectedGammaEvidence,
    ) -> ConeChecked<()> {
        let fit = super::fit(member, piece)?;
        for end in [0, 1] {
            self.plan_end(member, piece, fit, end, gamma)?;
        }
        Ok(())
    }
    fn plan_end(
        &mut self,
        member: &Prepared,
        piece: &AuthoredPiece,
        fit: &SpanFit,
        end: usize,
        gamma: &ConicalDirectedGammaEvidence,
    ) -> ConeChecked<()> {
        let (key, source) = incidence(piece, end);
        let nearest = end_points(fit, end);
        let points = self.planned_points(
            member.source.region,
            piece.span,
            key,
            source,
            nearest,
            gamma,
        )?;
        self.insert(
            member.source.region,
            piece.span,
            key,
            source,
            nearest,
            points,
        )?;
        Ok(())
    }
    fn insert(
        &mut self,
        region: SurfaceRegionId,
        span: CurveSpan,
        key: CornerSource,
        source: [f64; 2],
        nearest: [[f64; 3]; 2],
        points: [[f64; 3]; 2],
    ) -> ConeChecked<()> {
        let key = super::super::endpoints::canonical(key);
        let current = FinalEndpoint {
            source: source.map(f64::to_bits),
            nearest: nearest.map(|row| row.map(f64::to_bits)),
            points,
        };
        if let Some(previous) = self.entries.get(&key) {
            return compare(region, span, previous, &current);
        }
        self.entries.insert(key, current);
        Ok(())
    }
    fn planned_points(
        &self,
        region: SurfaceRegionId,
        span: CurveSpan,
        key: CornerSource,
        source: [f64; 2],
        nearest: [[f64; 3]; 2],
        gamma: &ConicalDirectedGammaEvidence,
    ) -> ConeChecked<[[f64; 3]; 2]> {
        if let Some(previous) = self.entries.get(&super::super::endpoints::canonical(key)) {
            let current = FinalEndpoint {
                source: source.map(f64::to_bits),
                nearest: nearest.map(|row| row.map(f64::to_bits)),
                points: previous.points,
            };
            compare(region, span, previous, &current)?;
            return Ok(previous.points);
        }
        transformed(region, span, nearest, gamma)
    }
    pub(super) fn validate(&self, prepared: &[Prepared]) -> ConeChecked<()> {
        for member in prepared {
            for piece in member.source.loops.iter().flatten() {
                self.validate_piece(member, piece)?;
            }
        }
        Ok(())
    }
    fn validate_piece(&self, member: &Prepared, piece: &AuthoredPiece) -> ConeChecked<()> {
        let fit = super::fit(member, piece)?;
        for end in [0, 1] {
            let (key, source) = incidence(piece, end);
            let expected = self.lookup(member.source.region, piece.span, key, source)?;
            let actual = end_points(fit, end);
            if actual.map(|row| row.map(f64::to_bits)) != expected.map(|row| row.map(f64::to_bits))
            {
                return Err(mismatch(
                    member.source.region,
                    piece.span,
                    "final row incidence",
                    point_bits(actual),
                    point_bits(expected),
                ));
            }
        }
        Ok(())
    }
    pub(in crate::surface_sketch::wrapped_profile::operation::cone) fn lookup(
        &self,
        region: SurfaceRegionId,
        span: CurveSpan,
        key: CornerSource,
        source: [f64; 2],
    ) -> ConeChecked<[[f64; 3]; 2]> {
        let key = super::super::endpoints::canonical(key);
        let previous = self
            .entries
            .get(&key)
            .ok_or_else(|| mismatch(region, span, "final endpoint membership", vec![], vec![1]))?;
        check_source(region, span, source, previous)?;
        Ok(previous.points)
    }
}
fn incidence(piece: &AuthoredPiece, end: usize) -> (CornerSource, [f64; 2]) {
    match end {
        0 => (piece.start, piece.poles[0]),
        _ => (piece.end, piece.poles[piece.poles.len() - 1]),
    }
}
fn compare(
    region: SurfaceRegionId,
    span: CurveSpan,
    previous: &FinalEndpoint,
    current: &FinalEndpoint,
) -> ConeChecked<()> {
    let old = bits(previous);
    let new = bits(current);
    if old != new {
        return Err(mismatch(region, span, "final shared incidence", new, old));
    }
    Ok(())
}
fn check_source(
    region: SurfaceRegionId,
    span: CurveSpan,
    source: [f64; 2],
    previous: &FinalEndpoint,
) -> ConeChecked<()> {
    if previous.source != source.map(f64::to_bits) {
        return Err(mismatch(
            region,
            span,
            "final incidence source",
            source.map(f64::to_bits).to_vec(),
            previous.source.to_vec(),
        ));
    }
    Ok(())
}
fn bits(endpoint: &FinalEndpoint) -> Vec<u64> {
    endpoint
        .source
        .into_iter()
        .chain(endpoint.nearest.into_iter().flatten())
        .chain(endpoint.points.into_iter().flatten().map(f64::to_bits))
        .collect()
}
fn mismatch(
    region: SurfaceRegionId,
    span: CurveSpan,
    field: &'static str,
    actual: Vec<u64>,
    expected: Vec<u64>,
) -> ConeFailure {
    failure(
        Some(region),
        Some(span),
        ConeCause::Binding {
            field,
            actual,
            expected,
        },
    )
}
fn transformed(
    region: SurfaceRegionId,
    span: CurveSpan,
    mut points: [[f64; 3]; 2],
    gamma: &ConicalDirectedGammaEvidence,
) -> ConeChecked<[[f64; 3]; 2]> {
    for (row, point) in points.iter_mut().enumerate() {
        point[2] = gamma.transform_z(row, point[2]).map_err(|cause| {
            failure(
                Some(region),
                Some(span),
                ConeCause::ActualPatch(Box::new(cause)),
            )
        })?;
    }
    Ok(points)
}
fn end_points(fit: &SpanFit, end: usize) -> [[f64; 3]; 2] {
    fit.poles
        .each_ref()
        .map(|row| row.points[if end == 0 { 0 } else { row.points.len() - 1 }])
}
fn point_bits(points: [[f64; 3]; 2]) -> Vec<u64> {
    points.into_iter().flatten().map(f64::to_bits).collect()
}
#[cfg(test)]
mod tests;
