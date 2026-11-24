//! Bind decoded pcurves and full ranges to authored caps and ruled-side incidences.
use super::*;
use martensite_tkbrep::{brep::BRepTool, topods};
use martensite_tkernel::standard::Handle;
use martensite_tkg2d::geom2d::Geom2dBezierCurve;

#[cfg(test)]
#[path = "storage_trims_tests.rs"]
mod tests;

#[path = "storage_cap_residuals.rs"]
mod cap_residuals;

pub(in super::super) struct TrimEvidence {
    pub cap_residuals: [CertifiedCylindricalTrim; 2],
    pub cap_pcurves: usize,
    pub side_pcurves: usize,
    pub maximum_tolerance: f64,
}

pub(super) fn certify(
    request: &Request,
    piece: &AuthoredPiece,
    curves: [&OwnedCurve; 2],
    bindings: &Bindings<'_>,
    cancelled: &mut CancelPoll<'_>,
) -> Result<TrimEvidence, StorageFailure> {
    let caps = cap_residuals::certify(request, piece, curves, bindings, cancelled)?;
    let (side_pcurves, side_tolerance) = side_trims(request, piece, bindings)?;
    let maximum_tolerance = caps.maximum_tolerance.max(side_tolerance);
    Ok(TrimEvidence {
        cap_pcurves: caps.proofs.len(),
        side_pcurves,
        maximum_tolerance,
        cap_residuals: caps.proofs,
    })
}

pub(in crate::surface_sketch::wrapped_profile) fn side_trims(
    request: &DevelopedRegionRequest,
    piece: &AuthoredPiece,
    bindings: &Bindings<'_>,
) -> Result<(usize, f64), StorageFailure> {
    let side_pcurves = side_bindings(piece);
    let mut maximum_tolerance: f64 = 0.;
    for (edge, face, poles) in &side_pcurves {
        let tolerance = check_binding(*edge, *face, poles, request.tolerance / 2., bindings)?;
        maximum_tolerance = maximum_tolerance.max(tolerance);
    }
    Ok((side_pcurves.len(), maximum_tolerance))
}

type TrimBinding = (BindingKey, BindingKey, Vec<[f64; 2]>);

fn check_binding(
    edge: BindingKey,
    face: BindingKey,
    poles: &[[f64; 2]],
    maximum: f64,
    bindings: &Bindings<'_>,
) -> Result<f64, StorageFailure> {
    pcurve(edge, face, poles, bindings)
        .and_then(|_| topology_tolerances(edge, face, maximum, bindings))
        .map_err(|cause| trim_failure(edge, face, cause))
}

fn trim_failure(edge: BindingKey, face: BindingKey, cause: StorageFailure) -> StorageFailure {
    StorageFailure {
        key: edge,
        cause: StorageCause::Trim {
            face,
            cause: Box::new(cause),
        },
    }
}

pub(super) fn cap_bindings(request: &Request, piece: &AuthoredPiece) -> [TrimBinding; 2] {
    let poles: Vec<_> = cap_coordinates::angular_controls(&piece.poles, request.radius);
    let roles = [
        (HistoryRole::FirstDepthEdge, HistoryRole::FirstDepthFace),
        (HistoryRole::LastDepthEdge, HistoryRole::LastDepthFace),
    ];
    roles.map(|(edge, face)| {
        (
            key(piece.span, edge),
            BindingKey {
                source: Source::Region(request.region),
                role: face,
            },
            poles.clone(),
        )
    })
}

fn side_bindings(piece: &AuthoredPiece) -> [TrimBinding; 4] {
    let face = key(piece.span, HistoryRole::SideFace);
    let connector = |source| BindingKey {
        source: Source::Corner(source),
        role: HistoryRole::NormalEdge,
    };
    [
        (
            key(piece.span, HistoryRole::FirstDepthEdge),
            face,
            vec![[0., 0.], [1., 0.]],
        ),
        (
            key(piece.span, HistoryRole::LastDepthEdge),
            face,
            vec![[0., 1.], [1., 1.]],
        ),
        (connector(piece.start), face, vec![[0., 0.], [0., 1.]]),
        (connector(piece.end), face, vec![[1., 0.], [1., 1.]]),
    ]
}

