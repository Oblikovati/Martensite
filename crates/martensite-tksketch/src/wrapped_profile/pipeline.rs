//! Freeze once; native evidence cannot satisfy the missing publication certificates.
use super::*;
use crate::surface_sketch::encoding;
use martensite_tkbo::{
    body_check::{ValidationReport, validate_body},
    profile_extrusion::{DisplayTolerance, Resolution, ToleranceCap},
};
use martensite_tkgeombase::gprop::GProps;
use martensite_tktopalgo::brep_gprop::{BRepGProp, VolumeGKFlags};
use std::collections::BTreeSet;

pub(super) struct Unqualified {
    pub bytes: Vec<u8>,
    pub keys: Vec<BindingKey>,
    pub report: ValidationReport,
    pub volume_estimate: f64,
    pub integration_estimate: f64,
    pub storage: Vec<storage::StoredProof>,
}

fn at<T>(region: SurfaceRegionId, stage: Stage, result: Result<T, Failure>) -> Result<T, Refusal> {
    result.map_err(|cause| Refusal {
        stage,
        region,
        cause: RefusalCause::Native(Box::new(cause)),
    })
}

pub(super) fn wrap_extrude(request: &Request) -> Result<Unqualified, Refusal> {
    let region = request.region;
    at(region, Stage::Admission, admit(request))?;
    let prepared = subdivision::prepare(request);
    let constructed = at(region, Stage::Construction, native::construct(&prepared))?;
    let bytes = at(region, Stage::Freeze, freeze(&constructed))?;
    let keys: Vec<_> = constructed.bindings.iter().map(|(key, _)| *key).collect();
    let (decoded, bindings) = at(region, Stage::Freeze, decode(&bytes, &keys))?;
    let storage = storage_stage(region, &prepared, &bindings)?;
    let report = at(
        region,
        Stage::Validation,
        ordered_checks(&decoded, request.tolerance),
    )?;
    let measure = at(region, Stage::Measure, estimate(&decoded))?;
    Ok(unqualified(bytes, keys, report, measure, storage))
}

fn storage_stage(
    region: SurfaceRegionId,
    request: &Request,
    bindings: &[(BindingKey, Shape)],
) -> Result<Vec<storage::StoredProof>, Refusal> {
    storage::certify(request, bindings).map_err(|cause| Refusal {
        stage: Stage::Storage,
        region,
        cause: RefusalCause::Storage(Box::new(cause)),
    })
}

fn unqualified(
    bytes: Vec<u8>,
    keys: Vec<BindingKey>,
    report: ValidationReport,
    measure: (f64, f64),
    storage: Vec<storage::StoredProof>,
) -> Unqualified {
    Unqualified {
        bytes,
        keys,
        report,
        volume_estimate: measure.0,
        integration_estimate: measure.1,
        storage,
    }
}

fn ordered_checks(shape: &Shape, tolerance: f64) -> Result<ValidationReport, Failure> {
    validate_body(shape, &resolution(tolerance)).map_err(|e| {
        Failure::new(&format!(
            "{}={} must be finite and positive",
            e.field, e.value
        ))
    })
}

fn freeze(constructed: &Constructed) -> Result<Vec<u8>, Failure> {
    encoding::encode(&constructed.solid, &constructed.bindings)
        .map_err(|e| Failure::new(&format!("freeze failed: {e:?}")))
}

pub(super) fn decode(
    bytes: &[u8],
    keys: &[BindingKey],
) -> Result<(Shape, Vec<(BindingKey, Shape)>), Failure> {
    // #432: certificates must bind the stored support frame without legacy normalization.
    encoding::decode_with_policy(
        bytes,
        keys,
        martensite_tkbrep::bintools::SurfaceFrameReadPolicy::PreserveStoredV1,
    )
    .map_err(|e| Failure::new(&format!("decode failed: {e:?}")))
}

fn resolution(tolerance: f64) -> Resolution {
    Resolution::new(
        tolerance,
        1e-7,
        1e-9,
        DisplayTolerance {
            chordal: 1e-3,
            angular: 0.1,
        },
        ToleranceCap::Refuse(tolerance),
    )
}

fn estimate(shape: &Shape) -> Result<(f64, f64), Failure> {
    let mut props = GProps::new();
    let status =
        BRepGProp::volume_properties_gk(shape, &mut props, 1e-8, true, VolumeGKFlags::default())?;
    let value = props.mass();
    if !value.is_finite() || !status.is_finite() || status < 0. {
        return Err(Failure::new(&format!(
            "volume={value}, integration status={status}; expected finite value and nonnegative estimate"
        )));
    }
    Ok((value, status))
}

#[derive(Default)]
struct AdmissionIds {
    curves: BTreeSet<SurfaceCurveId>,
    vertices: BTreeSet<CornerSource>,
}

fn admit(request: &Request) -> Result<(), Failure> {
    admit_limits(request)?;
    let mut ids = AdmissionIds::default();
    for contour in &request.loops {
        admit_contour(contour, request.radius, &mut ids)?;
    }
    Ok(())
}

fn admit_limits(request: &Request) -> Result<(), Failure> {
    let values = request_values(request);
    let invalid = values.iter().any(|v| !v.is_finite())
        || request.radius <= 0.
        || request.radius + request.depths[0] <= 0.
        || request.depths[0] >= request.depths[1]
        || request.tolerance <= 0.
        || request.loops.is_empty()
        || request.fit_level > 8;
    if invalid {
        return Err(Failure::new(&format!(
            "radius/depths/tolerance={values:?}, loops={}, fit level={}; expected positive radii, ordered depths, positive tolerance, contours and level <= 8",
            request.loops.len(),
            request.fit_level
        )));
    }
    Ok(())
}

fn admit_contour(
    contour: &[AuthoredPiece],
    radius: f64,
    ids: &mut AdmissionIds,
) -> Result<(), Failure> {
    if contour.len() < 3 {
        return Err(Failure::new(&format!(
            "contour has {} pieces; expected at least 3",
            contour.len()
        )));
    }
    for (i, piece) in contour.iter().enumerate() {
        admit_piece(piece, radius)?;
        admit_join(piece, &contour[(i + 1) % contour.len()], ids)?;
    }
    Ok(())
}

fn admit_join(
    piece: &AuthoredPiece,
    next: &AuthoredPiece,
    ids: &mut AdmissionIds,
) -> Result<(), Failure> {
    if !ids.curves.insert(piece.span.curve)
        || !ids.vertices.insert(piece.start)
        || piece.end != next.start
        || piece.poles.last() != next.poles.first()
    {
        return Err(Failure::new(&format!(
            "curve {:?}, endpoints {:?}/{:?}; expected unique authored IDs and exact closed joins",
            piece.span.curve, piece.start, piece.end
        )));
    }
    Ok(())
}

fn admit_piece(piece: &AuthoredPiece, radius: f64) -> Result<(), Failure> {
    let valid = (2..=4).contains(&piece.poles.len())
        && piece
            .poles
            .iter()
            .all(|[s, z]| s.is_finite() && z.is_finite() && (s / radius).abs() <= 1.5);
    if !valid {
        return Err(Failure::new(&format!(
            "curve {:?} has poles {:?}; expected 2..=4 finite poles inside |s/radius| <= 1.5 chart",
            piece.span.curve, piece.poles
        )));
    }
    Ok(())
}

fn request_values(request: &Request) -> [f64; 4] {
    [
        request.radius,
        request.depths[0],
        request.depths[1],
        request.tolerance,
    ]
}
