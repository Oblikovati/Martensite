//! Quantify the decoded cap traces only after coefficient/domain and frame admission.
use super::*;

pub(super) struct CapEvidence {
    pub proofs: [CertifiedCylindricalTrim; 2],
    pub maximum_tolerance: f64,
}

pub(super) fn certify(
    request: &Request,
    piece: &AuthoredPiece,
    curves: [&OwnedCurve; 2],
    bindings: &Bindings<'_>,
) -> Result<CapEvidence, StorageFailure> {
    let caps = cap_bindings(request, piece);
    let radii = request.depths.map(|depth| request.radius + depth);
    let maximum = request.tolerance / 2.;
    let a = certify_cap(&caps[0], curves[0], radii[0], maximum, bindings)?;
    let b = certify_cap(&caps[1], curves[1], radii[1], maximum, bindings)?;
    Ok(CapEvidence {
        maximum_tolerance: a.1.max(b.1),
        proofs: [a.0, b.0],
    })
}

fn certify_cap(
    binding: &TrimBinding,
    curve: &OwnedCurve,
    radius: f64,
    maximum: f64,
    bindings: &Bindings<'_>,
) -> Result<(CertifiedCylindricalTrim, f64), StorageFailure> {
    let (edge, face, poles) = binding;
    let tolerance = check_binding(*edge, *face, poles, maximum, bindings)?;
    let limit = cap_limit(*edge, *face, bindings)?;
    // The preceding bit comparison establishes these controls equal the decoded pcurve.
    let query = trace_query(curve, poles, radius, limit);
    let proof = certify_cylindrical_trim(query, || false)
        .map_err(|cause| trim_failure(*edge, *face, residual_failure(*edge, cause)))?;
    Ok((proof, tolerance))
}

fn trace_query<'a>(
    curve: &'a OwnedCurve,
    poles: &'a [[f64; 2]],
    radius: f64,
    limit: f64,
) -> CylindricalTrimQuery<'a> {
    CylindricalTrimQuery {
        curve: curve.borrowed(),
        pcurve: poles,
        radius,
        spatial_limit: limit,
        options: Default::default(),
    }
}

fn cap_limit(
    edge_key: BindingKey,
    face_key: BindingKey,
    bindings: &Bindings<'_>,
) -> Result<f64, StorageFailure> {
    let edge = native(edge_key, topods::edge(lookup(edge_key, bindings)?))?;
    let face = native(face_key, topods::face(lookup(face_key, bindings)?))?;
    let a = native(edge_key, BRepTool::edge_tolerance(&edge))?;
    let b = native(face_key, BRepTool::face_tolerance(&face))?;
    Ok(a.min(b))
}

fn residual_failure(key: BindingKey, cause: CylindricalTrimError) -> StorageFailure {
    StorageFailure {
        key,
        cause: StorageCause::CapResidual(Box::new(cause)),
    }
}
