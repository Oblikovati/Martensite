//! Corner identity follows exact authored parameter endpoints, never fitted coordinates.
use super::*;
pub(super) fn bind(originals: &[Request], prepared: &[Request]) -> Checked<()> {
    for (original, retained) in originals.iter().zip(prepared) {
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
