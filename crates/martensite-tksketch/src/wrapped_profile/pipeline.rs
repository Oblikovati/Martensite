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

use martensite_tkmath::predicates::roots::metric_bounds::{
    CertifiedCylindricalFlux, CertifiedCylindricalMeasureComparison,
};

type NativeMeasure = (ValidationReport, (f64, f64));
type SourceCertificates = (
    planar::PlanarEvidence,
    planar_cones::ConeEvidence,
    planar_clearance::OriginalClearance,
    MeasureEvidence,
    coverage::CoverageEvidence,
);
type FrozenParts = (Vec<u8>, Vec<BindingKey>, Shape, Vec<(BindingKey, Shape)>);

pub(super) struct MeasureEvidence {
    pub comparison: CertifiedCylindricalMeasureComparison,
    pub spans: Vec<CurveSpan>,
}

pub(super) struct Unqualified {
    pub bytes: Vec<u8>,
    pub coverage: coverage::CoverageEvidence,
    pub measures: MeasureEvidence,
    pub keys: Vec<BindingKey>,
    pub report: ValidationReport,
    pub volume_estimate: f64,
    pub integration_estimate: f64,
    pub storage: Vec<storage::StoredProof>,
    pub planar: planar::PlanarEvidence,
    pub cones: planar_cones::ConeEvidence,
    pub clearance: planar_clearance::OriginalClearance,
}

fn at<T>(region: SurfaceRegionId, stage: Stage, result: Result<T, Failure>) -> Result<T, Refusal> {
    result.map_err(|cause| Refusal {
        stage,
        region,
        cause: RefusalCause::Native(Box::new(cause)),
    })
}

pub(super) fn wrap_extrude(request: &Request) -> Result<Unqualified, Refusal> {
    at(request.region, Stage::Admission, admit(request))?;
    let planar = planar_stage(request)?;
    let prepared = planning_stage(request)?;
    let clearance = clearance_stage(request, &prepared)?;
    let (bytes, keys, decoded, bindings) = construct_frozen(&prepared)?;
    let coverage = coverage_stage(&prepared, &decoded, &bindings)?;
    let storage = storage_stage(request.region, request, &prepared, &bindings)?;
    bind_clearance(&clearance, request, &prepared, &storage)?;
    let cones = cone_stage(request, &prepared, &storage)?;
    let measures = measure_stage(request, &prepared, &bindings, &storage)?;
    let validation = checked_measure(request, &decoded)?;
    let certificates = (planar, cones, clearance, measures, coverage);
    Ok(unqualified(
        (bytes, keys),
        validation,
        storage,
        certificates,
    ))
}

fn coverage_stage(
    request: &Request,
    root: &Shape,
    bindings: &[(BindingKey, Shape)],
) -> Result<coverage::CoverageEvidence, Refusal> {
    coverage::certify(request, root, bindings).map_err(|cause| Refusal {
        stage: Stage::Coverage,
        region: request.region,
        cause: RefusalCause::Coverage(Box::new(cause)),
    })
}

fn construct_frozen(request: &Request) -> Result<FrozenParts, Refusal> {
    let constructed = at(
        request.region,
        Stage::Construction,
        native::construct(request),
    )?;
    frozen_parts(request.region, &constructed)
}

fn frozen_parts(
    region: SurfaceRegionId,
    constructed: &Constructed,
) -> Result<FrozenParts, Refusal> {
    let bytes = at(region, Stage::Freeze, freeze(constructed))?;
    let keys: Vec<_> = constructed.bindings.iter().map(|(key, _)| *key).collect();
    let (shape, bindings) = at(region, Stage::Freeze, decode(&bytes, &keys))?;
    Ok((bytes, keys, shape, bindings))
}

fn clearance_stage(
    original: &Request,
    prepared: &Request,
) -> Result<planar_clearance::OriginalClearance, Refusal> {
    planar_clearance::certify(original, prepared).map_err(|cause| Refusal {
        stage: Stage::Clearance,
        region: original.region,
        cause: RefusalCause::Clearance(Box::new(cause)),
    })
}

fn bind_clearance(
    evidence: &planar_clearance::OriginalClearance,
    original: &Request,
    prepared: &Request,
    stored: &[storage::StoredProof],
) -> Result<(), Refusal> {
    planar_clearance::bind(evidence, original, prepared, stored).map_err(|cause| Refusal {
        stage: Stage::Clearance,
        region: original.region,
        cause: RefusalCause::Clearance(Box::new(cause)),
    })
}

