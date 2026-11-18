//! Corner identity follows exact authored parameter endpoints, never fitted coordinates.
use super::*;
pub(super) fn bind<O, P>(originals: &[O], prepared: &[P]) -> Checked<()>
where
    O: AsRef<DevelopedRegionRequest>,
    P: AsRef<DevelopedRegionRequest>,
{
    for (original, retained) in originals.iter().zip(prepared) {
        let (original, retained) = (original.as_ref(), retained.as_ref());
        bind_region(original, retained)?;
    }
    Ok(())
}
fn bind_region(
    original: &DevelopedRegionRequest,
    retained: &DevelopedRegionRequest,
) -> Checked<()> {
    let authored: BTreeMap<_, _> = original
        .loops
        .iter()
        .flatten()
        .map(|p| (p.span.curve, p))
        .collect();
    for piece in retained.loops.iter().flatten() {
        let source = authored
            .get(&piece.span.curve)
            .ok_or(HistoryFailure::SourceBinding)?;
        endpoints(source, piece)?;
    }
    Ok(())
}
fn endpoints(original: &AuthoredPiece, retained: &AuthoredPiece) -> Checked<()> {
    let span = retained.span;
    let actual = [retained.start, retained.end];
    let expected = [
        corner(original, span, u64::from(span.index))?,
        corner(original, span, u64::from(span.index) + 1)?,
    ];
    if actual != expected {
        return Err(HistoryFailure::Corners {
            span,
            actual,
            expected,
        });
    }
    Ok(())
}
fn corner(original: &AuthoredPiece, span: CurveSpan, numerator: u64) -> Checked<CornerSource> {
    let whole = 1u64
        .checked_shl(u32::from(span.level))
        .ok_or(HistoryFailure::SourceBinding)?;
    if numerator == 0 {
        return Ok(original.start);
    }
    if numerator == whole {
        return Ok(original.end);
    }
    let shift = numerator.trailing_zeros().min(u32::from(span.level));
    let reduced = u32::try_from(numerator >> shift).map_err(|_| HistoryFailure::SourceBinding)?;
    Ok(CornerSource::Split {
        curve: span.curve,
        numerator: reduced,
        level: span.level - shift as u8,
    })
}

pub(super) fn bind_developed<O, P>(originals: &[O], prepared: &[P]) -> Checked<()>
where
    O: AsRef<DevelopedRegionRequest>,
    P: AsRef<DevelopedRegionRequest>,
{
    if originals.is_empty() || originals.len() != prepared.len() {
        return Err(HistoryFailure::SourceBinding);
    }
    bind(originals, prepared)?;
    for (original, retained) in originals.iter().zip(prepared) {
        complete_region(original.as_ref(), retained.as_ref())?;
    }
    Ok(())
}

fn complete_region(
    original: &DevelopedRegionRequest,
    retained: &DevelopedRegionRequest,
) -> Checked<()> {
    let expected = authored_curves(original);
    let spans = retained_spans(retained);
    if original.region != retained.region
        || !spans.keys().copied().eq(expected)
        || !spans
            .values()
            .all(|group| planar_clearance::intervals::partition(group))
    {
        return Err(HistoryFailure::SourceBinding);
    }
    Ok(())
}

fn authored_curves(original: &DevelopedRegionRequest) -> BTreeSet<SurfaceCurveId> {
    original
        .loops
        .iter()
        .flatten()
        .map(|p| p.span.curve)
        .collect()
}

fn retained_spans(retained: &DevelopedRegionRequest) -> BTreeMap<SurfaceCurveId, Vec<CurveSpan>> {
    let mut groups: BTreeMap<SurfaceCurveId, Vec<CurveSpan>> = BTreeMap::new();
    for piece in retained.loops.iter().flatten() {
        groups.entry(piece.span.curve).or_default().push(piece.span);
    }
    groups
}
