//! Publication replays every immutable source binding without substituting a topology count.
use super::*;
pub(super) fn spans<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &Prepared,
    proofs: &'a [storage::SpanProof],
) -> ConeChecked<Vec<ConicalMaterialSpan<'a>>> {
    proofs
        .iter()
        .map(|proof| span(original, prepared, proof))
        .collect()
}
fn span<'a>(
    original: &'a DevelopedRegionRequest,
    prepared: &Prepared,
    proof: &'a storage::SpanProof,
) -> ConeChecked<ConicalMaterialSpan<'a>> {
    let source = original_piece(original, proof.span)?;
    retained_fit(prepared, proof)?;
    let interval = CylindricalProfileInterval::new(u64::from(proof.span.index), proof.span.level)
        .map_err(|_| {
        mismatch(
            original.region,
            Some(proof.span),
            "filled-material dyadic source interval",
        )
    })?;
    Ok(ConicalMaterialSpan {
        original: &source.poles,
        interval,
        uv: &proof.pcurves[0],
    })
}
fn original_piece(
    original: &DevelopedRegionRequest,
    span: CurveSpan,
) -> ConeChecked<&AuthoredPiece> {
    original
        .loops
        .iter()
        .flatten()
        .find(|piece| piece.span.curve == span.curve)
        .ok_or_else(|| {
            mismatch(
                original.region,
                Some(span),
                "filled-material original span owner",
            )
        })
}
fn retained_fit(prepared: &Prepared, proof: &storage::SpanProof) -> ConeChecked<()> {
    let region = prepared.source.region;
    let fit = prepared
        .fits
        .get(&proof.span)
        .ok_or_else(|| mismatch(region, Some(proof.span), "filled-material fit owner"))?;
    if !proof.pcurves[0].iter().eq(&fit.uv) {
        return Err(mismatch(
            region,
            Some(proof.span),
            "filled-material retained fit UV",
        ));
    }
    Ok(())
}

pub(super) fn query<'a>(
    prepared: &Prepared,
    carriers: [ConicalTrimCarrier; 2],
    options: ConeClearanceOptions,
    spans: &'a [ConicalMaterialSpan<'a>],
) -> ConicalMaterialQuery<'a> {
    ConicalMaterialQuery {
        spans,
        carriers,
        reference_slant: prepared.reference_slant,
        chart_origin: prepared.chart_origin,
        depths: prepared.source.depths,
        spatial_limit: prepared.source.tolerance,
        options,
    }
}
pub(super) fn validate(
    prepared: &Prepared,
    proofs: &[storage::SpanProof],
    material: &MaterialEvidence,
) -> ConeChecked<()> {
    validate_domain(prepared, proofs, material)?;
    let spans = spans(&material.original, prepared, proofs)?;
    let query = query(prepared, material.carriers, material.options, &spans);
    if !developed::matches(&query, prepared, proofs, material)? {
        return Err(mismatch(
            prepared.source.region,
            None,
            "filled normal-map source binding",
        ));
    }
    validate_trims(prepared.source.region, proofs, material.carriers)
}
fn validate_domain(
    prepared: &Prepared,
    proofs: &[storage::SpanProof],
    material: &MaterialEvidence,
) -> ConeChecked<()> {
    selected_version(
        prepared.source.region,
        material.requested_planar_proof,
        material.planar_proof,
    )?;
    validate_original_program_policy(prepared.source.region, material)?;
    let uv = domain::request(prepared, proofs)?;
    if !material.uv.matches_request(&uv) {
        return Err(mismatch(
            prepared.source.region,
            None,
            "filled actual UV material source binding",
        ));
    }
    Ok(())
}

fn validate_trims(
    region: SurfaceRegionId,
    proofs: &[storage::SpanProof],
    carriers: [ConicalTrimCarrier; 2],
) -> ConeChecked<()> {
    for proof in proofs {
        trims(region, proof, carriers)?;
    }
    Ok(())
}
fn trims(
    region: SurfaceRegionId,
    proof: &storage::SpanProof,
    carriers: [ConicalTrimCarrier; 2],
) -> ConeChecked<()> {
    for (cap, carrier) in carriers.iter().enumerate() {
        let query = ConicalTrimQuery {
            curve: proof.curves[cap].borrowed(),
            pcurve: &proof.pcurves[cap],
            carrier: *carrier,
            spatial_limit: proof.trims[cap].requested_spatial_limit(),
            options: proof.trims[cap].requested_options(),
        };
        if !proof.trims[cap].matches_source(query) {
            return Err(mismatch(
                region,
                Some(proof.span),
                "filled actual carrier/trim source binding",
            ));
        }
    }
    Ok(())
}

fn validate_original_program_policy(
    region: SurfaceRegionId,
    material: &MaterialEvidence,
) -> ConeChecked<()> {
    if let Some(proof) = material.uv.integral_program_evidence()
        && proof.requested_proof_version() != material.requested_planar_proof
    {
        return Err(mismatch(
            region,
            None,
            "retained original integral program policy",
        ));
    }
    Ok(())
}
