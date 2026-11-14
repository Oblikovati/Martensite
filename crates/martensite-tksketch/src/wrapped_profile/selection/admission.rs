//! Shared support settings and globally unique source names precede planar construction.
use super::*;
use std::collections::BTreeSet;
const MAX_REGIONS: usize = 64;
const MAX_AUTHORED_CURVES: usize = 4096;
#[derive(Default)]
struct SourceOwners {
    regions: BTreeSet<SurfaceRegionId>,
    curves: BTreeMap<SurfaceCurveId, SurfaceRegionId>,
    vertices: BTreeMap<CornerSource, SurfaceRegionId>,
}
pub(super) fn admit(requests: &[Request]) -> Result<f64, SelectionFailure> {
    let first = requests.first().ok_or(SelectionFailure::Empty)?;
    limit("selected regions", requests.len(), MAX_REGIONS)?;
    let sources = requests.iter().flat_map(|r| &r.loops).map(Vec::len).sum();
    limit("authored curves", sources, MAX_AUTHORED_CURVES)?;
    let mut owners = SourceOwners::default();
    for request in requests {
        region(request, first, &mut owners)?;
    }
    Ok(first.tolerance)
}
fn region(
    request: &Request,
    first: &Request,
    owners: &mut SourceOwners,
) -> Result<(), SelectionFailure> {
    pipeline::admit(request).map_err(|cause| SelectionFailure::Admission {
        region: request.region,
        cause,
    })?;
    if !owners.regions.insert(request.region) {
        return Err(SelectionFailure::DuplicateRegion(request.region));
    }
    common_context(request, first)?;
    for piece in request.loops.iter().flatten() {
        source(request.region, piece, owners)?;
    }
    Ok(())
}
fn source(
    region: SurfaceRegionId,
    piece: &AuthoredPiece,
    owners: &mut SourceOwners,
) -> Result<(), SelectionFailure> {
    if let Some(first) = owners.curves.insert(piece.span.curve, region) {
        return Err(SelectionFailure::DuplicateCurve {
            curve: piece.span.curve,
            owners: [first, region],
        });
    }
    if let Some(first) = owners.vertices.insert(piece.start, region) {
        return Err(SelectionFailure::DuplicateVertex {
            vertex: piece.start,
            owners: [first, region],
        });
    }
    Ok(())
}
fn common_context(request: &Request, first: &Request) -> Result<(), SelectionFailure> {
    let actual = context(request);
    let expected = context(first);
    if actual != expected {
        return Err(SelectionFailure::Context {
            region: request.region,
            actual,
            expected,
        });
    }
    Ok(())
}
fn context(request: &Request) -> [u64; 4] {
    [
        request.radius,
        request.depths[0],
        request.depths[1],
        request.tolerance,
    ]
    .map(f64::to_bits)
}
fn limit(role: &'static str, actual: usize, maximum: usize) -> Result<(), SelectionFailure> {
    if actual > maximum {
        return Err(SelectionFailure::Limit {
            role,
            actual,
            maximum,
        });
    }
    Ok(())
}
