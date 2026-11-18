//! One authored incidence owns each rounded UV endpoint before native construction.
use super::*;
use martensite_cmath::{hypot, sin};
#[derive(Clone, Copy)]
struct Endpoint {
    point: [f64; 2],
    uv: [f64; 2],
}
pub(super) fn uv_endpoints(prepared: &mut Prepared) -> ConeChecked<()> {
    let mut corners = BTreeMap::new();
    for piece in prepared.source.loops.iter().flatten() {
        for (source, point) in [
            (piece.start, piece.poles[0]),
            (piece.end, piece.poles[piece.poles.len() - 1]),
        ] {
            insert(prepared, piece.span, source, point, &mut corners)?;
        }
    }
    apply(prepared, &corners)
}
fn insert(
    prepared: &Prepared,
    span: CurveSpan,
    source: CornerSource,
    point: [f64; 2],
    corners: &mut BTreeMap<CornerSource, Endpoint>,
) -> ConeChecked<()> {
    if let Some(previous) = corners.get(&source) {
        if previous.point != point {
            return Err(mismatch(
                prepared.source.region,
                span,
                previous.point,
                point,
            ));
        }
        return Ok(());
    }
    let u = prepared.chart_origin
        + native::developed_angle(point, prepared.wedge) / sin(prepared.semi_angle);
    let v = hypot(point[0], point[1]) - prepared.reference_slant;
    corners.insert(source, Endpoint { point, uv: [u, v] });
    Ok(())
}
fn apply(prepared: &mut Prepared, corners: &BTreeMap<CornerSource, Endpoint>) -> ConeChecked<()> {
    for piece in prepared.source.loops.iter().flatten() {
        let fit = prepared.fits.get_mut(&piece.span).ok_or_else(|| {
            failure(
                Some(prepared.source.region),
                Some(piece.span),
                ConeCause::Missing(prepared.source.region),
            )
        })?;
        for (index, source) in [(0, piece.start), (3, piece.end)] {
            let endpoint = corners.get(&source).ok_or_else(|| {
                failure(
                    Some(prepared.source.region),
                    Some(piece.span),
                    ConeCause::Missing(prepared.source.region),
                )
            })?;
            fit.uv[index] = endpoint.uv;
        }
    }
    Ok(())
}
fn mismatch(
    region: SurfaceRegionId,
    span: CurveSpan,
    expected: [f64; 2],
    actual: [f64; 2],
) -> ConeFailure {
    failure(
        Some(region),
        Some(span),
        ConeCause::Binding {
            field: "shared source corner",
            actual: actual.map(f64::to_bits).to_vec(),
            expected: expected.map(f64::to_bits).to_vec(),
        },
    )
}
