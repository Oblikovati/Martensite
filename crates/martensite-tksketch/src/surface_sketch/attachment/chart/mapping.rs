//! One detached selected face supplies the radius and its native trim search.
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
    let attempt = Attempt::new(
        AttachmentOperation::Develop,
        attachment.record.id,
        attachment.record.revision,
    );
    attempt.revision(revision)?;
    admit(&attempt, request)?;
    let support = codec::decode(&attempt, &attachment.record.brep)?;
    let radius = radius(&attempt, &support.face)?;
    let tolerance = attachment.record.resolution.sew;
    let trim_state = classify(
        &attempt,
        &support.face,
        tolerance,
        request.uv,
        request.outside,
    )?;
    report(&attempt, request, radius, trim_state)
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

fn admit(attempt: &Attempt, request: CylinderParameterRequest) -> Checked<()> {
    for (field, value) in [
        ("native u", request.uv[0]),
        ("native v", request.uv[1]),
        ("seam angle", request.seam_angle),
    ] {
        finite(attempt, field, value)?;
    }
    Ok(())
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
