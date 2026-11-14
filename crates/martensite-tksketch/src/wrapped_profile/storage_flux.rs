//! Conditional flux bound from decoded, recaptured and source-bound side/cap representations.
use super::*;
pub(super) struct FluxSpan {
    curves: [OwnedCurve; 2],
    cap_pcurves: [Vec<[f64; 2]>; 2],
    side_reversed: bool,
}
impl FluxSpan {
    pub(super) fn borrowed(&self) -> CylindricalFluxSpan<'_> {
        CylindricalFluxSpan {
            boundaries: [&self.curves[0], &self.curves[1]].map(OwnedCurve::borrowed),
            cap_pcurves: [&self.cap_pcurves[0], &self.cap_pcurves[1]],
            side_reversed: self.side_reversed,
        }
    }
}

pub(in super::super) fn certify_flux(
    original: &Request,
    prepared: &Request,
    bindings: &[(BindingKey, Shape)],
    proofs: &[StoredProof],
) -> Result<CertifiedCylindricalFlux, StorageFailure> {
    let parts = capture_parts(prepared, bindings, proofs)?;
    let spans: Vec<_> = parts.iter().map(FluxSpan::borrowed).collect();
    let query = flux_query(&spans, prepared);
    certify_cylindrical_boundary_flux(query, || false)
        .map_err(|cause| scalar_failure(original.region, cause))
}

pub(super) fn capture_parts(
    prepared: &Request,
    bindings: &[(BindingKey, Shape)],
    proofs: &[StoredProof],
) -> Result<Vec<FluxSpan>, StorageFailure> {
    let indexed = index(bindings)?;
    capture::caps(prepared, &indexed)?;
    capture_spans(prepared, &indexed, proofs)
}

pub(super) fn flux_query<'a>(
    spans: &'a [CylindricalFluxSpan<'a>],
    prepared: &Request,
) -> CylindricalFluxQuery<'a> {
    CylindricalFluxQuery {
        spans,
        radii: prepared.depths.map(|d| prepared.radius + d),
        options: CylindricalFluxOptions {
            max_spans: 4096,
            ..Default::default()
        },
    }
}

fn scalar_failure(region: SurfaceRegionId, cause: CylindricalFluxError) -> StorageFailure {
    StorageFailure {
        key: region_key(region),
        cause: StorageCause::Flux(Box::new(cause)),
    }
}

pub(super) fn capture_spans(
    prepared: &Request,
    bindings: &Bindings<'_>,
    proofs: &[StoredProof],
) -> Result<Vec<FluxSpan>, StorageFailure> {
    let indexed = proof_index(prepared, proofs)?;
    prepared
        .loops
        .iter()
        .flatten()
        .map(|piece| {
            let proof = select_proof(piece.span, &indexed)?;
            capture_span(prepared, piece, bindings, proof)
        })
        .collect()
}

fn select_proof<'a>(
    span: CurveSpan,
    proofs: &BTreeMap<CurveSpan, &'a StoredProof>,
) -> Result<&'a StoredProof, StorageFailure> {
    proofs.get(&span).copied().ok_or_else(|| {
        mismatch(
            key(span, HistoryRole::SideFace),
            "flux proof membership",
            vec![0],
            vec![1],
        )
    })
}

fn proof_index<'a>(
    prepared: &Request,
    proofs: &'a [StoredProof],
) -> Result<BTreeMap<CurveSpan, &'a StoredProof>, StorageFailure> {
    proof_count(prepared, proofs)?;
    let mut indexed = BTreeMap::new();
    for proof in proofs {
        if indexed.insert(proof.span, proof).is_some() {
            return Err(mismatch(
                key(proof.span, HistoryRole::SideFace),
                "flux proof membership",
                vec![2],
                vec![1],
            ));
        }
    }
    Ok(indexed)
}

fn proof_count(prepared: &Request, proofs: &[StoredProof]) -> Result<(), StorageFailure> {
    let expected = prepared.loops.iter().map(Vec::len).sum::<usize>();
    if proofs.len() != expected {
        return Err(mismatch(
            region_key(prepared.region),
            "flux proof count",
            vec![proofs.len() as u64],
            vec![expected as u64],
        ));
    }
    Ok(())
}

fn capture_span(
    prepared: &Request,
    piece: &AuthoredPiece,
    bindings: &Bindings<'_>,
    proof: &StoredProof,
) -> Result<FluxSpan, StorageFailure> {
    let curves = captured_span(piece, bindings)?;
    trims::side_trims(prepared, piece, bindings)?;
    let cap_pcurves = cap_poles(prepared, piece, bindings)?;
    let side_reversed = side_orientation(piece.span, bindings)?;
    let span = FluxSpan {
        curves,
        cap_pcurves,
        side_reversed,
    };
    bind_traces(prepared, piece.span, &span, proof)?;
    Ok(span)
}

fn cap_poles(
    prepared: &Request,
    piece: &AuthoredPiece,
    bindings: &Bindings<'_>,
) -> Result<[Vec<[f64; 2]>; 2], StorageFailure> {
    let caps = trims::cap_bindings(prepared, piece);
    let a = trims::pcurve(caps[0].0, caps[0].1, &caps[0].2, bindings)?;
    let b = trims::pcurve(caps[1].0, caps[1].1, &caps[1].2, bindings)?;
    Ok([a, b])
}

fn side_orientation(span: CurveSpan, bindings: &Bindings<'_>) -> Result<bool, StorageFailure> {
    use martensite_tkg3d::top_abs::Orientation;
    let key = key(span, HistoryRole::SideFace);
    let face = native(key, martensite_tkbrep::topods::face(lookup(key, bindings)?))?;
    match face.orientation() {
        Orientation::Forward => Ok(false),
        Orientation::Reversed => Ok(true),
        actual => Err(mismatch(
            key,
            "flux side orientation",
            vec![actual as u64],
            vec![Orientation::Forward as u64, Orientation::Reversed as u64],
        )),
    }
}

fn bind_traces(
    request: &Request,
    span: CurveSpan,
    actual: &FluxSpan,
    proof: &StoredProof,
) -> Result<(), StorageFailure> {
    for depth in 0..2 {
        let retained = &proof.trims.cap_residuals[depth];
        let query = trace_query(request, depth, actual, retained);
        if !retained.matches_source(query) {
            return Err(mismatch(
                key(span, HistoryRole::SideFace),
                "flux cap/source binding",
                vec![0],
                vec![1],
            ));
        }
    }
    Ok(())
}
fn trace_query<'a>(
    request: &Request,
    depth: usize,
    actual: &'a FluxSpan,
    retained: &CertifiedCylindricalTrim,
) -> CylindricalTrimQuery<'a> {
    CylindricalTrimQuery {
        curve: actual.curves[depth].borrowed(),
        pcurve: &actual.cap_pcurves[depth],
        radius: request.radius + request.depths[depth],
        spatial_limit: retained.requested_spatial_limit(),
        options: retained.requested_options(),
    }
}

pub(super) fn region_key(region: SurfaceRegionId) -> BindingKey {
    BindingKey {
        source: Source::Region(region),
        role: HistoryRole::Solid,
    }
}
