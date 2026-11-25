//! The independently decoded selected face supplies the radius and native trim search.
use super::*;
use martensite_tkbrep::brep_adaptor::BRepAdaptorSurface;
use martensite_tkg3d::adaptor3d::Adaptor3dSurface;
use martensite_tkmath::gp::Point2;
use martensite_tktopalgo::brep_class::BRepClassFaceClassifier;

pub(super) fn apply(
    attachment: &CylinderFaceAttachment,
    revision: SupportRevision,
    request: CylinderParameterRequest,
) -> Checked<DevelopedCylinderHit> {
    let attempt = current_attempt(attachment, revision)?;
    admit_uv(&attempt, request.uv)?;
    finite(&attempt, "seam angle", request.seam_angle)?;
    let support = codec::decode(&attempt, &attachment.record.brep)?;
    let radius = radius(&attempt, &support.face)?;
    map_face(
        &attempt,
        &support.face,
        radius,
        attachment.record.resolution.sew,
        request,
    )
}

pub(super) fn open_chart(
    attachment: &CylinderFaceAttachment,
    revision: SupportRevision,
    seam_angle: f64,
) -> Checked<CylinderAuthoringChart> {
    let attempt = current_attempt(attachment, revision)?;
    finite(&attempt, "seam angle", seam_angle)?;
    let selected = codec::decode(&attempt, &attachment.record.brep)?.face;
    Ok(CylinderAuthoringChart {
        attempt,
        radius: radius(&attempt, &selected)?,
        face: selected,
        seam_angle,
        tolerance: attachment.record.resolution.sew,
    })
}

pub(super) fn apply_chart(
    chart: &CylinderAuthoringChart,
    revision: SupportRevision,
    uv: [f64; 2],
    outside: OutOfSupportPolicy,
) -> Checked<DevelopedCylinderHit> {
    chart.attempt.revision(revision)?;
    admit_uv(&chart.attempt, uv)?;
    let request = CylinderParameterRequest {
        uv,
        seam_angle: chart.seam_angle,
        outside,
    };
    map_face(
        &chart.attempt,
        &chart.face,
        chart.radius,
        chart.tolerance,
        request,
    )
}

fn current_attempt(
    attachment: &CylinderFaceAttachment,
    revision: SupportRevision,
) -> Checked<Attempt> {
    let attempt = Attempt::new(
        AttachmentOperation::Develop,
        attachment.record.id,
        attachment.record.revision,
    );
    attempt.revision(revision)?;
    Ok(attempt)
}

fn map_face(
    attempt: &Attempt,
    face: &Face,
    radius: f64,
    tolerance: f64,
    request: CylinderParameterRequest,
) -> Checked<DevelopedCylinderHit> {
    let trim_state = classify(attempt, face, tolerance, request.uv, request.outside)?;
    report(attempt, request, radius, trim_state)
}
fn report(
    attempt: &Attempt,
    request: CylinderParameterRequest,
    radius: f64,
    trim_state: State,
) -> Checked<DevelopedCylinderHit> {
    let s = arithmetic::arc(attempt, radius, request.uv[0], request.seam_angle)?;
    Ok(DevelopedCylinderHit {
        uv: request.uv,
        coordinates: [s, arithmetic::axial(request.uv[1])],
        trim_state,
        trim_check: CheckMode::NativeSearch,
    })
}

fn admit_uv(attempt: &Attempt, uv: [f64; 2]) -> Checked<()> {
    finite(attempt, "native u", uv[0])?;
    finite(attempt, "native v", uv[1])
}
fn finite(attempt: &Attempt, field: &'static str, value: f64) -> Checked<()> {
    if !value.is_finite() {
        return Err(attempt.reject(AttachmentFailureCause::ChartValue { field, value }));
    }
    Ok(())
}
fn radius(attempt: &Attempt, face: &Face) -> Checked<f64> {
    let adaptor = attempt.native(
        "selected cylinder adaptor",
        BRepAdaptorSurface::with_face(face, false),
    )?;
    let cylinder = attempt.native("selected cylinder radius", adaptor.cylinder())?;
    Ok(cylinder.radius())
}
pub(in crate::surface_sketch::attachment) fn classify(
    attempt: &Attempt,
    face: &Face,
    tolerance: f64,
    uv: [f64; 2],
    outside: OutOfSupportPolicy,
) -> Checked<State> {
    let uv = Point2::new(uv[0], uv[1]);
    let classifier = attempt.native(
        "selected face trim classification",
        BRepClassFaceClassifier::with_face_point2d(face, &uv, tolerance, false, tolerance),
    )?;
    let state = classifier.state();
    if classifier.rejected()
        || state == State::Unknown
        || (state == State::Out && outside == OutOfSupportPolicy::Refuse)
    {
        return Err(attempt.reject(AttachmentFailureCause::ChartTrim { state }));
    }
    Ok(state)
}

#[cfg(test)]
mod tests;