pub(in crate::surface_sketch::wrapped_profile) fn pcurve(
    edge_key: BindingKey,
    face_key: BindingKey,
    expected: &[[f64; 2]],
    bindings: &Bindings<'_>,
) -> Result<Vec<[f64; 2]>, StorageFailure> {
    let edge = native(edge_key, topods::edge(lookup(edge_key, bindings)?))?;
    let face = native(face_key, topods::face(lookup(face_key, bindings)?))?;
    let captured = native(edge_key, BRepTool::curve_on_face(&edge, &face))?;
    pcurve_domain(
        edge_key,
        [captured.first, captured.last],
        captured.is_stored,
    )?;
    let curve: Handle<Geom2dBezierCurve> = captured.pcurve.downcast();
    let curve = retained_bezier_pcurve(edge_key, &curve)?;
    coefficients(edge_key, curve, expected)
}

fn pcurve_domain(key: BindingKey, range: [f64; 2], stored: bool) -> Result<(), StorageFailure> {
    let actual = [range[0].to_bits(), range[1].to_bits(), u64::from(stored)];
    let domain = [0f64.to_bits(), 1f64.to_bits(), 1];
    if actual != domain {
        return Err(mismatch(
            key,
            "retained pcurve range/storage",
            actual.to_vec(),
            domain.to_vec(),
        ));
    }
    Ok(())
}

fn retained_bezier_pcurve(
    key: BindingKey,
    curve: &Handle<Geom2dBezierCurve>,
) -> Result<&Geom2dBezierCurve, StorageFailure> {
    curve.get().ok_or_else(|| {
        mismatch(
            key,
            "retained polynomial Bezier pcurve class",
            vec![0],
            vec![1],
        )
    })
}

fn coefficients(
    key: BindingKey,
    curve: &Geom2dBezierCurve,
    expected: &[[f64; 2]],
) -> Result<Vec<[f64; 2]>, StorageFailure> {
    let poles = curve.poles();
    let points: Vec<_> = poles.iter().map(|p| [p.x(), p.y()]).collect();
    let mut actual: Vec<_> = points.iter().flatten().map(|v| v.to_bits()).collect();
    let mut expected_bits: Vec<_> = expected.iter().flatten().map(|v| v.to_bits()).collect();
    for index in 1..=poles.length() {
        actual.push(native(key, curve.weight(index))?.to_bits());
    }
    expected_bits.extend(std::iter::repeat_n(1f64.to_bits(), expected.len()));
    if actual != expected_bits {
        return Err(mismatch(
            key,
            "retained pcurve coefficients/weights",
            actual,
            expected_bits,
        ));
    }
    Ok(points)
}

pub(in crate::surface_sketch::wrapped_profile) fn topology_tolerances(
    edge_key: BindingKey,
    face_key: BindingKey,
    maximum: f64,
    bindings: &Bindings<'_>,
) -> Result<f64, StorageFailure> {
    let edge = native(edge_key, topods::edge(lookup(edge_key, bindings)?))?;
    let face = native(face_key, topods::face(lookup(face_key, bindings)?))?;
    raw_tolerances(edge_key, face_key, &edge, &face, maximum)?;
    let a = native(edge_key, BRepTool::edge_tolerance(&edge))?;
    let b = native(face_key, BRepTool::face_tolerance(&face))?;
    tolerance(edge_key, a, maximum)?;
    tolerance(face_key, b, maximum)?;
    Ok(a.max(b))
}

fn raw_tolerances(
    edge_key: BindingKey,
    face_key: BindingKey,
    edge: &topods::Edge,
    face: &topods::Face,
    maximum: f64,
) -> Result<(), StorageFailure> {
    use martensite_tkbrep::brep::access::{edge_tshape, face_tshape};
    let edge = native(edge_key, edge_tshape(edge, "wrapped trim edge tolerance"))?;
    let face = native(face_key, face_tshape(face, "wrapped trim face tolerance"))?;
    // The legacy tool floor can mask NaN and negative storage values; inspect storage first.
    tolerance(edge_key, edge.tolerance(), maximum)?;
    tolerance(face_key, face.tolerance(), maximum)
}

fn tolerance(key: BindingKey, actual: f64, maximum: f64) -> Result<(), StorageFailure> {
    if !actual.is_finite() || actual < 0. || actual > maximum {
        return Err(tolerance_failure(key, actual, maximum));
    }
    Ok(())
}

fn tolerance_failure(key: BindingKey, actual: f64, maximum: f64) -> StorageFailure {
    StorageFailure {
        key,
        cause: StorageCause::Tolerance { actual, maximum },
    }
}