fn measure_stage(
    original: &Request,
    prepared: &Request,
    bindings: &[(BindingKey, Shape)],
    proofs: &[storage::StoredProof],
) -> Result<MeasureEvidence, Refusal> {
    let flux = flux_stage(original, prepared, bindings, proofs)?;
    let normal = normal_at(
        original.region,
        normal_measure::certify(original, prepared, proofs),
    )?;
    normal_at(
        original.region,
        normal_measure::compare(original, prepared, bindings, proofs, (flux, normal)),
    )
}
fn normal_at<T>(
    region: SurfaceRegionId,
    result: Result<T, normal_measure::NormalMeasureFailure>,
) -> Result<T, Refusal> {
    result.map_err(|cause| Refusal {
        stage: Stage::Measure,
        region,
        cause: RefusalCause::NormalMeasure(Box::new(cause)),
    })
}

fn flux_stage(
    original: &Request,
    prepared: &Request,
    bindings: &[(BindingKey, Shape)],
    proofs: &[storage::StoredProof],
) -> Result<CertifiedCylindricalFlux, Refusal> {
    storage::certify_flux(original, prepared, bindings, proofs).map_err(|cause| Refusal {
        stage: Stage::Measure,
        region: original.region,
        cause: RefusalCause::Storage(Box::new(cause)),
    })
}

fn checked_measure(request: &Request, shape: &Shape) -> Result<NativeMeasure, Refusal> {
    let report = at(
        request.region,
        Stage::Validation,
        ordered_checks(shape, request.tolerance),
    )?;
    let measure = at(request.region, Stage::Measure, estimate(shape))?;
    Ok((report, measure))
}

fn cone_stage(
    original: &Request,
    prepared: &Request,
    stored: &[storage::StoredProof],
) -> Result<planar_cones::ConeEvidence, Refusal> {
    planar_cones::certify(original, prepared, stored).map_err(|cause| Refusal {
        stage: Stage::Cones,
        region: original.region,
        cause: RefusalCause::Cones(Box::new(cause)),
    })
}

fn planar_stage(request: &Request) -> Result<planar::PlanarEvidence, Refusal> {
    planar::certify(request).map_err(|cause| Refusal {
        stage: Stage::Planar,
        region: request.region,
        cause: RefusalCause::Planar(Box::new(cause)),
    })
}

fn planning_stage(request: &Request) -> Result<Request, Refusal> {
    subdivision::prepare(request).map_err(|cause| Refusal {
        stage: Stage::Planning,
        region: request.region,
        cause: RefusalCause::Planning(Box::new(cause)),
    })
}

fn storage_stage(
    region: SurfaceRegionId,
    original: &Request,
    request: &Request,
    bindings: &[(BindingKey, Shape)],
) -> Result<Vec<storage::StoredProof>, Refusal> {
    storage::certify(original, request, bindings).map_err(|cause| Refusal {
        stage: Stage::Storage,
        region,
        cause: RefusalCause::Storage(Box::new(cause)),
    })
}

fn unqualified(
    (bytes, keys): (Vec<u8>, Vec<BindingKey>),
    (report, measure): NativeMeasure,
    storage: Vec<storage::StoredProof>,
    (planar, cones, clearance, measures, coverage): SourceCertificates,
) -> Unqualified {
    Unqualified {
        measures,
        coverage,
        bytes,
        keys,
        report,
        volume_estimate: measure.0,
        integration_estimate: measure.1,
        storage,
        planar,
        cones,
        clearance,
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

pub(super) fn resolution(tolerance: f64) -> Resolution {
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

pub(super) fn admit(request: &Request) -> Result<(), Failure> {
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
        || request.max_fit_level > 8;
    if invalid {
        return Err(Failure::new(&format!(
            "radius/depths/tolerance={values:?}, loops={}, fit level={}; expected positive radii, ordered depths, positive tolerance, contours and level <= 8",
            request.loops.len(),
            request.max_fit_level
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
            .all(|[s, z]| s.is_finite() && z.is_finite() && (s / radius).is_finite());
    if !valid {
        return Err(Failure::new(&format!(
            "curve {:?} has poles {:?}; expected 2..=4 finite poles with finite s/radius",
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
